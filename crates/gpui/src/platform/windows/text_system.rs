use collections::HashMap;
use parking_lot::{lock_api::RwLockUpgradableReadGuard, RwLock};
use smallvec::SmallVec;
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::FALSE,
        Graphics::DirectWrite::{
            DWriteCreateFactory, IDWriteFactory, IDWriteFont, IDWriteFontCollection,
            IDWriteFontFamily, DWRITE_FACTORY_TYPE_SHARED,
        },
    },
};

use crate::{Font, FontFeatures, FontId, PlatformTextSystem, SharedString};

pub(crate) struct WindowsTextSystem(RwLock<WindowsTextSystemState>);

// Font-kit has a weird crossplatform implementation
// which does not work well on windows. (threading related issues)
// I decided not to use it at least for now.
// Instead, windows directwrite is used. (which is used by font-kit internally)
struct WindowsTextSystemState {
    memory_source: MemSource,
    system_source: SystemSource,
    // font: fontdue::Font,
    // fonts: Vec<FontKitFont>,
    font_selections: HashMap<Font, FontId>,
    font_ids_by_postscript_name: HashMap<String, FontId>,
    font_ids_by_family_name: HashMap<SharedString, SmallVec<[FontId; 4]>>,
    postscript_names_by_font_id: HashMap<FontId, String>,
}

impl WindowsTextSystem {
    pub(crate) fn new() -> Self {
        // let font = fontdue::Font::from_bytes(
        //     include_bytes!("a"),
        //     fontdue::FontSettings {
        //         ..Default::default()
        //     },
        // )
        // .unwrap();
        Self(RwLock::new(WindowsTextSystemState {
            memory_source: MemSource::empty(),
            system_source: SystemSource::new(),
            // font,
            // fonts: Vec::new(),
            font_selections: HashMap::default(),
            font_ids_by_postscript_name: HashMap::default(),
            font_ids_by_family_name: HashMap::default(),
            postscript_names_by_font_id: HashMap::default(),
        }))
    }
}

impl PlatformTextSystem for WindowsTextSystem {
    fn add_fonts(&self, fonts: &[std::sync::Arc<Vec<u8>>]) -> anyhow::Result<()> {
        todo!()
    }

    fn all_font_names(&self) -> Vec<String> {
        todo!()
    }

    fn all_font_families(&self) -> Vec<String> {
        todo!()
    }

    fn font_id(&self, font: &crate::Font) -> anyhow::Result<crate::FontId> {
        // let lock = self.0.upgradable_read();
        // if let Some(font_id) = lock.font_selections.get(font) {
        //     Ok(*font_id)
        // } else {
        //     let mut lock = RwLockUpgradableReadGuard::upgrade(lock);
        //     let candidates = if let Some(font_ids) = lock.font_ids_by_family_name.get(&font.family)
        //     {
        //         font_ids.as_slice()
        //     } else {
        //         let font_ids = lock.load_family(&font.family, font.features)?;
        //         lock.font_ids_by_family_name
        //             .insert(font.family.clone(), font_ids);
        //         lock.font_ids_by_family_name[&font.family].as_ref()
        //     };

        //     // let candidate_properties = candidates.iter().map(|font_id| lock.fonts)
        // }
        Ok(crate::FontId(0))
    }

    fn font_metrics(&self, font_id: crate::FontId) -> crate::FontMetrics {
        todo!()
    }

    fn typographic_bounds(
        &self,
        font_id: crate::FontId,
        glyph_id: crate::GlyphId,
    ) -> anyhow::Result<crate::Bounds<f32>> {
        todo!()
    }

    fn advance(
        &self,
        font_id: crate::FontId,
        glyph_id: crate::GlyphId,
    ) -> anyhow::Result<crate::Size<f32>> {
        todo!()
    }

    fn glyph_for_char(&self, font_id: crate::FontId, ch: char) -> Option<crate::GlyphId> {
        todo!()
    }

    fn glyph_raster_bounds(
        &self,
        params: &crate::RenderGlyphParams,
    ) -> anyhow::Result<crate::Bounds<crate::DevicePixels>> {
        todo!()
    }

