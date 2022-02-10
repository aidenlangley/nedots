function manh --description 'alias manh=$argv --help | less || $argv -h | less'
    command $argv --help | less || $argv -h | less $argv;
end
