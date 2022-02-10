# Check `length` of string & whether or not it's set, quietly, with `set`.
function not_empty -a var
    set -q var && string length -q -- $var
end
