use tui::{backend::TestBackend, buffer::Buffer, layout::Rect, symbols, widgets::Tabs, Terminal};

#[test]
fn widgets_tabs_should_not_panic_on_narrow_areas() {
    let backend = TestBackend::new(1, 1);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|mut f| {
            let tabs = Tabs::default().titles(&["Tab1", "Tab2"]);
            f.render_widget(
                tabs,
                Rect {
                    x: 0,
                    y: 0,
                    width: 1,
                    height: 1,
                },
            );
        })
        .unwrap();

    // FIXME: Assumes 0-wide margin when not explicity specified
    let expected = Buffer::with_lines(vec!["T"]);
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_tabs_should_truncate_the_last_item() {
    let backend = TestBackend::new(9, 1);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|mut f| {
            let tabs = Tabs::default().titles(&["Tab1", "Tab2"]);
            f.render_widget(
                tabs,
                Rect {
                    x: 0,
                    y: 0,
                    width: 8,
                    height: 1,
                },
            );
        })
        .unwrap();

    // FIXME: Assumes 0-wide margin when not explicitly specified
    // FIXME: Assumes VERTICAL divider when not explicity specified
    // if default divider changes test will fail
    let expected = Buffer::with_lines(vec![format!("Tab1 {} Ta", symbols::line::VERTICAL)]);
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_tabs_should_respect_left_margin() {
    // TODO: Possibly test other number of margins
    let backend = TestBackend::new(13, 1);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|mut f| {
            let tabs = Tabs::default().titles(&["Tab1", "Tab2"]).margin(2);
            f.render_widget(
                tabs,
                Rect {
                    x: 0,
                    y: 0,
                    width: 13,
                    height: 1,
                },
            );
        })
        .unwrap();

    // FIXME: Assumes VERTICAL divider when not explicity specified
    // if default divider changes test will fail
    let expected = Buffer::with_lines(vec![format!("  Tab1 {} Tab2", symbols::line::VERTICAL)]);
    terminal.backend().assert_buffer(&expected);
}
