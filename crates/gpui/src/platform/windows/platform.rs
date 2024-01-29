use std::sync::Arc;

use parking_lot::Mutex;
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageA, GetMessageA};

use crate::{
    Action, BackgroundExecutor, ForegroundExecutor, Platform, PlatformInput, SemanticVersion,
};

use super::{dispatcher::WindowsDispatcher, text_system::WindowsTextSystem, window::WindowsWindow};

pub(crate) struct WindowsPlatform(Mutex<WindowsPlatformState>);

pub(crate) struct WindowsPlatformState {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<WindowsTextSystem>,
    become_active: Option<Box<dyn FnMut()>>,
    resign_active: Option<Box<dyn FnMut()>>,
    reopen: Option<Box<dyn FnMut()>>,
    quit: Option<Box<dyn FnMut()>>,
    event: Option<Box<dyn FnMut(PlatformInput) -> bool>>,
    menu_command: Option<Box<dyn FnMut(&dyn Action)>>,
    validate_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
    will_open_menu: Option<Box<dyn FnMut()>>,
    menu_actions: Vec<Box<dyn Action>>,
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    finish_launching: Option<Box<dyn FnOnce()>>,
}

impl WindowsPlatform {
    pub(crate) fn new() -> Self {
        let dispatcher = Arc::new(WindowsDispatcher::new());
        Self(Mutex::new(WindowsPlatformState {
            background_executor: BackgroundExecutor::new(dispatcher.clone()),
            foreground_executor: ForegroundExecutor::new(dispatcher),
            text_system: Arc::new(WindowsTextSystem::new()),
            become_active: None,
            resign_active: None,
            reopen: None,
            quit: None,
            event: None,
            menu_command: None,
            validate_menu_command: None,
            will_open_menu: None,
            menu_actions: Default::default(),
            open_urls: None,
            finish_launching: None,
        }))
    }
}

impl Platform for WindowsPlatform {
    fn background_executor(&self) -> crate::BackgroundExecutor {
        self.0.lock().background_executor.clone()
    }

    fn foreground_executor(&self) -> crate::ForegroundExecutor {
        self.0.lock().foreground_executor.clone()
    }

    fn text_system(&self) -> std::sync::Arc<dyn crate::PlatformTextSystem> {
        self.0.lock().text_system.clone()
    }

    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>) {
        // self.0.lock().finish_launching = Some(on_finish_launching);
        on_finish_launching();

        unsafe {
            let mut message = std::mem::zeroed();

            while GetMessageA(&mut message, None, 0, 0).into() {
                DispatchMessageA(&message);
            }
        }
    }

    fn quit(&self) {
        todo!()
    }

    fn restart(&self) {
        todo!()
    }

    fn activate(&self, ignoring_other_apps: bool) {
        todo!()
    }

    fn hide(&self) {
        todo!()
    }

    fn hide_other_apps(&self) {
        todo!()
    }

    fn unhide_other_apps(&self) {
        todo!()
    }

    fn displays(&self) -> Vec<std::rc::Rc<dyn crate::PlatformDisplay>> {
        todo!()
    }

    fn display(&self, id: crate::DisplayId) -> Option<std::rc::Rc<dyn crate::PlatformDisplay>> {
        todo!()
    }

    fn active_window(&self) -> Option<crate::AnyWindowHandle> {
        todo!()
    }

    fn open_window(
        &self,
        handle: crate::AnyWindowHandle,
        options: crate::WindowOptions,
    ) -> Box<dyn crate::PlatformWindow> {
        Box::new(WindowsWindow::open())
    }

    fn set_display_link_output_callback(
        &self,
        display_id: crate::DisplayId,
        callback: Box<dyn FnMut() + Send>,
    ) {
        todo!()
    }

    fn start_display_link(&self, display_id: crate::DisplayId) {
        todo!()
    }

    fn stop_display_link(&self, display_id: crate::DisplayId) {
        todo!()
    }

    fn open_url(&self, url: &str) {
        todo!()
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.0.lock().open_urls = Some(callback);
    }

    fn prompt_for_paths(
        &self,
        options: crate::PathPromptOptions,
    ) -> futures::channel::oneshot::Receiver<Option<Vec<std::path::PathBuf>>> {
        todo!()
    }

    fn prompt_for_new_path(
        &self,
        directory: &std::path::Path,
    ) -> futures::channel::oneshot::Receiver<Option<std::path::PathBuf>> {
        todo!()
    }

    fn reveal_path(&self, path: &std::path::Path) {
        todo!()
    }

    fn on_become_active(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().become_active = Some(callback);
    }

    fn on_resign_active(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().resign_active = Some(callback);
    }

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().quit = Some(callback);
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().reopen = Some(callback);
    }

    fn on_event(&self, callback: Box<dyn FnMut(crate::PlatformInput) -> bool>) {
        self.0.lock().event = Some(callback);
    }

    fn set_menus(&self, menus: Vec<crate::Menu>, keymap: &crate::Keymap) {
        todo!()
    }

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn crate::Action)>) {
        self.0.lock().menu_command = Some(callback);
    }

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().will_open_menu = Some(callback);
    }

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn crate::Action) -> bool>) {
        self.0.lock().validate_menu_command = Some(callback);
    }

    fn os_name(&self) -> &'static str {
        "windows"
    }

    fn os_version(&self) -> anyhow::Result<crate::SemanticVersion> {
        Ok(SemanticVersion::default())
    }

    fn app_version(&self) -> anyhow::Result<crate::SemanticVersion> {
        Ok(SemanticVersion::default())
    }

    fn app_path(&self) -> anyhow::Result<std::path::PathBuf> {
        todo!()
    }

    fn local_timezone(&self) -> time::UtcOffset {
        todo!()
    }

    fn double_click_interval(&self) -> std::time::Duration {
        todo!()
    }

    fn path_for_auxiliary_executable(&self, name: &str) -> anyhow::Result<std::path::PathBuf> {
        todo!()
    }

    fn set_cursor_style(&self, style: crate::CursorStyle) {
        todo!()
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        todo!()
    }

    fn write_to_clipboard(&self, item: crate::ClipboardItem) {
        todo!()
    }

    fn read_from_clipboard(&self) -> Option<crate::ClipboardItem> {
        todo!()
    }

    fn write_credentials(
        &self,
        url: &str,
        username: &str,
        password: &[u8],
    ) -> crate::Task<anyhow::Result<()>> {
        todo!()
    }

    fn read_credentials(
        &self,
        url: &str,
    ) -> crate::Task<anyhow::Result<Option<(String, Vec<u8>)>>> {
        todo!()
    }

    fn delete_credentials(&self, url: &str) -> crate::Task<anyhow::Result<()>> {
        todo!()
    }
}
