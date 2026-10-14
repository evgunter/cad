---
id: certified-line-carrier-is-read-in-five-places
kind: issue
title: "is this edge a certified Line" is spelled separately in five splitting/join files; one helper
status: open
opened: 2026-10-06
priority: P4
cost: E
---



## What

"Is this edge's certified curve a straight `Line`" is answered inline in
each splitting file that needs it, each with its own reach into
`CurveGeom::Certified(..).carrier()`:

- `splitting/rules.rs` `straight` (the knife edge's contact site, PR 4098);
- `splitting/join.rs` `certify_section_area`'s per-half-edge `straight`
  read (the spur invariant);
- `splitting/containment.rs` (the `Line` arms that read a loop edge as a
  chord);
- `splitting/section.rs` (`HalfCarrier::Chord`);
- `splitting/classify.rs` (the crossing lane's `Line` arm).

One helper on the certified curve (or on `Body` by edge key) would answer
it once, so a carrier that later reads as straight (a degenerate conic, a
fitted line) is admitted or refused in one place.

## Found by

CLEAVE DR-4098's delta review (Style 5).
