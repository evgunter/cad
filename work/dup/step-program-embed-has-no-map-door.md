---
id: step-program-embed-has-no-map-door
kind: issue
title: The recorded Step program is walked through a scalar map by hand seven times in six test files; profile has no Step::map_scalar
status: open
opened: 2026-09-24
priority: P4
cost: D
---


## Finding

Filed by the `dup/scalar-lift-home` lane. Folding the `Point2` lifts
onto `Point2::map` put this lane inside **seven hand-written copies of
one walk**: a recorded `profile` program `Step<f64>` re-expressed
through a scalar map (six to a lane scalar, one a rescale at `f64`),
arm by arm — `Target::Point(p) => Target::Point(pt(p))`,
`ArcData::Center { c, winding, target } => …`, the radius through
`T::from_f64` — with `pt` the only piece that had a door.

Measured at the lane's fold head, two instruments over every tracked
file with no path argument:

- the arm every copy must contain,
  `git grep -n 'Target::StartArriving => Target::StartArriving'`:
  **7 hits in 6 files** — `crates/editor-core/tests/cert3r1_dump.rs`,
  `crates/editor-core/tests/m10_p_fence.rs`,
  `crates/profile/tests/cert4r1_e2e.rs` (`embed_step`),
  `crates/profile/tests/generic_replay.rs` ×2 (`embed_step`, and
  `scaled` further down the file — the same walk with `f` a SCALE,
  `f64` to `f64`, which the same door would serve as
  `map_scalar(|c| c * s)`),
  `crates/profile/tests/guided_replay.rs` (`embed`),
  `crates/profile/tests/review_fillet_stored_tangency_r1_probes.rs`
  (`embed`);
- signatures, `(Step|Target|ArcData)<f64>` `->` a `Step`/`Target`/
  `ArcData` at another scalar: the same six files, nothing else.

The copies are not identical: `cert4r1_e2e` and `generic_replay`'s
`embed_step` cover every `Step` variant, and the other five cover only
the variants their fixture records and `panic!`/`unreachable!` on the
rest. So the copies have already DRIFTED in coverage. A variant added
to `Step` fails to compile in the two exhaustive copies; the other five
compile and reach their catch-all arm only if a fixture records the new
variant.

**Band: P4.** All seven copies are tests. No `src` code builds on them,
and nothing in `src` walks a `Step` through a scalar map that a
`Step::map_scalar` door would serve. Two copies already catch a new
variant. That makes this non-architectural code improvement under
`work/README.md`'s bands, not P1.

**Blind spot.** A copy that never names `Target::StartArriving` (one
that embeds only `Step`s with no target) is invisible to the first
instrument; the second catches it only when the copy is a named
function or a typed closure.

## The shape of a home

`Step`, `Target` and `ArcData` are declared in
`crates/profile/src/path/program.rs` (one macro), which has no `map`.
By `geom`'s `scalar_lift` convention (`map` on a leaf, `map_scalar` on a
type with structure to carry) the door would be `Step::map_scalar`,
exhaustive over the variants, with `Point2::map` at the leaves. That
file is `paths`' ground; the row sits here because the copies are this
program's class and were measured by this program's unit. `paths` may
claim it by `git mv`.
