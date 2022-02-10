function sway_record_mp4
    set -l dt (date +%Y%m%d-%H%M.%S)
    set -l --unpath path "$HOME/media/videos/recording_$dt"

    # Globally set the path, so that we can access it later.
    set -x --unpath WFR_PATH $path
    echo (set --show WFR_PATH)
    sleep 3600

    # Start the recording.
    exec wf-recorder -g (slurp -d) -f "$path.mp4"
end
