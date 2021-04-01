# bema

self-contained slideshows in rust

# usage

## in CLI

![demo](https://raw.githubusercontent.com/yazgoo/bema/gh-pages/screenshot.gif)

Invoke the program with no argument.
For now, full definition images are only supported within [kitty](https://sw.kovidgoyal.net/kitty/),
otherwise the program will fallback on [blockish](https://github.com/yazgoo/blockish/).

## in GUI

![demo](https://raw.githubusercontent.com/yazgoo/bema/gh-pages/screenshot_gui.gif)

Invoke the program with "gui" as argument.
No syntax highlighting is available in this mode.

## with hovercraft

Just invoke the program with "hovercraft" as argument,
this will output an [hovercraft](https://hovercraft.readthedocs.io) file that you
can then interpret with hovercraft.

`cargo run --example basic hovercraft > pres.hc && hovercraft pres.hc`
