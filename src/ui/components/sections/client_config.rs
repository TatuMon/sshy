use ratatui::{
    layout::Rect,
    Frame,
};

use crate::model::sections_state::client_config_state::ClientConfigState;

/// USAR EL EJEMPLO DE VIM DE ACA
/// https://github.com/rhysd/tui-textarea/blob/main/examples/vim.rs

pub fn draw(f: &mut Frame, rect: &Rect, section_state: &ClientConfigState) {
    let textarea = section_state.get_textarea();
    f.render_widget(textarea, *rect);
}
