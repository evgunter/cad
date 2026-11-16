#!/usr/bin/env python3
"""Draw the band renders from the STLs `make-stl.sh` writes.

    python draw.py <stl-dir> <png-dir>

`<stl-dir>/sizes.txt` names each scene's radius or setback, one
`<scene> <size>` per line, and the panel labels carry it.

One PNG per scene: the operand and each result that built, side by
side, from one camera framed on the blend. The STL is the kernel's own
tessellation; this script only rasterizes it. It reads binary STL with
the tour renderer's reader (`demos/render.py`) and draws with a small
z-buffer of its own, because the tour renderer frames whole bodies
under matplotlib's painter's sort, which can neither crop to a blend
nor order the faces of a concave corner reliably. Normals are averaged
across facets that meet at under `CREASE` degrees, so a fillet reads as
a smooth surface and a sharp edge stays sharp, and an ink line is drawn
where the normal or the depth jumps.
"""

import sys
from pathlib import Path

import numpy as np
from matplotlib import font_manager
from PIL import Image, ImageDraw, ImageFont

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "demos"))
from render import read_binary_stl

PANEL = 520
SS = 3
CREASE = 30.0
BASE = np.array([0.66, 0.74, 0.86])
BG = np.array([1.0, 1.0, 1.0])

# The bodies the sketch plane extrudes along z whose profile is best seen
# standing up: drawn with world (x, y, z) at display (x, -z, y), a proper
# rotation, so the facets keep their outward winding.
STAND_UP = {"valley"}

# name: (title, elev, azim, focus centre in display axes, half-width, panels)
SCENES = {
    "lbar-inside": (
        "L-bar, the single inside edge (PR 6513)",
        30, 40, (0.7, 0.7, 0.5), 1.1, ["before", "fillet", "chamfer"],
    ),
    "boss-vertical": (
        "Inset boss, its vertical edge alone (PR 6513)",
        30, -70, (1.0, 0.5, 1.0), 1.1, ["before", "fillet", "chamfer"],
    ),
    "flush-pair": (
        "Flush pair as a mitre: plate top edge + boss vertical edge, root unrequested (PR 6513)",
        24, -140, (0.0, 0.4, 0.6), 0.75, ["before", "fillet", "chamfer"],
    ),
    "inset-rim": (
        "Inset boss's whole root rim, four concave edges (PR 6513)",
        40, -60, (1.25, 2.0, 0.8), 2.0, ["before", "fillet", "chamfer"],
    ),
    "valley": (
        "Valley between two touching tubes on a plate, filleted (PR 6510)",
        28, -65, (0.0, -0.5, 0.2), 1.6, ["before", "fillet"],
    ),
    "lune-cusp": (
        "Lune cusp, filleted (PR 6510)",
        55, -125, (0.8, 1.7, 0.5), 2.0, ["before", "fillet"],
    ),
    "rounded-plate": (
        "Rounded-rectangle plate, every edge filleted (PR 6520)",
        35, -55, (0.0, 0.0, 0.25), 1.15, ["before", "fillet", "chamfer"],
    ),
    "pocket-floor": (
        "Rounded pocket, its floor filleted (PR 6520)",
        50, -60, (1.5, 1.0, 0.75), 1.15, ["before", "fillet"],
    ),
    "pocket-mouth": (
        "Rounded pocket, its mouth filleted (PR 6520)",
        50, -60, (1.5, 1.0, 0.75), 1.15, ["before", "fillet"],
    ),
}


def camera(elev, azim):
    el, az = np.radians(elev), np.radians(azim)
    view = np.array([np.cos(el) * np.cos(az), np.cos(el) * np.sin(az), np.sin(el)])
    right = np.array([-np.sin(az), np.cos(az), 0.0])
    up = np.cross(view, right)
    return view, right, up


def smooth_normals(tris):
    """Per-corner normals averaged over facets within CREASE of each other."""
    n = np.cross(tris[:, 1] - tris[:, 0], tris[:, 2] - tris[:, 0])
    face = n / np.maximum(np.linalg.norm(n, axis=1, keepdims=True), 1e-300)
    weight = np.linalg.norm(n, axis=1)
    keys = np.round(tris.reshape(-1, 3) / 1e-7).astype(np.int64)
    _, vid = np.unique(keys, axis=0, return_inverse=True)
    vid = vid.reshape(-1)
    order = np.argsort(vid, kind="stable")
    bounds = np.flatnonzero(np.diff(vid[order])) + 1
    cos_crease = np.cos(np.radians(CREASE))
    corner = np.repeat(face, 3, axis=0)
    for group in np.split(order, bounds):
        if len(group) < 2:
            continue
        tri = group // 3
        fn = face[tri]
        agree = (fn @ fn.T) >= cos_crease
        acc = agree @ (fn * weight[tri, None])
        corner[group] = acc / np.maximum(np.linalg.norm(acc, axis=1, keepdims=True), 1e-300)
    return face, corner.reshape(-1, 3, 3)


