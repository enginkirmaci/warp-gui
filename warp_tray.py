#!/usr/bin/env python3
import os
import subprocess
import time

from PIL import Image
import gi
gi.require_version('AppIndicator3', '0.1')
from gi.repository import AppIndicator3, GLib, Gtk


SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
CONNECTED_ICON = os.path.join(SCRIPT_DIR, 'warp_connect.png')
DISCONNECTED_ICON = os.path.join(SCRIPT_DIR, 'warp_disconnected.png')


def run_warp_cli(args):
    try:
        result = subprocess.run(
            ['warp-cli'] + args,
            capture_output=True,
            text=True,
            timeout=30
        )
        return result.returncode == 0
    except Exception:
        return False


def get_status():
    try:
        result = subprocess.run(
            ['warp-cli', 'status'],
            capture_output=True,
            text=True,
            timeout=10
        )
        output = result.stdout.lower()
        if 'status update: connected' in output:
            return True
        if 'disconnected' in output:
            return False
        return False
    except Exception:
        return False


class WarpTray:
    def __init__(self):
        self.connected = get_status()
        self._create_icon()
    
    def _create_icon(self):
        icon_path = CONNECTED_ICON if self.connected else DISCONNECTED_ICON
        
        self.indicator = AppIndicator3.Indicator.new(
            'warp-tray',
            icon_path,
            AppIndicator3.IndicatorCategory.APPLICATION_STATUS
        )
        
        self.indicator.set_status(AppIndicator3.IndicatorStatus.ACTIVE)
        self._create_menu()
    
    def _create_menu(self):
        menu = Gtk.Menu()
        
        self.connect_item = Gtk.MenuItem.new_with_label('Connect')
        self.connect_item.connect('activate', self.on_connect)
        self.connect_item.set_sensitive(not self.connected)
        menu.append(self.connect_item)
        
        self.disconnect_item = Gtk.MenuItem.new_with_label('Disconnect')
        self.disconnect_item.connect('activate', self.on_disconnect)
        self.disconnect_item.set_sensitive(self.connected)
        menu.append(self.disconnect_item)
        
        separator = Gtk.SeparatorMenuItem()
        menu.append(separator)
        
        quit_item = Gtk.MenuItem.new_with_label('Quit')
        quit_item.connect('activate', self.on_quit)
        menu.append(quit_item)
        
        menu.show_all()
        self.indicator.set_menu(menu)
    
    def refresh_icon(self):
        self.connected = get_status()
        icon_path = CONNECTED_ICON if self.connected else DISCONNECTED_ICON
        
        self.indicator.set_icon(icon_path)
        
        self.connect_item.set_sensitive(not self.connected)
        self.disconnect_item.set_sensitive(self.connected)
    
    def on_connect(self, widget):
        if run_warp_cli(['connect']):
            time.sleep(2)
            self.refresh_icon()
    
    def on_disconnect(self, widget):
        if run_warp_cli(['disconnect']):
            time.sleep(1)
            self.refresh_icon()
    
    def on_quit(self, widget):
        Gtk.main_quit()
    
    def run(self):
        Gtk.main()


if __name__ == '__main__':
    app = WarpTray()
    app.run()
