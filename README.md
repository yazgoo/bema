# 🎤 bema 

Write your next slideshow in rust 🦀, get a self-contained binary 📦.

## 🖊  DSL

see [examples/](examples).

## ♻ frontends

### ⌨ Terminal

![demo](https://raw.githubusercontent.com/yazgoo/bema/gh-pages/screenshot.gif)

Invoke the program with no argument.
For now, full definition images are only supported within [kitty](https://sw.kovidgoyal.net/kitty/),
otherwise the program will fallback on [blockish](https://github.com/yazgoo/blockish/).

### 🖌  GUI

![demo](https://raw.githubusercontent.com/yazgoo/bema/gh-pages/screenshot_gui.gif)

Invoke the program with `gui` as argument.

### 🕸  in browser with hovercraft

Just invoke the program with `hovercraft` as argument,
this will output an [hovercraft](https://hovercraft.readthedocs.io) file (as well as images) that you
can then interpret with hovercraft.

`cargo run --example basic hovercraft > pres.hc && hovercraft pres.hc`
