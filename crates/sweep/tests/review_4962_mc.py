"""Independent oracle for PR 4962's fillet overrun (review probe).

Reads the `MC {...}` lines `review_4962_probes::p1_*` prints. A fillet
of radius r of an edge between faces a, b of the convex polytope B
removes the points of B the morphological OPENING of the two-face wedge
does not keep: x is removed iff dist(x, C) > r, C = {n_a.y <= c_a - r,
n_b.y <= c_b - r} (the ball centres that fit the wedge). This shares no
formula with the lane's kite-less-sector closed form or its polar
quadrature. The turn removes B ∩ (R1 ∪ R2):

    vol = vol(B∩R1) + vol(B∩R2) - vol(B∩R1∩R2)

each band by 2D quasi-random sampling of its normal section with the
exact chord of B along the edge (the predicate is constant along it),
the overlap by 3D quasi-random sampling in a box about the vertex, or
(a 4th argument, its sample count) over band 1's whole prism box, which
a near-flat turn's long thin overlap needs. The per-band sampling is
coarse (~1e-4 relative); the sharp check is the "vs alone+alone-overlap"
column, the turn's ΔV against each edge's plain cut-off (an established
path) less this overlap.

    python3 -I review_4962_mc.py <probe stdout> [n2] [n3] [n_prism]
"""
import json
import sys

import numpy as np


def r2_sequence(n, d, offset=0):
    # Roberts' R_d low-discrepancy sequence.
    phi = 2.0
    for _ in range(30):
        phi = (1 + phi) ** (1.0 / (d + 1))
    alpha = (1.0 / phi) ** np.arange(1, d + 1)
    k = np.arange(offset + 1, offset + n + 1, dtype=np.float64)[:, None]
    return (0.5 + k * alpha) % 1.0


def dist_to_c(x, na, ca, nb, cb):
    """Distance from points x (N,3) to {na.y<=ca, nb.y<=cb}."""
    da = x @ na - ca
    db = x @ nb - cb
    inside = (da <= 0) & (db <= 0)
    # Projection onto plane a, feasible for b.
    pa = x - da[:, None] * na
    fa = (pa @ nb - cb) <= 1e-15
    # Projection onto plane b.
    pb = x - db[:, None] * nb
    fb = (pb @ na - ca) <= 1e-15
    # Projection onto the line of both planes.
    dirn = np.cross(na, nb)
    dirn /= np.linalg.norm(dirn)
    m = np.array([na, nb, dirn])
    p0 = np.linalg.solve(m, np.array([ca, cb, 0.0]))
    w = x - p0
    pl = w - (w @ dirn)[:, None] * dirn
    dl = np.linalg.norm(pl, axis=1)
    big = np.full(len(x), np.inf)
    d = np.minimum(np.where(fa & (da > 0), da, big), np.where(fb & (db > 0), db, big))
    d = np.minimum(d, dl)
    return np.where(inside, 0.0, d)


def removed(x, band, r):
    na, ca, nb, cb = band
    return dist_to_c(x, na, ca - r, nb, cb - r) > r


def in_b(x, halves):
    ok = np.ones(len(x), dtype=bool)
    for n, c in halves:
        ok &= (x @ n) <= c
    return ok


def chord(x, tau, halves):
    lo = np.full(len(x), -np.inf)
    hi = np.full(len(x), np.inf)
    for n, c in halves:
        along = tau @ n
        at = c - x @ n
        if abs(along) < 1e-15:
            bad = at < 0
            lo = np.where(bad, np.inf, lo)
        elif along > 0:
            hi = np.minimum(hi, at / along)
        else:
            lo = np.maximum(lo, at / along)
    return np.maximum(hi - lo, 0.0)


def band_volume(band, a, b, halves, r, n):
    tau = (b - a) / np.linalg.norm(b - a)
    na, _, nb, _ = band
    e0 = np.cross(tau, na)
    e0 /= np.linalg.norm(e0)
    e1 = np.cross(tau, e0)
    reach = 1.2 * r / np.sin(0.5 * np.arccos(np.clip(-na @ nb, -1, 1)))
    tot, cnt = 0.0, 0
    chunk = 2_000_000
    for off in range(0, n, chunk):
        u = r2_sequence(min(chunk, n - off), 2, off) * 2 - 1
        x = a + reach * (u[:, :1] * e0 + u[:, 1:] * e1)
        keep = removed(x, band, r)
        tot += chord(x[keep], tau, halves).sum()
        cnt += len(u)
    return tot / cnt * (2 * reach) ** 2


