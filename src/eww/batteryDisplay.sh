#!/bin/bash

battery_capacity=$(cat /sys/class/power_supply/BAT0/capacity)
battery_status=$(cat /sys/class/power_supply/BAT0/status)
battery_time=$(echo $(cat /sys/class/power_supply/BAT0/charge_now) / $(cat /sys/class/power_supply/BAT0/current_now) | bc -l)
battery_unit="h"

if [[ $battery_time = .* ]]
then
    battery_time=$(echo "$battery_time * 60" | bc)
    battery_unit="m"
fi

if [[ $battery_status = "Discharging" ]]
then
    if [[ $battery_capacity -le 20 ]]
    then
        battery_status="Cri"
    else
        battery_status="Dis"
    fi
elif [[ $battery_status = "Charging" ]]
then
    battery_status="Cha"
elif [[ $battery_status = "Not Charging" ]]
then
    battery_status="Not"
else
    battery_status="..."
fi

echo "$battery_capacity%  ${battery_time%.*}$battery_unit  $battery_status"