#!/bin/bash
DURATION=400


format_volume () {
    volume="$(wpctl get-volume @DEFAULT_SINK@)e"
    echo "${volume:8:-1}"
}

change_volume () {
    wpctl set-volume -l 1.5 @DEFAULT_AUDIO_SINK@ $1
    notify-send -t $DURATION "Volume" "$(format_volume)"
}

toggle_mute () {
    wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle
    notify-send -t $DURATION "Mute" "$(format_volume)"
}


case $1 in
    "mute")
        toggle_mute
    ;;
    
    "v")
        change_volume $2
    ;;
    
    "b")
        brightnessctl -d intel_backlight s $2
        notify-send -t $DURATION Brightness "$(($(brightnessctl get) / 960))%"
    ;;
    
    *)
        notify-send "error in keybind notification script"
    ;;
esac
