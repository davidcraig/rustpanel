use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::gdk::{Display, WindowTypeHint};
use webkit2gtk::{WebView, WebContext, traits::WebViewExt};

fn main() {
    gtk::init().unwrap();

    let app = Application::new(Some("com.davidcraig.rustpanel"), Default::default());
    
    app.connect_activate(move |app| {
        // Get primary display
        if let Some(display) = Display::default() {
            let screen = display.default_screen();
            let n_monitors = screen.n_monitors();
            
            // Create a window for each monitor
            for monitor_num in 0..n_monitors {
                let rect = screen.monitor_geometry(monitor_num);
                
                // Create window
                let window = ApplicationWindow::new(app);
                window.set_decorated(false);
                window.set_app_paintable(true);
                
                // Set window type to be below other windows but above wallpaper
                window.set_type_hint(WindowTypeHint::Desktop);
                
                // Position and size window to fill monitor
                window.move_(rect.x, rect.y);
                window.set_default_size(rect.width, rect.height);
                
                // Create WebView
                let context = WebContext::default().unwrap();
                let webview = WebView::with_context(&context);
                
                // Load HTML content
                let html = format!(r#"
                    <html>
                    <head>
                        <style>
                            body {{
                                margin: 0;
                                padding: 0;
                                width: 100vw;
                                height: 100vh;
                                background-color: rgba(0, 0, 0, 0);
                                display: flex;
                                justify-content: center;
                                align-items: center;
                            }}
                            #content {{
                                color: white;
                                font-size: 24px;
                                text-shadow: 2px 2px 4px rgba(0,0,0,0.5);
                            }}
                        </style>
                    </head>
                    <body>
                        <div id="content">Monitor {}</div>
                    </body>
                    </html>
                "#, monitor_num);
                
                webview.load_html(&html, None);
                window.add(&webview);
                window.show_all();
            }
        }
    });

    app.run();
}
