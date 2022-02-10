# https://superuser.com/questions/215504/permissions-on-private-key-in-ssh-folder
function fix_ssh_perms -d 'Fix permissions on SSH pub & private keys'
    chown -R (whoami) $HOME/.ssh/
    chmod 700 $HOME/.ssh/
    chmod 600 $HOME/.ssh/id_*
    chmod 644 $HOME/.ssh/id_*.pub
end
