function sway -w sway -d 'alias sway=command sway'
    if test -z "$DISPLAY"; and string match -q '/dev/tty*' (tty)
        # Firefox is a little buggy with pure Wayland so we toggle this.
        set -x MOZ_ENABLE_WAYLAND 1

        # Other frameworks to Wayland.
        set -x QT_QPA_PLATFORMTHEME 'qt5ct'
        set -x QT_QPA_PLATFORM 'wayland'
        set -x QT_WAYLAND_DISABLE_WINDOWDECORATION 1

        # Run sway for us.
        exec sway
    end
end
