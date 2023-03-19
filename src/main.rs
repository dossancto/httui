use cursive::views::LinearLayout;

mod aparence;
mod components;
mod interaction;
use components::{request_config, request_params, request_result};

fn main() {
    let mut siv = cursive::default();

    aparence::style::initial_style(&mut siv);

    siv.add_global_callback('q', |s| s.quit());

    siv.add_fullscreen_layer(
        LinearLayout::horizontal()
            .child(request_config::request_config())
            .child(request_params::request_params())
            .child(request_result::request_result()),
    );

    siv.run();
}
