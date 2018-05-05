use std::io::{Read, Result as IoResult};

use bytes::Buf;
use slice_deque::SliceDeque;

pub struct RingBuf {
    deque: SliceDeque<u8>,
}

impl RingBuf {
    pub fn new() -> RingBuf {
        RingBuf {
            deque: SliceDeque::with_capacity(4096),
        }
    }

    pub fn len(&self) -> usize {
        self.deque.len()
    }

    pub fn read_from<R: Read>(&mut self, r: &mut R) -> IoResult<usize> {
        let pre_len = self.deque.len();
        self.deque.resize(pre_len + 4096, 0);
        match r.read(&mut self.deque.as_mut_slice()[pre_len..]) {
            Ok(len) => {
                self.deque.truncate(pre_len + len);
                Ok(len)
            }
            Err(err) => {
                self.deque.truncate(pre_len);
                Err(err)
            }
        }
    }

    pub fn read_from_exact<R: Read>(&mut self, r: &mut R, cnt: usize) -> IoResult<()> {
        let pre_len = self.deque.len();
        self.deque.resize(pre_len + cnt, 0);
        match r.read_exact(&mut self.deque.as_mut_slice()[pre_len..]) {
            Ok(()) => Ok(()),
            Err(err) => {
                self.deque.truncate(pre_len);
                Err(err)
            }
        }
    }
}

impl Buf for RingBuf {
    #[inline]
    fn remaining(&self) -> usize {
        self.deque.len()
    }

    #[inline]
    fn bytes(&self) -> &[u8] {
        self.deque.as_slice()
    }

    #[inline]
    fn advance(&mut self, cnt: usize) {
        if cnt > self.deque.len() {
            panic!("cannot advance beyond the end of the RingBuf");
        }
        // SAFETY: We've checked that `cnt` is within the current deque length,
        // and by virtue of it being an `usize`, we also know that it is not negative.
        // Therefore moving the deque head by `cnt` will not expose any uninitialized
        // memory.
        unsafe { self.deque.move_head(cnt as isize) }
    }
}
