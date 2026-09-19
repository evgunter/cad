#!/usr/bin/env python3
"""Ingredient-level localisation for a BILINEAR (pu = pv = 1) rational red.

For the refined cell that holds the sampler's argmax, rebuilds each ingredient
hull of `patch_bound::rational_cells` from the f64-REFINED net (in exact
rational arithmetic, so these are the hulls' real-number cores — the ring only
widens them by ulps) and compares it with the exact range of the same
ingredient of the ORIGINAL described surface over the same cell.
escape > 0  <=>  the described surface's truth lies OUTSIDE the hull.
"""
import sys
from fractions import Fraction as F
from exact_check import parse, Surf, spans_at, fr

def hull(xs): xs = list(xs); return (min(xs), max(xs))
def escape(truth, h):  # how far the truth range pokes out of the hull
    return max(h[0] - truth[0], truth[1] - h[1])

def main(path, which):
    surf, hexes = parse(path)
    o, r = Surf(surf['orig']), Surf(surf['refined'])
    assert (o.pu, o.pv) == (1, 1), "bilinear only"
    for h in hexes:
        if h[0] == 'orig' and h[1] == 'argmax' and h[2] == which:
            u, v = fr(h[3][2:]), fr(h[4][2:])
    su, sv = spans_at(r.ku, 1, u)[-1 if u == 1 else 0], spans_at(r.kv, 1, v)[-1 if v == 1 else 0]
    (ulo, uhi), (vlo, vhi) = (r.ku[su], r.ku[su + 1]), (r.kv[sv], r.kv[sv + 1])
    print(f"refined cell span({su},{sv}) u=[{float(ulo)},{float(uhi)}] v=[{float(vlo)},{float(vhi)}]")
    I, J = (su - 1, su), (sv - 1, sv)          # value window (degree 1)
    du, dv = 1 / (uhi - ulo), 1 / (vhi - vlo)  # p/(knot diff), p = 1
    d = surf['refined']; nv = d['nv']
    P = lambda i, j, c: d['control'][3 * (i * nv + j) + c]
    # centroid exactly as the Rust computes it: f64 sums, i outer / j inner
    cen = []
    for c in range(3):
        acc = 0.0
        for i in I:
            for j in J:
                acc += float(P(i, j, c))
        cen.append(F(acc / 4.0))
    # exact ingredient functions of the ORIGINAL surface
    def homo(net, k, l, uu, vv): return o.homog(net, k, l, 1, 1, uu, vv)
    grid = [(ulo + (uhi - ulo) * F(a, 8), vlo + (vhi - vlo) * F(b, 8)) for a in range(9) for b in range(9)]
    rows = []
    rows.append(('w', hull(r.W[i][j] for i in I for j in J), hull(homo(o.W, 0, 0, *g) for g in grid)))
    rows.append(('w_u', hull((r.W[I[1]][j] - r.W[I[0]][j]) * du for j in J), hull(homo(o.W, 1, 0, *g) for g in grid)))
    rows.append(('w_v', hull((r.W[i][J[1]] - r.W[i][J[0]]) * dv for i in I), hull(homo(o.W, 0, 1, *g) for g in grid)))
    for c in range(3):
        cc = cen[c]
        rows.append((f'(S-c)[{c}]', hull(P(i, j, c) - cc for i in I for j in J),
                     hull(homo(o.A[c], 0, 0, *g) / homo(o.W, 0, 0, *g) - cc for g in grid)))
        rows.append((f'At_u[{c}]', hull(((r.A[c][I[1]][j] - r.A[c][I[0]][j]) - cc * (r.W[I[1]][j] - r.W[I[0]][j])) * du for j in J),
                     hull(homo(o.A[c], 1, 0, *g) - cc * homo(o.W, 1, 0, *g) for g in grid)))
        rows.append((f'At_v[{c}]', hull(((r.A[c][i][J[1]] - r.A[c][i][J[0]]) - cc * (r.W[i][J[1]] - r.W[i][J[0]])) * dv for i in I),
                     hull(homo(o.A[c], 0, 1, *g) - cc * homo(o.W, 0, 1, *g) for g in grid)))
    print(f"{'ingredient':12s} {'hull(refined f64 net)':>50s} {'truth(original S) on cell':>50s} {'escape':>11s} rel")
    for name, h, t in rows:
        e = escape(t, h); scale = max(abs(h[0]), abs(h[1]))
        flag = 'VIOLATED' if e > 0 else 'holds'
        print(f"{name:12s} [{float(h[0])!r:>23},{float(h[1])!r:>23}] [{float(t[0])!r:>23},{float(t[1])!r:>23}] {float(e):11.3e} {float(e/scale):10.2e} {flag}")
    # the dust itself: exact refined control net vs the f64 one, on this cell
    worst = F(0)
    for i in I:
        for j in J:
            uu, vv = r.ku[i + 1], r.kv[j + 1]   # Greville abscissa of a degree-1 net
            wt = homo(o.W, 0, 0, uu, vv)
            worst = max(worst, abs(wt - r.W[i][j]) / wt)
            for c in range(3):
                pt = homo(o.A[c], 0, 0, uu, vv) / wt
                worst = max(worst, abs(pt - P(i, j, c)) / max(abs(pt), F(1, 10**6)))
    print(f"insertion dust on this cell's 4 refined control points/weights: worst relative error = {float(worst):.3e}")

if __name__ == '__main__':
    main(sys.argv[1], sys.argv[2])
