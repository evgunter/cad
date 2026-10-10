#!/usr/bin/env python3
"""The high-precision oracle for the section certificate's square-wall arm.

Reads the JSON lines `topo`'s `section_cert::square_search::dump_for_the_mpmath_oracle`
writes (one pose per line: the stored torus and cylinder, and the arm's answer)
and re-solves each pose's section at 40 significant digits from the STORED
values, by a route that never reads the meridian plane the arm works in:

- every ruling of the stored cylinder, `o + rc*(e1 cos th + e2 sin th) + t*d`,
  meets the torus where the quartic `(q + R^2 - r^2)^2 - 4R^2 (q - h^2)` in `t`
  vanishes (`q` the squared distance from the centre, `h` the height); its real
  roots are found by `polyroots`;
- the azimuths where a ruling stands inside the torus's slab `|h| <= r` are
  sampled densely, consecutive rulings' roots linked in order, and an interval
  whose root count changes is bisected down to `1e-13` rad, where the two roots
  that appear or vanish are joined (a fold);
- each linked component is classed: essential on the wall iff it crosses the
  seam `th = 0` an odd number of times, essential on the torus iff its azimuth
  or tube angle wraps an odd number of times.

Then, for every classified pose:

- the classes must match part for part, and `single` must mean one component;
- each witness must lie within `1e-12` of the scale of both carriers (true
  distances at 40 digits) and on a component of its part's classes, distinct
  components for distinct parts.

It prints the counts and the worst witness distance, and exits non-zero on any
violation. Run:

    cargo nextest run -p topo dump_for_the_mpmath_oracle --run-ignored only \\
        --no-capture | sed -n 's/^SQDUMP //p' > /tmp/sq.jsonl
    python3 scripts/oracles/torus_square_wall_mpmath.py /tmp/sq.jsonl
"""

import json
import math
import sys

from mpmath import mp, mpf, polyroots

mp.dps = 40
NODES = 1200


def vec(xs):
    return [mpf(x) for x in xs]


def add(a, b):
    return [x + y for x, y in zip(a, b, strict=True)]


def sub(a, b):
    return [x - y for x, y in zip(a, b, strict=True)]


def scale(a, k):
    return [x * k for x in a]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b, strict=True))


def cross(a, b):
    return [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]


def norm(a):
    return mp.sqrt(dot(a, a))


class Pose:
    def __init__(self, d):
        t, w = d["torus"], d["wall"]
        self.c, self.a, self.b1 = vec(t["c"]), vec(t["a"]), vec(t["b1"])
        self.b2 = cross(self.a, self.b1)
        self.R, self.r = mpf(t["R"]), mpf(t["r"])
        self.o, self.d, self.e1 = vec(w["o"]), vec(w["d"]), vec(w["e1"])
        self.e2 = cross(self.d, self.e1)
        self.rc = mpf(w["rc"])
        self.scale = self.R + self.r

    def base(self, th):
        return add(self.o, scale(add(scale(self.e1, mp.cos(th)), scale(self.e2, mp.sin(th))), self.rc))

    def quartic(self, th):
        """The ruling's quartic in `t`, highest power first."""
        w0 = sub(self.base(th), self.c)
        A, B, C = dot(self.d, self.d), dot(w0, self.d), dot(w0, w0)
        ha, h0 = dot(self.d, self.a), dot(w0, self.a)
        k = self.R ** 2 - self.r ** 2
        # q(t) = A t^2 + 2B t + C; s = q + k; h(t) = h0 + ha t.
        s = [A, 2 * B, C + k]
        q = [A, 2 * B, C]
        hh = [ha * ha, 2 * ha * h0, h0 * h0]
        sq = [0] * 5
        for i in range(3):
            for j in range(3):
                sq[i + j] += s[i] * s[j]
        poly = [sq[0], sq[1], sq[2] - 4 * self.R ** 2 * (q[0] - hh[0]),
                sq[3] - 4 * self.R ** 2 * (q[1] - hh[1]), sq[4] - 4 * self.R ** 2 * (q[2] - hh[2])]
        return poly

    def roots_and_gap(self, th):
        """The ruling's real roots, ascending, and the least distance
        between two of its four roots, real or not (how near the ruling is
        to touching the torus, where two roots meet)."""
        rs = polyroots(self.quartic(th), maxsteps=200, extraprec=200)
        tol = mpf(10) ** -18 * self.scale
        real = sorted(mp.re(x) for x in rs if abs(mp.im(x)) < tol)
        gap = min(abs(x - y) for i, x in enumerate(rs) for y in rs[i + 1:])
        return real, gap

    def roots(self, th):
        return self.roots_and_gap(th)[0]

    def point(self, th, t):
        return add(self.base(th), scale(self.d, t))

    def angles(self, x):
        w = sub(x, self.c)
        z = dot(w, self.a)
        rho = norm(cross(self.a, cross(w, self.a)))
        return mp.atan2(dot(w, self.b2), dot(w, self.b1)), mp.atan2(z, rho - self.R)

    def height(self, th):
        return dot(sub(self.base(th), self.c), self.a)

    def torus_distance(self, x):
        w = sub(x, self.c)
        z = dot(w, self.a)
        rho = norm(cross(self.a, cross(w, self.a)))
        return mp.sqrt((rho - self.R) ** 2 + z ** 2) - self.r

    def wall_distance(self, x):
        return norm(cross(sub(x, self.o), self.d)) - self.rc