def rasterize(tris, elev, azim, centre, half):
    size = PANEL * SS
    view, right, up = camera(elev, azim)
    light = view + 0.6 * up - 0.5 * right
    light /= np.linalg.norm(light)
    face, corner = smooth_normals(tris)
    keep = face @ view > 0
    tris, corner = tris[keep], corner[keep]
    rel = tris - np.asarray(centre)
    scale = size / (2.0 * half)
    sx = rel @ right * scale + size / 2
    sy = size / 2 - rel @ up * scale
    sz = rel @ view
    depth = np.full((size, size), -np.inf)
    normal = np.zeros((size, size, 3))
    for i in range(len(tris)):
        x, y, z = sx[i], sy[i], sz[i]
        x0, x1 = max(int(np.floor(x.min())), 0), min(int(np.ceil(x.max())), size - 1)
        y0, y1 = max(int(np.floor(y.min())), 0), min(int(np.ceil(y.max())), size - 1)
        if x0 > x1 or y0 > y1:
            continue
        den = (y[1] - y[2]) * (x[0] - x[2]) + (x[2] - x[1]) * (y[0] - y[2])
        if abs(den) < 1e-12:
            continue
        px, py = np.meshgrid(np.arange(x0, x1 + 1) + 0.5, np.arange(y0, y1 + 1) + 0.5)
        a = ((y[1] - y[2]) * (px - x[2]) + (x[2] - x[1]) * (py - y[2])) / den
        b = ((y[2] - y[0]) * (px - x[2]) + (x[0] - x[2]) * (py - y[2])) / den
        c = 1.0 - a - b
        inside = (a >= -1e-9) & (b >= -1e-9) & (c >= -1e-9)
        d = a * z[0] + b * z[1] + c * z[2]
        win = depth[y0 : y1 + 1, x0 : x1 + 1]
        hit = inside & (d > win)
        if not hit.any():
            continue
        win[hit] = d[hit]
        nrm = a[..., None] * corner[i, 0] + b[..., None] * corner[i, 1] + c[..., None] * corner[i, 2]
        normal[y0 : y1 + 1, x0 : x1 + 1][hit] = nrm[hit]
    covered = np.isfinite(depth)
    depth = np.where(covered, depth, 0.0)
    nn = normal / np.maximum(np.linalg.norm(normal, axis=2, keepdims=True), 1e-300)
    lam = np.clip(nn @ light, 0.0, 1.0)
    spec = np.clip(nn @ ((light + view) / np.linalg.norm(light + view)), 0.0, 1.0) ** 40
    lum = 0.32 + 0.62 * lam
    rgb = np.clip(lum[..., None] * BASE + 0.25 * spec[..., None], 0.0, 1.0)
    img = np.where(covered[..., None], rgb, BG)
    ink = np.zeros((size, size), bool)
    jump = 0.02 * half
    for dy, dx in ((0, 1), (1, 0)):
        a_cov, b_cov = covered[: size - dy, : size - dx], covered[dy:, dx:]
        edge = a_cov != b_cov
        both = a_cov & b_cov
        bend = (nn[: size - dy, : size - dx] * nn[dy:, dx:]).sum(axis=2) < np.cos(np.radians(CREASE))
        step = np.abs(depth[: size - dy, : size - dx] - depth[dy:, dx:]) > jump
        line = edge | (both & (bend | step))
        ink[: size - dy, : size - dx] |= line
    grow = ink.copy()
    grow[1:] |= ink[:-1]
    grow[:, 1:] |= ink[:, :-1]
    img[grow] = 0.18
    img = img.reshape(PANEL, SS, PANEL, SS, 3).mean(axis=(1, 3))
    return Image.fromarray((img * 255).round().astype(np.uint8))


def font(size):
    return ImageFont.truetype(font_manager.findfont("DejaVu Sans"), size)


def scene_png(stl_dir, name, spec, size):
    title, elev, azim, centre, half, panels = spec
    shown = [p for p in panels if (stl_dir / f"{name}-{p}.stl").exists()]
    head, foot = 44, 30
    sheet = Image.new("RGB", (PANEL * len(shown), PANEL + head + foot), "white")
    draw = ImageDraw.Draw(sheet)
    draw.text((12, 10), title, fill="black", font=font(19))
    for k, panel in enumerate(shown):
        tris = read_binary_stl(stl_dir / f"{name}-{panel}.stl")
        if name in STAND_UP:
            tris = np.stack([tris[..., 0], -tris[..., 2], tris[..., 1]], axis=-1)
        sheet.paste(rasterize(tris, elev, azim, centre, half), (PANEL * k, head))
        label = panel if panel == "before" else f"{panel} {size}"
        draw.text((PANEL * k + PANEL // 2, head + PANEL + 4), label, fill="black", font=font(18), anchor="ma")
        if k:
            draw.line([(PANEL * k, head), (PANEL * k, head + PANEL)], fill=(200, 200, 200), width=1)
    return sheet, shown


def main():
    stl_dir, png_dir = Path(sys.argv[1]), Path(sys.argv[2])
    png_dir.mkdir(parents=True, exist_ok=True)
    sizes = dict(line.split() for line in (stl_dir / "sizes.txt").read_text().splitlines() if line.strip())
    for name, spec in SCENES.items():
        if not (stl_dir / f"{name}-before.stl").exists():
            continue
        sheet, shown = scene_png(stl_dir, name, spec, sizes[name])
        if len(shown) < 2:
            print(f"{name}: no result built, nothing drawn")
            continue
        out = png_dir / f"{name}.png"
        sheet.save(out, optimize=True)
        print(f"{out}: {', '.join(shown)}")


if __name__ == "__main__":
    main()
