function man --description 'alias man=man $argv || $argv --help | less || $argv -h | less'
    command man $argv || $argv[-1] --help | less || $argv[-1] -h | less;
end
