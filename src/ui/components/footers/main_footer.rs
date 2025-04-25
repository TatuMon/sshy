use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use crate::{
    model::{vim_emulator::VimMode, Model},
    ui::{components::sections::Section, widgets::blocks},
};

const TOOLTIPS: [&str; 5] = [
    "(q) quit | (→) next section | (←) previous section | (↑|↓) navigate section",
    "(q) quit | (Enter) interactive mode",
    "(q) quit interactive mode | (ctrl+s) write to file | (h|j|k|l) navigate | (i) insert mode | (v) visual mode | (y) yank",
    "(Esc) normal mode | (h|j|k|l) navigate | (y) yank",
    "(Esc) normal mode",
];

pub fn draw_footer(f: &mut Frame, rect: &Rect, model: &Model) {
    let curr_tooltip = match model.get_current_section() {
        Section::ClientConfig if !model.get_client_config_state().is_interactive_on() => {
            TOOLTIPS[1]
        }
        Section::ClientConfig => match model.get_client_config_state().get_vim_state().get_mode() {
            VimMode::Normal => TOOLTIPS[2],
            VimMode::Visual => TOOLTIPS[3],
            _ => TOOLTIPS[4],
        },
        _ => TOOLTIPS[0],
    };

    let paragraph = Paragraph::new(curr_tooltip)
        .block(blocks::simple_block())
        .centered();

    f.render_widget(paragraph, *rect);
}
