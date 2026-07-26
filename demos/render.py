#!/usr/bin/env python3
"""Matplotlib fallback renderer for the demo tour (STL lane).

Pure-CPU, headless: binary-STL parsing with numpy, flat-shaded
Poly3DCollection with matplotlib's Agg backend. No GPU, no GL, no
FreeCAD — the zero-dependency fallback, and the renderer that draws
OUR OWN tessellation (FreeCAD re-tessellates from the B-rep, so this
lane is the mesh-path proof).

Scene list comes from <stl_dir>/scenes.json (written by the tour);
multi-body scenes draw every body into one axes with its own color.

Usage: python render.py <stl_dir> <out_dir>
"""

import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
from mpl_toolkits.mplot3d.art3d import Poly3DCollection

LIGHT = np.array([0.35, -0.45, 0.82])
LIGHT = LIGHT / np.linalg.norm(LIGHT)


def read_binary_stl(path):
    """Return (n, 3, 3) float64 triangle vertex array."""
    rec = np.dtype(
        [("normal", "<f4", 3), ("verts", "<f4", (3, 3)), ("attr", "<u2")]
    )
    raw = path.read_bytes()
    (count,) = np.frombuffer(raw, dtype="<u4", count=1, offset=80)
    tris = np.frombuffer(raw, dtype=rec, count=int(count), offset=84)
    return tris["verts"].astype(np.float64)


def orient(verts, up):
    """Rotate so the display up axis is +z (matplotlib's vertical)."""
    if up == "y":  # (x, y, z) -> (x, -z, y): y becomes vertical
        return np.stack(
            [verts[..., 0], -verts[..., 2], verts[..., 1]], axis=-1
        )
    return verts


def shade(verts, base):
    """Flat Lambert shading per triangle (recomputed normals)."""
    n = np.cross(verts[:, 1] - verts[:, 0], verts[:, 2] - verts[:, 0])
    norm = np.linalg.norm(n, axis=1, keepdims=True)
    n = n / np.where(norm == 0, 1, norm)
    lam = np.clip(n @ LIGHT, 0.0, 1.0)
    fill = np.clip(n @ np.array([-0.55, 0.35, 0.15]), 0.0, 1.0)  # soft fill
    lum = 0.30 + 0.60 * lam + 0.12 * fill
    rgb = np.clip(np.outer(lum, np.asarray(base)), 0, 1)
    return np.concatenate([rgb, np.ones((len(rgb), 1))], axis=1)


def cull_backfaces(verts, elev, azim):
    """Drop triangles facing away from the (orthographic) camera.

    Every tour body is a closed, outward-oriented mesh (the kernel's
    tier-3 +V invariant guarantees it), so backface culling is exact —
    and it removes the painter's-algorithm artifacts of hidden faces
    over-drawing near ones.
    """
    el, az = np.radians(elev), np.radians(azim)
    view = np.array(
        [np.cos(el) * np.cos(az), np.cos(el) * np.sin(az), np.sin(el)]
    )
    n = np.cross(verts[:, 1] - verts[:, 0], verts[:, 2] - verts[:, 0])
    return verts[n @ view > 0]


def draw(ax, scene, stl_dir):
    view = scene["view"]
    elev, azim, up = view["elev"], view["azim"], view["up"]
    lo = np.full(3, np.inf)
    hi = np.full(3, -np.inf)
    for body in scene["bodies"]:
        if body["stl"] is None:
            # A #111-pinned body: the STL writer refused its defective
            # tessellation typed; only the FreeCAD/STEP lane can draw it.
            print(f"WARNING: {scene['name']}: body has no STL (#111 pin) — skipped")
            continue
        verts = orient(read_binary_stl(stl_dir / body["stl"]), up)
        front = cull_backfaces(verts, elev, azim)
        colors = shade(front, body["color"])
        ax.add_collection3d(
            Poly3DCollection(
                front,
                facecolors=colors,
                edgecolors=colors,  # match faces: no wireframe, no AA cracks
                linewidths=0.3,
                zsort="average",
            )
        )
        lo = np.minimum(lo, verts.reshape(-1, 3).min(axis=0))
        hi = np.maximum(hi, verts.reshape(-1, 3).max(axis=0))
    c, half = (lo + hi) / 2, (hi - lo).max() / 2 * 1.02
    ax.set_xlim(c[0] - half, c[0] + half)
    ax.set_ylim(c[1] - half, c[1] + half)
    ax.set_zlim(c[2] - half, c[2] + half)
    ax.set_box_aspect((1, 1, 1))
    ax.view_init(elev, azim)
    ax.set_proj_type("ortho")
    ax.set_axis_off()


def main():
    stl_dir, out_dir = Path(sys.argv[1]), Path(sys.argv[2])
    out_dir.mkdir(parents=True, exist_ok=True)
    scenes = json.loads((stl_dir / "scenes.json").read_text())
    for scene in scenes:
        fig = plt.figure(figsize=(5, 5), dpi=130)
        ax = fig.add_subplot(projection="3d")
        draw(ax, scene, stl_dir)
        fig.tight_layout(pad=0.1)
        fig.savefig(out_dir / f"{scene['name']}.png", facecolor="white")
        plt.close(fig)
        print(f"rendered {out_dir / (scene['name'] + '.png')}")


if __name__ == "__main__":
    main()
