function erase_first
    set --local items (string collect $argv) || return
    set --erase --local items[1] && string collect $items
end
