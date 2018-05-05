use std::io::{Read, Result as IoResult};

use bytes::{Buf, BufMut};
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
        unsafe { self.deque.move_head(cnt as isize) }
    }
}

impl BufMut for RingBuf {
    #[inline]
    fn remaining_mut(&self) -> usize {
        usize::max_value() - self.deque.len()
    }

    #[inline]
    unsafe fn bytes_mut(&mut self) -> &mut [u8] {
        let len = self.deque.len();
        if self.deque.capacity() == len {
            self.deque.reserve(64); // Grow the deque
        }

        let cap = self.deque.capacity();

        let ptr = self.deque.as_mut_ptr();
        &mut ::std::slice::from_raw_parts_mut(ptr, cap)[len..]
    }

    #[inline]
    unsafe fn advance_mut(&mut self, cnt: usize) {
        let len = self.deque.len();
        let remaining = self.deque.capacity() - len;
        if cnt > remaining {
            // Reserve additional capacity, and ensure that the total length
            // will not overflow usize.
            self.deque.reserve(cnt);
        }

        self.deque.move_tail_unchecked(cnt as isize);
    }
}
