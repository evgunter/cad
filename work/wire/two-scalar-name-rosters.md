---
id: two-scalar-name-rosters
kind: issue
title: Two per-scalar name rosters: topo::AtRestPolicy::scalar_name and editor_core::lane::Lane::NAME spell the same five scalars differently
status: open
opened: 2026-09-24
---


Filed by LANE-4 (SCALAR), which moved the kernel's name roster onto the
per-scalar policy and left the editor's alone, as its spec required.

**The two rosters.**

| scalar | `topo::AtRestPolicy::scalar_name` (`crates/topo/src/props.rs`) | `editor_core::lane::Lane::NAME` (`crates/editor-core/src/lane.rs`) |
|---|---|---|
| `f64` | `"f64"` | `"f64"` |
| `Probe` | `"telemetry probe"` | `"Probe"` |
| `Interval` | `"interval"` | `"Interval"` |
| `Sym<T>` | `"symbolic"` | `"Sym"` |
| `Dual<T>` | `"dual"` | `"Dual"` |

The kernel roster is read into two refusals:
`geom_brep::PcurveCertifyError::FittedLaneUnsupported { scalar }`
(through `topo::pcurves`' `derive_general_image`, `mint_face` and
`validate_pcurves`) and `topo::TransformError::ApproxLaneUnsupported
{ lane }` (through `topo::transform`'s `map_surface`). The editor
roster is read into the editor's refusals that name a lane. Both are
per-scalar facts with one arm per scalar, and nothing checks that they
agree. A user who meets both refusals in one evaluation sees the same
scalar named two ways.

**Why LANE-4 did not unify them.** `topo` cannot name
`editor_core::lane::Lane`, which sits above it. Reusing either
spelling would change refusal text that rows pin:
`topo/tests/lane0_r2_probes.rs`, `sweep/tests/r1_lane0_e2e.rs`,
`topo::transform`'s `offset_fit_door_rows`, and
`topo/tests/m6_2_fitted_at_rest.rs` pin the kernel spellings. The LANE-4
spec (`docs/LANE-4-SPEC.md` §3) put unifying them out of the unit.

**The shape of a fix, for whoever takes it.** `editor-core` sits above
`topo`, so the editor could read its name off the policy (a `const`
cannot call a trait method, so `Lane::NAME` would become a method or
a row pinning the two lists equal). The other direction keeps two
lists and pins their correspondence in a row. Either way the question
is which spelling a user should read. That is a message decision, so
it belongs with WIRE.
