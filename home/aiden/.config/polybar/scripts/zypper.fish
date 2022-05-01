#! /usr/bin/env fish

set updates (zypper lu)
if test (count $updates) -gt 3
    echo (math (count $updates) - 2) ''
else
    echo ''
end
