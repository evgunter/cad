#!/usr/bin/env python3
"""Classify what the tier-3' census escalates on a near-tangent body, at 60 digits.

Reads `crates/sweep/examples/near_tangent_census_probe.rs`'s output (run with
`NT_DUMP=1`) on stdin: per body, its census findings (`FINDING`) and every
pair the census decided against (`PAIR`, the two entities by their exact
`f64` coordinates).

For each pair, every margin the census reads (`crates/topo/src/census.rs`:
`on_edge_interior`, `pair_vertex_face`, `pair_edge_edge`,
`crossing_in_both_interiors`, `collinear_overlap`, `pair_edge_face`,
`ef_overlap_cells`) is computed twice
on the same coordinates:

- in IEEE double, in the census's own operation order, which reproduces its
  escalated margins bit for bit (a reading counts as the census's only when
  it equals a `FINDING`'s margin exactly);
- in 60-digit decimal arithmetic, the same formula.

Each escalation is then classed:

- `b-arith`: the exact value of the census's own formula lies outside the
  band, so the f64 arithmetic made the margin (`b-arith/sliver` where the
  two edges nonetheless run within K*eps of each other for at least 1% of the
  shorter one, a sliver the exact reading would pass);
- `b-proxy`: the exact formula is in band, but the geometry it stands in for
  is not: the two segments lie at least K*eps apart, or a vertex lies at least
  K*eps from the face's region (a line or a plane read instead of a segment or
  a face);
- `a`: the two entities really lie within K*eps of each other where the
  census asked (two edges: for at least 1% of the shorter one), a sliver of
  the body.

`python3 scripts/oracles/near_tangent_census_classify.py [eps] < probe.log`
prints one line per escalation and a count per predicate, tilt and class.
Two edges that share a point also carry the census clause's one number for
them (`docs/DESIGN.md`, tier 3'): the far-end gap, their largest distance
over the shorter edge, placed zero, in band or definite.
A definite `EdgeEdgeCross` finding is read too: `b-arith` where the exact
crossing parameters put the crossing at an end of an edge.
"""

from __future__ import annotations

import json
import math
import re
import sys
from collections import defaultdict
from collections.abc import Callable
from decimal import Decimal, getcontext
from typing import Any

getcontext().prec = 60

D = Decimal
ZERO = D(0)


class Arith:
    """One arithmetic: IEEE double (`float`) or 60-digit decimal."""

    def __init__(self, num: Callable[[float], Any], sqrt: Callable[[Any], Any]) -> None:
        self.num, self.sqrt = num, sqrt

    def vec(self, p: list[float]) -> tuple[Any, Any, Any]:
        return (self.num(p[0]), self.num(p[1]), self.num(p[2]))

    def norm(self, a: Any) -> Any:
        return self.sqrt(dot(a, a))


F64 = Arith(float, math.sqrt)
EXACT = Arith(D, lambda x: x.sqrt())


def sub(a: Any, b: Any) -> Any:
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def add(a: Any, b: Any) -> Any:
    return (a[0] + b[0], a[1] + b[1], a[2] + b[2])


def mul(a: Any, s: Any) -> Any:
    return (a[0] * s, a[1] * s, a[2] * s)


def div(a: Any, s: Any) -> Any:
    return (a[0] / s, a[1] / s, a[2] / s)


def dot(a: Any, b: Any) -> Any:
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a: Any, b: Any) -> Any:
    return (
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    )


def edge(e: dict[str, Any], ar: Arith) -> tuple[Any, Any, Any, Any]:
    """`snapshot`'s `EdgeGeo`: start, end, unit direction, length."""
    p0, p1 = ar.vec(e["p"][0]), ar.vec(e["p"][1])
    chord = sub(p1, p0)
    ln = ar.norm(chord)
    return p0, p1, div(chord, ln), ln


def ee_readings(a: dict[str, Any], b: dict[str, Any], ar: Arith) -> dict[str, Any]:
    """Every margin `pair_edge_edge` may read on the pair, by name."""
    a0, _, da, la = edge(a, ar)
    b0, _, db, lb = edge(b, ar)
    nc = cross(da, db)
    sin = ar.norm(nc)
    out: dict[str, Any] = {"ee_parallel": sin * min(la, lb)}
    d = sub(b0, a0)
    if sin != 0:
        out["ee_gap"] = abs(dot(d, nc)) / sin
        nn = dot(nc, nc)
        sa = dot(cross(d, db), nc) / nn
        sb = dot(cross(d, da), nc) / nn
        out["ee_span"] = [sa, la - sa, sb, lb - sb]
    out["ee_line_gap"] = ar.norm(cross(d, da))
    t0 = dot(d, da)
    t1 = t0 + lb * dot(db, da)
    lo = max(min(t0, t1), ar.num(0.0))
    hi = min(max(t0, t1), la)
    out["ee_overlap"] = hi - lo
    return out


