#!/bin/bash

warning_level=20
critical_level=5
suspend_level=3


last_status=$(cat /sys/class/power_supply/BAT0/status)
timer=0
wait_time=300
loop_time=2

suspend_timer=10

notif_bat_full='false'

while true
do
    capacity=$(cat /sys/class/power_supply/BAT0/capacity)
    status=$(cat /sys/class/power_supply/BAT0/status)

    # Notify when fully charged once
    if [[ $capacity -ge 99 && $status = 'Charging' && $notif_bat_full = 'false' ]]
    then
        notify-send -u normal "Battery is full"

        notif_bat_full='true'

    # Notify about suspend soon
    elif [[ $capacity -le $suspend_level && $status = 'Discharging' && $suspend_timer -ge 0 ]]
    then
        suspend_timer=$((suspend_timer-loop_time))
        notify-send -t 2000 -u critical "Battery is at $capacity%" "Suspending in $suspend_timer seconds\!"
    
    # Notify about critically low
    elif [[ $capacity -le $critical_level && $status = 'Discharging' ]]
    then
        notify-send -u critical "BATTERY LOW $capacity%" "Laptop will suspend soon\!"

    # Notify battery level every few minutes if below threshold
    elif [[ $capacity -le $warning_level && $status = 'Discharging' && $timer -le 0 ]]
    then
        notify-send -u normal "Battery is at $capacity%"

        timer=$wait_time
    fi


    # Notify when charging state changed, and allow to notify if battery is full again
    if [[ ! $last_status = $status ]]
    then
        notify-send -t 2000 -u low "Battery is $status"

        notif_bat_full='false'
        suspend_timer=60
    fi

    if [[ $capacity -le $suspend_level && $status = 'Discharging' && $suspend_timer -eq 0 ]]
    then
        notify-send -t 1000 -u critical 'Battery Critical! Hibernating!' "Goodnight"

        sleep 1

        notify-send -u critical 'Laptop hibernated due to low battery'

        echo "Suspended at $(date), $(cat /sys/class/power_supply/BAT0/capacity)%" >> /home/penguin/.config/hypr/logs/batt_suspend.log

        systemctl hybrid-sleep
    fi


    last_status=$status

    if [[ $timer -gt 0 ]]
    then
        timer=$((timer-loop_time))
    fi

    sleep $loop_time
done

