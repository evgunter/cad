"""Exact oracle for the review probes of boxfinish/wall-roots.

Reads the case files the Rust probes write (REVIEW_PROBE_OUT) and checks
them with exact rationals (fractions) and 80-digit decimals for square
roots. Usage: python3 -I oracle.py <kind> <file>, kind in Q, W, B.
"""
import struct
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F

getcontext().prec = 80


def fx(h):
    return struct.unpack("<d", struct.pack("<Q", int(h, 16)))[0]


def fr(h):
    return F(fx(h))


def dsqrt(x):
    return (Decimal(x.numerator) / Decimal(x.denominator)).sqrt()


def dec(x):
    return Decimal(x.numerator) / Decimal(x.denominator)


def roots_exact(a, b, c):
    """Roots of a t^2 + 2 b t + c, as decimals, (-b -+ sqrt(b^2-ac))/a."""
    disc = b * b - a * c
    s = dsqrt(disc)
    A, B = dec(a), dec(b)
    return [(-B - s) / A, (-B + s) / A]


def check_q(path):
    bad = n = 0
    for line in open(path):
        p = line.split()
        if p[0] != "Q":
            continue
        n += 1
        r = [fx(h) for h in p[7:11]]
        enc = [(r[0], r[1]), (r[2], r[3])]
        m = p[11:]
        for i in range(0, len(m), 3):
            a, b, c = fr(m[i]), fr(m[i + 1]), fr(m[i + 2])
            tr = roots_exact(a, b, c)
            # The formula's root k is (-b -+ s)/a, the enclosure's index.
            for k in range(2):
                lo, hi = enc[k]
                if not (Decimal(lo) <= tr[k] <= Decimal(hi)):
                    bad += 1
                    if bad <= 10:
                        print(f"MISS root {k}: {tr[k]:.6e} not in [{lo:e}, {hi:e}]; a {float(a):e} b {float(b):e} c {float(c):e}")
    print(f"Q: {n} boxes, {bad} member roots outside their enclosure")


def wall_exact(q, d, r, span):
    """Exact shift, b, c and both roots t (decimal) of the wall quadratic
    about the nearest point, z axis through the origin."""
    w0 = (q[0], q[1])
    dp = (d[0], d[1])
    a2 = dp[0] ** 2 + dp[1] ** 2
    g = w0[0] * dp[0] + w0[1] * dp[1]
    free = -g / a2
    held = max(-span, min(span, free))
    w = (w0[0] + dp[0] * held, w0[1] + dp[1] * held)
    b = w[0] * dp[0] + w[1] * dp[1]
    c = w[0] ** 2 + w[1] ** 2 - r * r
    # roots in t: a2 t^2 + 2 g t + (|w0|^2 - r^2)
    tr = roots_exact(a2, g, w0[0] ** 2 + w0[1] ** 2 - r * r) if g * g - a2 * (w0[0] ** 2 + w0[1] ** 2 - r * r) > 0 else None
    return held, b, c, tr


