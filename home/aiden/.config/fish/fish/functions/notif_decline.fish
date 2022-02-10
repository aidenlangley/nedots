function notif_decline -a msg
    set --local notif (set_color -o brred)$msg(set_color normal)
    string unescape $notif
end
