---
id: a-foots-parameter-is-read-without-its-certificate
kind: issue
title: A foot's t is read without its on_carrier certificate (census direction_at, curve_pair walk and pose), so a polish tie across a spline's branches silently reverses a tangent unless the tie stays logged
status: open
opened: 2026-11-03
priority: P1
cost: M
design: true
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
