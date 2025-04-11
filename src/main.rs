use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::gdk::{Display, WindowTypeHint};
use webkit2gtk::{WebView, WebContext, traits::WebViewExt};
use std::fs;
use std::path::PathBuf;
use dirs;

fn get_template_path(monitor_num: i32) -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_default();
    path.push(".rustpanel");
    path.push("templates");
    path.push(format!("monitor{}.html", monitor_num));
    path
}

fn get_template_dir() -> String {
    let mut path = dirs::home_dir().unwrap_or_default();
    path.push(".rustpanel");
    path.push("templates");
    format!("file://{}", path.to_string_lossy())
}

fn main() {
    gtk::init().unwrap();

    let app = Application::new(Some("com.davidcraig.rustpanel"), Default::default());
    
    app.connect_activate(move |app| {
        if let Some(display) = Display::default() {
            let screen = display.default_screen();
            let n_monitors = screen.n_monitors();
            
            for monitor_num in 0..n_monitors {
                let rect = screen.monitor_geometry(monitor_num);
                
                let window = ApplicationWindow::new(app);
                window.set_decorated(false);
                window.set_app_paintable(true);
                window.set_type_hint(WindowTypeHint::Desktop);
                
                // Enable compositing
                window.set_visual(Some(&screen.rgba_visual().unwrap()));
                window.set_app_paintable(true);
                
                // Position and size
                window.move_(rect.x, rect.y);
                window.set_default_size(rect.width, rect.height);
                
                // Make background transparent
                window.connect_screen_changed(|window, _| {
                    if let Some(screen) = window.screen() {
                        if let Some(visual) = screen.rgba_visual() {
                            window.set_visual(Some(&visual));
                        }
                    }
                });
                
                window.connect_draw(|_, cr| {
                    cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
                    cr.set_operator(cairo::Operator::Source);
                    cr.paint().unwrap();
                    Inhibit(false)
                });
                
                // Create WebView with transparent background
                let context = WebContext::default().unwrap();
                let webview = WebView::with_context(&context);
                
                // Load HTML content from template file in user's home directory
                let template_path = get_template_path(monitor_num);
                let html = fs::read_to_string(&template_path)
                    .unwrap_or_else(|_| format!(
                        "<html><body style='background: transparent !important;'><div style='color: white;'>Monitor {}</div></body></html>",
                        monitor_num
                    ));

                // Get base URL for loading local resources
                let base_url = get_template_dir();
                
                webview.load_html(&html, Some(&base_url));
                window.add(&webview);
                window.show_all();
            }
        }
    });

    app.run();
}
