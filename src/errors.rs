use thiserror::Error;

pub mod bunch_errors {
    use super::*;

    #[derive(Debug, Clone, Error)]
    #[error("Index {} is out of bounds of slice {}", self.index, self.bound)]
    pub struct OutOfBounds {
        index: usize,
        bound: usize,
    }
    impl OutOfBounds {
        pub fn new(index: usize, bound: usize) -> OutOfBounds {
            OutOfBounds { index, bound }
        }
    }

    #[derive(Debug, Clone, Error)]
    #[error("Buffer is empty")]
    pub struct BufferEmpty {}

    #[derive(Debug, Clone, Error)]
    #[error("Buffer is Filled")]
    pub struct BufferFilled {}

    #[derive(Debug, Clone, Error)]
    #[error("The slice is too big to fit")]
    pub struct CantFitSlice {}
}
