#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DESKTOP_FILE="$HOME/.config/autostart/warp-gui.desktop"

if [ -d "$SCRIPT_DIR/.venv" ]; then
    PYTHON="$SCRIPT_DIR/.venv/bin/python"
else
    PYTHON="python3"
fi

mkdir -p "$HOME/.config/autostart"

cat > "$DESKTOP_FILE" << EOF
[Desktop Entry]
Type=Application
Name=Warp GUI
Comment=Cloudflare WARP system tray
Exec=$PYTHON SCRIPT_PATH/warp_tray.py
Icon=SCRIPT_PATH/warp_connect.png
Terminal=false
StartupNotify=false
EOF

sed -i "s|SCRIPT_PATH|$SCRIPT_DIR|g" "$DESKTOP_FILE"

echo "Autostart installed at $DESKTOP_FILE"
echo "Using: $PYTHON"
echo "Run 'rm $DESKTOP_FILE' to remove autostart"