---
id: the-localization-core-is-half-unified
kind: issue
title: The bisection localizations share a walk but not their spellings: the tangency floor, a piece's reach, the budget and the stop reason each live in more than one place
status: open
refs: [4980]
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

## 2026-10-13: PR 4980 did the five cites; what remains

PR 4980 did the following:
- `circle_roots` and `spiric_arc` (three walks) moved onto `topo::bisect`, whose `Step::Stop(R)` carries a typed reason at each site.
- `bisect::reach` and `bisect::tangency_floor(ρ, …)` are the one spelling of each, in `carrier_touch`, `spiric_arc` and `census::curved`.
- `carrier_speed::curvature_hi` reads `Conic::curvature_hi`, and `carrier_touch` reads it too.
- `geom::spiric_period_rate_bounds` is the generic whole-period spiric bound.

Its review found more copies of the same class. They are this row's scope now:

- **A second whole-period spiric bound and a reach copy.** `splitting/containment.rs` `spiric_speed`, called from `shell.rs`, is one. It should read `geom::spiric_period_rate_bounds` and `bisect::reach`.
- **A second "one spelling" of the per-kind lever.** `offset_axial.rs` `param_speeds` claims that title, beside `carrier_speed::speed_range`.
- **Test copies of the whole-period bound in `sweep`.** These are `pis_arc_capped_poses` and `contfp_reads_arcs_on_their_carriers`.
- **Three depth idioms beside a core that owns only the budget.** These are `spiric_arc`'s `MAX_DEPTH`, `census::curved`'s `Piece::depth`, and `circle_roots`' width row (`rows.width`).
- **A second spelling of `census::curved::stops`.** `census::curved::feet` stops a piece on `surely(reach, Zero)`.
- **An unpinned polish guard.** The `16·slack` guard (`census::curved::step_kept`'s bound) has no row: keeping every step leaves the suite green.

The walk-shaped sweep that found the first five cites could not see duplicated arithmetic, which is how these escaped. Sweep the constants and the formulas (`spiric_rate_bounds(`, `f_min`, `* h`, `speed *`), not only the stacks.
