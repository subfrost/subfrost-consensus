use crate::imports::__log;
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};
pub use std::fmt::{Error, Write};
use std::sync::Arc;

pub struct Stdout(());

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        let mut data = to_arraybuffer_layout::<Vec<u8>>(s.to_string().as_bytes().to_vec());
        unsafe {
          __log(to_ptr(&mut data) + 4);
        }
        return Ok(());
    }
}

pub fn stdout() -> Stdout {
    Stdout(())
}

#[macro_export]
macro_rules! println {
  ( $( $x:expr ),* ) => {
    {
      writeln!(stdout(), $($x),*).unwrap();
    }
  }
}

#[allow(unused_unsafe)]
pub fn log(v: Arc<Vec<u8>>) -> () {
    unsafe {
        __log(to_ptr(&mut to_arraybuffer_layout(v.as_ref())) + 4);
    }
}
