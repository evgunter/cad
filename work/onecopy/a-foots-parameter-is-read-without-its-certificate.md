---
id: a-foots-parameter-is-read-without-its-certificate
kind: issue
title: A foot's t is read without its on_carrier certificate (census direction_at, curve_pair walk and pose), so a polish tie across a spline's branches silently reverses a tangent unless the tie stays logged
status: open
opened: 2026-11-03
priority: P1
cost: M
refs: [6475]
---


Found by the FULL review of PR 6475 (`onecopy/small-reads`), which falsified ledger row F32.

`Curve3::foot` returns a `Foot` whose `t()` can be read bare, and some readers rest on it without reading `Foot::on_carrier` / `clear_of`:
- `topo::census` `direction_at` (~8958: `.foot(p, mid).t()` then `deriv(t)`);
- `geom_brep::curve_pair::walk` (~243, ~382–383);
- `curve_pair::pose` (~495, used at ~509, ~517, ~580).

The guarded readers read `on_carrier` first: `curve_pair/span.rs`, `pair_section.rs`, and `solid_contain/curved_edges.rs`.

Reviewer's probe: on a cubic hairpin, an `Interval` box of ±0.05 ties every polish candidate. `smaller` keeps the seed's branch, and `direction_at` returns the reversed tangent. The only cue is `curve_foot_polish`'s escalation, so PR 6475 keeps that tie logged. But the same site logs noise on a converged polish (3–4 escalations on a ±1e-12 point box), so the log cannot tell a tie that refining answers from noise.

Question for the designers: should a bare `t` stop being the easy path (for example, `Foot::t()` available only through a certified reading, or `direction_at` and the pose reading a certificate), so that the polish tie can then be a renaming? Or should the polish tell a cross-branch tie apart from a converged one and log only the former?

## Decided (2026-11-03, two designers, agreed on first reports)

The design-fork procedure weighed this; the draw is on `analysis/design-fork/onecopy-foot-certificate`. Both designers recommend the same final state and call it an elaboration of DESIGN's crate table (`Curve3::foot` and `Curve3::on_carrier` as `geom`'s two readings) and of geom-brep's C5 sentence ("no parameter a caller holds can produce a false off", Ev, PR 5154). No ratified text changes, so it lands without Ev.

**Premise.** `Foot::t()` gives one name to three facts:
- `(t, gap)` is a bound: `|q − C(t)| = gap`.
- `t` is the point's parameter, which only `on_carrier`'s `On` (a zero gap in the band) or the carrier's kind (nearest on a line or circle) certifies.
- `t` is a seed for a further search, which certifies nothing.

Making the polish tell a cross-branch tie from a converged one is not an alternative to fixing this. It would put a second predicate beside the certificate, double-log one fact, and leave the bare read writable. It also misses the f64 case where sampling lands on the wrong branch, which no tie log can catch.

**Final state.**
- **`geom`.**
  - `OnCarrier::On { t, margin }` carries the parameter.
  - `Foot::t()` is removed.
  - `Foot::nearest() -> Option<T>` is `Some` only where the foot is nearest by kind (line, circle). It folds in `nearest_foot`/`is_nearest`.
  - `Foot::seed() -> T` certifies nothing by contract.
  - `Foot::gap()` and `point()` stay.
  - Once no reader can rest on the pick, `smaller` (`curve_foot_polish`) goes through `check_unlogged` (ledger row F32). Its argument is now structural: the only parameter a reader can hold is `On`'s, decided on the gap. This removes the measured noise: at `Interval` every spline foot logs ≥4 escalations and every ellipse or spiric foot ≥1, whatever the box, which is a standing bisect cue `drive::log_read` can never satisfy.
- **`geom-brep`.**
  - `Span::read` forwards `On`'s `t` into `Reading::On`.
  - `pair_section::reads` is `On { t, .. } => Some(t)`.
  - The walk's `polished_feet` and pose's `foot`/`line_foot` read `seed()`.
  - `pose::window`'s `along_b` reads `seed()` beside `gap()`.
  - `chart_singularity::foot` is `nearest()` plus its branch rule.
  - `Place::Inside` carries no `t`; its only outside caller discards it.
- **`topo`.**
  - `census::direction_at` is deleted. `tangent_sectors` reads `census::curved::tangent_on_interior`, which is already certified and already refuses typed. `direction_at` was a copy of it, made beside its certified home.
  - `chord_join`'s foot closure and `reduce`'s circle split read `On { t }`.
  - `curved_edges::passes` reads `seed()`.
  - Sweep `reach::foot_on` reads `nearest()`.
- **Pins.**
  - Invert PR 6475's `a_tie_across_a_hairpins_branches_reaches_the_log`: the tie must NOT reach the log, and the certificate's or the span's reading must (escalate or refuse).
  - Add an f64 row with the seed on the far branch, if one can be built.

**Off-question findings, to file when the unit runs:**
- A spline's `on_carrier` at `Interval` exhausts its piece budget (`Unwalked(Exhausted)`, ~5,800 logged piece escalations) at every box from ±1e-3 to ±0.05.
- `curved_edges::passes` hand-composes `on_carrier` + `Span::place` beside `Span::read`.
- F31 and F32 are one class: a heuristic's choice decided on `Band::least()` through the logged door. One door for such choices (`check_unlogged` on the least band, with a row required) would make the next instance land there by construction.

**Sequencing.** This lands after PR 6475. It is filed as the unit `a-foot-has-no-bare-parameter`.
