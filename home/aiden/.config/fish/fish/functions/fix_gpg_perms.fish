# https://superuser.com/questions/954509/what-are-the-correct-permissions-for-the-gnupg-enclosing-folder-gpg-warning
function fix_gpg_perms -d 'Fix permissions on GPG directory'
    chown -R (whoami) $HOME/.gnupg/
    find ~/.gnupg -type f -exec chmod 600 {} \;
    find ~/.gnupg -type d -exec chmod 700 {} \;
end
