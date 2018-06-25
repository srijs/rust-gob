use std::io::{Cursor, Write};

use bytes::Buf;
use iovec::IoVec;

use internal::gob::Message;
use internal::utils::BufVec;

use error::Error;

pub struct OutputPart {
    len_buf_len: u8,
    tag_buf_len: u8,
    len_buf: [u8; 9],
    tag_buf: [u8; 9],
    buf: Vec<u8>,
    pos: usize,
}

impl OutputPart {
    pub(crate) fn new(tag: i64, buf: Vec<u8>) -> Self {
        let mut len_buf = [0u8; 9];
        let mut tag_buf = [0u8; 9];
        let tag_buf_len = {
            let mut tag_msg = Message::new(Cursor::new(&mut tag_buf));
            tag_msg.write_int(tag);
            tag_msg.get_ref().position() as u8
        };
        let len_buf_len = {
            let mut len_msg = Message::new(Cursor::new(&mut len_buf));
            len_msg.write_uint(buf.len() as u64 + tag_buf_len as u64);
            len_msg.get_ref().position() as u8
        };

        OutputPart {
            len_buf_len,
            tag_buf_len,
            len_buf,
            tag_buf,
            buf,
            pos: 0,
        }
    }
}

impl Buf for OutputPart {
    fn remaining(&self) -> usize {
        ((self.len_buf_len + self.tag_buf_len) as usize + self.buf.len()) - self.pos
    }

    fn bytes(&self) -> &[u8] {
        let pre_buf_len = (self.len_buf_len + self.tag_buf_len) as usize;
        if self.pos < self.len_buf_len as usize {
            &self.len_buf[self.pos..self.len_buf_len as usize]
        } else if self.pos < pre_buf_len as usize {
            &self.tag_buf[self.pos - self.len_buf_len as usize..self.tag_buf_len as usize]
        } else {
            &self.buf[self.pos - pre_buf_len..]
        }
    }

    fn advance(&mut self, cnt: usize) {
        self.pos += cnt;
    }

    fn bytes_vec<'a>(&'a self, dst: &mut [&'a IoVec]) -> usize {
        let mut pos = self.pos;
        let mut idx = 0;
        let pre_buf_len = (self.len_buf_len + self.tag_buf_len) as usize;
        if idx < dst.len() && pos < self.len_buf_len as usize {
            dst[idx] = IoVec::from_bytes(&self.len_buf[pos..self.len_buf_len as usize]).unwrap();
            idx += 1;
            pos = self.len_buf_len as usize;
        }
        if idx < dst.len() && pos < pre_buf_len {
            dst[idx] = IoVec::from_bytes(
                &self.tag_buf[pos - self.len_buf_len as usize..self.tag_buf_len as usize],
            ).unwrap();
            idx += 1;
            pos = pre_buf_len;
        }
        if idx < dst.len() && pos < pre_buf_len + self.buf.len() {
            dst[idx] = IoVec::from_bytes(&self.buf[pos - pre_buf_len..]).unwrap();
            idx += 1;
        }
        idx
    }
}

pub trait Output {
    fn serialize_part(&mut self, part: OutputPart) -> Result<(), Error>;
}

impl<'a, O: Output> Output for &'a mut O {
    fn serialize_part(&mut self, part: OutputPart) -> Result<(), Error> {
        Output::serialize_part(*self, part)
    }
}

pub struct OutputBuffer {
    inner: BufVec<OutputPart>,
}

impl OutputBuffer {
    pub(crate) fn new() -> Self {
        OutputBuffer {
            inner: BufVec::new(),
        }
    }
}

impl Output for OutputBuffer {
    fn serialize_part(&mut self, part: OutputPart) -> Result<(), Error> {
        self.inner.push(part);
        Ok(())
    }
}

impl Buf for OutputBuffer {
    fn remaining(&self) -> usize {
        self.inner.remaining()
    }

    fn bytes(&self) -> &[u8] {
        self.inner.bytes()
    }

    fn advance(&mut self, cnt: usize) {
        self.inner.advance(cnt)
    }

    fn bytes_vec<'a>(&'a self, dst: &mut [&'a IoVec]) -> usize {
        self.inner.bytes_vec(dst)
    }
}

pub struct OutputWrite<W>(W);

impl<W: Write> OutputWrite<W> {
    pub(crate) fn new(w: W) -> Self {
        OutputWrite(w)
    }

    pub fn get_ref(&self) -> &W {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut W {
        &mut self.0
    }

    pub fn into_inner(self) -> W {
        self.0
    }
}

impl<W: Write> Output for OutputWrite<W> {
    fn serialize_part(&mut self, part: OutputPart) -> Result<(), Error> {
        ::std::io::copy(&mut part.reader(), &mut self.0)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use std::ops::Deref;

    use bytes::Buf;
    use iovec::IoVec;
    use partial_io::{GenNoErrors, PartialRead, PartialWithErrors};

    use internal::gob::Message;

    use super::OutputPart;

    #[test]
    fn part_collect() {
        let part = OutputPart::new(42, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(part.collect::<Vec<_>>(), vec![7, 84, 1, 2, 3, 4, 5, 6])
    }

    quickcheck! {
        fn part_bytes(tag: i64, buf: Vec<u8>, ops: PartialWithErrors<GenNoErrors>) -> bool {
            let mut tag_msg = Message::new(Vec::new());
            tag_msg.write_int(tag);

            let mut ref_msg = Message::new(Vec::new());
            ref_msg.write_uint((tag_msg.get_ref().len() + buf.len()) as u64);
            ref_msg.write_int(tag);
            ref_msg.get_mut().extend_from_slice(&buf);

            let part = OutputPart::new(tag, buf);
            let reader = part.reader();
            let mut partial_reader = PartialRead::new(reader, ops);
            let mut part_vec = Vec::new();
            partial_reader.read_to_end(&mut part_vec).unwrap();

            part_vec == *ref_msg.get_ref()
        }
    }

    #[test]
    fn part_bytes_vec() {
        let mut part = OutputPart::new(42, vec![1, 2, 3, 4, 5, 6]);

        {
            let mut vecs = vec![IoVec::from_bytes(&[0]).unwrap(); 3];
            let n = part.bytes_vec(vecs.as_mut_slice());
            assert_eq!(n, 3);
            assert_eq!(vecs[0].deref(), &[7]);
            assert_eq!(vecs[1].deref(), &[84]);
            assert_eq!(vecs[2].deref(), &[1, 2, 3, 4, 5, 6]);
        }

        part.advance(1);

        {
            let mut vecs = vec![IoVec::from_bytes(&[0]).unwrap(); 3];
            let n = part.bytes_vec(vecs.as_mut_slice());
            assert_eq!(n, 2);
            assert_eq!(vecs[0].deref(), &[84]);
            assert_eq!(vecs[1].deref(), &[1, 2, 3, 4, 5, 6]);
        }

        part.advance(1);

        {
            let mut vecs = vec![IoVec::from_bytes(&[0]).unwrap(); 3];
            let n = part.bytes_vec(vecs.as_mut_slice());
            assert_eq!(n, 1);
            assert_eq!(vecs[0].deref(), &[1, 2, 3, 4, 5, 6]);
        }

        part.advance(1);

        {
            let mut vecs = vec![IoVec::from_bytes(&[0]).unwrap(); 3];
            let n = part.bytes_vec(vecs.as_mut_slice());
            assert_eq!(n, 1);
            assert_eq!(vecs[0].deref(), &[2, 3, 4, 5, 6]);
        }
    }
}
