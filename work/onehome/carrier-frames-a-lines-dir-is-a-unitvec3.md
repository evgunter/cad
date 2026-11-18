---
id: carrier-frames-a-lines-dir-is-a-unitvec3
kind: unit
title: Carrier frames (v): a line's dir is a UnitVec3, with its chord and Ray audit
status: parked
opened: 2026-11-17
priority: P2
cost: H
parent: carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes
blocked_on: [carrier-frames-the-types-and-the-lift]
design: true
---


Split out of `carrier-frames-the-types-and-the-lift` (orchestrator,
2026-11-17). Check 1 never margins a line, so the frame half (iii) does
not depend on this row.

## The design point

`geom::Line` serves as two things:

- **The carrier**, `Curve3::Line`, whose parameter is arc length by
  convention.
- **An affine chord**, whose parameter is scaled by `|dir|`.
  `Line::param` and `coefficient` divide by `|dir|²`, and `Line::unit`
  normalizes on demand. Its users include:
  - `geom::Ray`: "an advance is in units of `|dir|`";
  - `topo`'s `ring_path`, a production chord: `Curve3::line(from,
    to - from)` over `(0, 1)` in the piece-to-carrier conversion;
  - the test chords and "any dir length" rows of `geom`, `geom-brep`
    and `topo`.

Squaring `dir` where a line is stated reparameterizes every chord,
production chords included. The audit decides whether chords get a
type of their own, so that the carrier alone holds a `UnitVec3`, or
whether every chord span is rescaled by its length.

A prototype of the reparameterization is on branch
`onehome/carrier-frames` before its revert commit. It moved about ten
rows in `geom` and `geom-brep` alone, the chord fixtures among them.
