set -x DOCKER_HOST "unix://$XDG_RUNTIME_DIR/podman/podman.sock"
set -x EDITOR '/usr/bin/code'
set -x PATH "$HOME/.cargo/bin" "$HOME/.local/bin" $PATH
set -x SSH_AUTH_SOCK "$XDG_RUNTIME_DIR/gcr/ssh"

# GUI
set -x QT_QPA_PLATFORMTHEME 'qt5ct'
set -x SXHKD_SHELL '/usr/bin/sh'

if status is-interactive
end
