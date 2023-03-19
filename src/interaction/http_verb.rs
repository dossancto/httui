use cursive::Cursive;

pub fn open_menu(s: &mut Cursive) {
    let verb_list = vec!["GET", "POST", "PUT", "DELETE"];

    s.add_layer(
        cursive::views::OnEventView::new(
            cursive::views::Dialog::new()
                .title("Sekect the Verb to use")
                .content(
                    cursive::views::SelectView::new()
                        .with_all_str(verb_list)
                        .on_submit(|s, _verb: &str| {
                            s.pop_layer();
                        }),
                ),
        )
        .on_event(cursive::event::Key::Esc, |s| {
            s.pop_layer();
        }),
    );
}
