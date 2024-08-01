pub mod styles;
pub mod text_input;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
};

use crate::utils::{data_parsing::parse_usize_to_u16, math::percentage_representation};

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

/// helper function to create a centered rect to fit the given paragraph in
pub fn centered_rect_for_paragraph(
    paragraph: &Paragraph,
    max_width_percentage: u16,
    max_height_percentage: u16,
    r: Rect,
) -> Rect {
    let paragraph_width = parse_usize_to_u16(paragraph.line_width()).unwrap_or(u16::MAX);
    let paragraph_height =
        parse_usize_to_u16(paragraph.line_count(paragraph_width)).unwrap_or(u16::MAX);

    let width_percentage = percentage_representation(r.width, paragraph_width);
    let height_percentage = percentage_representation(r.height, paragraph_height);

    let vertical_constraints =
        if height_percentage > max_height_percentage {
            [
                Constraint::Percentage((100 - max_height_percentage) / 2),
                Constraint::Percentage(max_height_percentage),
                Constraint::Percentage((100 - max_height_percentage) / 2),
            ]
        } else {
            [
                Constraint::Percentage((100 - height_percentage) / 2),
                Constraint::Length(paragraph_height + 2), // The 2 is because line_count doesn't
                                                          // include the pixels needed for the
                                                          // borders
                Constraint::Percentage((100 - height_percentage) / 2),
            ]
        };

    let horiz_constraints =
        if width_percentage > max_width_percentage {
            [
                Constraint::Percentage((100 - max_width_percentage) / 2),
                Constraint::Percentage(max_width_percentage),
                Constraint::Percentage((100 - max_width_percentage) / 2),
            ]
        } else {
            [
                Constraint::Percentage((100 - width_percentage) / 2),
                Constraint::Length(paragraph_width + 2),
                Constraint::Percentage((100 - width_percentage) / 2),
            ]
        };

    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vertical_constraints)
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(horiz_constraints)
        .split(popup_layout[1])[1] // Return the middle chunk
}
