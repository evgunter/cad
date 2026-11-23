---
id: carrier-frames-a-lines-dir-is-a-unitvec3
kind: unit
title: Carrier frames (v): a line's dir is a UnitVec3, with its chord and Ray audit
status: closed
opened: 2026-11-17
priority: P2
cost: H
parent: carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes
pr: 7349
branch: onehome/carrier-directions
closed: 2026-11-23
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

**The doors** (`geom-core` `linalg/unit_vec.rs` and `rigid_motion.rs`,
`geom` `curves.rs`):
- `UnitVec3::stated(Vec3<f64>)` adopts a stated `f64` direction's bits
  where `|v·v − 1| ≤ rounding_charge(v·v + 1)`, the frame door's column
  test (16ε about a unit vector). It answers `None` past that, and the
  import refuses typed.
- `geom::stated_direction` is the fail-loud form, test builds only: a
  debug assertion, and poison in a release build.
- The construction doors name the construction that makes a direction
  unit: `UnitVec3::through_isometry` (a rigid motion's or a reflection's
  image), `RigidMotion::swing` (`w·cos α + u·sin α` over one frame) and
  `UnitVec3::half_turned`. Each measures its value at a point scalar: the
  bits are kept within the charge, squared past it (a chain of maps does
  not drift off unit), and poisoned past `√ε` or where not finite. At an
  enclosure kind the bits are kept unless the bracket of `v·v − 1`
  excludes zero, which is poison. The check runs in release builds.
- `UnitVec3::checked` is that measurement for a vector with no door of
  its own: a frame squared into bare vectors (`SquareFrame`, the
  section certifier's `Swing`), a fixture's literal.
- `UnitVec3::chord(v)` is `v / |v|` beside `|v|`, poison at a zero chord,
  and `Line::chord` reads it. `reversed` and `turned_toward` are exact.
- A transformed line whose direction the door squared has its edge's
  span stretched by the length divided by (`map_carrier_stretched`,
  `EdgeCurve::carried`), so its ends stay on its vertices.
- The lift is `UnitVec3::map_scalar`: the polar unit enclosure at
  `Interval`, carried within a witness kind, and poison into another
  kind's lane. Test fixtures lift through it, not entry by entry.

**What moved.**
- The `f64` corpus dump is unchanged against main.
- Every editor-core digest holds.
- One width re-pin, `flat_cone_harmonic_images`, from the polar
  enclosure's `η`.
- The direction-scale rows are retired or retargeted: a scaled
  direction is no line's.
- Certification's metred guard for a zero `dir` is removed: a zero
  vector has no direction to mint.
- The chord-era vestiges are gone: `Line::unit`, the census's
  re-normalized line direction and the line × cone meter's speed.
  `coefficient`'s `/|dir|²` stays, for bits a far-carrier meet reads.
- The PR body lists each change with its cause and mutant.
