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
hook type passed as `Option<_>` to the three passes that call them
(`tier3_local_checks_marked`, `mint_offset`, `map_approx`), `Some`
only from the `f64` seam arms and `f64`-concrete callers. The
"not derivable at this scalar" absence stops sharing a `None` with
"may not certify". Spec: `docs/LANE-0-SPEC.md` (deleted at merge).
Block SCALAR-B4 slot 1. Ground: TOPO, SHELL, TRIM, the unowned
`topo/src/props.rs`; announced.
