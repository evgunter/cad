#!/usr/bin/env python3
"""Exact-rational referee for the nurbs_cert diag dump (tess/nurbs-bound-diag).

Reads a `diag_replay` log, rebuilds the ORIGINAL surface and the f64-REFINED
surface (what patch_bound::rational_cells actually hulls) from their hex
dumps, and evaluates the second partials in exact rational arithmetic
(fractions.Fraction) at the sampler's argmax points. Every f64 input is an
exact rational, so nothing here rounds until the final decimal print.
"""
import sys, struct, re
from fractions import Fraction as F
from decimal import Decimal, getcontext
getcontext().prec = 40

def f64(h): return struct.unpack('>d', bytes.fromhex(h))[0]
def fr(h): return F(f64(h))

def parse(path):
    surf, hexes = {}, []
    for line in open(path):
        p = line.split()
        if not p: continue
        if p[0] == 'DUMP':
            tag = p[1]; d = surf.setdefault(tag, {})
            if p[2].startswith('pu='):
                for kv in p[2:]:
                    k, v = kv.split('='); d[k] = int(v)
            elif p[2] in ('knots_u_hex', 'knots_v_hex', 'weights_hex', 'control_hex'):
                d[p[2][:-4]] = [fr(x) for x in p[3].split(',')]
        elif p[0] == 'HEX':
            hexes.append(p[1:])
    return surf, hexes

def diff_u(net, knots, p):
    # net[i][j]; derivative coefficient differencing along i (NURBS Book 3.4)
    n = len(net)
    out = []
    for i in range(n - 1):
        den = knots[i + p + 1] - knots[i + 1]
        out.append([p * (b - a) / den for a, b in zip(net[i], net[i + 1])])
    return out, knots[1:-1]

def transpose(net): return [list(r) for r in zip(*net)]

def basis(knots, p, span, t):
    # Cox-de Boor restricted to a NAMED span (so a knot can be read one-sided)
    N = {span: F(1)}
    for d in range(1, p + 1):
        M = {}
        for i in range(span - d, span + 1):
            v = F(0)
            if i in N and knots[i + d] != knots[i]:
                v += (t - knots[i]) / (knots[i + d] - knots[i]) * N[i]
            if (i + 1) in N and knots[i + d + 1] != knots[i + 1]:
                v += (knots[i + d + 1] - t) / (knots[i + d + 1] - knots[i + 1]) * N[i + 1]
            M[i] = v
        N = M
    return N

def spans_at(knots, p, t):
    # every nonempty span whose CLOSED extent contains t
    return [i for i in range(p, len(knots) - p - 1)
            if knots[i] < knots[i + 1] and knots[i] <= t <= knots[i + 1]]

def ev(net, ku, pu, kv, pv, su, sv, u, v):
    Nu, Nv = basis(ku, pu, su, u), basis(kv, pv, sv, v)
    return sum(Nu[i] * Nv[j] * net[i][j] for i in Nu for j in Nv)

class Surf:
    def __init__(s, d):
        s.pu, s.pv, s.nu, s.nv = d['pu'], d['pv'], d['nu'], d['nv']
        s.ku, s.kv = d['knots_u'], d['knots_v']
        w, c = d['weights'], d['control']
        s.W = [[w[i * s.nv + j] for j in range(s.nv)] for i in range(s.nu)]
        s.A = [[[w[i * s.nv + j] * c[3 * (i * s.nv + j) + k] for j in range(s.nv)]
                for i in range(s.nu)] for k in range(3)]
    def ders_net(s, net, k, l):
        ku, pu, kv, pv = s.ku, s.pu, s.kv, s.pv
        for _ in range(k):
            if pu == 0: return None
            net, ku = diff_u(net, ku, pu); pu -= 1
        for _ in range(l):
            if pv == 0: return None
            t, kv = diff_u(transpose(net), kv, pv); net = transpose(t); pv -= 1
        return net, ku, pu, kv, pv
    def homog(s, net, k, l, su, sv, u, v):
        r = s.ders_net(net, k, l)
        if r is None: return F(0)
        net, ku, pu, kv, pv = r
        # span index shifts down by one per differencing (knots trimmed at front)
        return ev(net, ku, pu, kv, pv, su - k, sv - l, u, v)
    def jet2(s, su, sv, u, v):
        h = lambda net, k, l: s.homog(net, k, l, su, sv, u, v)
        w = {(k, l): h(s.W, k, l) for k in range(3) for l in range(3) if k + l <= 2}
        out = {}
        for c in range(3):
            a = {kl: h(s.A[c], *kl) for kl in w}
            S = a[0, 0] / w[0, 0]
            Su = (a[1, 0] - S * w[1, 0]) / w[0, 0]
            Sv = (a[0, 1] - S * w[0, 1]) / w[0, 0]
            out.setdefault('uu', []).append((a[2, 0] - 2 * Su * w[1, 0] - S * w[2, 0]) / w[0, 0])
            out.setdefault('vv', []).append((a[0, 2] - 2 * Sv * w[0, 1] - S * w[0, 2]) / w[0, 0])
            out.setdefault('uv', []).append((a[1, 1] - Su * w[0, 1] - Sv * w[1, 0] - S * w[1, 1]) / w[0, 0])
        return out
    def transposed(s):
        t = object.__new__(Surf)
        t.pu, t.pv, t.nu, t.nv, t.ku, t.kv = s.pv, s.pu, s.nv, s.nu, s.kv, s.ku
        t.W = transpose(s.W); t.A = [transpose(a) for a in s.A]
        return t

def dec(q): return Decimal(q.numerator) / Decimal(q.denominator)
def norm(vec): return dec(sum(x * x for x in vec)).sqrt()

def main(path):
    surf, hexes = parse(path)
    orig, ref = Surf(surf['orig']), Surf(surf['refined'])
    bounds, arg = {}, {}
    for h in hexes:
        if h[1] == 'bound':
            bounds[h[0]] = [fr(x) for x in h[2].split(',')]
        elif h[1] == 'argmax':
            arg[(h[0], h[2])] = (fr(h[3][2:]), fr(h[4][2:]), [fr(x) for x in h[5][4:].split(',')])
    for tag in ('orig', 'transposed'):
        So, Sr = (orig, ref) if tag == 'orig' else (orig.transposed(), ref.transposed())
        if tag not in bounds: continue
        for k, name in enumerate(('uu', 'uv', 'vv')):
            u, v, vec = arg[(tag, name)]
            b = bounds[tag][k]
            print(f"--- {tag} {name}: u={float(u)!r} v={float(v)!r}")
            print(f"    bound                 = {dec(b):.25E}")
            print(f"    f64-sampled norm(vec) = {norm(vec):.25E}   (exact norm of the f64 jet vector)")
            for label, S in (('TRUE original S', So), ('TRUE f64-refined S_r', Sr)):
                for su in spans_at(S.ku, S.pu, u):
                    for sv in spans_at(S.kv, S.pv, v):
                        t = S.jet2(su, sv, u, v)[name]
                        n2 = sum(x * x for x in t)
                        verdict = 'EXCEEDS bound' if n2 > b * b else 'within bound'
                        print(f"    {label:22s} span({su},{sv}) = {norm(t):.25E}  {verdict}"
                              f"  (true-bound)/bound={float((dec(n2).sqrt()-dec(b))/dec(b)):.3e}")
                        if label.startswith('TRUE orig'):
                            print("        per-channel f64-minus-true:",
                                  [f"{float(a - c):.3e}" for a, c in zip(vec, t)])

if __name__ == '__main__':
    main(sys.argv[1])
