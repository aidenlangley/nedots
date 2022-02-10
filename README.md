# nedots

## Installer



## Project Structure

`etc` & `home` contain symlinks to config files that are less structured than
those that typically live in `.config`, such as X11 config files that live
amongst other config files that we don't care about. We symlink these so that we
can conveniently edit them without concerning ourselves with all config files.
