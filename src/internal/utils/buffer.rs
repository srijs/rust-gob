use std::io::{Read, Result as IoResult};

use bytes::Buf;

pub struct Buffer {
    bytes: Vec<u8>,
    offset: usize,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            bytes: Vec::with_capacity(4096),
            offset: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.bytes.len() - self.offset
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes.as_slice()[self.offset..]
    }

    fn trim(&mut self) {
        let off = self.offset;
        if off > 0 {
            let len = self.bytes.len();
            unsafe {
                let src = self.bytes.as_ptr().offset(off as isize);
                let dst = self.bytes.as_mut_ptr();
                ::std::ptr::copy(src, dst, len - off);
                self.bytes.set_len(len - off);
            }
            self.offset = 0;
        }
    }

    pub fn read_from<R: Read>(&mut self, r: &mut R) -> IoResult<usize> {
        self.trim();
        let pre_len = self.bytes.len();
        self.bytes.resize(pre_len + 4096, 0);
        match r.read(&mut self.bytes.as_mut_slice()[pre_len..]) {
            Ok(len) => {
                self.bytes.truncate(pre_len + len);
                Ok(len)
            }
            Err(err) => {
                self.bytes.truncate(pre_len);
                Err(err)
            }
        }
    }
}

impl Buf for Buffer {
    #[inline]
    fn remaining(&self) -> usize {
        self.len()
    }

    #[inline]
    fn bytes(&self) -> &[u8] {
        self.as_slice()
    }

    #[inline]
    fn advance(&mut self, cnt: usize) {
        let len = self.len();
        if cnt > len {
            panic!("cannot advance beyond the end of the RingBuf");
        }
        self.offset += cnt;
    }
}
