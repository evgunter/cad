"""Side-by-side renders, ours | OCCT, of the soups `occt_compare.py` and our probe wrote.

Draws with `draw_soup.py`'s own rasterizer, so both columns share its viewpoints and shading.

    python -I occt_draw.py <draw_soup-dir> <ours-dir> <occt-dir> <png-dir> [tag ...]
"""

import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np

sys.path.insert(0, sys.argv[1])
from draw_soup import load, raster

FLUSH = ((-0.7, -0.7, -0.1), (0.7, 0.7, 0.9))
INSET = ((-0.6, -0.6, -0.3), (0.4, 0.4, 0.7))
NEAR = ((-0.15, 0.25, 0.25), (0.75, 0.85, 0.8))
FAR = ((-0.15, 3.15, 0.25), (0.75, 3.75, 0.8))
WHOLE = ((0.0, 0.0, 0.0), (6.0, 4.0, 1.75))
# The shots of `draw_soup.py`'s odd_plus_one, per fixture.
SHOTS = {
    "flush": [(-25, 45, FLUSH), (-45, 10, FLUSH)],
    "inset": [(30, 45, INSET), (30, 15, INSET)],
    "flush120": [(-25, 60, FLUSH), (-45, 20, FLUSH)],
    "bracket": [(35, -50, NEAR), (25, -140, NEAR), (35, 50, FAR)],
}
REQ = {
    "a": "odd + one pair edge",
    "b": "all three",
    "c": "odd alone",
    "d": "one pair edge",
    "e": "both pair edges",
}


def side_by_side(ours, occt, title, shots, out):
    cols = [(p, who) for p, who in ((ours, "ours"), (occt, "OCCT 8.0.1")) if p is not None]
    soups = [load(p) for p, _ in cols]
    fig, axes = plt.subplots(len(shots), len(cols), figsize=(6 * len(cols), 6.4 * len(shots)), dpi=90,
                             squeeze=False)
    for row, (elev, azim, window) in zip(axes, shots, strict=True):
        lo, hi = np.array(window[0]), np.array(window[1])
        for ax, (v, fs), (_, who) in zip(row, soups, cols, strict=True):
            ax.imshow(raster(v, fs, elev, azim, 700, (lo, hi)))
            ax.set_axis_off()
            ax.set_title(f"{who}: {title}\nelev {elev}°, azim {azim}°", fontsize=9)
    fig.tight_layout(rect=(0, 0, 1, 0.98))
    fig.savefig(out)
    plt.close(fig)


def main():
    ours, occt, png = Path(sys.argv[2]), Path(sys.argv[3]), Path(sys.argv[4])
    png.mkdir(parents=True, exist_ok=True)
    tags = sys.argv[5:]
    for fixture, shots in SHOTS.items():
        for req, what in REQ.items():
            for verb in ("chamfer", "fillet"):
                tag = f"{fixture}-{req}-{verb}"
                if tags and tag not in tags:
                    continue
                a, b = ours / f"ours-{tag}.soup", occt / f"occt-{tag}.soup"
                a, b = (a if a.exists() else None), (b if b.exists() else None)
                if a is None and b is None:
                    continue
                s = 0.05 if fixture == "bracket" else 0.1
                side_by_side(a, b, f"{fixture}, ({req}) {what}, {verb} {s}", shots, png / f"occt-{tag}.png")


if __name__ == "__main__":
    main()
