use web_view::*;

fn main() {
    let webview = web_view::builder()
        .title("Desktop Webview")
        .content(Content::Url("https://www.google.com"))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .build()
        .unwrap();

    webview.run().unwrap();
}
