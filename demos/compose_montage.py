#!/usr/bin/env python3
"""Compose the montage contact sheet from per-scene renders.

Reads <outdir>/scenes.json for order + captions, loads
<renderdir>/<name>.png (from any renderer), trims near-white
margins, and lays the scenes out on a grid with captions.

`--montage=NAME` writes the sheet under a different filename and
`--banner=TEXT` adds a provenance banner line under the title — the
FreeCAD/OCC STEP-lane montage uses both, on the SAME grid/captions/
scene order so the two sheets are cell-for-cell comparable.
`--title=TEXT` overrides the sheet title (the wild-corpus sheet is not
the demo tour, so it must not claim the tour's title). A scene
whose render is missing gets a labeled placeholder cell (reason from
<renderdir>/<name>.fail.txt when present) — never a silent gap. A
scene whose render exists but is deliberately EMPTY says so too, from
<renderdir>/<name>.note.txt: a lane that had nothing to draw and a
lane that wedged both produce a blank cell, and the two must not look
the same.

Usage: python compose_montage.py <outdir> <renderdir>
           [--montage=NAME] [--banner=TEXT] [--title=TEXT]
"""

import sys
import textwrap
from pathlib import Path

import manifest
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


def placeholder(ax, name, renderdir):
    """Labeled failure cell: name the failure, never leave a silent gap."""
    fail = renderdir / f"{name}.fail.txt"
    reason = fail.read_text().strip() if fail.exists() else "no render produced"
    ax.add_patch(
        plt.Rectangle(
            (0, 0), 1, 1, transform=ax.transAxes, facecolor="0.93",
            edgecolor="0.6", linewidth=1.2, zorder=0,
        )
    )
    ax.text(
        0.5, 0.5, f"RENDER FAILED\n{textwrap.fill(reason, 36)}",
        transform=ax.transAxes, ha="center", va="center",
        fontsize=8, color="0.25", family="monospace",
    )


def note_stamp(ax, name, renderdir):
    """Neutral stamp for a cell that is MISSING SOMETHING on purpose.

    Two shapes of missing, and the note names which: a cell that is
    blank because every body it would draw was skipped, and a cell that
    drew a picture with a body LEFT OUT of it. The second is the more
    deceiving — it looks finished — so it gets the stamp too, over the
    picture.

    Deliberately NOT the failure placeholder: no box, no red, no
    "FAILED". This cell is a gate the tour declared and pinned, not
    something that went wrong, and a reader who cannot tell the two
    apart learns the wrong thing from the sheet.
    """
    note = renderdir / f"{name}.note.txt"
    if not note.exists():
        return
    ax.text(
        0.5, 0.5, textwrap.fill(note.read_text().strip(), 34),
        transform=ax.transAxes, ha="center", va="center",
        fontsize=8, color="0.35", family="monospace", style="italic",
    )


def main():
    args = sys.argv[1:]
    montage_name = "montage.png"
    banner = None
    title = "B-rep kernel demo tour — sweeps, booleans, split, and the M4 recipe layer"
    pos = []
    for a in args:
        if a.startswith("--montage="):
            montage_name = a.split("=", 1)[1]
        elif a.startswith("--banner="):
            banner = a.split("=", 1)[1]
        elif a.startswith("--title="):
            title = a.split("=", 1)[1]
        else:
            pos.append(a)
    outdir, renderdir = Path(pos[0]), Path(pos[1])
    scenes = [s for s in manifest.read_scenes(outdir) if s.montage]
    rows = -(-len(scenes) // COLS)
    fig = plt.figure(figsize=(3.4 * COLS, 3.1 * rows), dpi=120)
    for i, scene in enumerate(scenes, start=1):
        ax = fig.add_subplot(rows, COLS, i)
        png = renderdir / f"{scene.name}.png"
        if png.exists():
            ax.imshow(trim(plt.imread(png)))
            note_stamp(ax, scene.name, renderdir)
        else:
            placeholder(ax, scene.name, renderdir)
        ax.set_title(scene.caption, fontsize=11, pad=3)
        ax.set_axis_off()
    if banner:
        fig.suptitle(title, fontsize=15, y=0.995, va="top")
        fig.text(
            0.5, 0.968, banner, ha="center", va="top",
            fontsize=12, color="#8a3324", weight="bold",
        )
        rect_top = 0.955
    else:
        fig.suptitle(title, fontsize=15)
        rect_top = 0.97
    fig.tight_layout(pad=0.4, rect=(0, 0, 1, rect_top))
    fig.savefig(renderdir / montage_name, facecolor="white")
    print(f"rendered {renderdir / montage_name}")


if __name__ == "__main__":
    main()
