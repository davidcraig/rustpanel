use gtk::gdk::WindowTypeHint;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use glib::timeout_add_seconds;
use glib::Continue;

mod widgets;

fn main() {
    let app = Application::new(Some("com.example.rust-desktop-widgets"), Default::default());

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Rust Desktop Widgets");
        window.set_default_size(800, 600);
        window.set_decorated(false); // Non-interactive window
        window.set_keep_above(true); // Keep window above other windows
        window.set_app_paintable(true); // Allow custom drawing

        // Set window type hint to behave like a desktop widget
        if let Some(gdk_window) = window.window() {
            gdk_window.set_type_hint(WindowTypeHint::Desktop);
        }

        window.set_accept_focus(false); // Make the window non-interactive

        // Initialize widgets
        widgets::initialize_widgets(&window);

        window.show_all();
    });

    // Run the application
    app.run();

    // Keep the application running in the background
    timeout_add_seconds(1, || {
        // Update widget display logic here
        widgets::update_widgets();
        Continue(true)
    });
}
