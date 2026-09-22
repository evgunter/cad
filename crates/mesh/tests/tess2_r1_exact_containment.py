#!/usr/bin/env python3
"""TESS-2 r1 review probe: does every PatchCell enclose the DESCRIBED patch?

Reads the bit dump produced by geom-brep's `tess2r1_dump_cells_for_the_exact_referee`
and decides containment in exact rational arithmetic (`fractions.Fraction`),
with NO allowance. Shares nothing with the kernel but the description's bits.

A cell lies inside one span of the DESCRIBED knot vectors (refinement only adds
knots strictly inside spans), so the one-sided polynomial piece selected at the
cell's midpoint is the right reading on the whole closed cell — and the cell's
enclosure must contain that piece's value at every point of the closed cell.
"""
import struct
import sys
from fractions import Fraction as F


def fr(word):
    return F(struct.unpack(">d", bytes.fromhex(word))[0])


def frs(s):
    return [fr(w) for w in s.split(",")]


def fl(word):
    return struct.unpack(">d", bytes.fromhex(word))[0]


def diff_first(net, knots, degree):
    out = []
    for i in range(len(net) - 1):
        span = knots[i + degree + 1] - knots[i + 1]
        out.append([degree * (b - a) / span for a, b in zip(net[i], net[i + 1])])
    return out, knots[1:-1]


def transpose(net):
    return [list(r) for r in zip(*net)]


def basis_on_span(knots, degree, span, t):
    active = {span: F(1)}
    for d in range(1, degree + 1):
        nxt = {}
        for i in range(span - d, span + 1):
            acc = F(0)
            if i in active and knots[i + d] != knots[i]:
                acc += (t - knots[i]) / (knots[i + d] - knots[i]) * active[i]
            if (i + 1) in active and knots[i + d + 1] != knots[i + 1]:
                acc += (knots[i + d + 1] - t) / (knots[i + d + 1] - knots[i + 1]) * active[i + 1]
            nxt[i] = acc
        active = nxt
    return active


class Patch:
    def __init__(self, f):
        self.du, self.dv = f["du"], f["dv"]
        self.ku, self.kv = f["knots_u"], f["knots_v"]
        nu, nv = f["nu"], f["nv"]
        w, ctrl = f["weights"], f["control"]
        self.w_net = [[w[i * nv + j] for j in range(nv)] for i in range(nu)]
        self.h_nets = [
            [[w[i * nv + j] * ctrl[3 * (i * nv + j) + c] for j in range(nv)] for i in range(nu)]
            for c in range(3)
        ]
        self._cache = {}

    def derived(self, tag, net, k, ell):
        key = (tag, k, ell)
        if key in self._cache:
            return self._cache[key]
        ku, du, kvv, dv = self.ku, self.du, self.kv, self.dv
        cur = net
        for _ in range(k):
            if du == 0:
                self._cache[key] = None
                return None
            cur, ku = diff_first(cur, ku, du)
            du -= 1
        for _ in range(ell):
            if dv == 0:
                self._cache[key] = None
                return None
            flipped, kvv = diff_first(transpose(cur), kvv, dv)
            cur = transpose(flipped)
            dv -= 1
        self._cache[key] = (cur, ku, du, kvv, dv)
        return self._cache[key]

    def val(self, tag, net, k, ell, su, sv, u, v):
        d = self.derived(tag, net, k, ell)
        if d is None:
            return F(0)
        cur, ku, du, kvv, dv = d
        bu = basis_on_span(ku, du, su - k, u)
        bv = basis_on_span(kvv, dv, sv - ell, v)
        return sum(bu[i] * bv[j] * cur[i][j] for i in bu for j in bv)

    def jet(self, su, sv, u, v):
        """All five partials of S = A/w, exact, per channel."""
        orders = [(k, e) for k in range(3) for e in range(3) if k + e <= 2]
        w = {o: self.val("w", self.w_net, o[0], o[1], su, sv, u, v) for o in orders}
        out = {t: [] for t in ("s_u", "s_v", "s_uu", "s_uv", "s_vv")}
        for c, net in enumerate(self.h_nets):
            a = {o: self.val(f"a{c}", net, o[0], o[1], su, sv, u, v) for o in orders}
            w0 = w[0, 0]
            s = a[0, 0] / w0
            su1 = (a[1, 0] - s * w[1, 0]) / w0
            sv1 = (a[0, 1] - s * w[0, 1]) / w0
            out["s_u"].append(su1)
            out["s_v"].append(sv1)
            out["s_uu"].append((a[2, 0] - 2 * su1 * w[1, 0] - s * w[2, 0]) / w0)
            out["s_vv"].append((a[0, 2] - 2 * sv1 * w[0, 1] - s * w[0, 2]) / w0)
            out["s_uv"].append((a[1, 1] - su1 * w[0, 1] - sv1 * w[1, 0] - s * w[1, 1]) / w0)
        return out


