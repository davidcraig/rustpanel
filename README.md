# RustPanel

A desktop application written in rust. It allows you to add any html and css as a wallpaper and interactive desktop.

## Building source

clone repo.

    cargo build --release

    sudo cp ./target/release/rustpanel /usr/local/bin/rustpanel

## Configuration

Config will live in ~/.rustpanel (you will need to make this folder)

    mkdir -p ~/.rustpanel/templates
    cp

It's config lives in ~/.rustpanel - in here you should create a `templates` folder which can contain your monitor0.html and if you have more than 1 monitor you can add monitor1 and so on. Copy your wallpapers you'd like into this folder then in your html you can use it for the body background image, which will load the wallpaper as your desktop background.

We can then utilize the power of html, css and js to further customise our desktop, such as interactive widgets, clocks, scrapers, etc.

## Startup

The following instructions will work on PopOS and should work on any similar distro (Gnome based).

    mkdir -p ~/.config/autostart

```bash
cat > ~/.config/autostart/rustpanel.desktop << EOL
[Desktop Entry]
Type=Application
Exec=/usr/local/bin/rustpanel
Path=/home/$USER/.rustpanel/templates
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true
Name=RustPanel
Comment=Start RustPanel on boot
EOL
```