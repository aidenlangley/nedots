function upgrade --description 'refresh package manager then upgrade packages, flatpaks & firmware'
    sudo zypper refresh; sudo zypper dup -y; sudo flatpak update -y; sudo fwupdmgr update -y;
end

# Supports cancel
function cancel --on-signal INT
    kill $PPID && exit 1
end
