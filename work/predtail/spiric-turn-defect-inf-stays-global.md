---
id: spiric-turn-defect-inf-stays-global
kind: issue
title: A spiric's inf over the turn defect is the global minor radius, though the defect window bounds it locally
status: open
opened: 2026-10-26
priority: P3
cost: E
---


Split from PR 5724's review (`pred/turn-rule-follow-ups`).
`geom::carrier_speed::turn_defect_speed_lo`
(`crates/geom/src/carrier_speed.rs:143`) returns `speed_range`'s inf on
a spiric: its minor radius `r`, the least `|C′|` over the whole period
(`|C′|² = r²(1 + (d/f)²·sin²v)`). The sup over the defect is already
local (`geom::spiric_window_rate_bounds`,
`crates/geom/src/curves.rs:1285`, over the defect's reach). The same
window gives a local inf: `|C′|` over it is at least
`r·√(1 + (d·sin_min/f(rho_hi))²)`, `sin_min = max(0, |sin m| − reach)`
and `f` at the window's largest `ρ`.

On an ellipse the global inf made `Over`/`Short` escalate on the fast
side (see `turn-rule-reads-short-at-the-global-speed-floor`, closed on
the conics). On a near-tangent spiric the speed off the pinch is up to
`×354` the minor radius (`a_spirics_defect_sup_bounds_the_speed_there_and_is_local`
in `crates/geom/tests/r2_turn_probes.rs`), so certification's check 2
(`crates/geom-brep/src/certify.rs`, through `turn_defect_rate`) reads a
spiric window over or short of a turn by up to that factor of the band
as an escalation. The fix needs a `spiric_rate_bounds` lower twin and a
row that pins the verdicts which move.
