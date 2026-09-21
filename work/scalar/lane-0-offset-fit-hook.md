---
id: lane-0-offset-fit-hook
kind: unit
title: LANE-0: the f64-only offset-fit absence becomes an Option hook, out of the lane traits
status: open
opened: 2026-09-21
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
