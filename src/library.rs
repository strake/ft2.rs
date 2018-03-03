use core::ptr::null_mut;
use libc::{self, c_long, c_void, size_t};
use {Face, FtResult, Stroker};
use ffi;
use Nul;

extern "C" fn alloc_library(_memory: ffi::FT_Memory, size: c_long) -> *mut c_void {
    unsafe { libc::malloc(size as size_t) }
}

extern "C" fn free_library(_memory: ffi::FT_Memory, block: *mut c_void) {
    unsafe { libc::free(block) }
}

extern "C" fn realloc_library(_memory: ffi::FT_Memory,
                              _cur_size: c_long,
                              new_size: c_long,
                              block: *mut c_void) -> *mut c_void {
    unsafe { libc::realloc(block, new_size as size_t) }
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum LcdFilter {
    LcdFilterNone    = ffi::FT_LCD_FILTER_NONE,
    LcdFilterDefault = ffi::FT_LCD_FILTER_DEFAULT,
    LcdFilterLight   = ffi::FT_LCD_FILTER_LIGHT,
    LcdFilterLegacy  = ffi::FT_LCD_FILTER_LEGACY,
}

static mut MEMORY: ffi::FT_MemoryRec = ffi::FT_MemoryRec {
    user: 0 as *mut c_void,
    alloc: alloc_library,
    free: free_library,
    realloc: realloc_library,
};

pub struct Library {
    raw: ffi::FT_Library
}

impl Library {
    /// This function is used to create a new FreeType library instance and add the default
    /// modules. It returns a struct encapsulating the freetype library. The library is correctly
    /// discarded when the struct is dropped.
    pub fn init() -> FtResult<Self> { unsafe {
        let mut raw = null_mut();
        ::error::from_ftret(ffi::FT_New_Library(&mut MEMORY, &mut raw))?;
        ffi::FT_Add_Default_Modules(raw);
        Ok(Library { raw })
    } }

    /// Open a font file using its pathname. `face_index` should be 0 if there is only 1 font
    /// in the file.
    pub fn new_face<P>(&self, path: P, face_index: isize) -> FtResult<Face<'static>>
      where P: AsRef<Nul<u8>> { unsafe {
        let mut face = null_mut();
        ::error::from_ftret(ffi::FT_New_Face(self.raw, path.as_ref().as_ptr() as *const _, face_index as ffi::FT_Long, &mut face))?;
        Ok(Face::from_raw(self.raw, face))
    } }

    pub fn new_stroker(&self) -> FtResult<Stroker> { unsafe {
        let mut stroker = null_mut();
        ::error::from_ftret(ffi::FT_Stroker_New(self.raw, &mut stroker))?;
        Ok(Stroker::from_raw(self.raw, stroker))
    } }

    /// Similar to `new_face`, but loads file data from a byte array in memory
    pub fn new_memory_face<'a>(&self, buffer: &'a [u8], face_index: isize) -> FtResult<Face<'a>> { unsafe {
        let mut face = null_mut();
        ::error::from_ftret(ffi::FT_New_Memory_Face(self.raw, buffer.as_ptr(), buffer.len() as ffi::FT_Long,
                                                    face_index as ffi::FT_Long, &mut face))?;
        Ok(Face::from_raw(self.raw, face))
    } }

    pub fn set_lcd_filter(&self, lcd_filter: LcdFilter) -> FtResult<()> {
        ::error::from_ftret(unsafe { ffi::FT_Library_SetLcdFilter(self.raw, lcd_filter as u32) })
    }

    /// Get the underlying library object
    pub fn raw(&self) -> ffi::FT_Library { self.raw }
}

impl Drop for Library {
    fn drop(&mut self) {
        ::error::from_ftret(unsafe { ffi::FT_Done_Library(self.raw) }).expect("Failed to drop library");
    }
}
