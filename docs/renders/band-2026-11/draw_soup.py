"""Z-buffered orthographic renders of a triangle soup ("v x y z" / "f a b c kind"), the bracket soups of `bracket_soups.rs`."""
import sys
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

COLORS = {0: (0.80, 0.82, 0.86), 1: (0.38, 0.58, 0.88), 2: (0.96, 0.58, 0.22), 3: (0.32, 0.78, 0.42), 4: (0.8, 0.2, 0.8)}

def load(path):
    vs, fs = [], []
    for line in open(path):
        p = line.split()
        if p[0] == "v":
            vs.append([float(x) for x in p[1:4]])
        else:
            fs.append((int(p[1]), int(p[2]), int(p[3]), int(p[4])))
    return np.array(vs), fs

def camera(elev, azim):
    e, a = np.radians(elev), np.radians(azim)
    d = np.array([np.cos(e) * np.cos(a), np.cos(e) * np.sin(a), np.sin(e)])  # toward viewer
    right = np.cross([0, 0, 1], d); right /= np.linalg.norm(right)
    up = np.cross(d, right)
    return d, right, up

def raster(v, fs, elev, azim, size, window):
    d, right, up = camera(elev, azim)
    lo, hi = window
    c = (lo + hi) / 2
    span = (hi - lo).max() * 0.75
    P = np.stack([(v - c) @ right, (v - c) @ up, (v - c) @ d], axis=1)
    px = (P[:, 0] / span * 0.5 + 0.5) * size
    py = (0.5 - P[:, 1] / span * 0.5) * size
    img = np.ones((size, size, 3)); zb = np.full((size, size), -np.inf)
    light = 0.6 * d + 0.5 * up + 0.3 * right; light /= np.linalg.norm(light)
    for a, b, cc, k in fs:
        tri = v[[a, b, cc]]
        n = np.cross(tri[1] - tri[0], tri[2] - tri[0]); nn = np.linalg.norm(n)
        if nn == 0:
            continue
        n /= nn
        if n @ d <= 0:
            continue  # back face
        shade = 0.30 + 0.70 * max(n @ light, 0.0)
        col = np.array(COLORS[k]) * shade
        xs, ys, zs = px[[a, b, cc]], py[[a, b, cc]], P[[a, b, cc], 2]
        x0, x1 = int(max(np.floor(xs.min()), 0)), int(min(np.ceil(xs.max()), size - 1))
        y0, y1 = int(max(np.floor(ys.min()), 0)), int(min(np.ceil(ys.max()), size - 1))
        if x0 > x1 or y0 > y1:
            continue
        gx, gy = np.meshgrid(np.arange(x0, x1 + 1) + 0.5, np.arange(y0, y1 + 1) + 0.5)
        den = (ys[1] - ys[2]) * (xs[0] - xs[2]) + (xs[2] - xs[1]) * (ys[0] - ys[2])
        if abs(den) < 1e-12:
            continue
        w0 = ((ys[1] - ys[2]) * (gx - xs[2]) + (xs[2] - xs[1]) * (gy - ys[2])) / den
        w1 = ((ys[2] - ys[0]) * (gx - xs[2]) + (xs[0] - xs[2]) * (gy - ys[2])) / den
        w2 = 1 - w0 - w1
        inside = (w0 >= -1e-9) & (w1 >= -1e-9) & (w2 >= -1e-9)
        z = w0 * zs[0] + w1 * zs[1] + w2 * zs[2]
        sub = zb[y0:y1 + 1, x0:x1 + 1]
        hit = inside & (z > sub)
        sub[hit] = z[hit]
        img[y0:y1 + 1, x0:x1 + 1][hit] = col
    return img

def render(path, out, title, views, window=None, size=900):
    v, fs = load(path)
    window = window or (v.min(0), v.max(0))
    fig, axes = plt.subplots(1, len(views), figsize=(6 * len(views), 6.8), dpi=110)
    for ax, (elev, azim) in zip(np.atleast_1d(axes), views):
        ax.imshow(raster(v, fs, elev, azim, size, window)); ax.set_axis_off()
        ax.set_title(f"{title}\nelev {elev}°, azim {azim}°", fontsize=9)
    fig.tight_layout(rect=(0, 0, 1, 0.97)); fig.savefig(out); plt.close(fig)

if __name__ == "__main__":
    d = sys.argv[1]
    for name, title in [("bracket-fillet", "bracket filleted all round, r = 0.05"),
                        ("bracket-chamfer", "bore-less bracket chamfered all round, s = 0.05")]:
        render(f"{d}/{name}.soup", f"{d}/{name}-iso.png", title, [(30, -60), (30, 120)])
        render(f"{d}/{name}.soup", f"{d}/{name}-corners.png", title + " — the boss's base, near y = 0.5",
               [(35, -50), (35, 40)], window=(np.array([-0.15, 0.25, 0.25]), np.array([0.75, 0.85, 0.8])))
