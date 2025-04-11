use gtk::prelude::*;
use gtk::{ApplicationWindow};
use webkit2gtk::{WebView, WebContext, WebViewExt};
use gdk::Display;

pub struct WidgetWindow {
    window: ApplicationWindow,
    webview: WebView,
    monitor_num: i32,
}

impl WidgetWindow {
    pub fn new(app: &gtk::Application, widget_type: &str, monitor_num: i32) -> Self {
        let window = ApplicationWindow::new(app);
        window.set_title(&format!("Widget - {}", widget_type));
        window.set_decorated(false);
        window.set_keep_above(true);
        window.set_app_paintable(true);

        // Position window on correct monitor
        if let Some(display) = Display::default() {
            if let Some(monitor) = display.monitor(monitor_num) {
                let rect = monitor.geometry();
                window.move_(rect.x(), rect.y());
                window.set_default_size(200, 100);
            }
        }

        // Create WebView with transparent background
        let context = WebContext::default().unwrap();
        let webview = WebView::with_context(&context);
        webview.set_background_color(&gdk::RGBA::new(0.0, 0.0, 0.0, 0.0));

        // Load the appropriate HTML content based on widget type
        let html = match widget_type {
            "clock" => include_str!("widgets/clock.html"),
            "stats" => include_str!("widgets/stats.html"),
            _ => "<html><body>Unknown widget type</body></html>",
        };
        
        webview.load_html(html, None);
        window.add(&webview);
        window.show_all();

        Self {
            window,
            webview,
            monitor_num,
        }
    }
}

pub fn initialize_widgets(window: &ApplicationWindow) {
    // Create a vertical box container
    let container = gtk::Box::new(gtk::Orientation::Vertical, 10);
    container.set_margin_start(20);
    container.set_margin_end(20);
    container.set_margin_top(20);
    container.set_margin_bottom(20);

    // Create and style the label
    let label = gtk::Label::new(Some("Hello, Desktop Widget!"));
    label.set_halign(gtk::Align::Center);
    label.set_valign(gtk::Align::Center);
    
    // Make the label text larger
    label.set_markup("<span font='20'>Hello, Desktop Widget!</span>");
    
    // Style the label with CSS
    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_data(b"
        label {
            color: white;
            background-color: rgba(64, 64, 64, 0.8);
            padding: 10px;
            border-radius: 5px;
        }
    ").expect("Failed to load CSS");
    
    label.style_context().add_provider(
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Add the label to the container
    container.add(&label);
    
    // Add the container to the window
    window.add(&container);
}

pub fn update_widgets() {
    // Logic to update widgets periodically (e.g., refresh data)
    // For now, this is a placeholder
}