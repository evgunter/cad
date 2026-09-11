"""CERT-8 review probe (reviewer lane 8r2): standalone rho/pin analysis.

Reproduces `certified_arms`' assembly in plain Python so the skew
discount and the swap row's pin can be searched over random derivative
nets independently of the kernel. Adopted with the unit; lint-conformed
only (multi-statement lines split, `zip` strictness and the RNG's
non-cryptographic use made explicit) — no behaviour changed.
"""

import math
import random


# The search below samples derivative nets. `random` is the right tool
# for that and the wrong one for anything with a security premise, so
# the two suppressions live here, once, rather than at eight call sites.
def rnd_int(lo, hi):
    """A sampled net size."""
    return random.randint(lo, hi)  # noqa: S311 — sampling, not keying


def rnd_uniform(lo, hi):
    """A sampled control-difference coordinate."""
    return random.uniform(lo, hi)  # noqa: S311 — sampling, not keying


def sub(a, b):
    return tuple(x - y for x, y in zip(a, b, strict=True))


def add(a, b):
    return tuple(x + y for x, y in zip(a, b, strict=True))


def scale(a, s):
    return tuple(x * s for x in a)


def dot(a, b):
    return sum(x * y for x, y in zip(a, b, strict=True))


def cross(a, b):
    return (
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    )


def norm(a):
    return math.sqrt(dot(a, a))


def net_inf(q, sup):
    if not q:
        return 0.0
    c = (0.0, 0.0, 0.0)
    for v in q:
        c = add(c, v)
    md = min(dot(v, c) for v in q)
    n = norm(c)
    if n == 0:
        return 0.0
    raw = md / n
    if raw != raw:
        return 0.0
    return min(max(raw, 0.0), sup)


def assemble(qu, qv, ratio=1.0):
    su = max((norm(v) for v in qu), default=0.0)
    sv = max((norm(v) for v in qv), default=0.0)
    cr = [cross(a, b) for a in qu for b in qv]
    cs = max((norm(v) for v in cr), default=0.0)
    inf_u = net_inf(qu, su) / ratio
    inf_v = net_inf(qv, sv) / ratio
    sup_u = su * ratio
    sup_v = sv * ratio
    area = net_inf(cr, cs) / ratio**2
    return inf_u, inf_v, sup_u, sup_v, area


def rho(inf_u, inf_v, sup_u, sup_v, area, clamp=True):
    T = (sup_u / inf_u) ** 2 + (sup_v / inf_v) ** 2
    D = (area / (inf_u * inf_v)) ** 2
    root = math.sqrt(max(T * T - 4 * D, 0.0))
    r = math.sqrt(2 * D / (T + root))
    return min(r, 1.0) if clamp else r


print("=== fixture: flat_chart(4,1) ===")
qu = [(4.0, 0.0, 0.0)] * 2
qv = [(0.0, 1.0, 0.0)] * 2
b = assemble(qu, qv)
r = rho(*b)
print(b, "rho", r, "arms", (b[0] * r, b[1] * r))

print("=== swap chart (0.5 -> 8) ===")
qu = [(0.5, 0, 0), (0.5, 0, 0), (8.0, 0, 0), (8.0, 0, 0)]
qv = [(0.0, 1.0, 0.0)] * 3
b = assemble(qu, qv)
r = rho(*b)
print(b, "rho", r)
inf_u, inf_v, sup_u, sup_v, area = b
print(" arm_u =", inf_u * r, " inf_u =", inf_u, " pin arm<=inf:", inf_u * r <= inf_u)
print(
    " IF the assembly multiplied SUP by rho: arm=",
    sup_u * r,
    " <= inf_u?",
    sup_u * r <= inf_u,
)
# full swap: infs and sups exchanged in the assembly
T = (inf_u / sup_u) ** 2 + (inf_v / sup_v) ** 2
D = (area / (sup_u * sup_v)) ** 2
r2 = math.sqrt(2 * D / (T + math.sqrt(max(T * T - 4 * D, 0))))
print(
    " IF gate+assembly both read SUP: arm=",
    sup_u * min(r2, 1.0),
    " <= inf_u?",
    sup_u * min(r2, 1.0) <= inf_u,
)

print("=== search for rho > 1 (clamp removed) ===")
random.seed(7)
best = 0.0
bestcase = None
for _ in range(400000):
    n = rnd_int(1, 3)
    m = rnd_int(1, 3)
    qu = [tuple(rnd_uniform(-3, 3) for _ in range(3)) for _ in range(n)]
    qv = [tuple(rnd_uniform(-3, 3) for _ in range(3)) for _ in range(m)]
    b = assemble(qu, qv)
    if b[0] <= 1e-9 or b[1] <= 1e-9:
        continue
    try:
        r = rho(*b, clamp=False)
    except ZeroDivisionError:
        continue
    if r > best:
        best = r
        bestcase = (qu, qv, b)
print(" max rho found:", best)
if best > 1.0:
    print(" WITNESS:", bestcase)
print("=== is D<=1 always? max D found: ===")
random.seed(11)
maxD = 0
for _ in range(400000):
    n = rnd_int(1, 3)
    m = rnd_int(1, 3)
    qu = [tuple(rnd_uniform(-3, 3) for _ in range(3)) for _ in range(n)]
    qv = [tuple(rnd_uniform(-3, 3) for _ in range(3)) for _ in range(m)]
    b = assemble(qu, qv)
    if b[0] <= 1e-9 or b[1] <= 1e-9:
        continue
    D = (b[4] / (b[0] * b[1])) ** 2
    if D > maxD:
        maxD = D
        wD = (qu, qv, b)
print(" max D:", maxD)
if maxD > 1.0:
    print(" D-witness:", wD)
