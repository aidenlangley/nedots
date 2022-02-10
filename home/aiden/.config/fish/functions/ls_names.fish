function ls_names -a dir
    set --local items (ls -lA $dir | awk '{print $9}')
    erase_first $items
end
