#! /usr/bin/env fish

function unmute
    dunstctl set-paused false && \
        dunstify 'dunst' 'Unmuted notifications.' -i polari -u low &
end

function mute
    dunstify 'dunst' 'Muting notifications...' -i polari -u low &
    # We're about to mute notifications, so the above notification will
    # effectively not show, since set-paused hides notifications. So, sleep
    # for 2 seconds.
    sleep 2
    # Close all to prevent notification from showing up once we unmute.
    dunstctl close-all
    dunstctl set-paused true
end

switch $argv[1]
case 'toggle'
    if test (dunstctl is-paused) = "true"
        unmute
    else
        mute
    end
case '*'
    if test (dunstctl is-paused) = "true"
        echo ' '(dunstctl count waiting)
    else
        echo ''
    end
end
