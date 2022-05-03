use core::ptr::null_mut;
use {ffi, Bitmap};

pub struct BitmapGlyph {
    library_raw: ffi::FT_Library,
    raw: ffi::FT_BitmapGlyph
}

impl BitmapGlyph {
    pub unsafe fn from_raw(library_raw: ffi::FT_Library, raw: ffi::FT_BitmapGlyph) -> Self {
        ffi::FT_Reference_Library(library_raw);
        BitmapGlyph { library_raw, raw }
    }

    #[inline(always)]
    pub fn left(&self) -> i32 { unsafe { (*self.raw).left } }

    #[inline(always)]
    pub fn top(&self) -> i32 { unsafe { (*self.raw).top } }

    #[inline(always)]
    pub fn bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_raw(&(*self.raw).bitmap) }
    }

    #[inline(always)]
    pub fn raw(&self) -> &ffi::FT_BitmapGlyphRec { unsafe { &*self.raw } }
}

impl ::fallible::TryClone for BitmapGlyph {
    type Error = ::error::Error;
    fn try_clone(&self) -> ::FtResult<Self> { 
        let mut target = null_mut();
    unsafe {
        ::error::from_ftret(ffi::FT_Glyph_Copy(self.raw as ffi::FT_Glyph, &mut target))?;
        Ok(BitmapGlyph::from_raw(self.library_raw, target as ffi::FT_BitmapGlyph))
    } }
}

impl Drop for BitmapGlyph {
    fn drop(&mut self) { unsafe {
        ffi::FT_Done_Glyph(self.raw as ffi::FT_Glyph);
        ::error::from_ftret(ffi::FT_Done_Library(self.library_raw)).expect("Failed to drop bitmap glyph");
    } }
}
