use cursive::view::{Margins, Resizable as _};
use cursive::views::{self, Dialog, EditView, LinearLayout, TextArea, TextView};
use cursive::{Cursive, View};

pub fn request_result() -> LinearLayout {
    LinearLayout::vertical()
        .child(TextView::new("Request Result"))
        .child(TextArea::new().content("{}").full_height().full_width())
}