def ef_readings(e: dict[str, Any], f: dict[str, Any], ar: Arith) -> dict[str, Any]:
    """Every margin `pair_edge_face` and `ef_overlap_cells` may read."""
    p0, p1, dr, _ = edge(e, ar)
    o, n = ar.vec(f["origin"]), ar.vec(f["normal"])
    # `pair_edge_face` reads the far end as `p0 + dir * len`.
    _, _, _, ln = edge(e, ar)
    p1 = add(p0, mul(dr, ln))
    cut = [
        ar.norm(cross(sub(ar.vec(w["p"]), p0), dr)) for ring in f["loops"] for w in ring
    ]
    return {
        "ef_residual": [dot(sub(p0, o), n), dot(sub(p1, o), n)],
        "ef_cut_gap": cut,
    }


def ve_readings(v: dict[str, Any], e: dict[str, Any], ar: Arith) -> dict[str, Any]:
    """`on_edge_interior`'s line gap (pass 2)."""
    p0, _, dr, _ = edge(e, ar)
    return {"ve_line_gap": ar.norm(cross(sub(ar.vec(v["p"]), p0), dr))}


def vf_readings(v: dict[str, Any], f: dict[str, Any], ar: Arith) -> dict[str, Any]:
    """`pair_vertex_face`'s residual (pass 3)."""
    o, n = ar.vec(f["origin"]), ar.vec(f["normal"])
    return {"vf_residual": dot(sub(ar.vec(v["p"]), o), n)}


READINGS = {"ee": ee_readings, "ef": ef_readings, "ve": ve_readings, "vf": vf_readings}


def flat(v: Any) -> list[Any]:
    return list(v) if isinstance(v, list) else [v]


# ---- the geometry a predicate stands in for (exact) ----------------------


def point_segment(p: Any, a: Any, b: Any) -> Decimal:
    ab = sub(b, a)
    t = max(ZERO, min(D(1), dot(sub(p, a), ab) / dot(ab, ab)))
    return EXACT.norm(sub(p, add(a, mul(ab, t))))


def golden(f: Callable[[Decimal], Decimal]) -> Decimal:
    """The minimiser over [0, 1] of a convex `f`."""
    lo, hi, g = ZERO, D(1), (D(5).sqrt() - 1) / 2
    for _ in range(160):
        m1, m2 = hi - g * (hi - lo), lo + g * (hi - lo)
        if f(m1) <= f(m2):
            hi = m2
        else:
            lo = m1
    return (lo + hi) / 2


def segments(a: dict[str, Any], b: dict[str, Any], keps: Decimal) -> dict[str, Any]:
    """The two segments' least distance, and the length of the shorter one
    lying within K*eps of the other (an interval: the distance is convex)."""
    a0, a1, _, la = edge(a, EXACT)
    b0, b1, _, lb = edge(b, EXACT)
    (s0, s1, ls), (o0, o1) = (
        ((a0, a1, la), (b0, b1)) if la <= lb else ((b0, b1, lb), (a0, a1))
    )

    def f(s: Decimal) -> Decimal:
        return point_segment(add(s0, mul(sub(s1, s0), s)), o0, o1)

    s_min = golden(f)
    dist = f(s_min)

    def bound(inside: Decimal, outside: Decimal) -> Decimal:
        if f(outside) <= keps:
            return outside
        for _ in range(160):
            mid = (inside + outside) / 2
            inside, outside = (mid, outside) if f(mid) <= keps else (inside, mid)
        return inside

    run = (bound(s_min, D(1)) - bound(s_min, ZERO)) * ls if dist <= keps else ZERO
    return {
        "dist": dist,
        "run": run,
        # The census clause's one number for cells that share a point:
        # their largest distance over the shorter cell, at an end of it
        # (the distance is convex), so its far end's.
        "gap": max(point_segment(s0, o0, o1), point_segment(s1, o0, o1)),
        "shared_point": bool(set(a["pk"]) & set(b["pk"])),
        "ends": min(EXACT.norm(sub(p, q)) for p in (a0, a1) for q in (b0, b1)),
    }


def point_face(p: Any, f: dict[str, Any]) -> tuple[Decimal, bool]:
    """A point's distance to a planar face's region, and whether its
    projection lies inside it."""
    o, n = EXACT.vec(f["origin"]), EXACT.vec(f["normal"])
    n = div(n, EXACT.norm(n))
    r = dot(sub(p, o), n)
    q = sub(p, mul(n, r))
    seed = (D(0), D(0), D(1)) if abs(n[2]) < D("0.9") else (D(1), D(0), D(0))
    u = cross(seed, n)
    u = div(u, EXACT.norm(u))
    w = cross(n, u)
    x, y = dot(q, u), dot(q, w)
    inside, bound = False, None
    for ring in f["loops"]:
        pts = [EXACT.vec(v["p"]) for v in ring]
        for i, s in enumerate(pts):
            t = pts[(i + 1) % len(pts)]
            sx, sy, tx, ty = dot(s, u), dot(s, w), dot(t, u), dot(t, w)
            if (sy > y) != (ty > y) and x < sx + (y - sy) * (tx - sx) / (ty - sy):
                inside = not inside
            d = point_segment(p, s, t)
            bound = d if bound is None else min(bound, d)
    assert bound is not None
    return (abs(r) if inside else bound), inside


