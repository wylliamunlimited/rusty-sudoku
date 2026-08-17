"""Render captured terminal frames into a looping GIF."""

import argparse
import os
import re
from PIL import Image, ImageChops, ImageDraw, ImageFont

MARGIN = 4

MPL = "/Users/wylliamcheng/miniconda3/lib/python3.13/site-packages/matplotlib/mpl-data/fonts/ttf"
MONO = os.path.join(MPL, "DejaVuSansMono.ttf")
BRAILLE = os.path.join(MPL, "DejaVuSans.ttf")

THEMES = {
    "dark": dict(bg="#0d1117", fg="#d8dee9", dim="#4b535e", chrome="#161b22", dot=("#ff5f57", "#febc2e", "#28c840")),
    "midnight": dict(bg="#11111b", fg="#cdd6f4", dim="#585b70", chrome="#181825", dot=("#f38ba8", "#f9e2af", "#a6e3a1")),
    "light": dict(bg="#ffffff", fg="#24292f", dim="#afb8c1", chrome="#f6f8fa", dot=("#ff5f57", "#febc2e", "#28c840")),
}

CSI = re.compile(r"\x1b\[([0-9;]*)m")


def parse(text):
    """Split an ANSI-styled frame into rows of (char, dim) pairs."""
    rows = []
    for line in text.split("\n"):
        cells, dim, i = [], False, 0
        while i < len(line):
            m = CSI.match(line, i)
            if m:
                for code in (m.group(1) or "0").split(";"):
                    if code in ("", "0"):
                        dim = False
                    elif code == "2":
                        dim = True
                i = m.end()
                continue
            cells.append((line[i], dim))
            i += 1
        rows.append(cells)
    while rows and not rows[-1]:
        rows.pop()
    return rows


def fit_braille_size(cell_w, start):
    """Pick the braille point size whose advance best fills one cell."""
    best, best_err = start, 1e9
    for size in range(max(6, start - 10), start + 14):
        f = ImageFont.truetype(BRAILLE, size)
        err = abs(f.getlength("⣿") - cell_w)
        if err < best_err:
            best, best_err = size, err
    return best


def render(frames, theme, size, pad, chrome, scale):
    t = THEMES[theme]
    mono = ImageFont.truetype(MONO, size)
    cell_w = round(mono.getlength("M"))
    asc, desc = mono.getmetrics()
    cell_h = round((asc + desc) * 1.12)

    braille = ImageFont.truetype(BRAILLE, fit_braille_size(cell_w, size))

    tile = (cell_w + MARGIN * 2, cell_h + MARGIN * 2)
    cache = {}

    def glyph(ch):
        """One glyph rasterised into its own tile, drawn at a fixed origin.

        Box-drawing glyphs overhang their advance so that neighbours join up.
        Rasterising a whole run in one call makes FreeType *add* those
        overlapping coverages, which brightens a seam at every cell boundary.
        Compositing per-glyph tiles with a max instead keeps the join solid.
        """
        if ch not in cache:
            img = Image.new("L", tile, 0)
            d = ImageDraw.Draw(img)
            if 0x2800 <= ord(ch) <= 0x28FF:
                ox = MARGIN + (cell_w - braille.getlength(ch)) / 2
                d.text((ox, MARGIN), ch, font=braille, fill=255)
            else:
                d.text((MARGIN, MARGIN), ch, font=mono, fill=255)
            cache[ch] = img
        return cache[ch]

    def stamp(layer, ch, px, py):
        box = (px - MARGIN, py - MARGIN)
        region = layer.crop((box[0], box[1], box[0] + tile[0], box[1] + tile[1]))
        layer.paste(ImageChops.lighter(region, glyph(ch)), box)

    cols = max(len(r) for f in frames for r in f)
    rows = max(len(f) for f in frames)
    bar = int(cell_h * 1.4) if chrome else 0

    w = int(cols * cell_w + pad * 2)
    h = int(rows * cell_h + pad * 2 + bar)

    images = []
    for frame in frames:
        img = Image.new("RGB", (w, h), t["bg"])
        d = ImageDraw.Draw(img)

        if chrome:
            d.rectangle([0, 0, w, bar], fill=t["chrome"])
            r = bar * 0.16
            for i, colour in enumerate(t["dot"]):
                cx = pad + r + i * r * 3.4
                cy = bar / 2
                d.ellipse([cx - r, cy - r, cx + r, cy + r], fill=colour)

        layers = {False: Image.new("L", (w, h), 0), True: Image.new("L", (w, h), 0)}
        for y, row in enumerate(frame):
            py = pad + bar + y * cell_h
            for x, (ch, dim) in enumerate(row):
                if ch != " ":
                    stamp(layers[dim], ch, pad + x * cell_w, py)

        img.paste(t["fg"], (0, 0), layers[False])
        img.paste(t["dim"], (0, 0), layers[True])

        if scale != 1:
            img = img.resize((int(w * scale), int(h * scale)), Image.LANCZOS)
        # Dithering scatters the antialiased edges of the box-drawing strokes
        # into a visible seam at every cell boundary. The palette is tiny and
        # smooth, so nothing needs dithering.
        images.append(
            img.quantize(colors=64, method=Image.MEDIANCUT, dither=Image.Dither.NONE)
        )

    return images


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("frames")
    ap.add_argument("out")
    ap.add_argument("--count", type=int, default=26)
    ap.add_argument("--theme", default="dark", choices=list(THEMES))
    ap.add_argument("--size", type=int, default=22)
    ap.add_argument("--pad", type=int, default=18)
    ap.add_argument("--duration", type=int, default=80)
    ap.add_argument("--scale", type=float, default=1.0)
    ap.add_argument("--no-chrome", action="store_true")
    a = ap.parse_args()

    raw = open(a.frames, encoding="utf-8").read()
    frames = [parse(f) for f in raw.split("\x0c") if f][: a.count]

    images = render(frames, a.theme, a.size, a.pad, not a.no_chrome, a.scale)
    images[0].save(
        a.out,
        save_all=True,
        append_images=images[1:],
        duration=a.duration,
        loop=0,
        optimize=True,
        disposal=2,
    )
    print(f"{a.out}  {len(images)} frames  {images[0].size[0]}x{images[0].size[1]}  "
          f"{os.path.getsize(a.out) / 1024:.0f} KB")


if __name__ == "__main__":
    main()
