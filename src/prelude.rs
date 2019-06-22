// use failure::Context;
// use failure::Fail;
pub use failure::ResultExt; // For .context()
pub use lazy_static::*;
pub use log::*;
pub use serde_derive::*;
pub use std::collections::{HashMap, HashSet, VecDeque};
pub use std::path::Path;
pub use std::path::PathBuf;
pub use std::rc::Rc;

pub type Result<T> = std::result::Result<T, failure::Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn predule_test_dummy() {
        assert_eq!(0, 0);
    }
}
