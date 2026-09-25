---
id: lane-0-offset-fit-hook
kind: unit
title: LANE-0: the f64-only offset-fit absence becomes an Option hook, out of the lane traits
status: closed
opened: 2026-09-21
closed: 2026-09-21
branch: scalar/lane-0
pr: 2981
---

## What

The first unit of `H5`'s ruling 3 (PR 2701, `## RATIFIED`): the three
lane methods that block everything but `f64` — `PropsQuadLane::
recertify_approx`, `PropsQuadLane::approx_offset_surface`,
`PcurveFittedLane::remap_certificate` — leave the traits and become one
hook type, `geom_brep::OffsetFitLane`, passed as `Option<_>` to the
three passes that call them (`tier3_local_checks_marked`,
`mint_offset`, `map_approx`). The "not derivable at this scalar"
absence stops sharing a `None` with "may not certify".

**Where the `Some` comes from** (SCALAR orchestrator's ruling,
2026-09-21, on the unit's two reviews): one per-scalar seam and no
other — `topo::props::AtRestPolicy::offset_fit_lane`, DL3's per-scalar
policy home, which `H5` §RATIFIED ruling 3 keeps standing as the seam
the cut leaves (`ShellLane` folds into it). `f64` answers
`Some(OffsetFitLane::fit())` and the other four scalars answer `None`,
each arm stating that the fit is derived at `f64` only. Five sites
read it: check 1's battery and the two validation walks that serve it
(`validate.rs`), the offset mint (`replace_face.rs`) and the
transform's surface map (`transform.rs`). No lane trait of its own:
`PcurveFittedLane` carries no supertrait edge to it, and the generic
doors between a public caller and a read site name `AtRestPolicy` in
their bounds.

Spec: `docs/LANE-0-SPEC.md` (deleted at merge).
Block SCALAR-B4 slot 1. Ground: TOPO, SHELL, TRIM, the unowned
`topo/src/props.rs`, WIRE's `eval/wire.rs` and `crates/verbs` for the
bound; announced.

## Closed (2026-09-21) — PR 2981

`geom_brep::OffsetFitLane<T>` (`Copy`, three fn-pointer fields,
`recertify`/`mint`/`remap`, one constructor `OffsetFitLane::<f64>::fit()`
wiring `recertify_approx`, `approx_offset_surface` and the moved
`remap_offset_certificate`); the three passes
(`tier3_local_checks_marked`, `mint_offset`, `map_approx`) take it as
`Option<OffsetFitLane<T>>`, `None` keeping the three
`ApproxLaneUnsupported` variants, payloads and `Display` exactly; the
three methods and their fifteen impls gone from `PropsQuadLane` and
`PcurveFittedLane`. **The seam, by the orchestrator's ruling** (not a
change to ratified text; `H5` ruling 3 keeps `AtRestPolicy` as the
per-scalar seam the cut leaves standing): the `Some` is read at
`topo::AtRestPolicy::offset_fit_lane()` — `f64` → `Some(fit())`, the
four others `None` with their reason — at the five read sites in
`validate.rs`, `replace_face.rs`, `transform.rs`; the reviewed head's
`OffsetFitScalar` (a fourth per-scalar trait bundled as a supertrait
of `PcurveFittedLane`, both arms' MAJOR) is deleted; threading the door
through the 18 public doors (~1,200 call sites) was measured and
declined because every generic caller up to `EvalScalar` would read
the `f64`-only door off a per-scalar seam anyway. Cost of the ruling:
47 bound edits in 7 files (27 public `topo` doors, `boolean/ops.rs`,
`editor-core`'s `eval/wire.rs`, `verbs/run.rs`) plus 29 generic test
helpers, zero ordinary call-site edits; no scalar loses a door
(`AtRestPolicy`'s roster is `PropsQuadLane`'s). Every `f64` result and
certificate bit-identical, every refusal at the other scalars
message-identical (both arms, base vs head, through the public doors
at `f64`/`Dual64`/`Sym`/`Probe`/`Interval`). Pins: the wiring by
`fn_addr_eq` (three rows in `offset_fit_lane.rs`), the mapped pair
against the free `Tol` door, the census narrowed to the one routine
`remap` may reach, `offset_fit_lane.rs` the sixth `CHAIN` stretch with
its two `tolerance: f64` parameters declared, the fixture's bow the
largest that certifies at every eps row (`1.5e-2`, refining at
`1e-12`); two probe files kept as asserting rows. Reviews: dual, both
REQUEST CHANGES, both MAJORs bilateral; eighteen dispositions, two
declines with reason (the bowed-patch fold has no reachable home —
filed on TINT; bundling the injected doors is LANE-4's call).
`geom-brep/README.md`'s O5 row re-worded (naming only) and DL3's
method list with it. Rows: WIRE
`chart-coherence-lane-absence-unstated` (re-homed from FIX, which left
the tracker mid-review), TINT
`bowed-patch-fixture-has-three-homes-and-no-reachable-one`.
