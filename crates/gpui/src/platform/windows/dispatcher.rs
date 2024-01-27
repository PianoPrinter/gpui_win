use windows_sys::Win32::System::Threading::GetCurrentThreadId;

use crate::PlatformDispatcher;

pub(crate) struct WindowsDispatcher {}

impl WindowsDispatcher {
    pub fn new() -> Self {
        WindowsDispatcher {}
    }
}

impl PlatformDispatcher for WindowsDispatcher {
    fn is_main_thread(&self) -> bool {
        let thread_id = unsafe { GetCurrentThreadId() };

        thread_id == main_thread_id()
    }

    fn dispatch(&self, runnable: async_task::Runnable, label: Option<crate::TaskLabel>) {
        todo!()
    }

    fn dispatch_on_main_thread(&self, runnable: async_task::Runnable) {
        todo!()
    }

    fn dispatch_after(&self, duration: std::time::Duration, runnable: async_task::Runnable) {
        todo!()
    }

    fn tick(&self, background_only: bool) -> bool {
        todo!()
    }

    fn park(&self) {
        todo!()
    }

    fn unparker(&self) -> parking::Unparker {
        todo!()
    }
}

fn main_thread_id() -> u32 {
    static mut MAIN_THREAD_ID: u32 = 0;
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".CRT$XCU"]
    static INIT_MAIN_THREAD_ID: unsafe fn() = {
        unsafe fn initer() {
            MAIN_THREAD_ID = GetCurrentThreadId();
        }
        initer
    };

    unsafe { MAIN_THREAD_ID }
}
