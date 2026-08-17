# Asset generation

`assets/menu.gif` is rendered offline rather than screen-recorded, so it is
reproducible and does not depend on a terminal emulator or its font settings.

```sh
# 1. capture menu frames (the cube animation is a pure function of the frame
#    counter, so the same command always produces the same frames)
cargo run --release --example dump_menu_frames -- 30 /tmp/frames.txt

# 2. rasterise them into a looping GIF
python3 tools/render_gif.py /tmp/frames.txt assets/menu.gif --count 26
```

26 frames is one quarter turn. The cube has four-fold rotational symmetry, so
that is the shortest seamless loop — frame 26 differs from frame 0 by 22
characters out of a full screen, which is invisible in motion. A full
revolution is 105 frames and loops worse, because 2*pi / 0.06 is not an
integer.

Requires Pillow, and DejaVu Sans Mono for the box-drawing glyphs plus DejaVu
Sans for Braille. No macOS system monospace font covers the Braille block, so
`render_gif.py` picks both up from matplotlib's bundled font directory.

`--theme dark|midnight|light`, `--size`, `--scale` and `--no-chrome` adjust the
output.
