#! /usr/bin/env python

from argparse import ArgumentParser, Namespace
from json import load
from pathlib import Path
from typing import Any, Dict, List
from os import system, makedirs


HOME = Path.home()
PROG_DIR = f'{HOME}/.nedots'
NUFFIN = "Got nuffin' to do!"


def parse_args() -> Namespace:
    args = ArgumentParser(
        description='nedots utility - installs & administrates packages & \
            dotfiles.'
    )
    args.add_argument('-v', '--verbose', action='store_const', const='v')

    subcmds = args.add_subparsers(dest='subcmd')
    subcmds.add_parser(
        'addchanges', aliases=['add', 'a'],
        help='add the latest changes from local configs listed in nedots.json.'
    )

    installer_args: ArgumentParser = subcmds.add_parser(
        'install', aliases=['i'],
        help='install packages & configs.'
    )
    installer_args.add_argument(
        '-d', '--distro',
        choices=['fedora'], default='fedora',
        help='install packages for this distro.'
    )

    installer_subcmds = installer_args.add_subparsers(dest='installer_subcmd')
    installer_subcmds.add_parser(
        'configs', aliases=['c'],
        help='install dotfiles.'
    )

    pkgs_args = installer_subcmds.add_parser(
        'pkgs', aliases=['p'],
        help='install packages.'
    )
    pkgs_args.add_argument(
        '-e', '--extras', action='store_true',
        help='install extra packages - these typically take a longer time to \
            install, or require some set up beforehand, such as activating \
            or installing repositories first.'
    )

    pkgs_subcmds = pkgs_args.add_subparsers(dest='pkgs_subcmd')
    pkgs_subcmds.add_parser(
        'common', aliases=['c'],
        help='install common packages.'
    )
    pkgs_subcmds.add_parser(
        'xorg', aliases=['x'],
        help='install xorg related packages.'
    )
    pkgs_subcmds.add_parser(
        'bspwm', aliases=['b'],
        help='install bspwm packages.'
    )
    pkgs_subcmds.add_parser(
        'wayland', aliases=['w'],
        help='install wayland packages.'
    )
    pkgs_subcmds.add_parser(
        'sway', aliases=['s'],
        help='install sway packages.'
    )

    installer_subcmds.add_parser(
        'flatpaks', aliases=['f'],
        help='install flatpaks.'
    )

    wm_args: ArgumentParser = installer_subcmds.add_parser(
        'wm', aliases=['w'],
        help='install a window manager.'
    )
    wm_args.add_argument(
        'wm',
        choices=[
            'bspwm',
            'sway'
        ],
        help='window manager to install.'
    )

    return args.parse_args()


def load_config() -> Dict[str, Any]:
    return load(open(f'{PROG_DIR}/nedots.json'))


def install_fedora(pkgs: List[str]) -> None:
    if pkgs.__len__() > 0:
        print('Installing fedora packages:')
        system('sudo dnf install {pkgs}'.format(pkgs=' '.join(pkgs)))


def install_flatpaks(remote: str, pkgs: List[str]) -> None:
    if pkgs.__len__() > 0:
        print('Installing flatpaks:')
        system(
            f'flatpak install {remote} {pkgs}'.format(pkgs=' '.join(pkgs))
        )


def add_dotfile_changes(files: List[Path], dotfile_dirs: List[Path]) -> None:
    for path in files:
        parent_dir = path.parent
        dest_dir = f'{PROG_DIR}{parent_dir}'
        makedirs(dest_dir, exist_ok=True)
        system(f'cp {path} {PROG_DIR}{parent_dir}')

    for path in dotfile_dirs:
        dest_dir = f'{PROG_DIR}/{path}'
        makedirs(dest_dir, exist_ok=True)
        system(f'cp -r {path} {PROG_DIR}{path.parent}')


def run() -> None:
    args: Namespace = parse_args()
    print(args)

    config: Dict[str, Any] = load_config()

    if ['addchanges', 'add', 'a'].count(args.subcmd) > 0:
        cfiles: Dict[str, Any] = config['files']
        cdirs: Dict[str, Any] = config['directories']

        files: List[Path] = []
        for path in cfiles['etc']:
            files.append(Path(f'/etc/{path}'))

        for path in cfiles['home']:
            files.append(Path(f'{HOME}/{path}'))

        dotfile_dirs: List[Path] = []
        for path in cdirs['home']:
            dotfile_dirs.append(Path(f'{HOME}/{path}'))

        return add_dotfile_changes(files, dotfile_dirs)

    elif ['install', 'i'].count(args.subcmd) > 0:

        if ['configs', 'c'].count(args.installer_subcmd) > 0:
            cfiles: Dict[str, Any] = config['files']
            cdirs: Dict[str, Any] = config['directories']

            for path in cfiles['etc']:
                source = f'{PROG_DIR}/etc/{path}'
                system(f'sudo cp {source} /etc/{path}')

            for path in cfiles['home']:
                source = f'{PROG_DIR}{HOME}/{path}'
                system(f'sudo cp {source} {HOME}/{path}')

            for path in cdirs['home']:
                source = f'{PROG_DIR}{HOME}/{path}'
                makedirs(f'{HOME}/{source}', exist_ok=True)
                system(f'sudo cp -r {source} {HOME}/{path}')

        elif ['pkgs', 'p'].count(args.installer_subcmd) > 0:
            packages: Dict[str, Any] = config['packages']

            if args.extras:
                packages = packages['extras']

            else:
                packages = packages['core']

            if args.distro == 'fedora':
                if ['xorg', 'x'].count(args.pkgs_subcmd) > 0:
                    install_fedora(packages['fedora.xorg'])

                elif ['bspwm', 'b'].count(args.pkgs_subcmd) > 0:
                    install_fedora(packages['fedora.bspwm'])

                elif ['wayland', 'w'].count(args.pkgs_subcmd) > 0:
                    install_fedora(packages['fedora.wayland'])

                elif ['sway', 's'].count(args.pkgs_subcmd) > 0:
                    install_fedora(packages['fedora.sway'])

                else:
                    install_fedora(packages['fedora.common'])

        elif ['flatpaks', 'f'].count(args.installer_subcmd) > 0:
            packages: Dict[str, Any] = config['packages.flatpak']

        else:
            return print(NUFFIN)

    else:
        return print(NUFFIN)


run()