def span_of(knots, degree, t):
    """The nonempty described span whose closed extent contains `t`, preferring
    the one `t` starts (Cox-de Boor's own tie-break)."""
    cands = [
        i
        for i in range(degree, len(knots) - degree - 1)
        if knots[i] < knots[i + 1] and knots[i] <= t <= knots[i + 1]
    ]
    return cands[0] if cands else None


def parse(path):
    fixtures = []
    cur = None
    with open(path) as fh:
        for raw in fh:
            line = raw.rstrip("\n")
            t = line.strip().split()
            if not t:
                continue
            if t[0] == "FIXTURE":
                cur = {"name": t[1], "du": int(t[2].split("=")[1]), "dv": int(t[3].split("=")[1]), "cells": []}
            elif cur is None:
                continue
            elif t[0] == "KNOTS_U":
                cur["knots_u"] = frs(t[1])
            elif t[0] == "KNOTS_V":
                cur["knots_v"] = frs(t[1])
            elif t[0] == "NUNV":
                cur["nu"], cur["nv"] = int(t[1]), int(t[2])
            elif t[0] == "WEIGHTS":
                cur["weights"] = frs(t[1])
            elif t[0] == "CONTROL":
                cur["control"] = frs(t[1])
            elif t[0] == "CELL":
                u = t[3].split(",")
                v = t[5].split(",")
                cur["cells"].append({"idx": int(t[1]), "u": (fr(u[0]), fr(u[1])), "v": (fr(v[0]), fr(v[1])), "encl": {}})
            elif t[0] in ("s_u", "s_v", "s_uu", "s_uv", "s_vv"):
                lo, hi = t[2].split(",")
                cur["cells"][-1]["encl"].setdefault(t[0], []).append((fr(lo), fr(hi), fl(lo), fl(hi)))
            elif t[0] == "ENDFIXTURE":
                fixtures.append(cur)
                cur = None
    return fixtures


FRACTIONS = [F(0), F(1, 4), F(1, 2), F(3, 4), F(1)]


def main(path):
    total_esc = 0
    for f in parse(path):
        p = Patch(f)
        checked = 0
        escapes = []
        worst = (F(0), "")
        for cell in f["cells"]:
            u0, u1 = cell["u"]
            v0, v1 = cell["v"]
            um, vm = (u0 + u1) / 2, (v0 + v1) / 2
            su = span_of(f["knots_u"], f["du"], um)
            sv = span_of(f["knots_v"], f["dv"], vm)
            if su is None or sv is None:
                print(f"  !! {f['name']} cell {cell['idx']}: no described span")
                continue
            for a in FRACTIONS:
                for b in FRACTIONS:
                    u = u0 + (u1 - u0) * a
                    v = v0 + (v1 - v0) * b
                    jet = p.jet(su, sv, u, v)
                    for tag, vals in jet.items():
                        encl = cell["encl"].get(tag)
                        if encl is None:
                            continue
                        for c, q in enumerate(vals):
                            lo, hi, lof, hif = encl[c]
                            checked += 1
                            if q < lo:
                                gap = lo - q
                            elif q > hi:
                                gap = q - hi
                            else:
                                continue
                            scale = max(abs(q), F(1, 10**12))
                            rel = gap / scale
                            if rel > worst[0]:
                                worst = (rel, f"cell {cell['idx']} {tag} ch{c} at (a={a},b={b}) true={float(q):.17e} encl=[{lof:.17e},{hif:.17e}]")
                            escapes.append((float(rel), cell["idx"], tag, c, float(q), lof, hif))
        print(f"{f['name']}: {checked} exact containment checks over {len(f['cells'])} sampled cells; escapes {len(escapes)}")
        if escapes:
            escapes.sort(reverse=True)
            for e in escapes[:8]:
                print(f"    ESCAPE rel={e[0]:.3e} cell {e[1]} {e[2]} ch{e[3]}: true {e[4]:.17e} not in [{e[5]:.17e},{e[6]:.17e}]")
            print(f"    worst: {worst[1]}")
        total_esc += len(escapes)
    print(f"TOTAL ESCAPES {total_esc}")
    return 1 if total_esc else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1]))
