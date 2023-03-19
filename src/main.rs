use cursive::view::Resizable as _;
use cursive::views::{self, Dialog, EditView, LinearLayout, TextView};
use cursive::Cursive;

mod aparence;
mod interaction;

fn main() {
    let mut siv = cursive::default();

    aparence::style::initial_style(&mut siv);

    let layout = views::LinearLayout::vertical()
        .child(TextView::new("This is a dynamic theme example!"))
        .child(
            EditView::new()
                .content("Woo! colors!")
                .on_submit(|s, text| {
                    s.add_layer(
                        cursive::views::OnEventView::new(Dialog::new().title(text)).on_event(
                            cursive::event::Key::Esc,
                            |s| {
                                s.pop_layer();
                            },
                        ),
                    )
                }),
        );

    siv.add_fullscreen_layer(
        LinearLayout::horizontal()
            .child(
                Dialog::around(layout)
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
            .child(EditView::new().on_submit(|s, _t| {
                s.clear();
            })),
    );

    siv.run();
}
