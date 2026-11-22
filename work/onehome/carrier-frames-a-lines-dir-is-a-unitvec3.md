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

## Done

`geom::Line.dir` is a `UnitVec3<T>`, and its parameter is arc length.
The audit chose to rescale chords: no chord type. A chord from `a` to
`b` is `Line::chord(a, b)`, the line along `b − a` squared and the
chord's length, so the segment is the span `(0, |b − a|)`. `Ray`'s
advance is metres.

**The doors** (`geom-core` `linalg/unit_vec.rs`, `geom` `curves.rs`):
- `UnitVec3::stated(Vec3<f64>)` adopts a stated `f64` direction's bits
  where `|v·v − 1| ≤ rounding_charge(v·v + 1)`, the frame door's column
  test (16ε about a unit vector). It answers `None` past that.
- `geom::stated_direction`, `Line::stated` and `Curve3::line_stated` are
  the fail-loud form: a debug assertion, and poison in a release build.
- `UnitVec3::constructed(v)`, at any scalar, adopts the bits of a
  direction unit by the caller's construction: a rigid image of a unit
  vector, `a·cos α + d·sin α` over an orthonormal pair, a chord over its
  own length. Its debug check reads the value's defect against the same
  charge.
- The lift is `UnitVec3::map_scalar`: the polar unit enclosure at
  `Interval`, carried within a witness kind, and poison into another
  kind's lane.

**What moved.**
- The `f64` corpus dump is unchanged against main.
- Every editor-core digest holds.
- One width re-pin, `flat_cone_harmonic_images`, from the polar
  enclosure's `η`.
- The direction-scale rows are retired or retargeted: a scaled
  direction is no line's.
- Certification's metred guard for a zero `dir` is removed: a zero
  vector has no direction to mint.
- The PR body lists each change with its cause and mutant.
