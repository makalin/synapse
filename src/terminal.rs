use gpui::*;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Terminal {
    output: Arc<Mutex<String>>,
    pty: Option<portable_pty::MasterPty>,
    writer: Option<Box<dyn Write + Send>>,
}

impl Terminal {
    pub fn new(cx: &mut ViewContext<Terminal>) -> View<Self> {
        let output = Arc::new(Mutex::new(String::new()));
        let output_clone = output.clone();

        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .expect("Failed to create PTY");

        let cmd = CommandBuilder::new("sh");
        let _child = pair
            .slave
            .spawn_command(cmd)
            .expect("Failed to spawn command");

        let reader = pair.master.try_clone_reader().expect("Failed to clone reader");
        let writer = pair.master.take_writer().expect("Failed to take writer");

        // Spawn thread to read from PTY
        thread::spawn(move || {
            let mut buffer = [0u8; 1024];
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break, // EOF
                    Ok(n) => {
                        let text = String::from_utf8_lossy(&buffer[..n]);
                        if let Ok(mut output) = output_clone.lock() {
                            output.push_str(&text);
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        cx.new_view(|_cx| Self {
            output,
            pty: Some(pair.master),
            writer: Some(writer),
        })
    }

    fn send_input(&mut self, input: &str, cx: &mut ViewContext<Self>) {
        if let Some(ref mut writer) = self.writer {
            if let Err(e) = writer.write_all(input.as_bytes()) {
                eprintln!("Failed to write to PTY: {}", e);
            }
            if let Err(e) = writer.flush() {
                eprintln!("Failed to flush PTY: {}", e);
            }
        }
        cx.notify();
    }
}

impl Render for Terminal {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let output = self.output.lock().unwrap().clone();
        let lines: Vec<&str> = output.lines().collect();
        let display_lines = if lines.len() > 100 {
            lines[lines.len() - 100..].to_vec()
        } else {
            lines
        };

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(rgb(0x000000))
            .text_color(rgb(0x00ff00))
            .font_family("Monaco")
            .font_size(px(12.0))
            .p(px(8.0))
            .overflow_y_scroll()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .children(display_lines.iter().map(|line| {
                        div()
                            .w_full()
                            .text(line)
                            .whitespace_pre()
                    })),
            )
    }
}

