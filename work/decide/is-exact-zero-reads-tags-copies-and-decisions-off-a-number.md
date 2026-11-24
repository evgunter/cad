---
id: is-exact-zero-reads-tags-copies-and-decisions-off-a-number
kind: issue
title: geom_core::is_exact_zero's callers read three things off a number: a tag the kernel wrote, a copy against its original, and a branch that changes what is built
status: open
opened: 2026-11-15
priority: P2
cost: H
---


Filed by BOXFINISH (2026-11-15) from design fork 272 (`[ev]` PR 6993),
where three designers audited `geom_core::is_exact_zero`'s callers
after Ev's concern that an exact zero at a numeric scalar sits
uncomfortably with the band. Sites as read on main at `19afaa14`; the
three audits counted 17, 21 and 22 non-test sites and agree on the
classes. None asks whether a computed quantity vanishes by identity:
that is the structural question (`docs/DESIGN.md` `[scalars]`), which
only a symbolic scalar answers.

**1. A zero the kernel wrote to mean something** (`[unrepresentable]`:
"encodings are tagged, never in-band magic values").
- An iso line's slope component being 0 means "this image is a row or
  a column": `topo` `replace_face::iso_image_row`, `geom-brep`
  `description::row_of`, `pcurve_cache/patch.rs`. The held-line
  representation ratified on `[ev]` PR 6975
  (`work/boxfinish/an-iso-line-states-its-ends-twice-as-its-map-and-as-stored-ends.md`)
  makes this a variant.
- A ray's start being 0 means "starts at its line's origin": `geom`
  `curves.rs`, three sites.
- A torus's major radius being 0 means "a sphere": `geom` `cover.rs`
  `Tube::new` (not checked: whether that zero is always kernel-written).
- A table's bulge being 0 means "a line": `profile` `lower_to` and
  `bulge_leg` (the bulge may also be a user's computed value, which
  puts it in class 3).

**2. Two copies of one stored number compared bit for bit.**
- A row's fixed value against the chart's domain end:
  `description::domain_side`, `topo` `boolean/spline_known.rs`
  `corners` (the answer names a side or a corner).
- Held weights that are "one value": `pcurve_cache.rs`
  `arc_rim_normal_weight`, `one_value`, and the arc-rim end-weights
  check, which refuses on a mismatch.
- Control points sharing a coordinate: `pcurve_cache/patch.rs`.
These go away where the value is stored once, or become a named "same
stored bits" read; not checked: that each copy is verbatim upstream.

**3. A branch that changes what is built, decided by bit-equality**
(`[verdicts]`: bit-equality is never the test).
- The bulge's line-or-arc choice (`profile::bulge_leg`) and the
  zero-chord `+x` stand-in (`profile` `path.rs` `chord_director`).
- The chart corner or side a value sits exactly at, where the value
  can be computed (`spline_known::corners`, `description::chart_side`).
- A shortcut that skips a decision where both terms are 0
  (`pcurve_cache.rs` `cylinder_rim_axial`).
Each becomes a margined decision, or reads the structure from a type.

**Closed when** no caller reads "exactly zero" off a number: class 1 is
variants, class 2 single storage or a named bit read, class 3
decisions, and `is_exact_zero` is gone or private to the scalar impls.
This is several units across `geom`, `geom-brep`, `topo` and `profile`;
split it by owner when it is picked up.

**A second door of the same kind, new on main (2026-11-22):**
`Real::is_exact_zero_in_every_channel` (`geom-core` `real.rs`; `Dual`
reads its derivative too; `Sym` forwards to its value), behind
`UnitVec3::is_exactly`, whose callers are the least turn's identity
(`rigid_motion.rs`), `mate/value.rs` and the sweep skin's still step
(`sweep/skin.rs`, `!t.is_exactly(next)`, which gates a step and so
looks like a branch that changes what is built). Three designers
confirmed it by reading; none traced the callers. It is sorted with
the rest: a spelling choice asks the expression (DESIGN `[scalars]`),
a branch that changes what is built is a decision, and for each use
say first what breaks without it. Also for this audit: a bulge leg
that stores a sign nothing decided (`arc_bulge`, `literal_sign(b)`,
and what its tie arm stores), found by BOXFINISH's signed-radius lane.
(BOXFINISH orchestrator)

**The second door, closed (DOCTAIL lane, 2026-11-22)**, by `doctail/the-least-turn-and-aligned-read-exactness-off-the-nominal-at-sym`:
- `Real::is_exact_zero_in_every_channel` is removed.
- The least turn's identity shortcut is gone: the Rodrigues arm is `v + k × v + k × (k × v)/(1 + c)`, so a pair that is one direction turns by the identity through `k = 0`.
- `Chart::charted` reads no identity.
- `UnitVec3::is_exactly` is `f64`-only; its one caller is the sweep skin's still step, at `f64`. That step still gates a step on bit-equality, which this row's class 3 keeps.
- `mate::turn::zero` reads `SolveScalar::zero_everywhere`. At `Sym<f64>` that answers false. At `Sym<Interval>` it answers the box's point zero, `[0, 0]`, which holds over the box the lane claims over. `SolveScalar::exactly_zero` at `Sym` forwards to its value the same way: false at `Sym<f64>`, the box's point zero at `Sym<Interval>`.

The `is_exact_zero` callers listed above (classes 1–3) are unchanged.

**Met again by BOXFINISH's expression-question sweep (2026-11-22),
none moved by it:** the three `profile` callers, as listed above
(`lower_to`, `bulge_leg`, `path.rs` `chord_director`).
`chord_director`'s zero-chord `+x` stand-in is now also the quotient
the hop's mint is handed (`path::hop_director`), so it is still what a
zero chord stores at every scalar. One read of the same family that is
not `is_exact_zero`: `ValidatedProfile::value_channel` picks the `f64`
witness's spelling of a recorded direction by whether its components
are single points (`point_value`), a structural read off a number that
learns no identity and stores the same real either way.

**What PR 7393 (BOXFINISH's iso-image unit) changed in this list, and one home it adds (2026-11-24):**
- Class 1's iso-line sites are closed: `replace_face::iso_image_row`, `description::row_of` and `pcurve_cache/patch.rs` read the image's `ChartLine`, a variant, and `topo::clearance::iso_of` (a fourth, found by that unit's sweep) does too.
- Class 2's "a row's fixed value against the chart's domain end" has one home now: `geom_core::period::domain_end(x, (lo, hi))` (`crates/geom-core/src/period.rs`), which `description::domain_side` and `boolean/spline_known.rs` `corners` both call. It is still `is_exact_zero` of the difference: at `Dual` and `Sym` it reads the value channel alone, pinned at each scalar by `period::domain_rows::a_value_is_at_an_end_only_as_the_same_stored_number` (two symbols with one value read as one; an identity by expression whose numeric channel has width does not). **This is a site that asks the expression once `Real::Expression` / `Expression::proves_zero` (PR 7488) is on main**: whether a held value IS the domain's end is a question about the two expressions, not their values at this evaluation.
- `Period::moved` reads no exactness: one period up is `hi + (x − lo)` and one down `lo + (x − hi)`, arithmetic at every scalar (the `Dual` derivative is the move's own, the `Sym` result an expression of `x`), so it is not on this list. A first cut of that unit did read `is_exact_zero` there; the review found it dropped a derivative at `Dual`.
