use crate::Error;

const MAGICK: u32 = 0x1738916A;

pub struct DirectReadHack {
    magick: u32,
    ptr: *const u8,
    xor: u32,
}

impl DirectReadHack {
    pub fn new(ptr: *const u8) -> Self {
        let mut res = Self {
            magick: MAGICK,
            ptr,
            xor: 0,
        };
        res.xor = res.gen_xor();
        res
    }

    fn gen_xor(&self) -> u32 {
        MAGICK ^ (self.ptr as usize as u32)
    }

    pub fn is_valid(&self) -> bool {
        self.magick == MAGICK && self.xor == self.gen_xor()
    }

    pub fn serialise(&self, dest: &mut [u8]) {
        if dest.len() >= core::mem::size_of::<Self>() {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    self as *const Self as *const u8,
                    dest.as_mut_ptr(),
                    core::mem::size_of::<Self>(),
                )
            }
        }
    }

    pub unsafe fn serialise_ptr(&self, dest: *mut u8, max_len: usize) {
        if max_len >= core::mem::size_of::<Self>() {
            core::ptr::copy_nonoverlapping(
                self as *const Self as *const u8,
                dest,
                core::mem::size_of::<Self>(),
            )
        }
    }

    pub fn deserialise(src: &[u8]) -> Result<Self, Error> {
        if src.len() >= core::mem::size_of::<Self>() {
            unsafe {
                let mut res = core::mem::MaybeUninit::uninit().assume_init();
                core::ptr::copy_nonoverlapping(
                    src.as_ptr(),
                    &mut res as *mut Self as *mut u8,
                    core::mem::size_of::<Self>(),
                );
                Ok(res)
            }
        } else {
            Err(Error::DataError)
        }
    }

    pub unsafe fn deserialise_ptr(src: *const u8, buf_size: usize) -> Result<Self, Error> {
        if buf_size >= core::mem::size_of::<Self>() {
            let mut res = core::mem::MaybeUninit::uninit().assume_init();
            core::ptr::copy_nonoverlapping(
                src,
                &mut res as *mut Self as *mut u8,
                core::mem::size_of::<Self>(),
            );
            Ok(res)
        } else {
            Err(Error::DataError)
        }
    }

    pub fn pointer<T: Sized>(&self) -> *const T {
        self.ptr as *const T
    }
}
