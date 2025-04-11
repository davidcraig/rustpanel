use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::gdk::{Display, WindowTypeHint};
use webkit2gtk::{WebView, WebContext, traits::WebViewExt};
use std::fs;

fn main() {
    gtk::init().unwrap();

    let app = Application::new(Some("com.example.background"), Default::default());
    
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
                window.move_(rect.x, rect.y);
                window.set_default_size(rect.width, rect.height);
                
                let context = WebContext::default().unwrap();
                let webview = WebView::with_context(&context);
                
                // Load HTML content from template file
                let template_path = format!("src/templates/monitor{}.html", monitor_num);
                let html = fs::read_to_string(&template_path)
                    .unwrap_or_else(|_| format!(
                        "<html><body><div style='color: white;'>Monitor {}</div></body></html>",
                        monitor_num
                    ));
                
                webview.load_html(&html, None);
                window.add(&webview);
                window.show_all();
            }
        }
    });

    app.run();
}
