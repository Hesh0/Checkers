extern crate glfw;

use glfw::{Action, Context, Key, Window, WindowMode, WindowEvent};

fn main() {
    let width = 900;
    let height = width / 16 * 9;
    let title = "Mesh Checkers";

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw.create_window(width, height, title, WindowMode::Windowed)
        .expect("Failed to create window.");

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut Window, event: WindowEvent)
{
    match event {
        WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}
