// WIN_DIRTY <- commented out things, for windows port
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui::*;
use smallvec::smallvec;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x222222))
            .size_full()
            .justify_center()
            .items_center()
            .child(
                div()
                    .size_72()
                    .m_10()
                    .rounded(Pixels(32.0))
                    .bg(rgb(0xff6b63))
                    .shadow(smallvec![BoxShadow {
                        color: Hsla::from(rgb(0xff6159)),
                        blur_radius: Pixels(30.0),
                        offset: Point::default(),
                        spread_radius: Pixels::default(),
                    }]),
            )
            .child(
                div()
                    .size_72()
                    .m_10()
                    .rounded(Pixels(32.0))
                    .bg(rgb(0xffc738))
                    .shadow(smallvec![BoxShadow {
                        color: Hsla::from(rgb(0xffbd2e)),
                        blur_radius: Pixels(30.0),
                        offset: Point::default(),
                        spread_radius: Pixels::default(),
                    }]),
            )
            .child(
                div()
                    .size_72()
                    .m_10()
                    .rounded(Pixels(32.0))
                    .bg(rgb(0x32d34a))
                    .shadow(smallvec![BoxShadow {
                        color: Hsla::from(rgb(0x28c941)),
                        blur_radius: Pixels(30.0),
                        offset: Point::default(),
                        spread_radius: Pixels::default(),
                    }]),
            )
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
