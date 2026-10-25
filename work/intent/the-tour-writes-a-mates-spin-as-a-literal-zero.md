---
id: the-tour-writes-a-mates-spin-as-a-literal-zero
kind: issue
title: the tour writes each mate's spin as a literal 0 rad, which mints a variable where the design writes the constant 0
status: closed
opened: 2026-10-21
priority: P2
cost: E
branch: intent/tour-spin-is-the-constant-zero
closed: 2026-10-21
---


## What

`demos/tour/src/assembly.rs` `seated_at` (line 352) writes a plane
mate's spin as `Formula::literal(0.0, Dimension::Angle)`. Under
`[variables.literals]` a literal is a variable, so each of the bench's
three mates mints an angle variable that nobody wrote. The design's
constant zero is `Formula::zero`, which `Placed::identity` already
writes. DESIGN-FORK-LOG rows 159 and 161 found the same defect
elsewhere: literals written where the design says zeros.

Found by the seat designer pair (2026-10-21,
`a-mate-placed-seat-is-unproven-at-the-doors-budget`). At `Sym` the
literal binds as a symbol, so the bench's seats need `R(a, θ)·a = a`
through a symbolic θ. That is part of why they don't prove at the
door's budget today.

## Owed

`seated_at` writes `Formula::zero(Dimension::Angle)` for the spin. Check
`place_at`'s returned values, and every other demo or helper writing
`literal(0.0, …)` where the constant is meant, for the same defect.
Re-bless what moves, with the reason.

## Closed

2026-10-21, branch `intent/tour-spin-is-the-constant-zero`.
`seated_at` writes `Formula::zero(Dimension::Angle)` for the spin. The
sweep fixed three more zeros that mean "nothing written here":

- `prism_part`'s origin corner, which was `0 mm` and is now the
  constant, so the named extents measure the prism;
- the plate's `0 m - half_spacing`, which is now `neg(half_spacing)`;
- the teapot spout's split phase;
- circle centres the author leaves at the construction's own origin or
  line (orchestrator ruling, after row 161's `axis_through`): `plate`'s
  hole centre `y`, `chain`'s pin centre `y` and its base pin's `x`,
  `ring`'s centre `y`, and the teapot spout's centre. This re-blesses
  `crates/viewer/tests/gallery_ring.pncad`, and the ring ops stream in
  `creation_ops.rs` now spells the centre's `y` as the constant.

It also fixed one helper, the viewer's `test_support` fallback pin
(`unread`). `place_at`, `pin_at`, `Placed::identity`, the tour's
`world.rs`, pncad-py's slot and spin doors, `demos/wild` and the guide
write no such literal.

Left as authored values: drawn outline coordinates (polygon and
meridian vertices), the library's literal doors, and the stand's
computed seat slide.
The PR body has the full hit list. No render input moved: the tour's
STL, STEP, uv and scene manifests are byte-identical. Only content
hashes moved. The bench's `run_checks` still reports 3 findings.
Test-fixture hits are filed as
`test-fixtures-write-a-placements-zero-values-as-literals`.
