mod errors;

use errors::*;
use std::ptr;

#[derive(Debug)]
pub struct Bunch<T: Clone> {
    pub data: Box<[T]>,
    pub filled: usize,
}

impl<T: Clone + Default> Bunch<T> {
    pub fn new(size: usize) -> Bunch<T> {
        let v: Vec<T> = vec![T::default(); size];

        Bunch {
            data: v.into_boxed_slice(),
            filled: 0,
        }
    }

    pub fn get(&self, index: usize) -> Result<T, bunch_errors::OutOfBounds> {
        if index >= self.filled {
            return Err(bunch_errors::OutOfBounds::new(index, self.filled));
        }
        Ok(unsafe { self.data.get_unchecked(index) }.clone())
    }

    pub fn set(&mut self, index: usize, data: T) -> Result<(), bunch_errors::OutOfBounds> {
        if index >= self.filled {
            return Err(bunch_errors::OutOfBounds::new(index, self.filled));
        }

        let cell = unsafe { self.data.get_unchecked_mut(index) };
        *cell = data;

        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> Result<T, bunch_errors::OutOfBounds> {
        if index >= self.filled {
            return Err(bunch_errors::OutOfBounds::new(index, self.filled));
        }

        let to_return = unsafe { self.data.get_unchecked_mut(index) }.clone();

        unsafe {
            let dst_ptr = self.data.as_mut_ptr().offset(index as isize);
            let src_ptr = dst_ptr.offset(1);

            ptr::copy_nonoverlapping(src_ptr, dst_ptr, self.filled - index - 1);
        }

        self.filled -= 1;

        Ok(to_return)
    }

    // pub fn insert(
    //     &mut self,
    //     index: usize,
    //     data: T,
    // ) -> Result<Option<T>, bunch_errors::OutOfBounds> {
    //     if index > self.filled || index >= self.data.len() {
    //         return Err(bunch_errors::OutOfBounds::new(index, self.filled));
    //     }

    //     let to_return = if self.filled == self.data.len() {
    //         let ret = Some(unsafe { self.data.get_unchecked_mut(self.filled) }.clone());

    //         ret
    //     } else {
    //         None
    //     };
    // }

    pub fn pop(&mut self) -> Result<T, bunch_errors::BufferEmpty> {
        if self.filled == 0 {
            return Err(bunch_errors::BufferEmpty {});
        }

        self.filled -= 1;
        let to_return = self.data[self.filled].clone();

        Ok(to_return)
    }

    pub fn push(&mut self, data: T) -> Result<(), bunch_errors::BufferFilled> {
        if self.filled == self.data.len() {
            return Err(bunch_errors::BufferFilled {});
        }

        let cell = unsafe { self.data.get_unchecked_mut(self.filled) };
        *cell = data;
        self.filled += 1;

        Ok(())
    }
}

#[derive(Debug)]
pub struct BufferedVec<T: Clone> {
    parts: Vec<Bunch<T>>,
    buffer: Bunch<T>,
    max_buffer_size: usize,
}

impl<T: Clone> BufferedVec<T> {}
