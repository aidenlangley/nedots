#! /usr/bin/env python

import argparse
import json

def setup():
    argparser = argparse.ArgumentParser(description = 'Display `bspwm` node flags')
    argparser.add_argument(
        'json_tree', type = str, default = '',
        help = '`JSON` output provided by caller, typically the output of \
            `bspc query -T -n focused`',
    )

    return argparser.parse_args()

def print_node_flags(node_tree: str):
    # We're only concerned with displaying these flags.
    flag_states = {
        'S': node_tree['sticky'],
        'X': node_tree['locked'],
        'M': node_tree['marked'],
        'P': node_tree['private']
    }

    # Filter out false flags.
    if list(flag_states.values()).count(True) > 0:
        flags = [ ' ' ]
        for flag, state in flag_states.items():
            if state:
                flags.append(flag)

        print(''.join(flags))
    else:
        # When we've got nothing, we want to clear the output.
        print('')

args = setup()
print_node_flags(json.loads(args.json_tree))
