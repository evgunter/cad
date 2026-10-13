---
id: the-localization-core-is-half-unified
kind: issue
title: The bisection localizations share a walk but not their spellings: the tangency floor, a piece's reach, the budget and the stop reason each live in more than one place
status: open
opened: 2026-10-13
priority: P3
cost: M
---

Found by the round-2 review of PR 4946 (`census-curved-record-lanes-read-circles-only`, branch `fuse/census-reads-every-curved-edge`).

PR 4946 put `boolean::carrier_touch` and `census::curved` on one walk,
`crate::bisect`, and stopped there. One class, five cites:

- **The tangency floor is spelled twice, with `κ` meaning opposite
  things.** `carrier_touch::clusters` reads `√(κ·escalate)/8` with `κ`
  a reach (a length); `census::curved::floor` reads `√(escalate/κ)/8`
  with `κ` a curvature, re-deriving `Conic::curvature_hi` as
  `accel_hi / lo²`.
- **A piece's reach is spelled bare in `carrier_touch`** (`speed ·
  |b − a| · ½`) and through `Margin::metered_sup` in `census::curved`
  (`Piece::reach`).
- **Two more walks keep their own stacks and 4096 budgets:**
  `boolean::circle_roots` (`SUBDIVISION_BUDGET`) and
  `splitting::spiric_arc` (`MAX_PIECES`).
- **`carrier_speed::spiric_rates` is the generic twin of
  `geom::spiric_curvature_sup`**, and the dependency runs the wrong way:
  the `f64` sizing bound should read the generic one.
- **`bisect::Step::Stop` loses its reason.** `carrier_touch` (`over`) and
  `census::curved` (`met`) carry side flags to say why a walk stopped.
