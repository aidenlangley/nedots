function upgrade --description 'upgrade rpm, flatpak & firmware'
    sudo dnf upgrade -y && flatpak update -y; fwupdmgr update -y;
end

# Supports cancel
function cancel --on-signal INT
    kill $PPID && exit 1
end
