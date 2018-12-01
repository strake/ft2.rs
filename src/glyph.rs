use core::ptr::null_mut;
use {ffi, BBox, BitmapGlyph, FtResult, Matrix, RenderMode, Vector};

/// Represents a retrieved glyph from the library
///
/// Note that when this glyph is dropped, so is the library
pub struct Glyph {
    library_raw: ffi::FT_Library,
    raw: ffi::FT_Glyph
}

impl Glyph {
    /// Create a freetype-rs glyph object from c constituent parts
    pub unsafe fn from_raw(library_raw: ffi::FT_Library, raw: ffi::FT_Glyph) -> Self {
        ffi::FT_Reference_Library(library_raw);
        Glyph { library_raw, raw }
    }

    /// Transform a glyph image if its format is scalable.
    pub fn transform(&self, mut matrix: Option<Matrix>, mut delta: Option<Vector>) -> FtResult<()> {
        let mut p_matrix = null_mut();
        let mut p_delta = null_mut();

        if let Some(ref mut m) = matrix {
            p_matrix = m as *mut Matrix;
        }
        if let Some(ref mut d) = delta {
            p_delta = d as *mut Vector;
        }
        ::error::from_ftret(unsafe { ffi::FT_Glyph_Transform(self.raw, p_matrix, p_delta) })
    }

    /// Return a glyph's ‘control box’. The control box encloses all the outline's points,
    /// including Bézier control points. Though it coincides with the exact bounding box for most
    /// glyphs, it can be slightly larger in some situations (like when rotating an outline that
    /// contains Bézier outside arcs).
    ///
    /// Computing the control box is very fast, while getting the bounding box can take much more
    /// time as it needs to walk over all segments and arcs in the outline. To get the latter, you
    /// can use the ‘ftbbox’ component, which is dedicated to this single task.
    pub fn get_cbox(&self, bbox_mode: ffi::FT_Glyph_BBox_Mode) -> BBox {
        let mut acbox = ffi::FT_BBox {
            xMin: 0,
            yMin: 0,
            xMax: 0,
            yMax: 0,
        };
        unsafe { ffi::FT_Glyph_Get_CBox(self.raw, bbox_mode, &mut acbox) };
        acbox
    }

    /// Convert a given glyph object to a bitmap glyph object.
    pub fn to_bitmap(&self, render_mode: RenderMode, mut origin: Option<Vector>) -> FtResult<BitmapGlyph> {
        let mut the_glyph = self.raw;
        let mut p_origin = null_mut();

        if let Some(ref mut o) = origin {
            p_origin = o as *mut Vector;
        }

        Ok(unsafe {
            ::error::from_ftret(ffi::FT_Glyph_To_Bitmap(&mut the_glyph, render_mode as u32, p_origin, 0))?;
            BitmapGlyph::from_raw(self.library_raw, the_glyph as ffi::FT_BitmapGlyph)
        })
    }

    pub fn advance_x(&self) -> isize {
        unsafe { (*self.raw).advance.x as isize }
    }

    pub fn advance_y(&self) -> isize {
        unsafe { (*self.raw).advance.y as isize }
    }

    /// An enumeration type used to describe the format of a given glyph image. Note that this
    /// version of FreeType only supports two image formats, even though future font drivers will
    /// be able to register their own format.
    #[inline(always)]
    pub fn format(&self) -> ffi::FT_Glyph_Format {
        unsafe { (*self.raw).format }
    }

    /// Get the underlying c glyph struct (The system actually calls this a GlyphRec because it can
    /// be a different struct in different circumstances)
    #[inline(always)]
    pub fn raw(&self) -> &ffi::FT_GlyphRec {
        unsafe { &*self.raw }
    }
}

impl ::fallible::TryClone for Glyph {
    type Error = ::error::Error;
    fn try_clone(&self) -> FtResult<Self> { unsafe {
        let mut target = null_mut();
        ::error::from_ftret(ffi::FT_Glyph_Copy(self.raw, &mut target))?;
        Ok(Glyph::from_raw(self.library_raw, target))
    } }
}

impl Drop for Glyph {
    fn drop(&mut self) { unsafe {
        ffi::FT_Done_Glyph(self.raw);
        ::error::from_ftret(ffi::FT_Done_Library(self.library_raw)).expect("Failed to drop library");
    } }
}
