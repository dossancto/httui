use cursive::view::{Margins, Resizable as _};
use cursive::views::{Dialog, LinearLayout, TextView, EditView};
use cursive::Cursive;

use crate::interaction;

fn mount_layout() -> LinearLayout {
    LinearLayout::vertical()
        .child(TextView::new("Your URL"))
        .child(EditView::new().on_submit(|s, text| {
            s.add_layer(
                cursive::views::OnEventView::new(Dialog::new().title(text)).on_event(
                    cursive::event::Key::Esc,
                    |s| {
                        s.pop_layer();
                    },
                ),
            )
        }))
}

pub fn request_config() -> LinearLayout {
    LinearLayout::vertical().child(
        Dialog::around(mount_layout())
            .padding(Margins {
                left: 2,
                right: 2,
                top: 1,
                bottom: 1,
            })
            .h_align(cursive::align::HAlign::Center)
            .title("Theme example")
            .button("Change", |s| {
                interaction::http_verb::open_menu(s);
            })
            .button("Quit", Cursive::quit)
            .resized(
                cursive::view::SizeConstraint::AtLeast(50),
                cursive::view::SizeConstraint::Full,
            ),
    )
}