def check_w(path):
    import math
    stats = {}
    worse_c = better_c = n = 0
    worse_t = better_t = 0
    slack_miss = 0
    worst = []
    for line in open(path):
        if not line.startswith("W "):
            continue
        head, mid, tail1, tail2 = line.split("|")[0], line.split("|")[1], line.split("|")[2], line.split("|")[3]
        h = head.split()
        fam = int(h[1])
        q = [fr(x) for x in h[2:5]]
        d = [fr(x) for x in h[5:8]]
        r, span = fr(h[8]), fr(h[9])
        nshift, nb, nc, oshift = [fx(x) for x in mid.split()]
        ob, oc, nt1, ot1 = [fx(x) for x in tail1.split()]
        nt0, ot0, slack = [fx(x) for x in tail2.split()]
        held, b, c, tr = wall_exact(q, d, r, span)
        n += 1
        sc = max(abs(c), F(1, 10**300))
        en, eo = abs(F(nc) - c) / sc if math.isfinite(nc) else None, abs(F(oc) - c) / sc if math.isfinite(oc) else None
        st = stats.setdefault(fam, {"n": 0, "cn": 0.0, "co": 0.0, "bn": 0.0, "bo": 0.0, "tn": 0.0, "to": 0.0, "cworse": 0, "tworse": 0, "tbetter": 0})
        st["n"] += 1
        if en is not None and eo is not None:
            st["cn"] = max(st["cn"], float(en))
            st["co"] = max(st["co"], float(eo))
            if en > eo:
                worse_c += 1
            elif en < eo:
                better_c += 1
        scale_b = abs(d[0]) * (abs(q[0]) + abs(q[1])) + abs(b) + F(1, 10**300)
        if math.isfinite(nb):
            st["bn"] = max(st["bn"], float(abs(F(nb) - b) / scale_b))
        if math.isfinite(ob):
            st["bo"] = max(st["bo"], float(abs(F(ob) - b) / scale_b))
        if tr is not None and all(map(math.isfinite, [nt1, ot1, nt0, ot0])):
            for k, (nt, ot) in enumerate([(nt0, ot0), (nt1, ot1)]):
                t = tr[k]
                scale = abs(t) + 1
                e_n = abs(Decimal(nt) - t) / scale
                e_o = abs(Decimal(ot) - t) / scale
                st["tn"] = max(st["tn"], float(e_n))
                st["to"] = max(st["to"], float(e_o))
                if e_n > e_o * 4 and e_n > Decimal("1e-15"):
                    st["tworse"] += 1
                    worst.append((float(e_n), float(e_o), fam, float(t)))
                if e_o > e_n * 4 and e_o > Decimal("1e-15"):
                    st["tbetter"] += 1
                if abs(Decimal(nt) - t) > Decimal(slack):
                    slack_miss += 1
                    if slack_miss <= 5:
                        print(f"SLACK MISS fam {fam}: root {k} off by {float(abs(Decimal(nt) - t)):e}, slack {slack:e}")
    print(f"W: {n} cases; new c more accurate in {better_c}, less in {worse_c}; roots outside their own slack: {slack_miss}")
    for fam in sorted(stats):
        s = stats[fam]
        print(f"  fam {fam}: n {s['n']}, max rel c err new {s['cn']:.2e} old {s['co']:.2e}; b err/scale new {s['bn']:.2e} old {s['bo']:.2e}; "
              f"root rel err new {s['tn']:.2e} old {s['to']:.2e}; new >4x worse {s['tworse']}, >4x better {s['tbetter']}")
    worst.sort(reverse=True)
    for w in worst[:5]:
        print("  worst new-vs-old root:", w)


def check_b(path):
    n = bmiss = cmiss = rmiss = 0
    wn = wo = 0.0
    wcn = wco = 0.0
    zero_b = 0
    for line in open(path):
        if not line.startswith("B "):
            continue
        p = line.split()
        fam = int(p[1])
        dl = [fx(x) for x in p[2:8]]
        r, span = fr(p[8]), fr(p[9])
        nb = (fx(p[10]), fx(p[11]))
        nc = (fx(p[12]), fx(p[13]))
        ob = (fx(p[14]), fx(p[15]))
        oc = (fx(p[16]), fx(p[17]))
        rt = (fx(p[18]), fx(p[19]))
        m = p[20:]
        n += 1
        if nb == (0.0, 0.0):
            zero_b += 1
        d = [(F(dl[0]) + F(dl[1])) / 2, (F(dl[2]) + F(dl[3])) / 2, (F(dl[4]) + F(dl[5])) / 2]
        lo_c = hi_c = None
        for i in range(0, len(m), 3):
            q = [fr(m[i]), fr(m[i + 1]), fr(m[i + 2])]
            held, b, c, tr = wall_exact(q, d, r, span)
            if not (F(nb[0]) <= b <= F(nb[1])):
                bmiss += 1
                if bmiss <= 5:
                    print(f"B MISS fam {fam}: b {float(b):e} not in [{nb[0]:e}, {nb[1]:e}]")
            if not (F(nc[0]) <= c <= F(nc[1])):
                cmiss += 1
                if cmiss <= 5:
                    print(f"C MISS fam {fam}: c {float(c):e} not in [{nc[0]:e}, {nc[1]:e}]")
            if rt[0] == rt[0] and tr is not None:
                if not (Decimal(rt[0]) <= tr[1] <= Decimal(rt[1])):
                    rmiss += 1
                    if rmiss <= 5:
                        print(f"ROOT MISS fam {fam}: {tr[1]:.6e} not in [{rt[0]:e}, {rt[1]:e}]")
        wn += (nc[1] - nc[0]); wo += (oc[1] - oc[0])
    print(f"B: {n} boxes; members outside: b {bmiss}, c {cmiss}, exit root {rmiss}; b read exactly [0,0] in {zero_b}")


