# 🗣  bema 

[![Discord](https://img.shields.io/discord/591914197219016707.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/F684Y8rYwZ)

Write your next slideshow in rust 🦀, as a self-contained binary 📦.

## 🦀 DSL

See [examples/basic.rs](examples/basic.rs).

## 👀 frontends

There are several ways you can display your slideshow.

### 🖥  GUI

![demo](https://raw.githubusercontent.com/yazgoo/bema/gh-pages/screenshot_gui.gif)
`cargo run --example basic gui`

Invoke the program with `gui` as argument.
Press `escape` for help on usage keys.

### 💾 Terminal

![demo](https://raw.githubusercontent.com/yazgoo/bema/gh-pages/screenshot.gif)
`cargo run --example basic`

Invoke the program with no argument.
For now, full definition images are only supported within [kitty](https://sw.kovidgoyal.net/kitty/),
otherwise the program will fallback on [blockish](https://github.com/yazgoo/blockish/).
Use arrow keys or `hjkl` to navigate, `q` to quit.

### 🕸  in browser with hovercraft

`cargo run --example basic hovecraft`

Just invoke the program with `hovercraft` as argument.
This will output an [hovercraft](https://hovercraft.readthedocs.io) file (as well as images) that you
can then interpret with hovercraft:

`cargo run --example basic hovercraft > pres.hc && hovercraft pres.hc`
