use std::{ffi::c_void, rc::Rc, sync::Arc};

use parking_lot::Mutex;
use windows::{
    core::*, Win32::Foundation::*, Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::UI::WindowsAndMessaging::*,
};

use crate::{PlatformInput, PlatformWindow};

use super::{display::WindowsDisplay, vulkan_renderer::VulkanRenderer};

// NOOOOOO FIX THIS!!!! im too lazy to do this properly, revisit later.
static mut WINDOW_STATE: *const c_void = std::ptr::null();

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                let rc1: Arc<Mutex<WindowsWindowState>> = Arc::from_raw(WINDOW_STATE as _);
                let rc2 = rc1.clone();
                std::mem::forget(rc1);
                let mut lock = rc2.lock();
                if let Some(mut callback) = lock.request_frame_callback.take() {
                    drop(lock);
                    callback();
                    rc2.lock().request_frame_callback = Some(callback);
                }
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}

pub(crate) struct WindowsWindow(Arc<Mutex<WindowsWindowState>>);

struct WindowsWindowState {
    renderer: VulkanRenderer,
    request_frame_callback: Option<Box<dyn FnMut()>>,
    event_callback: Option<Box<dyn FnMut(PlatformInput) -> bool>>,
    activate_callback: Option<Box<dyn FnMut(bool)>>,
    resize_callback: Option<Box<dyn FnMut(crate::Size<crate::Pixels>, f32)>>,
    fullscreen_callback: Option<Box<dyn FnMut(bool)>>,
    moved_callback: Option<Box<dyn FnMut()>>,
    should_close_callback: Option<Box<dyn FnMut() -> bool>>,
    close_callback: Option<Box<dyn FnOnce()>>,
    appearance_changed_callback: Option<Box<dyn FnMut()>>,
}

unsafe impl Send for WindowsWindowState {}

impl WindowsWindow {
    pub fn open() -> Self {
        unsafe {
            let instance = GetModuleHandleA(None).unwrap();
            debug_assert!(instance.0 != 0);

            let window_class = s!("window");

            let wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
                hInstance: instance.into(),
                lpszClassName: window_class,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wndproc),
                ..Default::default()
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            let hwnd = CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                window_class,
                s!("GPUI - Windows"), // For now, hardcoded window title
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                instance,
                None,
            );

            let window = Self(Arc::new(Mutex::new(WindowsWindowState {
                renderer: VulkanRenderer::new(instance.0, hwnd.0),
                request_frame_callback: None,
                event_callback: None,
                activate_callback: None,
                resize_callback: None,
                fullscreen_callback: None,
                moved_callback: None,
                should_close_callback: None,
                close_callback: None,
                appearance_changed_callback: None,
            })));

            WINDOW_STATE = Arc::into_raw(window.0.clone()) as _;

            window
        }
    }
}

impl PlatformWindow for WindowsWindow {
    fn bounds(&self) -> crate::WindowBounds {
        // todo!()
        crate::WindowBounds::Fullscreen
    }

    fn content_size(&self) -> crate::Size<crate::Pixels> {
        // todo!()
        crate::Size {
            width: crate::Pixels(1424.0),
            height: crate::Pixels(714.0),
        }
    }

    fn scale_factor(&self) -> f32 {
        // todo!()
        1.0
    }

    fn titlebar_height(&self) -> crate::Pixels {
        // todo!()
        crate::Pixels(10.0)
    }

    fn appearance(&self) -> crate::WindowAppearance {
        // todo!()
        crate::WindowAppearance::Dark
    }

    fn display(&self) -> std::rc::Rc<dyn crate::PlatformDisplay> {
        // todo!()
        Rc::new(WindowsDisplay())
    }

    fn mouse_position(&self) -> crate::Point<crate::Pixels> {
        // todo!()
        crate::Point {
            x: crate::Pixels(0.0),
            y: crate::Pixels(0.0),
        }
    }

    fn modifiers(&self) -> crate::Modifiers {
        // todo!()
        crate::Modifiers::default()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        todo!()
    }

    fn set_input_handler(&mut self, input_handler: crate::PlatformInputHandler) {
        todo!()
    }

    fn take_input_handler(&mut self) -> Option<crate::PlatformInputHandler> {
        todo!()
    }

    fn prompt(
        &self,
        level: crate::PromptLevel,
        msg: &str,
        detail: Option<&str>,
        answers: &[&str],
    ) -> futures::channel::oneshot::Receiver<usize> {
        todo!()
    }

    fn activate(&self) {
        todo!()
    }

    fn set_title(&mut self, title: &str) {
        todo!()
    }

    fn set_edited(&mut self, edited: bool) {
        todo!()
    }

    fn show_character_palette(&self) {
        todo!()
    }

    fn minimize(&self) {
        todo!()
    }

    fn zoom(&self) {
        todo!()
    }

    fn toggle_full_screen(&self) {
        todo!()
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut()>) {
        self.0.as_ref().lock().request_frame_callback = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(crate::PlatformInput) -> bool>) {
        self.0.as_ref().lock().event_callback = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.as_ref().lock().activate_callback = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(crate::Size<crate::Pixels>, f32)>) {
        self.0.as_ref().lock().resize_callback = Some(callback);
    }

    fn on_fullscreen(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.as_ref().lock().fullscreen_callback = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.0.as_ref().lock().moved_callback = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.0.as_ref().lock().should_close_callback = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.0.as_ref().lock().close_callback = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().appearance_changed_callback = Some(callback);
    }

    fn is_topmost_for_position(&self, position: crate::Point<crate::Pixels>) -> bool {
        todo!()
    }

    fn invalidate(&self) {
        todo!()
    }

    fn draw(&self, scene: &crate::Scene) {
        let mut this = self.0.lock();
        this.renderer.draw(scene);
    }

    fn sprite_atlas(&self) -> Arc<dyn crate::PlatformAtlas> {
        self.0.lock().renderer.sprite_atlas().clone()
    }
}
