# scott

A simple cross-platform GUI file manager written in rust using [egui](https://github.com/emilk/egui) and [eframe](https://github.com/emilk/egui/tree/master/crates/eframe).

## Building

### Windows build

Building for windows, from WSL:

Make sure a windows compatible linker is installed:

```bash
sudo apt install mingw-w64
```

Then build the app:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

## Troubleshooting

1. In case of the following error: ` Found no glutin configs matching the template: ConfigTemplate`.

   Install the `mesa-utils` package:
   `sudo apt install mesa-utils`

   And make sure this command displays some output:
   `glxinfo | grep OpenGL`
