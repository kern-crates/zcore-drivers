pub mod sdl;

use alloc::vec::Vec;

use crate::display::{ColorFormat, DisplayInfo};
use crate::scheme::{DisplayScheme, Scheme};

pub struct MockDisplay {
    info: DisplayInfo,
    fb: Vec<u8>,
}

impl MockDisplay {
    pub fn new(width: u32, height: u32) -> Self {
        let format = ColorFormat::RGB888;
        let fb_size = (width * height * format.bytes() as u32) as usize;
        let info = DisplayInfo {
            width,
            height,
            format,
            fb_size,
        };
        let fb = vec![0; fb_size];
        Self { info, fb }
    }
}

impl Scheme for MockDisplay {
    fn name(&self) -> &str {
        "mock-display"
    }
}

impl DisplayScheme for MockDisplay {
    #[inline]
    fn info(&self) -> DisplayInfo {
        self.info
    }

    #[inline]
    unsafe fn raw_fb(&self) -> &mut [u8] {
        core::slice::from_raw_parts_mut(self.fb.as_ptr() as _, self.info.fb_size)
    }
}