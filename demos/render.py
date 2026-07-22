#!/usr/bin/env python3
"""Render the demo-tour STLs to PNGs (one per body + a contact sheet).

Pure-CPU, headless: binary-STL parsing with numpy, flat-shaded
Poly3DCollection with matplotlib's Agg backend. No GPU, no GL.

Usage: python render.py <stl_dir> <out_dir>
"""

import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
from mpl_toolkits.mplot3d.art3d import Poly3DCollection

# Per-body presentation: base RGB, (elev, azim) view, and which STL
# axis is "up" for display (extrusions sweep +z; revolves spin about y).
BODIES = {
    "bracket": {"color": (0.36, 0.56, 0.86), "view": (32, -55), "up": "z"},
    "plate": {"color": (0.86, 0.51, 0.27), "view": (42, -60), "up": "z"},
    "vase": {"color": (0.42, 0.72, 0.50), "view": (16, -55), "up": "y"},
    "donut": {"color": (0.78, 0.42, 0.72), "view": (38, -50), "up": "y"},
    "pulley": {"color": (0.74, 0.68, 0.30), "view": (22, -55), "up": "y"},
    "wedge": {"color": (0.44, 0.68, 0.78), "view": (35, -40), "up": "y"},
    # The boolean leg (M3 PR 5). voidbox_cutaway is exported only when
    # the multi-shell cutaway subtract succeeds; missing STLs from this
    # group are skipped with a warning (the tour narrates why).
    "table": {"color": (0.62, 0.45, 0.28), "view": (12, -55), "up": "z",
              "optional": True},
    "openbox": {"color": (0.40, 0.60, 0.72), "view": (38, -125), "up": "z",
                "optional": True},
    "voidbox": {"color": (0.58, 0.58, 0.64), "view": (30, -55), "up": "z",
                "optional": True},
    "voidbox_cutaway": {"color": (0.58, 0.58, 0.64), "view": (28, -55),
                        "up": "z", "optional": True},
}

LIGHT = np.array([0.35, -0.45, 0.82])  # camera side: views sit at azim ~ -50
LIGHT = LIGHT / np.linalg.norm(LIGHT)


def read_binary_stl(path):
    """Return (n, 3, 3) float32 triangle vertex array."""
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


def draw(ax, verts, cfg):
    v = orient(verts, cfg["up"])
    front = cull_backfaces(v, *cfg["view"])
    colors = shade(front, cfg["color"])
    ax.add_collection3d(
        Poly3DCollection(
            front,
            facecolors=colors,
            edgecolors=colors,  # match faces: no wireframe, no AA cracks
            linewidths=0.3,
            zsort="average",
        )
    )
    lo, hi = v.reshape(-1, 3).min(axis=0), v.reshape(-1, 3).max(axis=0)
    c, half = (lo + hi) / 2, (hi - lo).max() / 2 * 1.02
    ax.set_xlim(c[0] - half, c[0] + half)
    ax.set_ylim(c[1] - half, c[1] + half)
    ax.set_zlim(c[2] - half, c[2] + half)
    ax.set_box_aspect((1, 1, 1))
    ax.view_init(*cfg["view"])
    ax.set_proj_type("ortho")
    ax.set_axis_off()


def main():
    stl_dir, out_dir = Path(sys.argv[1]), Path(sys.argv[2])
    out_dir.mkdir(parents=True, exist_ok=True)
    meshes = {}
    for name, cfg in BODIES.items():
        path = stl_dir / f"{name}.stl"
        if not path.exists():
            if cfg.get("optional"):
                print(f"skipping {name} (no {path} — see the tour narration)")
                continue
            sys.exit(f"missing {path} — run the tour first")
        meshes[name] = read_binary_stl(path)

    for name, cfg in BODIES.items():
        if name not in meshes:
            continue
        fig = plt.figure(figsize=(5, 5), dpi=130)
        ax = fig.add_subplot(projection="3d")
        draw(ax, meshes[name], cfg)
        ax.set_title(name, fontsize=13, pad=0)
        fig.tight_layout(pad=0.1)
        fig.savefig(out_dir / f"{name}.png", facecolor="white")
        plt.close(fig)
        print(f"rendered {out_dir / f'{name}.png'} "
              f"({len(meshes[name])} triangles)")

    present = [n for n in BODIES if n in meshes]
    cols = 3
    rows = -(-len(present) // cols)
    fig = plt.figure(figsize=(10.5, 3.5 * rows), dpi=120)
    for i, name in enumerate(present, start=1):
        ax = fig.add_subplot(rows, cols, i, projection="3d")
        draw(ax, meshes[name], BODIES[name])
        ax.set_title(name, fontsize=12, pad=0)
    fig.suptitle("B-rep kernel demo tour — sweeps + M3 boolean ops",
                 fontsize=14)
    fig.tight_layout(pad=0.2, rect=(0, 0, 1, 0.96))
    fig.savefig(out_dir / "montage.png", facecolor="white")
    plt.close(fig)
    print(f"rendered {out_dir / 'montage.png'}")


if __name__ == "__main__":
    main()
