use core::fmt;
use core::marker::PhantomData;
use Nul;
use {ffi, FtResult, GlyphSlot, Matrix, Vector};

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum KerningMode {
    KerningDefault  = ffi::FT_KERNING_DEFAULT,
    KerningUnfitted = ffi::FT_KERNING_UNFITTED,
    KerningUnscaled = ffi::FT_KERNING_UNSCALED,
}

bitflags! {
    pub struct LoadFlag: i32 {
        const DEFAULT                    = ::ffi::FT_LOAD_DEFAULT;
        const NO_SCALE                   = ::ffi::FT_LOAD_NO_SCALE;
        const NO_HINTING                 = ::ffi::FT_LOAD_NO_HINTING;
        const RENDER                     = ::ffi::FT_LOAD_RENDER;
        const NO_BITMAP                  = ::ffi::FT_LOAD_NO_BITMAP;
        const VERTICAL_LAYOUT            = ::ffi::FT_LOAD_VERTICAL_LAYOUT;
        const FORCE_AUTOHINT             = ::ffi::FT_LOAD_FORCE_AUTOHINT;
        const CROP_BITMAP                = ::ffi::FT_LOAD_CROP_BITMAP;
        const PEDANTIC                   = ::ffi::FT_LOAD_PEDANTIC;
        const IGNORE_GLOBAL_ADVANCE_WITH = ::ffi::FT_LOAD_IGNORE_GLOBAL_ADVANCE_WIDTH;
        const NO_RECURSE                 = ::ffi::FT_LOAD_NO_RECURSE;
        const IGNORE_TRANSFORM           = ::ffi::FT_LOAD_IGNORE_TRANSFORM;
        const MONOCHROME                 = ::ffi::FT_LOAD_MONOCHROME;
        const LINEAR_DESIGN              = ::ffi::FT_LOAD_LINEAR_DESIGN;
        const NO_AUTOHINT                = ::ffi::FT_LOAD_NO_AUTOHINT;
        const TARGET_NORMAL              = ::ffi::FT_LOAD_TARGET_NORMAL;
        const TARGET_LIGHT               = ::ffi::FT_LOAD_TARGET_LIGHT;
        const TARGET_MONO                = ::ffi::FT_LOAD_TARGET_MONO;
        const TARGET_LCD                 = ::ffi::FT_LOAD_TARGET_LCD;
        const TARGET_LCD_V               = ::ffi::FT_LOAD_TARGET_LCD_V;
        const COLOR                      = ::ffi::FT_LOAD_COLOR;
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct Face<'a> {
    library_raw: ffi::FT_Library,
    raw: ffi::FT_Face,
    glyph: GlyphSlot,
    _phantom: PhantomData<&'a ()>
}

impl<'a> ::fallible::TryClone for Face<'a> {
    type Error = ::error::Error;
    fn try_clone(&self) -> FtResult<Self> { unsafe {
        ::error::from_ftret(ffi::FT_Reference_Library(self.library_raw))?;
        ::error::from_ftret(ffi::FT_Reference_Face(self.raw))?;
        Ok(Face { ..*self })
    } }
}

impl<'a> Face<'a> {
    pub unsafe fn from_raw(library_raw: ffi::FT_Library, raw: ffi::FT_Face) -> Self {
        ffi::FT_Reference_Library(library_raw);
        Face {
            library_raw, raw,
            glyph: GlyphSlot::from_raw(library_raw, (*raw).glyph),
            _phantom: PhantomData
        }
    }

    pub fn attach_file(&self, filepathname: &str) -> FtResult<()> {
        ::error::from_ftret(unsafe { ffi::FT_Attach_File(self.raw, filepathname.as_ptr() as *const _) })
    }

    pub fn reference(&self) -> FtResult<()> {
        ::error::from_ftret(unsafe { ffi::FT_Reference_Face(self.raw) })
    }

    pub fn set_char_size(&self, char_width: isize, char_height: isize, horz_resolution: u32,
                         vert_resolution: u32) -> FtResult<()> {
        ::error::from_ftret(unsafe {
            ffi::FT_Set_Char_Size(self.raw, char_width as ffi::FT_F26Dot6,
                                  char_height as ffi::FT_F26Dot6, horz_resolution,
                                  vert_resolution)
        })
    }

    pub fn set_pixel_sizes(&self, pixel_width: u32, pixel_height: u32) -> FtResult<()> {
        ::error::from_ftret(unsafe { ffi::FT_Set_Pixel_Sizes(self.raw, pixel_width, pixel_height) })
    }

    pub fn load_glyph(&self, glyph_index: u32, load_flags: LoadFlag) -> FtResult<()> {
        ::error::from_ftret(unsafe { ffi::FT_Load_Glyph(self.raw, glyph_index, load_flags.bits) })
    }

    pub fn load_char(&self, char_code: usize, load_flags: LoadFlag) -> FtResult<()> {
        ::error::from_ftret(unsafe { ffi::FT_Load_Char(self.raw, char_code as ffi::FT_ULong, load_flags.bits) })
    }

    pub fn set_transform(&self, matrix: &mut Matrix, delta: &mut Vector) {
        unsafe { ffi::FT_Set_Transform(self.raw, matrix, delta); }
    }

    pub fn get_char_index(&self, charcode: usize) -> u32 {
        unsafe { ffi::FT_Get_Char_Index(self.raw, charcode as ffi::FT_ULong) }
    }

    pub fn get_kerning(&self, left_char_index: u32, right_char_index: u32, kern_mode: KerningMode)
        -> FtResult<Vector> {
        let mut vec = Vector { x: 0, y: 0 };

        ::error::from_ftret(unsafe {
            ffi::FT_Get_Kerning(self.raw, left_char_index, right_char_index,
                                kern_mode as u32, &mut vec)
        })?;
        Ok(vec)
    }

    // According to FreeType doc, each time you load a new glyph image,
    // the previous one is erased from the glyph slot.
    #[inline(always)]
    pub fn glyph(&self) -> &GlyphSlot { &self.glyph }

    #[inline(always)]
    pub fn has_horizontal(&self) -> bool {
        ffi::FT_HAS_HORIZONTAL(self.raw)
    }

    #[inline(always)]
    pub fn has_vertical(&self) -> bool {
        ffi::FT_HAS_VERTICAL(self.raw)
    }

    #[inline(always)]
    pub fn has_kerning(&self) -> bool {
        ffi::FT_HAS_KERNING(self.raw)
    }

    #[inline(always)]
    pub fn is_scalable(&self) -> bool {
        ffi::FT_IS_SCALABLE(self.raw)
    }

    #[inline(always)]
    pub fn is_sfnt(&self) -> bool {
        ffi::FT_IS_SFNT(self.raw)
    }

    #[inline(always)]
    pub fn is_fixed_width(&self) -> bool {
        ffi::FT_IS_FIXED_WIDTH(self.raw)
    }

    #[inline(always)]
    pub fn has_fixed_sizes(&self) -> bool {
        ffi::FT_HAS_FIXED_SIZES(self.raw)
    }

    #[inline(always)]
    pub fn has_glyph_names(&self) -> bool {
        ffi::FT_HAS_GLYPH_NAMES(self.raw)
    }

    #[inline(always)]
    pub fn is_cid_keyed(&self) -> bool {
        ffi::FT_IS_CID_KEYED(self.raw)
    }

    #[inline(always)]
    pub fn is_tricky(&self) -> bool {
        ffi::FT_IS_TRICKY(self.raw)
    }

    #[inline(always)]
    pub fn has_color(&self) -> bool {
        ffi::FT_HAS_COLOR(self.raw)
    }

    #[inline(always)]
    pub fn raw(&self) -> &ffi::FT_FaceRec {
        unsafe { &*self.raw }
    }

    #[inline(always)]
    pub fn raw_mut(&mut self) -> &mut ffi::FT_FaceRec {
        unsafe { &mut *self.raw }
    }

    #[inline(always)]
    pub fn ascender(&self) -> ffi::FT_Short {
        unsafe { (*self.raw).ascender }
    }

    #[inline(always)]
    pub fn descender(&self) -> ffi::FT_Short {
        unsafe { (*self.raw).descender }
    }

    #[inline(always)]
    pub fn em_size(&self) -> ffi::FT_Short {
        unsafe { (*self.raw).units_per_EM as i16 }
    }

    #[inline(always)]
    pub fn height(&self) -> ffi::FT_Short {
        unsafe { (*self.raw).height }
    }

    #[inline(always)]
    pub fn max_advance_width(&self) -> ffi::FT_Short {
        unsafe { (*self.raw).max_advance_width }
    }

    #[inline(always)]
    pub fn max_advance_height(&self) -> ffi::FT_Short {
        unsafe { (*self.raw).max_advance_height }
    }

    #[inline(always)]
    pub fn underline_position(&self) -> ffi::FT_Short {
        unsafe { (*self.raw).underline_position }
    }

    #[inline(always)]
    pub fn underline_thickness(&self) -> ffi::FT_Short {
        unsafe { (*self.raw).underline_thickness }
    }

    pub fn family_name(&self) -> Option<&Nul<u8>> {
        unsafe { ((*self.raw).family_name as *mut u8).as_ref() }
            .map(|p| unsafe { Nul::new_unchecked(p) })
    }

    pub fn style_name(&self) -> Option<&Nul<u8>> {
        unsafe { ((*self.raw).style_name as *mut u8).as_ref() }
            .map(|p| unsafe { Nul::new_unchecked(p) })
    }

    pub fn size_metrics(&self) -> Option<ffi::FT_Size_Metrics> {
        if self.raw.is_null() {
            None
        } else {
            let size = unsafe { (*self.raw).size };
            if size.is_null() {
                None
            } else {
                Some(unsafe { (*size).metrics })
            }
        }
    }

    pub fn postscript_name(&self) -> Option<&Nul<u8>> {
        unsafe { (ffi::FT_Get_Postscript_Name(self.raw) as *mut u8).as_ref() }
            .map(|p| unsafe { Nul::new_unchecked(p) })
    }
}

impl<'a> fmt::Debug for Face<'a> {
    fn fmt(&self, form: &mut fmt::Formatter) -> fmt::Result {
        let name = self.style_name()
                       .and_then(|s| ::core::str::from_utf8(&s[..]).ok())
                       .unwrap_or("[unknown name]");
        form.write_str("Font Face: ")?;
        form.write_str(&name[..])
    }
}

impl<'a> Drop for Face<'a> {
    fn drop(&mut self) { unsafe {
        ::error::from_ftret(ffi::FT_Done_Face(self.raw)).expect("Failed to drop face");
        ::error::from_ftret(ffi::FT_Done_Library(self.library_raw)).expect("Failed to drop library");
    } }
}
