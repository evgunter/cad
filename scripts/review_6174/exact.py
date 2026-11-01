"""Exact roots of the dumped line x quadric quadratics, against the
computed roots and their slack. Floats are read exactly (Decimal of the
round-trip repr is exact for the binary value only through Fraction)."""
import sys
from fractions import Fraction as F
from decimal import Decimal, getcontext
getcontext().prec = 80


def fr(s):
    return F(float(s))  # float(s) round-trips the Rust {:?} repr exactly


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def sub(a, b):
    return [x - y for x, y in zip(a, b)]


def roots(A, B, C):
    """Real roots of A t^2 + 2B t + C, as Decimals."""
    disc = B * B - A * C
    if disc <= 0:
        return None
    sq = Decimal(disc.numerator) / Decimal(disc.denominator)
    sq = sq.sqrt()
    A_, B_ = Decimal(A.numerator) / Decimal(A.denominator), Decimal(B.numerator) / Decimal(B.denominator)
    r = sorted([(-B_ - sq) / A_, (-B_ + sq) / A_])
    return r


stats = {}
bad = []
for line in open(sys.argv[1]):
    f = line.split()
    kind = f[0]
    if kind == 'wall':
        about = f[1]
        q, d, o, a = [list(map(fr, f[2 + 3 * i:5 + 3 * i])) for i in range(4)]
        r = fr(f[14]); t0, t1, slack = map(float, f[15:18])
        w0 = sub(q, o)
        # w_perp(t) = w0p + t dp, with dp = d - a (d.a), w0p = w0 - a (w0.a)
        w0p = [x - y * dot(w0, a) for x, y in zip(w0, a)]
        dp = [x - y * dot(d, a) for x, y in zip(d, a)]
        A, B, C = dot(dp, dp), dot(w0p, dp), dot(w0p, w0p) - r * r
        size = float(r)
    elif kind == 'sphere':
        about = f[1]
        q, d, c = [list(map(fr, f[2 + 3 * i:5 + 3 * i])) for i in range(3)]
        r = fr(f[11]); t0, t1, slack = map(float, f[12:15])
        w0 = sub(q, c)
        A, B, C = dot(d, d), dot(w0, d), dot(w0, w0) - r * r
        size = float(r)
    else:
        about = f[1]
        q, d, c, a = [list(map(fr, f[2 + 3 * i:5 + 3 * i])) for i in range(4)]
        cosv = fr(f[15]); t0, t1, slack = map(float, f[16:19])
        cos2 = cosv * cosv
        w0 = sub(q, c)
        da, wa = dot(d, a), dot(w0, a)
        A = da * da - dot(d, d) * cos2
        B = wa * da - dot(w0, d) * cos2
        C = wa * wa - dot(w0, w0) * cos2
        size = 1.0
    ex = roots(A, B, C)
    if ex is None:
        bad.append(('noroots', line.strip()[:80]))
        continue
    got = sorted([t0, t1])
    err = max(abs(Decimal(g) - e) for g, e in zip(got, ex))
    err = float(err)
    key = (kind, about)
    s = stats.setdefault(key, [0, 0, 0.0, 0.0])
    s[0] += 1
    ratio = err / slack if slack > 0 else (float('inf') if err > 0 else 0.0)
    s[2] = max(s[2], ratio)
    s[3] = max(s[3], err)
    if err > slack:
        s[1] += 1
        if len(bad) < 30:
            bad.append((kind, about, 'err', err, 'slack', slack, 'q', [float(x) for x in q][:3]))
for k, v in sorted(stats.items()):
    print(f"{k}: n={v[0]} slack-violations={v[1]} worst err/slack={v[2]:.3g} worst err={v[3]:.3g}")
for b in bad[:30]:
    print(b)
