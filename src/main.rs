use core_editor::ui::tui_core::{TuiCore, Event, SpecialKey};
use core_editor::editor::buffer::Buffer;

fn main() {
    let tui = TuiCore::new(80, 24);
    let mut buffer = Buffer::new();
    let rx = tui.create_event_channel();

    loop {
        tui.draw_buffer(&buffer);

        if let Ok(event) = rx.recv() {
            match event {
                Event::Input(ch) => {
                    if ch == '\x1B' {
                        break; // ESC para salir
                    } else {
                        buffer.insert_char(ch);
                    }
                },
                Event::SpecialKey(key) => match key {
                    SpecialKey::ArrowUp => buffer.move_cursor(0, -1),
                    SpecialKey::ArrowDown => buffer.move_cursor(0, 1),
                    SpecialKey::ArrowLeft => buffer.move_cursor(-1, 0),
                    SpecialKey::ArrowRight => buffer.move_cursor(1, 0),
                    SpecialKey::Backspace => buffer.delete_char(),
                    SpecialKey::Delete => {
                        buffer.move_cursor(1, 0);
                        buffer.delete_char();
                    },
                    SpecialKey::Home => buffer.move_to_start_of_line(),
                    SpecialKey::End => buffer.move_to_end_of_line(),
                    SpecialKey::PageUp => buffer.move_cursor(0, -10),
                    SpecialKey::PageDown => buffer.move_cursor(0, 10),
                },
                Event::Tick => {}
            }
        }
    }
}