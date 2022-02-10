function notif_affirm -a msg
    set --local notif (set_color green)$msg(set_color normal)
    string unescape $notif
end
