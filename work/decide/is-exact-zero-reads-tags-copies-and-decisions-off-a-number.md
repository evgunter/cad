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
(BOXFINISH orchestrator)
