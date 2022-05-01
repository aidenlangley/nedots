set -x DOCKER_HOST "unix://$XDG_RUNTIME_DIR/podman/podman.sock"
set -x EDITOR '/usr/bin/code'
set -x PATH "$HOME/.cargo/bin" "$HOME/.local/bin" $PATH

# GUI
set -x QT_QPA_PLATFORMTHEME 'qt5ct'
set -x SXHKD_SHELL '/usr/bin/sh'

if status is-interactive
end
