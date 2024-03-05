#!/bin/bash

change_all_sink_volume () {
    for SINK in $(pactl list sinks | grep "Sink #" | cut -b7-)
    do
        pactl set-sink-volume $SINK $1
    done
}

mute_all_sinks () {
    for SINK in $(pactl list sinks | grep "Sink #" | cut -b7-)
    do
        pactl set-sink-mute $SINK toggle
    done
}

DURATION=400



case $1 in
    "mute")
        mute_all_sinks
        notify-send -t $DURATION "Mute" "$(pamixer --get-mute)"
    ;;
    
    "v")
        change_all_sink_volume $2
        notify-send -t $DURATION "Volume" "$(pamixer --get-volume-human)"
    ;;
    
    "b")
        brightnessctl -d intel_backlight s $2
        notify-send -t $DURATION Brightness "$(($(brightnessctl get) / 4))%"
    ;;
    
    *)
        notify-send "error in keybind notification script"
    ;;
esac
