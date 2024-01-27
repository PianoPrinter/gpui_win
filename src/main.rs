// WIN_DIRTY <- commented out things, for windows port

use gpui::*;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .child(div().size_80().rounded(Pixels(32.0)).bg(rgb(0xff7d32)))
            .child(div().size_80().rounded(Pixels(32.0)).bg(rgb(0x2eff32)))
            .child(div().size_80().rounded(Pixels(32.0)).bg(rgb(0x2e7dff)))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| HelloWorld {
                text: "World".into(),
            })
        });
    });
}