class Band:
    def __init__(self, eps: float, k: float = 10.0) -> None:
        self.zero, self.escalate = eps, eps * k

    def place(self, m: Any) -> str:
        a = abs(m)
        if a <= self.zero:
            return "zero"
        return "definite" if a >= self.escalate else "band"


def classify(
    kind: str,
    a: dict[str, Any],
    b: dict[str, Any],
    pred: str,
    index: int,
    exact: Any,
    band: Band,
) -> tuple[str, str]:
    keps = D(band.escalate)
    if kind == "ee":
        g = segments(a, b, keps)
        _, _, da, _ = edge(a, EXACT)
        _, _, db, _ = edge(b, EXACT)
        sin = EXACT.norm(cross(da, db))
        # Each edge's heading away from the point they share.
        shared = set(a["pk"]) & set(b["pk"])
        away = [
            d if e["pk"][0] in shared else mul(d, D(-1)) for e, d in ((a, da), (b, db))
        ]
        heads = "same" if dot(away[0], away[1]) > 0 else "opposite"
        one = "one vertex" if set(a["v"]) & set(b["v"]) else "two vertices"
        if set(a.get("faces", [])) & set(b.get("faces", [])):
            one += ", one face"
        where = (
            f"share a point ({one}), {heads} heads"
            if g["shared_point"]
            else f"ends {g['ends']:.2e} apart"
        )
        shape = (
            f"sin {sin:.3e}, {where}, dist {g['dist']:.3e}, K*eps run {g['run']:.3e}"
        )
        if g["shared_point"]:
            shape += f", far-end gap {g['gap']:.3e} ({band.place(g['gap'])})"
        # Two segments from one point at angle t lie within K*eps of each
        # other for K*eps/t, whatever t; a sliver is a run that is a visible
        # share of the shorter edge, which this takes as 1%.
        short = min(edge(a, EXACT)[3], edge(b, EXACT)[3])
        sliver = g["dist"] < keps and g["run"] >= short / 100
        if band.place(exact) != "band":
            cls = "b-arith/sliver" if sliver else "b-arith"
            return cls, f"exact {exact:.3e} ({band.place(exact)}); {shape}"
        if g["dist"] >= keps:
            return "b-proxy", f"segments apart, {pred} reads lines; {shape}"
        if not sliver:
            return (
                "b-proxy",
                f"meet only at their shared point, {pred} reads lines; {shape}",
            )
        return "a", shape
    if band.place(exact) != "band":
        return "b-arith", f"exact {exact:.3e} ({band.place(exact)})"
    if kind == "ve":
        p0, p1, _, _ = edge(b, EXACT)
        dist = point_segment(EXACT.vec(a["p"]), p0, p1)
        if band.place(exact) != "band":
            return "b-arith", f"exact {exact:.3e} ({band.place(exact)})"
        if dist >= keps:
            return "b-proxy", f"vertex {dist:.3e} from the segment, reads line"
        return "a", f"vertex {dist:.3e} from the segment"
    if kind == "vf":
        dist, inside = point_face(EXACT.vec(a["p"]), b)
        side = "inside" if inside else "outside"
        if band.place(exact) != "band":
            return "b-arith", f"exact {exact:.3e} ({band.place(exact)})"
        if dist >= keps:
            return (
                "b-proxy",
                f"vertex {dist:.3e} from the face (projects {side}), reads plane",
            )
        return "a", f"vertex {dist:.3e} from the face (projects {side})"
    if pred == "ef_residual":
        p0, _, dr, ln = edge(a, EXACT)
        p = p0 if index == 0 else add(p0, mul(dr, ln))
        dist, inside = point_face(p, b)
        side = "inside" if inside else "outside"
        if dist >= keps:
            return (
                "b-proxy",
                f"end {index} {dist:.3e} from the face (projects {side}), reads plane",
            )
        return "a", f"end {index} {dist:.3e} from the face (projects {side})"
    w = [v for ring in b["loops"] for v in ring][index]
    p0, p1, _, ln = edge(a, EXACT)
    dist = point_segment(EXACT.vec(w["p"]), p0, p1)
    if dist >= keps:
        return "b-proxy", f"vertex {w['v']} {dist:.3e} from the segment, reads line"
    return "a", f"vertex {w['v']} {dist:.3e} from the segment"