if __name__ == "__main__":
    if sys.argv[1] not in ("C", "C2", "W2", "BW"):
        {"Q": check_q, "W": check_w, "B": check_b}[sys.argv[1]](sys.argv[2])


def check_c(path):
    """The cone about the line's nearest point to the apex, held to the
    run: exact B and C against new and old, cos^2 taken as its f64."""
    import math
    n = worse = better = 0
    wn = wo = 0.0
    rows = []
    for line in open(path):
        if not line.startswith("C "):
            continue
        p = line.split()
        q = [fr(x) for x in p[1:4]]
        d = [fr(x) for x in p[4:7]]
        apex = [fr(x) for x in p[7:10]]
        cos2, span = fr(p[10]), fr(p[11])
        nB, nC, oB, oC = [fx(x) for x in p[12:16]]
        n += 1
        w0 = [q[i] - apex[i] for i in range(3)]
        a2 = sum(x * x for x in d)
        g = sum(w0[i] * d[i] for i in range(3))
        held = max(-span, min(span, -g / a2))
        w = [w0[i] + d[i] * held for i in range(3)]
        da, wa = d[2], w[2]
        B = da * wa - sum(w[i] * d[i] for i in range(3)) * cos2
        C = wa * wa - sum(x * x for x in w) * cos2
        # scale: the terms' magnitudes
        sB = abs(da * wa) + abs(sum(w[i] * d[i] for i in range(3)) * cos2) + F(1, 10**300)
        sC = wa * wa + sum(x * x for x in w) * cos2 + F(1, 10**300)
        eb = (abs(F(nB) - B) / sB, abs(F(oB) - B) / sB) if math.isfinite(nB) and math.isfinite(oB) else None
        ec = (abs(F(nC) - C) / sC, abs(F(oC) - C) / sC) if math.isfinite(nC) and math.isfinite(oC) else None
        if eb:
            wn = max(wn, float(eb[0])); wo = max(wo, float(eb[1]))
            rows.append((float(eb[0]), float(eb[1]), float(ec[0]) if ec else None, float(ec[1]) if ec else None))
        if ec:
            if ec[0] > 4 * ec[1] and ec[0] > F(1, 10**15):
                worse += 1
            if ec[1] > 4 * ec[0] and ec[1] > F(1, 10**15):
                better += 1
    mc_n = max(r[2] for r in rows if r[2] is not None); mc_o = max(r[3] for r in rows if r[3] is not None)
    print(f"C: {n} cones; max B err/terms new {wn:.2e} old {wo:.2e}; max C err/terms new {mc_n:.2e} old {mc_o:.2e}; C new >4x worse {worse}, >4x better {better}")


if __name__ == "__main__" and sys.argv[1] == "C":
    check_c(sys.argv[2])