def pairings(n, k, start=0):
    """Every set of `k` disjoint adjacent index pairs in `0..n`, flattened."""
    if k == 0:
        yield set()
        return
    for j in range(start, n - 1):
        for rest in pairings(n, k - 1, j + 2):
            yield {j, j + 1} | rest


class Union:
    def __init__(self):
        self.p = {}

    def find(self, x):
        self.p.setdefault(x, x)
        while self.p[x] != x:
            self.p[x] = self.p[self.p[x]]
            x = self.p[x]
        return x

    def join(self, x, y):
        self.p[self.find(x)] = self.find(y)


def slab_runs(pose, coarse=20000):
    """Azimuth intervals whose rulings may reach the slab, as float pairs
    (a float scan: it only bounds where the dense sampling goes)."""
    def f(v):
        return [float(x) for x in v]

    c, a, o, e1, e2, d = f(pose.c), f(pose.a), f(pose.o), f(pose.e1), f(pose.e2), f(pose.d)
    rc, r = float(pose.rc), float(pose.r)
    drift = abs(sum(x * y for x, y in zip(d, a, strict=True))) * 4 * float(pose.scale)
    lim = 1.02 * r + drift + 1e-9 * r
    tau = 2 * math.pi

    def height(th):
        b = [o[i] + rc * (e1[i] * math.cos(th) + e2[i] * math.sin(th)) - c[i] for i in range(3)]
        return sum(x * y for x, y in zip(b, a, strict=True))

    ins = [abs(height(tau * i / coarse)) <= lim for i in range(coarse)]
    if all(ins):
        return None
    runs = []
    start = ins.index(False)
    cur = None
    for k in [(start + k) % coarse for k in range(coarse)]:
        if ins[k] and cur is None:
            cur = k
        if not ins[k] and cur is not None:
            hi = k if k > cur else k + coarse
            runs.append(((cur - 1) * tau / coarse, hi * tau / coarse))
            cur = None
    if cur is not None:
        runs.append(((cur - 1) * tau / coarse, (start + coarse) * tau / coarse))
    return runs


def islands(pose, ths):
    """`ths` with azimuths added wherever two samples may hide a change of
    root count between them: where a sample's ruling stands near touching
    the torus (two of its roots near each other), each interval beside it
    is bisected toward the least root distance until the count changes (a
    pair of roots appearing or vanishing, now sampled) or the valley's
    floor stands clear of it."""
    gaps = [pose.roots_and_gap(th) for th in ths]
    near = mpf(10) ** -2 * pose.scale
    out = []

    def descend(tha, ga, thb, gb, count, depth):
        if depth > 90:
            return []
        mid = (tha + thb) / 2
        real, gm = pose.roots_and_gap(mid)
        if len(real) != count:
            return [mid]
        if gm < min(ga, gb) or depth < 4:
            return [
                *descend(tha, ga, mid, gm, count, depth + 1),
                mid,
                *descend(mid, gm, thb, gb, count, depth + 1),
            ]
        return []

    for k in range(len(ths) - 1):
        out.append(ths[k])
        (ra, ga), (rb, gb) = gaps[k], gaps[k + 1]
        if len(ra) == len(rb) and min(ga, gb) < near:
            out.extend(descend(ths[k], ga, ths[k + 1], gb, len(ra), 0))
    out.append(ths[-1])
    return out


