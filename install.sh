#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DESKTOP_FILE="$HOME/.config/autostart/warp-gui.desktop"

BINARY="$SCRIPT_DIR/target/release/warp-gui"

if [ ! -f "$BINARY" ]; then
  echo "Building warp-gui..."
  cargo build --release --manifest-path "$SCRIPT_DIR/Cargo.toml"
else
  echo "Binary exists, skipping build."
fi

mkdir -p "$HOME/.config/autostart"

cat > "$DESKTOP_FILE" << EOF
[Desktop Entry]
Type=Application
Name=Warp GUI
Comment=Cloudflare WARP system tray
Exec=$BINARY
Icon=$SCRIPT_DIR/warp_connect.png
Terminal=false
StartupNotify=false
EOF

echo "Autostart installed at $DESKTOP_FILE"
echo "Binary: $BINARY"
echo "Run 'rm $DESKTOP_FILE' to remove autostart"