def check_c2(path):
    """Cone errors in units of the start's rounding: B in u*far*|d|^2,
    C in u*far*(|w|+far*u)."""
    import math
    u = F(2) ** -53
    eB = []; eC = []
    for line in open(path):
        if not line.startswith("C "):
            continue
        p = line.split()
        q = [fr(x) for x in p[1:4]]; d = [fr(x) for x in p[4:7]]; apex = [fr(x) for x in p[7:10]]
        cos2, span = fr(p[10]), fr(p[11])
        nB, nC, oB, oC = [fx(x) for x in p[12:16]]
        if not all(map(math.isfinite, [nB, nC, oB, oC])):
            continue
        w0 = [q[i] - apex[i] for i in range(3)]
        a2 = sum(x * x for x in d); g = sum(w0[i] * d[i] for i in range(3))
        held = max(-span, min(span, -g / a2))
        w = [w0[i] + d[i] * held for i in range(3)]
        B = d[2] * w[2] - sum(w[i] * d[i] for i in range(3)) * cos2
        C = w[2] * w[2] - sum(x * x for x in w) * cos2
        far = max(abs(x) for x in w0) + abs(held)
        nw = max(abs(x) for x in w)
        sb = u * far + F(1, 10**300); sc = u * far * (nw + u * far) + F(1, 10**300)
        eB.append((float(abs(F(nB) - B) / sb), float(abs(F(oB) - B) / sb)))
        eC.append((float(abs(F(nC) - C) / sc), float(abs(F(oC) - C) / sc)))
    for name, e in (("B", eB), ("C", eC)):
        e.sort(key=lambda t: -t[0])
        mn = max(t[0] for t in e); mo = max(t[1] for t in e)
        import statistics
        print(f"cone {name}: {len(e)} cases, in units of the start's rounding: max new {mn:.2f} old {mo:.2f}; "
              f"median new {statistics.median(t[0] for t in e):.3f} old {statistics.median(t[1] for t in e):.3f}")


if __name__ == "__main__" and sys.argv[1] == "C2":
    check_c2(sys.argv[2])


def check_w2(path):
    """Wall c error about the nearest point in units of u*far*near, the
    charge's own scale (Centred::disc_error charges 16 of them)."""
    import math
    u = F(2) ** -53
    rows = []
    for line in open(path):
        if not line.startswith("W "):
            continue
        h, mid, t1, t2 = line.split("|")
        h = h.split(); fam = int(h[1])
        q = [fr(x) for x in h[2:5]]; d = [fr(x) for x in h[5:8]]
        r, span = fr(h[8]), fr(h[9])
        nshift, nb, nc, oshift = [fx(x) for x in mid.split()]
        ob, oc, _, _ = [fx(x) for x in t1.split()]
        if not (math.isfinite(nc) and math.isfinite(oc)):
            continue
        held, b, c, tr = wall_exact(q, d, r, span)
        far = max(abs(x) for x in q) * 2 + abs(held)
        w = (q[0] + d[0] * held, q[1] + d[1] * held)
        near = max(abs(w[0]), abs(w[1])) * 2 + r
        sc = u * far * near
        rows.append((float(abs(F(nc) - c) / sc), float(abs(F(oc) - c) / sc), fam))
    mn = max(rows); mo = max(rows, key=lambda t: t[1])
    print(f"wall c in units of u*far*near: worst new {mn}, worst old {mo}")


if __name__ == "__main__" and sys.argv[1] == "W2":
    check_w2(sys.argv[2])


def check_bw(path):
    """Over each box: is the held c (or b) wider than the base's?"""
    import math
    rows = {"b": [0, 0, 0, 1.0], "c": [0, 0, 0, 1.0]}  # wider, narrower, same, worst ratio
    byfam = {}
    for line in open(path):
        if not line.startswith("B "):
            continue
        p = line.split()
        fam = int(p[1])
        nb, nc, ob, oc = (fx(p[10]), fx(p[11])), (fx(p[12]), fx(p[13])), (fx(p[14]), fx(p[15])), (fx(p[16]), fx(p[17]))
        for k, (n, o) in (("b", (nb, ob)), ("c", (nc, oc))):
            wn, wo = n[1] - n[0], o[1] - o[0]
            if not (math.isfinite(wn) and math.isfinite(wo)):
                continue
            r = rows[k]
            if wn > wo * (1 + 1e-9) + 1e-300:
                r[0] += 1
                ratio = wn / wo if wo > 0 else math.inf
                if ratio > r[3]:
                    r[3] = ratio
                byfam.setdefault((k, fam), 0)
                byfam[(k, fam)] += 1
            elif wn < wo * (1 - 1e-9):
                r[1] += 1
            else:
                r[2] += 1
    for k, r in rows.items():
        print(f"{k}: new wider in {r[0]} boxes (worst x{r[3]:.3g}), narrower in {r[1]}, same in {r[2]}")
    print("wider by family:", byfam)


if __name__ == "__main__" and sys.argv[1] == "BW":
    check_bw(sys.argv[2])
