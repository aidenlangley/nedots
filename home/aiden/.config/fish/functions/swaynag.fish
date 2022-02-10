function swaynag --wraps=swaynag --description 'swaynag w/ config'
  command swaynag -c $HOME/.config/swaynag/config.conf $argv;
end
