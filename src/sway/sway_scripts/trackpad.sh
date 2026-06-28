#!/bin/sh

TOGGLE_FILE="/tmp/.trackpad_toggle"

if [ -f "$TOGGLE_FILE" ]; then
    rm "$TOGGLE_FILE"
    swaymsg input "1267:12729:ASUE140D:00_04F3:31B9_Touchpad" dwt disabled
    notify-send "Trackpad enabled while typing"
else
    touch "$TOGGLE_FILE"
    swaymsg input "1267:12729:ASUE140D:00_04F3:31B9_Touchpad" dwt enabled
    notify-send "Trackpad disabled while typing"
fi
