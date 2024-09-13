use std::io::{self, Read, Write};
use std::time::Duration;
use std::thread;
use std::sync::mpsc;
use std::process::Command;
use crate::editor::buffer::Buffer;

pub enum Event {
    Input(char),
    SpecialKey(SpecialKey),
    Tick,
}

pub enum SpecialKey {
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Backspace,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
}

#[derive(Clone, Copy)]
pub enum Color {
    Black, Red, Green, Yellow, Blue, Magenta, Cyan, White,
}

pub struct TuiCore {
    width: u16,
    height: u16,
}

impl TuiCore {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
    }

    pub fn draw_char(&self, x: u16, y: u16, ch: char) {
        print!("\x1B[{};{}H{}", y + 1, x + 1, ch);
        io::stdout().flush().unwrap();
    }

    pub fn draw_text(&self, x: u16, y: u16, text: &str, color: Color) {
        let color_code: i32 = match color {
            Color::Black => 30, Color::Red => 31, Color::Green => 32, Color::Yellow => 33,
            Color::Blue => 34, Color::Magenta => 35, Color::Cyan => 36, Color::White => 37,
        };
        print!("\x1B[{};{}H\x1B[{}m{}\x1B[0m", y + 1, x + 1, color_code, text);
        io::stdout().flush().unwrap();
    }

    pub fn get_dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn create_event_channel(&self) -> mpsc::Receiver<Event> {
        let (tx, rx) = mpsc::channel();
        let input_tx = tx.clone();

        // Configurar el terminal en modo sin buffer y sin bloqueo
        Command::new("sh")
            .arg("-c")
            .arg("stty raw -echo")
            .status()
            .unwrap();

        thread::spawn(move || {
            let stdin = io::stdin();
            let mut stdin = stdin.lock();
            let mut buffer: [i32; _] = [0; 1]; // Leer un byte a la vez

            loop {
                if stdin.read(&mut buffer).is_ok() {
                    let event: Event = match buffer[0] {
                        27 => {
                            let mut seq: [i32; _] = [0; 2];
                            if stdin.read(&mut seq).is_ok() {
                                match seq {
                                    [91, 65] => Event::SpecialKey(SpecialKey::ArrowUp),
                                    [91, 66] => Event::SpecialKey(SpecialKey::ArrowDown),
                                    [91, 67] => Event::SpecialKey(SpecialKey::ArrowRight),
                                    [91, 68] => Event::SpecialKey(SpecialKey::ArrowLeft),
                                    [91, 51] => {
                                        let mut tilde: [i32; _] = [0; 1];
                                        if stdin.read(&mut tilde).is_ok() && tilde[0] == 126 {
                                            Event::SpecialKey(SpecialKey::Delete)
                                        } else {
                                            continue;
                                        }
                                    }
                                    [91, 72] => Event::SpecialKey(SpecialKey::Home),
                                    [91, 70] => Event::SpecialKey(SpecialKey::End),
                                    [91, 53] => {
                                        let mut tilde: [i32; _] = [0; 1];
                                        if stdin.read(&mut tilde).is_ok() && tilde[0] == 126 {
                                            Event::SpecialKey(SpecialKey::PageUp)
                                        } else {
                                            continue;
                                        }
                                    }
                                    [91, 54] => {
                                        let mut tilde: [i32; _] = [0; 1];
                                        if stdin.read(&mut tilde).is_ok() && tilde[0] == 126 {
                                            Event::SpecialKey(SpecialKey::PageDown)
                                        } else {
                                            continue;
                                        }
                                    }
                                    _ => continue,
                                }
                            } else {
                                continue;
                            }
                        }
                        8 | 127 => Event::SpecialKey(SpecialKey::Backspace),
                        10 => Event::Input('\n'), // Manejar la tecla Enter
                        c => Event::Input(c as char),
                    };
                    input_tx.send(event).unwrap();
                }
            }
        });

        thread::spawn(move || {
            loop {
                tx.send(Event::Tick).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });

        rx
    }

    pub fn draw_buffer(&self, buffer: &Buffer) {
        self.clear_screen();
        let (cursor_x, cursor_y) = buffer.get_cursor_position();

        for (y, line) in buffer.get_lines().iter().enumerate() {
            if y as u16 >= self.height {
                break;
            }
            self.draw_text(0, y as u16, line, Color::White);
        }

        print!("\x1B[{};{}H", cursor_y + 1, cursor_x + 1);
        io::stdout().flush().unwrap();
    }
}

impl Drop for TuiCore {
    fn drop(&mut self) {
        // Restaurar la configuración del terminal al modo normal
        Command::new("sh")
            .arg("-c")
            .arg("stty sane")
            .status()
            .unwrap();
    }
}
