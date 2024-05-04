# Color editor

GUI program to edit color values in any text file. May be useful for tuning color schemes. Tested on Linux.

https://github.com/pekkaran/color-editor/assets/40019233/db5a195a-dd24-4449-b384-7a729613a353

In the example video the program is shown at the bottom left. It has been opened to edit an [alacritty](https://github.com/alacritty/alacritty) terminal emulator configuration file. On the background there are open two alacritty windows showing text editors. The right one shows the configuration file. Now, alacritty has a feature to reload the configuration whenever the file changes, and the color editor has been set to save the file on every edit. As a result the color picker tunes the colors in real-time.

In this example the text file is structured (TOML), but it doesn't have to be. The color strings are found using regular expressions from any type of text file.

## Installation and usage

[Install Rust](https://www.rust-lang.org/tools/install), then at the root of the repository run:

```bash
cargo run -- my_text_file.txt
```

See help with `cargo run -- --help`.

Be sure to backup your files before editing them.

## Features

Currently supports color strings of the form `#47ab3d`, `0x47ab3d`, `#4a3`, `0x4a3`. Supporting other formats such as `[125, 200, 83]` should not be too difficult.
