use std::collections::VecDeque;

use bytes::{Buf, IntoBuf};
use iovec::IoVec;

pub struct BufVec<B> {
    remaining: usize,
    bufs: VecDeque<B>,
}

impl<B: Buf> BufVec<B> {
    pub fn new() -> Self {
        BufVec {
            remaining: 0,
            bufs: VecDeque::new(),
        }
    }

    pub fn push<T>(&mut self, value: T)
    where
        T: IntoBuf<Buf = B>,
    {
        let buf = value.into_buf();
        if buf.remaining() > 0 {
            self.remaining += buf.remaining();
            self.bufs.push_back(buf)
        }
    }
}

impl<B: Buf> Buf for BufVec<B> {
    fn remaining(&self) -> usize {
        self.remaining
    }

    fn bytes(&self) -> &[u8] {
        if let Some(buf) = self.bufs.front() {
            buf.bytes()
        } else {
            &[]
        }
    }

    fn advance(&mut self, mut cnt: usize) {
        self.remaining -= cnt;
        while cnt > 0 {
            let mut should_pop = false;
            if let Some(buf) = self.bufs.front_mut() {
                let rem = buf.remaining();
                let adv = ::std::cmp::min(cnt, rem);
                buf.advance(adv);
                cnt -= adv;
                if !buf.has_remaining() {
                    should_pop = true;
                }
            }
            if should_pop {
                self.bufs.pop_front();
            }
        }
    }

    fn bytes_vec<'a>(&'a self, dst: &mut [&'a IoVec]) -> usize {
        let mut dst_idx = 0;
        let mut buf_idx = 0;
        while dst_idx < dst.len() {
            if let Some(buf) = self.bufs.get(buf_idx) {
                dst_idx += buf.bytes_vec(&mut dst[dst_idx..]);
                buf_idx += 1;
            } else {
                break;
            }
        }
        dst_idx
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use bytes::{Buf, BufMut};
    use iovec::IoVec;

    use super::BufVec;

    quickcheck! {
        fn push_and_collect(chunks: Vec<Vec<u8>>) -> bool {
            let mut bv = BufVec::new();
            let mut bytes = Vec::new();
            for chunk in chunks {
                bytes.put_slice(&chunk);
                bv.push(Cursor::new(chunk));
            }
            let collected = bv.collect::<Vec<_>>();

            bytes == collected
        }
    }

    quickcheck! {
        fn push_and_concat_iovec(chunks: Vec<Vec<u8>>, lens: Vec<u8>) -> bool {
            let mut bv = BufVec::new();
            let mut bytes = Vec::new();
            for chunk in chunks {
                bytes.put_slice(&chunk);
                bv.push(Cursor::new(chunk));
            }

            let mut total = 0;
            let mut collected = Vec::new();
            for len in lens {
                let mut num = 0;
                {
                    let mut vecs = vec![IoVec::from_bytes(&[0u8]).unwrap(); len as usize];
                    let n = bv.bytes_vec(&mut vecs);
                    for vec in &vecs[..n] {
                        num += vec.len();
                        collected.put_slice(vec);
                    }
                }
                {
                    bv.advance(num);
                    total += num;
                }
            }

            bytes[..total] == collected[..]
        }
    }
}
