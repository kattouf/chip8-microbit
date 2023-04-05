pub struct SimpleError(pub &'static str);

impl core::fmt::Debug for SimpleError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "SimpleError({})", self.0)
    }
}

impl core::fmt::Display for SimpleError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "SimpleError({})", self.0)
    }
}

pub type SimpleResult<T> = core::result::Result<T, SimpleError>;
