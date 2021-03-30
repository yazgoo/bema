# bema

presentation in the terminal, within a self contained binary.

![demo](https://raw.githubusercontent.com/yazgoo/bema/gh-pages/screenshot.gif)

# usage

## in CLI

Invoke the program with no argument.
For now, full definition images are only supported within [kitty](https://sw.kovidgoyal.net/kitty/),
otherwise the program will fallback on [blockish](https://github.com/yazgoo/blockish/).

## with hovercraft

Just invoke the program with "hovercraft" as argument,
this will output an [hovercraft](https://hovercraft.readthedocs.io) file that you
can then interpret with hovercraft.

`cargo run --example basic hovercraft > pres.hc && hovercraft pres.hc`