    fn rasterize_glyph(
        &self,
        params: &crate::RenderGlyphParams,
        raster_bounds: crate::Bounds<crate::DevicePixels>,
    ) -> anyhow::Result<(crate::Size<crate::DevicePixels>, Vec<u8>)> {
        // let (metrics, bitmap) = self
        //     .0
        //     .read()
        //     .font
        //     .rasterize(params.glyph_id as char, params.font_size.0);
        // anyhow::Ok((
        //     crate::Size {
        //         width: metrics.width,
        //         height: metrics.height,
        //     },
        //     bitmap,
        // ))
        todo!()
    }

    fn layout_line(
        &self,
        text: &str,
        font_size: crate::Pixels,
        runs: &[crate::FontRun],
    ) -> crate::LineLayout {
        // todo!()
        // crate::LineLayout {
        //     ascent: crate::Pixels(10.0),
        //     descent: crate::Pixels(10.0),
        //     font_size: crate::Pixels(10.0),
        //     len: 10,
        //     width: crate::Pixels(10.0),
        //     runs: vec![crate::ShapedRun {
        //         font_id: crate::FontId(0),
        //         glyphs: smallvec::smallvec![crate::ShapedGlyph {
        //             id: crate::GlyphId(0),
        //             index: 0,
        //             is_emoji: false,
        //             position: crate::Point {
        //                 x: crate::Pixels(10.0),
        //                 y: crate::Pixels(10.0)
        //             }
        //         }],
        //     }],
        // }
        crate::LineLayout::default()
    }

    fn wrap_line(
        &self,
        text: &str,
        font_id: crate::FontId,
        font_size: crate::Pixels,
        width: crate::Pixels,
    ) -> Vec<usize> {
        todo!()
    }
}

impl WindowsTextSystemState {
    fn load_family(
        &mut self,
        name: &SharedString,
        features: FontFeatures,
    ) -> crate::Result<SmallVec<[FontId; 4]>> {
        // let mut font_ids = SmallVec::new();
        // let family = self
        //     .memory_source
        //     .select_family_by_name(name.as_ref())
        //     .or_else(|_| self.system_source.select_family_by_name(name.as_ref()))
        //     .unwrap();

        // let font_count = unsafe { family.GetFontCount() };

        // for font_index in 0..font_count {
        //     let dwrite_font = unsafe { family.GetFont(font_index) }.unwrap();
        //     let dwrite_font_face = unsafe { dwrite_font.CreateFontFace() }.unwrap();
        //     dwrite_font_face.GetGlyphIndices(codepoints, codepointcount, glyphindices)
        // }
        todo!()
    }
}

struct FamilyEntry {
    family_name: String,
    postscript_name: String,
    font: IDWriteFont,
}

struct MemSource {
    families: Vec<FamilyEntry>,
}

impl MemSource {
    pub fn empty() -> Self {
        Self { families: vec![] }
    }

    pub fn select_family_by_name(&self, family_name: &str) -> Result<IDWriteFontFamily, ()> {
        todo!()
    }
}

struct SystemSource {
    system_font_collection: IDWriteFontCollection,
}

impl SystemSource {
    pub fn new() -> Self {
        let factory: IDWriteFactory =
            unsafe { DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED) }.unwrap();
        let mut font_collection = None;
        unsafe { factory.GetSystemFontCollection(&mut font_collection, false) };

        Self {
            system_font_collection: font_collection.unwrap(),
        }
    }

    pub fn select_family_by_name(&self, family_name: &str) -> Result<IDWriteFontFamily, ()> {
        let mut index = 0;
        let mut exists = FALSE;

        unsafe {
            self.system_font_collection.FindFamilyName(
                &HSTRING::from(family_name),
                &mut index,
                &mut exists,
            )
        }
        .unwrap();

        if exists == FALSE {
            return Err(()); // SelectError: NotFound
        }

        let family = unsafe { self.system_font_collection.GetFontFamily(index) }.unwrap();

        Ok(family)
    }
}
