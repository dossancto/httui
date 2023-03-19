use cursive::views::LinearLayout;

mod aparence;
mod components;
mod interaction;

fn main() {
    let mut siv = cursive::default();

    aparence::style::initial_style(&mut siv);

    siv.add_global_callback('q', |s| s.quit());

    siv.add_fullscreen_layer(
        LinearLayout::horizontal().child(components::request_config::request_config()),
    );

    siv.run();
}
