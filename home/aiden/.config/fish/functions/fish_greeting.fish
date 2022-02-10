function fish_greeting -d "Informative greeting"
    set uptime (set_color -b yellow black)'~'(uptime --pretty | sed -e 's/up//;s/^ *//')'.'
    echo (set_color -o)"I've been up for $uptime"(set_color normal)
    echo (set_color -i)'"So long, and thanks for the all the fish."'(set_color normal)
end