def overlap(bands, ends, halves, r, n):
    (a1, b1), (a2, b2) = ends
    shared = [p for p in (a1, b1) if any(np.linalg.norm(p - q) < 1e-9 for q in (a2, b2))][0]
    # Pass 1: bound the overlap, growing the box until no hit touches it.
    h = 4.0 * r
    while True:
        lo, hi = shared - h, shared + h
        u = r2_sequence(4_000_000, 3, 7)
        x = lo + u * (hi - lo)
        m = in_b(x, halves) & removed(x, bands[0], r) & removed(x, bands[1], r)
        assert m.any()
        cell = (hi - lo) * 0.01
        if (x[m].min(axis=0) > lo + cell).all() and (x[m].max(axis=0) < hi - cell).all():
            break
        h *= 2
    span = hi - lo
    lo = np.maximum(lo, x[m].min(axis=0) - 0.02 * span)
    hi = np.minimum(hi, x[m].max(axis=0) + 0.02 * span)
    hits, cnt = 0, 0
    chunk = 2_000_000
    for off in range(0, n, chunk):
        u = r2_sequence(min(chunk, n - off), 3, off + 99)
        x = lo + u * (hi - lo)
        m = in_b(x, halves) & removed(x, bands[0], r) & removed(x, bands[1], r)
        hits += int(m.sum())
        cnt += len(u)
    vol = np.prod(hi - lo)
    p = hits / cnt
    return p * vol, np.sqrt(p * (1 - p) / cnt) * vol


def overlap_prism(bands, ends, halves, r, n):
    """The overlap sampled over band 1's whole prism box: every point of
    B ∩ R1 lies within `reach` of edge 1's line, between B's extent."""
    a, b = ends[0]
    tau = (b - a) / np.linalg.norm(b - a)
    na, _, nb, _ = bands[0]
    e0 = np.cross(tau, na)
    e0 /= np.linalg.norm(e0)
    e1 = np.cross(tau, e0)
    reach = 1.2 * r / np.sin(0.5 * np.arccos(np.clip(-na @ nb, -1, 1)))
    # B's extent along the edge, over the section square.
    corners = np.array([a + reach * (i * e0 + j * e1) for i in (-1, 1) for j in (-1, 1)])
    ext = []
    for c0 in corners:
        lo, hi = -np.inf, np.inf
        for nn, cc in halves:
            al = tau @ nn
            at = cc - c0 @ nn
            if abs(al) > 1e-15:
                if al > 0:
                    hi = min(hi, at / al)
                else:
                    lo = max(lo, at / al)
        ext += [lo, hi]
    t0, t1 = min(ext) - reach, max(ext) + reach
    hits, cnt = 0, 0
    chunk = 2_000_000
    for off in range(0, n, chunk):
        u = r2_sequence(min(chunk, n - off), 3, off + 5)
        x = (a + (t0 + u[:, :1] * (t1 - t0)) * tau
             + reach * ((2 * u[:, 1:2] - 1) * e0 + (2 * u[:, 2:] - 1) * e1))
        m = in_b(x, halves) & removed(x, bands[0], r) & removed(x, bands[1], r)
        hits += int(m.sum())
        cnt += len(u)
    vol = (t1 - t0) * (2 * reach) ** 2
    p = hits / cnt
    return p * vol, np.sqrt(p * (1 - p) / cnt) * vol


def main():
    n2 = int(sys.argv[2]) if len(sys.argv) > 2 else 8_000_000
    n3 = int(sys.argv[3]) if len(sys.argv) > 3 else 40_000_000
    for line in open(sys.argv[1]):
        if not line.startswith("MC "):
            continue
        d = json.loads(line[3:])
        halves = []
        for n, c in d["halves"]:
            n = np.array(n)
            k = np.linalg.norm(n)
            halves.append((n / k, c / k))
        r = d["r"]
        bands, ends = [], []
        for q in d["reqs"]:
            a, b = np.array(q["a"]), np.array(q["b"])
            on = [i for i, (n, c) in enumerate(halves)
                  if abs(n @ a - c) < 1e-9 and abs(n @ b - c) < 1e-9]
            assert len(on) == 2, on
            fa, fb = on
            bands.append((halves[fa][0], halves[fa][1], halves[fb][0], halves[fb][1]))
            ends.append((np.array(q["a"]), np.array(q["b"])))
        v1 = band_volume(bands[0], *ends[0], halves, r, n2)
        v2 = band_volume(bands[1], *ends[1], halves, r, n2)
        ov, se = overlap(bands, ends, halves, r, n3)
        want = d["sign"] * (v1 + v2 - ov)
        diff = d["dv"] - want
        if len(sys.argv) > 4:
            ov, se = overlap_prism(bands, ends, halves, r, int(sys.argv[4]))
        alone = (d["alone"][0] + d["alone"][1] - d["sign"] * ov
                 if None not in d["alone"] else float("nan"))
        print(
            f"{d['what']:45s} vs alone+alone-overlap {d['dv'] - alone:+.2e} |"
            f" kernel {d['dv']:.10f} mc {want:.10f} diff {diff:+.2e} "
            f"(overlap {ov:.3e} ± {se:.1e}, pad {d['pad']})"
        )


main()
