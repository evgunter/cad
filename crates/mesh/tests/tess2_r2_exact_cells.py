#!/usr/bin/env python3
"""Exact-rational check of dumped PatchCells: every enclosure must contain
the DESCRIBED patch's partial at points inside the cell."""
import sys, struct, random, importlib.util
from fractions import Fraction as F

spec = importlib.util.spec_from_file_location(
    "ref", sys.argv[2] if len(sys.argv) > 2 else "crates/mesh/tests/nurbs_exact_referee.py")
ref = importlib.util.module_from_spec(spec); spec.loader.exec_module(ref)

def fh(h):
    return struct.unpack(">d", bytes.fromhex(h))[0]

def fr(h):
    return F(fh(h))

class Surf:
    def __init__(self, pu, pv, nu, nv, ku, kv, w, p):
        self.pu, self.pv, self.nu, self.nv = pu, pv, nu, nv
        self.ku, self.kv = ku, kv
        self.wnet = [[w[i*nv+j] for j in range(nv)] for i in range(nu)]
        self.anets = [[[w[i*nv+j]*p[3*(i*nv+j)+c] for j in range(nv)] for i in range(nu)] for c in range(3)]
        self.cache = {}
    def derived(self, which, k, l):
        key = (which, k, l)
        if key in self.cache:
            return self.cache[key]
        net = self.wnet if which == 'w' else self.anets[which]
        knots_u, du = self.ku, self.pu
        knots_v, dv = self.kv, self.pv
        out = None
        ok = True
        for _ in range(k):
            if du == 0: ok = False; break
            net, knots_u = ref.diff_along_first(net, knots_u, du); du -= 1
        if ok:
            for _ in range(l):
                if dv == 0: ok = False; break
                fl, knots_v = ref.diff_along_first(ref.transpose(net), knots_v, dv)
                net = ref.transpose(fl); dv -= 1
        res = (net, knots_u, du, knots_v, dv) if ok else None
        self.cache[key] = res
        return res
    def value(self, which, k, l, su, sv, u, v):
        d = self.derived(which, k, l)
        if d is None:
            return F(0)
        net, knots_u, du, knots_v, dv = d
        bu = ref.basis_on_span(knots_u, du, su - k, u)
        bv = ref.basis_on_span(knots_v, dv, sv - l, v)
        return sum(bu[i]*bv[j]*net[i][j] for i in bu for j in bv)
    def jet(self, su, sv, u, v):
        orders = [(k,l) for k in range(3) for l in range(3) if k+l <= 2]
        w = {o: self.value('w', o[0], o[1], su, sv, u, v) for o in orders}
        out = {'s_u': [], 's_v': [], 's_uu': [], 's_uv': [], 's_vv': []}
        for c in range(3):
            a = {o: self.value(c, o[0], o[1], su, sv, u, v) for o in orders}
            s = a[0,0]/w[0,0]
            s_u = (a[1,0] - s*w[1,0])/w[0,0]
            s_v = (a[0,1] - s*w[0,1])/w[0,0]
            out['s_u'].append(s_u); out['s_v'].append(s_v)
            out['s_uu'].append((a[2,0] - 2*s_u*w[1,0] - s*w[2,0])/w[0,0])
            out['s_vv'].append((a[0,2] - 2*s_v*w[0,1] - s*w[0,2])/w[0,0])
            out['s_uv'].append((a[1,1] - s_u*w[0,1] - s_v*w[1,0] - s*w[1,1])/w[0,0])
        return out

def span_of(knots, p, t):
    # span i (nonempty) with knots[i] <= t < knots[i+1]; last span closed.
    n = len(knots) - p - 1
    for i in range(p, n):
        if knots[i] <= t < knots[i+1]:
            return i
    for i in range(n-1, p-1, -1):
        if knots[i] < knots[i+1]:
            return i
    raise RuntimeError("no span")

def main():
    rng = random.Random(12345)
    lines = open(sys.argv[1]).read().splitlines()
    i = 0
    total_checks = 0; escapes = []; poison = 0; surfaces = 0; refused = 0
    max_cells = int(sys.argv[3]) if len(sys.argv) > 3 else 200
    while i < len(lines):
        ln = lines[i]
        if ln.startswith("REFUSED"):
            refused += 1; i += 1; continue
        if not ln.startswith("SURFACE"):
            i += 1; continue
        _, name, pu, pv, nu, nv = ln.split()
        pu, pv, nu, nv = map(int, (pu, pv, nu, nv))
        ku = [fr(h) for h in lines[i+1].split()[1].split(",")]
        kv = [fr(h) for h in lines[i+2].split()[1].split(",")]
        w = [fr(h) for h in lines[i+3].split()[1].split(",")]
        p = [fr(h) for h in lines[i+4].split()[1].split(",")]
        splits, arm = lines[i+5].split()[1], lines[i+5].split()[3]
        i += 6
        cells = []
        while lines[i] != "END":
            cells.append(lines[i]); i += 1
        i += 1
        surfaces += 1
        S = Surf(pu, pv, nu, nv, ku, kv, w, p)
        sel = cells
        if len(cells) > max_cells:
            sel = [cells[0], cells[-1]] + rng.sample(cells, max_cells)
        worst_margin = None
        for cl in sel:
            head, *comps = cl.split(" | ")
            hx = head.split()[1:]
            ulo, uhi, vlo, vhi = (fr(h) for h in hx)
            su = span_of(ku, pu, (ulo+uhi)/2); sv = span_of(kv, pv, (vlo+vhi)/2)
            pts = [(ulo,vlo),(ulo,vhi),(uhi,vlo),(uhi,vhi),((ulo+uhi)/2,(vlo+vhi)/2),
                   (ulo, (vlo+vhi)/2), ((ulo+uhi)/2, vlo)]
            for _ in range(2):
                a = F(rng.randrange(1, 997), 997); b = F(rng.randrange(1, 997), 997)
                pts.append((ulo + (uhi-ulo)*a, vlo + (vhi-vlo)*b))
            encl = {}
            for nm, txt in zip(['s_u','s_v','s_uu','s_uv','s_vv'], comps):
                toks = txt.split()
                encl[nm] = [(toks[2*c], toks[2*c+1]) for c in range(3)]
            for (u, v) in pts:
                jet = S.jet(su, sv, u, v)
                for nm in encl:
                    for c in range(3):
                        lo, hi = encl[nm][c]
                        if lo == "poison":
                            poison += 1; continue
                        lo, hi = fr(lo), fr(hi)
                        val = jet[nm][c]
                        total_checks += 1
                        if not (lo <= val <= hi):
                            exc = (lo - val) if val < lo else (val - hi)
                            scale = max(abs(float(lo)), abs(float(hi)), 1e-300)
                            escapes.append((name, arm, splits, nm, c, float(u), float(v), float(lo), float(val), float(hi), float(exc)/scale))
        print(f"{name} {arm} splits={splits} deg={pu}x{pv} n={nu}x{nv} cells={len(cells)} checked={len(sel)} escapes_so_far={len(escapes)}", flush=True)
    print(f"surfaces={surfaces} refused={refused} checks={total_checks} poison={poison} escapes={len(escapes)}")
    for e in escapes[:40]:
        print("ESCAPE", e)

main()
