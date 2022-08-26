mod errors;

use errors::*;
use std::ptr;

#[derive(Debug)]
pub struct Bunch<T: Clone> {
    data: Box<[T]>,
    filled: usize,
}

impl<T: Clone + Default> Bunch<T> {
    pub fn new(size: usize) -> Bunch<T> {
        let v: Vec<T> = vec![T::default(); size];

        Bunch {
            data: v.into_boxed_slice(),
            filled: 0,
        }
    }

    pub fn filled(&self) -> usize {
        return self.filled;
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

            ptr::copy(src_ptr, dst_ptr, self.filled - index - 1);
        }

        self.filled -= 1;

        Ok(to_return)
    }

    pub fn insert(
        &mut self,
        index: usize,
        data: T,
    ) -> Result<Option<T>, bunch_errors::OutOfBounds> {
        if index > self.filled || index >= self.data.len() {
            return Err(bunch_errors::OutOfBounds::new(index, self.filled));
        }

        let old_filled = self.filled;
        let to_return = if self.filled == self.data.len() {
            let ret = Some(unsafe { self.data.get_unchecked_mut(self.filled) }.clone());

            ret
        } else {
            self.filled += 1;
            None
        };

        let cell = unsafe { self.data.get_unchecked_mut(index) };
        unsafe {
            let dst_ptr = (cell as *mut T).offset(1);
            let src_ptr = cell as *mut T;

            ptr::copy(src_ptr, dst_ptr, old_filled - index);
        }
        *cell = data;

        Ok(to_return)
    }

    pub fn pop(&mut self) -> Result<T, bunch_errors::BufferEmpty> {
        if self.filled == 0 {
            return Err(bunch_errors::BufferEmpty {});
        }

        self.filled -= 1;
        let to_return = unsafe { self.data.get_unchecked(self.filled) }.clone();

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

    pub fn append(&mut self, data: &[T]) -> Result<(), bunch_errors::CantFitSlice> {
        if data.len() > self.data.len() - self.filled {
            return Err(bunch_errors::CantFitSlice {});
        }

        unsafe {
            let dst_ptr = self.data.get_unchecked_mut(self.filled) as *mut T;
            let src_ptr = data.get_unchecked(0) as *const T;

            ptr::copy_nonoverlapping(src_ptr, dst_ptr, data.len());
        }

        self.filled += data.len();

        Ok(())
    }

    pub unsafe fn append_unchecked(&mut self, data: &[T]) {
        let dst_ptr = self.data.get_unchecked_mut(self.filled) as *mut T;
        let src_ptr = data.get_unchecked(0) as *const T;

        ptr::copy_nonoverlapping(src_ptr, dst_ptr, data.len());

        self.filled += data.len();
    }

    pub fn erase(&mut self) {
        self.filled = 0;
    }
}

#[derive(Debug)]
pub struct BufferedVec<T: Clone + Default> {
    parts: Vec<Bunch<T>>,
    buffer: Bunch<T>,
    max_buffer_size: usize,
    len: usize,
    last_non_empty: usize,
}

impl<T: Clone + Default> BufferedVec<T> {
    pub fn new(buffer_size: usize) -> BufferedVec<T> {
        BufferedVec {
            parts: Vec::new(),
            buffer: Bunch::<T>::new(buffer_size),
            max_buffer_size: buffer_size,
            len: 0,
            last_non_empty: 0,
        }
    }
    pub fn with_capacity(buffer_size: usize, capacity: usize) -> BufferedVec<T> {
        let mut parts_capacity = capacity / buffer_size;
        if capacity % buffer_size != 0 {
            parts_capacity += 1;
        }
        BufferedVec {
            parts: Vec::with_capacity(parts_capacity),
            buffer: Bunch::<T>::new(buffer_size),
            max_buffer_size: buffer_size,
            len: 0,
            last_non_empty: 0,
        }
    }

    fn add_new_bunch(&mut self) {
        self.parts.push(Bunch::<T>::new(self.max_buffer_size));
    }

    fn flush_buffer(&mut self) {
        if self.buffer.filled() == 0 {
            return;
        }

        if self.parts.len() == 0 {
            self.add_new_bunch();
        }

        let fbunch = unsafe { self.parts.get_unchecked_mut(self.last_non_empty) };
        if fbunch.filled > 0 {
            let sep_index = self.max_buffer_size - fbunch.filled;
            unsafe { fbunch.append_unchecked(&self.buffer.data[0..sep_index]) };

            let sbunch_index = self.parts.len();
            if self.parts.len() - 1 == self.last_non_empty {
                self.add_new_bunch();
                let sbunch = unsafe { self.parts.get_unchecked_mut(sbunch_index) };
                unsafe { sbunch.append_unchecked(&self.buffer.data[sep_index..]) };
            } else {
                let sbunch = unsafe { self.parts.get_unchecked_mut(self.last_non_empty + 1) };
                unsafe { sbunch.append_unchecked(&self.buffer.data[sep_index..]) };
            }
            self.last_non_empty += 1;
        } else {
            unsafe { fbunch.append_unchecked(&self.buffer.data) };
        }

        self.buffer.erase();
    }

    pub fn push(&mut self, value: T) {
        if self.buffer.filled == self.max_buffer_size {
            self.flush_buffer();
        }

        self.buffer.push(value).unwrap();
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Ok(v) = self.buffer.pop() {
            self.len -= 1;
            return Some(v);
        }

        if self.parts.len() == 0
            || unsafe { self.parts.get_unchecked(self.last_non_empty) }.filled == 0
        {
            return None;
        }

        let to_return = unsafe {
            self.parts
                .get_unchecked_mut(self.last_non_empty)
                .pop()
                .unwrap_unchecked()
        };
        self.len -= 1;

        if unsafe { self.parts.get_unchecked(self.last_non_empty) }.filled == 0
            && self.last_non_empty != 0
        {
            self.last_non_empty -= 1;
        }
        Some(to_return)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        (self.parts.len() * self.max_buffer_size) + self.max_buffer_size
    }
}
