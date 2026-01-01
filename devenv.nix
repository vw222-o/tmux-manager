{ pkgs, ... }:

{
  packages = with pkgs; [
    git
    bun
    glib

    at-spi2-atk
    atkmm
    cairo
    gdk-pixbuf
    glib
    gtk3
    harfbuzz
    librsvg
    libsoup_3
    pango
    webkitgtk_4_1
    openssl

    pkg-config
    gobject-introspection
    cargo 
    cargo-tauri
    nodejs
  ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  # https://devenv.sh/scripts/
  scripts.hello.exec = ''
    echo hello from $GREET
  '';
}

# ./linuxdeploy-x86_64.AppImage --appdir src-tauri/target/release/bundle/appimage/app.AppDir/ --output appimage