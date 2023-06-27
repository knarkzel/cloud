// Modules
mod error;

// Exports
pub use error::{Error, Result};
pub use rmp_serde;
pub use cloud_macro::cloud;

#[repr(C)]
pub struct CloudSlice {
    pub len: usize,
    pub ptr: *mut u8,
}
