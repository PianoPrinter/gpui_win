use parking_lot::RwLock;

use crate::PlatformTextSystem;

pub(crate) struct WindowsTextSystem(RwLock<WindowsTextSystemState>);

struct WindowsTextSystemState {}

impl WindowsTextSystem {
    pub(crate) fn new() -> Self {
        Self(RwLock::new(WindowsTextSystemState {}))
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

    fn font_id(&self, descriptor: &crate::Font) -> anyhow::Result<crate::FontId> {
        // todo!()
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
        todo!()
    }

    fn layout_line(
        &self,
        text: &str,
        font_size: crate::Pixels,
        runs: &[crate::FontRun],
    ) -> crate::LineLayout {
        // todo!()
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
