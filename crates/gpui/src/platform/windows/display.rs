use crate::PlatformDisplay;

#[derive(Debug)]
pub(crate) struct WindowsDisplay();

impl PlatformDisplay for WindowsDisplay {
    fn id(&self) -> crate::DisplayId {
        // todo!()
        crate::DisplayId(0)
    }

    fn uuid(&self) -> anyhow::Result<uuid::Uuid> {
        todo!()
    }

    fn bounds(&self) -> crate::Bounds<crate::GlobalPixels> {
        todo!()
    }
}
