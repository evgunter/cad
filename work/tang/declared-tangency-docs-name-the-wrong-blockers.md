---
id: declared-tangency-docs-name-the-wrong-blockers
kind: issue
title: CoaxialEvidence, cs_pair_frame and RimRouting::Cusp's text cite a #1372 channel that will not exist and name the wrong missing arm
status: open
opened: 2026-10-01
priority: P4
cost: E
---


## What

Three stale texts, found by the DEV-1 circle-arm design pair
(2026-10-01). Each is one edit:

- `CoaxialEvidence`'s doc (`crates/geom-brep/src/intersect.rs`) and
  `cs_pair_frame`'s paragraph (`crates/topo/src/boolean/join.rs`) say
  the honest carrier is "the parameter-identity channel (#1372), which
  does not exist". `docs/AXIS-DECLARATION-DESIGN.md` ratified that the
  channel is axis-shaped, and it is not #1372
  (`work/wire/axis-shaped-identity-channel.md`).
- `RimRouting::Cusp`'s refusal text (`boolean/rim_wedge.rs` module
  docs, `refusal_routes.rs`) gives "the witness lane has no torus arm"
  as the reason the family is unbuilt. For a cylinder×sphere rim it has
  no circle arm either, so the message names the wrong gap.
- `tangent_locus`'s doc and the reduce `(Zero, Positive)` rung's
  comment name "the LINE-only type, consumers read a direction, the
  declared-coaxiality channel" as what blocks a circle arm. The
  separation invariant they also cite is measured satisfied
  (`crates/topo/tests/verbs_cylsph_tangent_residuals.rs`). Rewrite it
  once the DEV-1 fork is ruled, since the ruling decides what blocks.
