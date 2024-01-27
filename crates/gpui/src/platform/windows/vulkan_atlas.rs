use parking_lot::Mutex;

use crate::PlatformAtlas;

pub(crate) struct VulkanAtlas(Mutex<VulkanAtlasState>);

struct VulkanAtlasState {}

impl VulkanAtlas {
    pub(crate) fn new() -> Self {
        Self(Mutex::new(VulkanAtlasState {}))
    }
}

impl PlatformAtlas for VulkanAtlas {
    fn get_or_insert_with<'a>(
        &self,
        key: &crate::AtlasKey,
        build: &mut dyn FnMut() -> anyhow::Result<(
            crate::Size<crate::DevicePixels>,
            std::borrow::Cow<'a, [u8]>,
        )>,
    ) -> anyhow::Result<crate::AtlasTile> {
        todo!()
    }
}
