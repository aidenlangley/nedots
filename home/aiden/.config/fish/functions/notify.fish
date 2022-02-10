function notify -a msg
    set --local notif (set_color -o bryellow)$msg(set_color normal)
    string unescape $notif
end
