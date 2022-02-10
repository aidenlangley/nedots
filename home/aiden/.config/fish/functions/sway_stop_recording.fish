function sway_stop_recording
    command killall wf-recorder

    # Notify the user.
    command notify-send 'Finished recording' "Saved to $WFR_PATH.mp4"

    # Unset the variables to indicate that we've stopped recording.
    set -gue WFR_PATH
end