GAP = re.compile(r"far-end gap \S+ \((\w+)\)")
TAG = re.compile(r"^(\S+) nt e(\d) a(\d+) d(\S+) (pc|cp) ([UIS])$")
FINDING = re.compile(r"^  FINDING (.*?) \| esc pm_census_(\S+) (\S+)$")
CROSS = re.compile(
    r"^  FINDING (.*?) \| UndeclaredContact \{ contact: EdgeEdgeCross "
    r"\{ a: (EdgeKey\(\S+\)), b: (EdgeKey\(\S+\)) \}"
)


def main() -> None:
    band = Band(float(sys.argv[1]) if len(sys.argv) > 1 else 1e-9)
    findings: dict[str, set[tuple[str, float]]] = defaultdict(set)
    rows: list[tuple[str, str, float, Any, str, str, str]] = []
    matched: dict[str, set[tuple[str, float]]] = defaultdict(set)
    crosses: dict[str, set[tuple[str, str]]] = defaultdict(set)
    cross_rows: list[str] = []
    for line in sys.stdin:
        c = CROSS.match(line.rstrip("\n"))
        if c:
            crosses[c.group(1)].add((c.group(2), c.group(3)))
            continue
        m = FINDING.match(line.rstrip("\n"))
        if m:
            findings[m.group(1)].add((m.group(2), float(m.group(3))))
            continue
        if not line.startswith("PAIR "):
            continue
        p = json.loads(line[5:])
        if p["kind"] not in READINGS:
            continue
        reads = READINGS[p["kind"]]
        f64, exact = reads(p["a"], p["b"], F64), reads(p["a"], p["b"], EXACT)
        crossing = (p["a"]["key"], p["b"]["key"])
        if p["kind"] == "ee" and crossing in crosses[p["tag"]]:
            # A definite crossing finding: where the exact spans put it.
            spans = flat(exact["ee_span"])
            at = min(abs(x) for x in spans)
            cls = "b-arith" if band.place(at) == "zero" else "definite"
            shared = (
                "share a point" if set(p["a"]["pk"]) & set(p["b"]["pk"]) else "apart"
            )
            cross_rows.append(
                f"{p['tag']} | EdgeEdgeCross {crossing[0]} {crossing[1]} | {cls} | "
                f"f64 spans {min(abs(x) for x in flat(f64['ee_span'])):.3e}, "
                f"exact {at:.3e}, {shared}"
            )
            crosses[p["tag"]].discard(crossing)
        for pred, vals in f64.items():
            for i, v in enumerate(flat(vals)):
                if (pred, v) not in findings[p["tag"]]:
                    continue
                matched[p["tag"]].add((pred, v))
                x = flat(exact[pred])[i]
                cls, why = classify(p["kind"], p["a"], p["b"], pred, i, x, band)
                pair = f"{p['a']['key']} {p['b']['key']}"
                rows.append((p["tag"], pred, v, x, pair, cls, why))
    by: dict[tuple[str, str, str], int] = defaultdict(int)
    for tag, pred, v, x, pair, cls, why in rows:
        m = TAG.match(tag)
        by[(pred, m.group(4) if m else "?", cls)] += 1
        print(f"{tag} | {pred} f64 {v:.6e} exact {x:.6e} | {pair} | {cls} | {why}")
    unmatched = sum(len(fs - matched[t]) for t, fs in findings.items())
    print(
        f"# escalations: {sum(len(f) for f in findings.values())}, unmatched: {unmatched}"
    )
    for t, fs in sorted(findings.items()):
        for pred, v in sorted(fs - matched[t]):
            print(f"# unmatched {t} | {pred} {v:.6e}")
    for r in cross_rows:
        print(r)
    tally: dict[str, int] = defaultdict(int)
    for r in cross_rows:
        tally[r.split(" | ")[2]] += 1
    left = sum(len(c) for c in crosses.values())
    print(f"# EdgeEdgeCross findings: {dict(tally)}, unmatched: {left}")
    print("# predicate, d, class: escalations")
    for k in sorted(by):
        print(f"# {k[0]} {k[1]} {k[2]}: {by[k]}")
    # The census clause's arms on the pairs that share a point: a definite
    # far-end gap is legal, one in band a sliver, a zero a coincidence.
    gaps: dict[tuple[str, str], int] = defaultdict(int)
    for _, _, _, _, _, cls, why in rows:
        g = GAP.search(why)
        if g:
            gaps[(cls, g.group(1))] += 1
    print("# shared-point pairs by class and far-end gap: count")
    for k in sorted(gaps):
        print(f"# {k[0]} gap {k[1]}: {gaps[k]}")


if __name__ == "__main__":
    main()
