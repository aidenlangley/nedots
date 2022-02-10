function ls_files -a dir
    set --local items (ls -plA $dir | grep -v / | awk '{print $9}')
    erase_first $items
end
