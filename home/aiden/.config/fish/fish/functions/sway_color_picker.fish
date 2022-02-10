function sway_color_picker
    set slurp (slurp -p)
    grim -g "$slurp" -t ppm - | \
        convert - -format '%[pixel:p{0,0}]' txt:- | \
        tail -n 1 | cut -d ' ' -f 4 | wl-copy && \
        notify-send -u low -t 3000 'Color Picker' '...HTML color copied to clipboard'
end
