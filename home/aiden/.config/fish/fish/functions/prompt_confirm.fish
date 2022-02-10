# Prompt user for confirmation, passes on Y or an empty string, otherwise exit
function prompt_confirm -a msg
    # If we're given a message, print it.
    if not_empty $msg; echo $msg; end

    # Read input from user.
    read -l -P (notify 'Continue? (Y/n) ') reply

    # Exit if they declined to confirm.
    if test "$reply" != 'Y'; and not_empty $reply
        exit 1
    end
end
