---
id: signed-penetration-depth
kind: issue
title: Clearance reports a coincidence, not a signed penetration depth
status: open
opened: 2026-09-03
---

## What

M10-5's strictly-positive question (`self_intersection`, funnel site
`SELF_INTERSECTION_GAP`) can now REPORT an interpenetration — that arm
was unreachable when the unit was first reviewed, and PR 1638's exhibit
arm makes it reachable — but what it reports is that two points on the
two surfaces are within ε of each other. It is never that the two
bodies OVERLAP, and never by how much.

The witness a consumer receives (`crates/editor-core/src/clearance.rs:443`,
`GeometryWitness::distance`) is an unsigned `f64` distance between two
points. On two unit blocks overlapping by half their width, the reported
distance is ~1e-16: a coincidence at the crossing surface, pinned in
`crates/editor-core/tests/m10_5_r2_probes_interval.rs`
(`an_interpenetration_is_reported_violated_not_refused`).

## Why it is like that

The margin at the site is `d − 0` where `d` is an interval NORM
(`crates/editor-core/src/clearance.rs:112` and the site's row in
`docs/predicate-dimension-audit.md`), so its enclosure is never
negative. `Sign::Negative` — the arm every metric bound reaches a
violation through — cannot occur at this site, by construction. The
only definite finding available is `Sign::Zero`, which is exactly "these
two surfaces touch".

That is the sound direction: an unsigned norm cannot lie about a sign it
does not carry. It is also a real gap between what E7 §3 asks for and
what a consumer gets.

## What a consumer cannot do with today's answer

- Rank two self-intersections by severity (a 1 µm nick and a 0.5 m
  overlap report the same `distance ≈ 0`).
- Tell a grazing tangency from a gross overlap without re-deriving the
  inside/outside test itself.
- Drive a repair: there is no direction and no depth to back out along.

## What a fix would need

A SIGNED quantity, which means a containment test the norm does not
carry — for a pair of trimmed faces, the sign of the separation relative
to each body's own inside. The candidate shapes:

1. Evaluate each witness point against the OTHER body's inside/outside
   predicate and carry a `sign` beside `distance`. Cheapest, and it is
   the shape the exhibit arm already has the points for
   (`crates/editor-core/src/clearance.rs:2027`), but it needs a
   point-in-solid predicate at the certified lane, which M10 does not
   have yet.
2. Carry the signed distance from the crossing curve, which is the
   quantity a repair actually wants, and is a genuinely larger piece of
   work (it is a surface-surface intersection, not a proximity query).

Neither fits M10-5, which is why this is filed rather than done.

## Home

`work/m10/` — the code is `crates/editor-core/src/clearance.rs`, an M10
deliverable, and the gap was raised by M10-5's review (PR 1638).