def section(pose):
    """The traced components: (classes per component, node -> component)."""
    uf = Union()
    edges = []  # (node, node, seam)
    nodes = {}  # id -> (th, t)
    nid = [0]

    def new_nodes(th, rs):
        ids = []
        for t in rs:
            nodes[nid[0]] = (th, t)
            ids.append(nid[0])
            uf.find(nid[0])
            nid[0] += 1
        return ids

    def link(tha, ida, thb, idb, seam, depth=0):
        if len(ida) == len(idb):
            for x, y in zip(ida, idb, strict=True):
                edges.append((x, y, seam))
            return
        if thb - tha > mpf(10) ** -13 and depth < 80:
            mid = (tha + thb) / 2
            idm = new_nodes(mid, pose.roots(mid))
            link(tha, ida, mid, idm, False, depth + 1)
            link(mid, idm, thb, idb, seam, depth + 1)
            return
        if len(ida) % 2 != len(idb) % 2:
            raise RuntimeError("an odd change of root count at th %s" % tha)
        big, small, big_is_a = (ida, idb, True) if len(ida) > len(idb) else (idb, ida, False)
        ts = [nodes[x][1] for x in small]
        # The roots that appear or vanish come in adjacent pairs, each
        # pair joined through its fold (two pairs at once where a ruling
        # touches the top or bottom circle at ±t): the pairing that leaves
        # the rest closest to the other side's roots.
        best = None
        for removed in pairings(len(big), (len(big) - len(small)) // 2):
            rest = [x for i, x in enumerate(big) if i not in removed]
            cost = sum(abs(nodes[x][1] - t) for x, t in zip(rest, ts, strict=True))
            if best is None or cost < best[0]:
                best = (cost, removed, rest)
        _, removed, rest = best
        for j in sorted(removed)[::2]:
            uf.join(big[j], big[j + 1])
            edges.append((big[j], big[j + 1], False))
        for x, y in zip(rest, small, strict=True):
            edges.append((x, y, seam) if big_is_a else (y, x, seam))

    runs = slab_runs(pose)
    tau = 2 * mp.pi
    if runs is None:
        ths = islands(pose, [tau * k / NODES for k in range(NODES + 1)])[:-1]
        NODES_ = len(ths)
        ids = [new_nodes(th, pose.roots(th)) for th in ths]
        for k in range(NODES_):
            k2 = (k + 1) % NODES_
            th2 = ths[k2] + (tau if k2 == 0 else 0)
            link(ths[k], ids[k], th2, ids[k2], k2 == 0)
    else:
        for lo, hi in runs:
            ths = islands(pose, [mpf(lo) + (mpf(hi) - mpf(lo)) * k / NODES for k in range(NODES + 1)])
            ids = [new_nodes(th, pose.roots(th)) for th in ths]
            if ids[0] or ids[-1]:
                raise RuntimeError("a slab run's end ruling meets the torus")
            for k in range(len(ths) - 1):
                # The seam is th = 0 (mod 2π): an edge crossing a multiple
                # of 2π crosses it.
                seam = math.floor(float(ths[k]) / (2 * math.pi)) != math.floor(float(ths[k + 1]) / (2 * math.pi))
                link(ths[k], ids[k], ths[k + 1], ids[k + 1], seam)
    for x, y, _ in edges:
        uf.join(x, y)
    comps = {}
    for x in nodes:
        comps.setdefault(uf.find(x), len(comps))
    odd = [[False, False, False] for _ in comps]
    ang = {x: pose.angles(pose.point(*nodes[x])) for x in nodes}
    for x, y, seam in edges:
        k = comps[uf.find(x)]
        (ux, vx), (uy, vy) = ang[x], ang[y]
        odd[k][0] ^= abs(ux - uy) > mp.pi
        odd[k][1] ^= abs(vx - vy) > mp.pi
        odd[k][2] ^= seam
    classes = [(o[0] or o[1], o[2]) for o in odd]
    return classes, nodes, {x: comps[uf.find(x)] for x in nodes}


def component_of(pose, nodes, comp_of, w):
    rel = sub(w, pose.o)
    th = mp.atan2(dot(rel, pose.e2), dot(rel, pose.e1)) % (2 * mp.pi)
    t = dot(rel, pose.d)
    best = None
    for x, (thx, tx) in nodes.items():
        dth = abs(((thx - th + mp.pi) % (2 * mp.pi)) - mp.pi) * pose.rc
        dist = dth + abs(tx - t)
        if best is None or dist < best[0]:
            best = (dist, x)
    return None if best is None else comp_of[best[1]]


def check(line):
    """One pose's violations, its worst witness distance, and whether it
    was traced."""
    d = json.loads(line)
    ans = d["answer"]
    if ans["kind"] != "components":
        return [], mpf(0), False
    pose = Pose(d)
    try:
        classes, nodes, comp_of = section(pose)
    except RuntimeError as e:
        return ["%s: %s" % (d["what"], e)], mpf(0), True
    worst, bad = mpf(0), []
    got = sorted((p["torus"], p["wall"]) for p in ans["parts"])
    want = sorted(classes)
    if got != want:
        bad.append("%s: traced %s, classified %s" % (d["what"], want, got))
    if ans["single"] != (len(classes) == 1):
        bad.append("%s: single %s, traced %s" % (d["what"], ans["single"], want))
    seen = set()
    for p in ans["parts"]:
        w = vec(p["witness"])
        off = max(abs(pose.torus_distance(w)), abs(pose.wall_distance(w)))
        worst = max(worst, off / pose.scale)
        if off > mpf(10) ** -12 * pose.scale:
            bad.append("%s: the witness %s is %s off" % (d["what"], p["witness"], mp.nstr(off, 5)))
        k = component_of(pose, nodes, comp_of, w)
        if k is None or classes[k] != (p["torus"], p["wall"]) or k in seen:
            bad.append("%s: the witness %s is on component %s" % (d["what"], p["witness"], k))
        seen.add(k)
    return bad, worst, True


def main(path):
    from multiprocessing import Pool

    with Pool() as pool:
        results = pool.map(check, open(path).readlines())
    bad = [b for r in results for b in r[0]]
    n = sum(1 for r in results if r[2])
    worst = max((r[1] for r in results), default=mpf(0))
    print("%d poses traced at %d digits, %d refused; worst witness distance %s of the scale"
          % (n, mp.dps, len(results) - n, mp.nstr(worst, 3)))
    for b in bad:
        print("VIOLATION", b)
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1]))
