#!/usr/bin/env python3
"""Compose the montage contact sheet from per-scene renders.

Reads <outdir>/scenes.json for order + captions, loads
<renderdir>/<name>.png (from either renderer), trims near-white
margins, and lays the scenes out on a grid with captions.

Usage: python compose_montage.py <outdir> <renderdir>
"""

import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np

COLS = 4


def trim(img, thresh=0.985, pad=6):
    """Crop near-white margins (keeps a small border)."""
    gray = img[..., :3].mean(axis=2)
    content = gray < thresh
    rows = np.flatnonzero(content.any(axis=1))
    cols = np.flatnonzero(content.any(axis=0))
    if rows.size == 0 or cols.size == 0:
        return img
    r0, r1 = max(rows[0] - pad, 0), min(rows[-1] + pad, img.shape[0] - 1)
    c0, c1 = max(cols[0] - pad, 0), min(cols[-1] + pad, img.shape[1] - 1)
    return img[r0 : r1 + 1, c0 : c1 + 1]


def main():
    outdir, renderdir = Path(sys.argv[1]), Path(sys.argv[2])
    scenes = json.loads((outdir / "scenes.json").read_text())
    rows = -(-len(scenes) // COLS)
    fig = plt.figure(figsize=(3.4 * COLS, 3.1 * rows), dpi=120)
    for i, scene in enumerate(scenes, start=1):
        ax = fig.add_subplot(rows, COLS, i)
        img = trim(plt.imread(renderdir / f"{scene['name']}.png"))
        ax.imshow(img)
        ax.set_title(scene["caption"], fontsize=11, pad=3)
        ax.set_axis_off()
    fig.suptitle(
        "B-rep kernel demo tour — sweeps, booleans, split, and the M4 recipe layer",
        fontsize=15,
    )
    fig.tight_layout(pad=0.4, rect=(0, 0, 1, 0.97))
    fig.savefig(renderdir / "montage.png", facecolor="white")
    print(f"rendered {renderdir / 'montage.png'}")


if __name__ == "__main__":
    main()
