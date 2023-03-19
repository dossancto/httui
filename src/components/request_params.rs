use cursive::view::{Margins, Resizable as _};
use cursive::views::{self, Dialog, EditView, LinearLayout, TextView};
use cursive::{Cursive, View};

pub fn request_params() -> LinearLayout {
    LinearLayout::vertical()
        .child(TextView::new("Request Headers"))
        .child(views::TextArea::new().resized(
            cursive::view::SizeConstraint::AtLeast(80),
            cursive::view::SizeConstraint::AtLeast(20),
        ))
        .child(TextView::new("Request Body"))
        .child(views::TextArea::new().resized(
            cursive::view::SizeConstraint::AtLeast(80),
            cursive::view::SizeConstraint::AtLeast(20),
        ))
}

fn request_header() -> LinearLayout {
    LinearLayout::vertical()
        .child(TextView::new("Request Body"))
        .child(views::TextArea::new().content("O corpo"))
}

pub fn requst_body() {}
