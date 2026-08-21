#!/usr/bin/env python3
"""Matplotlib fallback renderer for the demo tour (STL lane).

Pure-CPU, headless: binary-STL parsing with numpy, flat-shaded
Poly3DCollection with matplotlib's Agg backend. No GPU, no GL, no
FreeCAD — the zero-dependency fallback, and the renderer that draws
OUR OWN tessellation (FreeCAD re-tessellates from the B-rep, so this
lane is the mesh-path proof).

Scene list comes from <stl_dir>/scenes.json, read through `manifest`.
BOTH producers reach this renderer: the tour's manifest on the kernel
lane, and the wild generator's on `render-wild.sh`'s. Multi-body
scenes draw every body into one axes with its own color.

`--author=TEXT` stamps an `Author` tEXt chunk into every PNG (on top
of matplotlib's own deterministic `Software` chunk). The wild-corpus
lane (`render-wild.sh`) uses it as its provenance signature: there
matplotlib is the PRIMARY renderer, not a fallback, and the guard
(`check_render_provenance.py`) accepts a committed wild cell only with
that stamp — so a tour fallback frame (unstamped) can never pass as a
wild cell. The tour's fallback path passes no `--author`, exactly so
its frames stay recognizable as fallback output.

Usage: python render.py <stl_dir> <out_dir> [--author=TEXT]
"""

import sys
from pathlib import Path

import manifest
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


def orient(verts, axes):
    """Rotate world vertices into the z-up display frame.

    `axes` is the scene's world -> display spec from `manifest`, which
    is where the convention it encodes is defined.

    This applies the spec rather than calling `manifest.apply_axes`,
    which does the same thing for one 3-tuple: here the input is an
    (n, 3, 3) array and the whole point is to move every vertex in one
    numpy op. The two applications cannot disagree about the CAMERA,
    which is what the convention is for -- both read the same spec, and
    a sign or an axis lives only in the table. They could disagree
    about how a spec is applied, which is arithmetic a reader can check
    against the line above in full.
    """
    return np.stack([sign * verts[..., src] for src, sign in axes], axis=-1)


def shade(verts, base, alpha=1.0):
    """Flat Lambert shading per triangle (recomputed normals)."""
    n = np.cross(verts[:, 1] - verts[:, 0], verts[:, 2] - verts[:, 0])
    norm = np.linalg.norm(n, axis=1, keepdims=True)
    n = n / np.where(norm == 0, 1, norm)
    lam = np.clip(n @ LIGHT, 0.0, 1.0)
    fill = np.clip(n @ np.array([-0.55, 0.35, 0.15]), 0.0, 1.0)  # soft fill
    lum = 0.30 + 0.60 * lam + 0.12 * fill
    rgb = np.clip(np.outer(lum, np.asarray(base)), 0, 1)
    return np.concatenate([rgb, np.full((len(rgb), 1), alpha)], axis=1)


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
    """Draw every body of one scene and frame the axes around them.

    This lane draws the STL only; `step` is never read here, so the
    two producers disagreeing on it is not this renderer's question.
    """
    elev, azim = scene.elev, scene.azim
    axes = scene.world_to_display()
    lo = np.full(3, np.inf)
    hi = np.full(3, -np.inf)
    for body in scene.bodies:
        verts = orient(read_binary_stl(stl_dir / body.stl), axes)
        # Transparency (a scene property, carried in the manifest):
        # backfaces are what a see-through body is FOR, so a
        # transparent body keeps them and the painter's z-sort does
        # the rest. Opaque bodies still cull exactly, as before.
        alpha = 1.0 - body.transparency / 100.0
        front = verts if alpha < 1.0 else cull_backfaces(verts, elev, azim)
        colors = shade(front, body.color, alpha)
        ax.add_collection3d(
            Poly3DCollection(
                front,
                facecolors=colors,
                # Opaque: edges match faces (no wireframe, no AA
                # cracks). Transparent: no edges at all — a
                # semi-transparent edge drawn over its own face
                # double-darkens every triangle boundary and the body
                # reads as a wire mesh.
                edgecolors=colors if alpha >= 1.0 else "none",
                linewidths=0.3 if alpha >= 1.0 else 0.0,
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
    args = sys.argv[1:]
    author = None
    pos = []
    for a in args:
        if a.startswith("--author="):
            author = a.split("=", 1)[1]
        else:
            pos.append(a)
    stl_dir, out_dir = Path(pos[0]), Path(pos[1])
    metadata = {"Author": author} if author else None
    out_dir.mkdir(parents=True, exist_ok=True)
    for scene in manifest.read_scenes(stl_dir):
        fig = plt.figure(figsize=(5, 5), dpi=130)
        ax = fig.add_subplot(projection="3d")
        draw(ax, scene, stl_dir)
        fig.tight_layout(pad=0.1)
        png = out_dir / f"{scene.name}.png"
        fig.savefig(png, facecolor="white", metadata=metadata)
        plt.close(fig)
        print(f"rendered {png}")


if __name__ == "__main__":
    main()
