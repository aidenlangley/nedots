function sway_convert_mp4_to_gif -a input_filename
    stop_recording

    # Use ffmpeg to convert our mp4 to a gif - the filter_complex handles
    # fps, res and colours.
    command notify-send 'Converting to GIF...' 'This may take a while.' -u urgent
    command ffmpeg -i "$input_filename.mp4" -f gif "$input_filename.gif" -filter_complex "[0:v] fps=30,scale=w=720:h=-1,split [a][b];[a] palettegen [p];[b][p] paletteuse"

    # Notify user & clean up.
    command notify-send 'Conversion complete' "Saved to $input_filename.gif"
    command rm "$input_filename.mp4"
end
