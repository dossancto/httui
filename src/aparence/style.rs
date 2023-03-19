use cursive::{theme::{BorderStyle, Palette}, Cursive};
use cursive::traits::With;

pub fn setup_palette(palette: &mut Palette) {
    use cursive::theme::BaseColor::*;

    {
        // First, override some colors from the base palette.
        use cursive::theme::Color::TerminalDefault;
        use cursive::theme::PaletteColor::*;

        palette[Background] = TerminalDefault;
        palette[View] = TerminalDefault;
        palette[Primary] = White.dark();
        palette[TitlePrimary] = Blue.light();
        palette[Secondary] = Blue.light();
        palette[Highlight] = Blue.dark();
    }

    {
        // Then override some styles.
        use cursive::theme::Effect::*;
        use cursive::theme::PaletteStyle::*;
        use cursive::theme::Style;
        palette[Highlight] = Style::from(Blue.light()).combine(Bold);
    }
}

pub fn initial_style(siv: &mut Cursive){
    siv.set_theme(cursive::theme::Theme {
        shadow: true,
        borders: BorderStyle::Simple,
        palette: Palette::retro().with(|palette| {setup_palette(palette)}),
    });
}
