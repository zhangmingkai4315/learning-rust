pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

pub enum RingBufferMode {
    // TODO: override old or stop writing new item
}

pub struct RingBuffer<T> {
    buffer: *mut T,
    capacity: isize,
    read_offset: isize,
    write_offset: isize,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        assert_ne!(capacity, 0);
        let align = std::mem::align_of::<T>();
        let element_size = std::mem::size_of::<T>();
        let layout = std::alloc::Layout::from_size_align(element_size * capacity, align)
            .expect("constructing memory fail");
        let ptr = unsafe { std::alloc::alloc(layout) } as *mut T;

        RingBuffer {
            capacity: capacity as isize,
            buffer: ptr,
            read_offset: 0,
            write_offset: 0,
        }
    }

    // fn overwrite(&mut self, element: T) {
    //     if self.is_full() {
    //         let _ = self.read().expect("Read failed on a full buffer.");
    //     }
    //     self.write(element)
    //         .expect("Write failed on a not full buffer.");
    // }

    fn is_empty(&self) -> bool {
        self.read_offset == self.write_offset
    }

    fn is_full(&self) -> bool {
        self.write_offset - self.read_offset == self.capacity
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            Err(Error::EmptyBuffer)
        } else {
            let value = unsafe {
                let read_ptr = self.buffer.offset(self.read_offset);
                std::ptr::read(read_ptr)
            };

            self.read_offset += 1;
            Ok(value)
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            Err(Error::FullBuffer)
        } else {
            unsafe {
                let write_ptr = self.buffer.offset(self.write_offset % self.capacity);
                std::ptr::write(write_ptr, element);
            }
            self.write_offset += 1;
            Ok(())
        }
    }

    fn realign(&mut self) {
        if self.read_offset >= self.capacity {
            self.read_offset -= self.capacity;
            self.write_offset -= self.capacity;
        }
    }

    pub fn clear(&mut self) {
        loop {
            match self.read() {
                Ok(_) => {}
                _ => break,
            }
        }
        self.read_offset = 0;
        self.write_offset = 0;
    }
}

#[cfg(test)]
mod test {
    use crate::ring_buffer::RingBuffer;

    #[test]
    fn test_rb() {
        let mut rb = RingBuffer::new(10);
        for i in 1..=10 {
            rb.write(i);
        }
        for i in 1..=10 {
            let val = rb.read().unwrap_or(0);
            assert_eq!(val, i);
        }

        let mut rb = RingBuffer::new(5);
        for i in 1..=10 {
            rb.write(i);
        }
    }
}
