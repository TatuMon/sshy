use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub struct AppLayout {
    pub known_hosts_list: Rect,
    pub public_keys_list: Rect,
    pub client_config: Rect,
    pub main_footer: Rect,
}

impl AppLayout {
    pub fn from_frame(f: &Frame) -> AppLayout {
        let frame_rect = f.area();

        let main_footer_rect = AppLayout::calc_main_footer_rect(&frame_rect);
        let known_hosts_list_rect = AppLayout::calc_known_hosts_list_rect(&frame_rect);
        let public_keys_list_rect = AppLayout::calc_public_keys_list_rect(&frame_rect);
        let client_config_rect = AppLayout::calc_client_config_rect(&frame_rect);

        AppLayout {
            known_hosts_list: known_hosts_list_rect,
            public_keys_list: public_keys_list_rect,
            client_config: client_config_rect,
            main_footer: main_footer_rect,
        }
    }

    fn calc_main_footer_rect(frame_rect: &Rect) -> Rect {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(3)])
            .split(*frame_rect)[1]
    }

    fn calc_known_hosts_list_rect(rect: &Rect) -> Rect {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33)])
            .split(*rect);

        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50), Constraint::Length(3)])
            .split(columns[0])[0]
    }

    fn calc_public_keys_list_rect(rect: &Rect) -> Rect {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33)])
            .split(*rect);

        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50), Constraint::Length(3)])
            .split(columns[0])[1]
    }

    fn calc_client_config_rect(rect: &Rect) -> Rect {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33), Constraint::Fill(1)])
            .split(*rect);

        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100), Constraint::Length(3)])
            .split(columns[1])[0]
    }
}
