---
id: tangent-arc-near-a-full-turn-loses-its-end-vertex
kind: issue
title: paths: a tangent arc whose target is nearly behind the tip computes its quarter-tangent through a cancelling denominator; the stored arc misses its own end vertex, silently at f64
status: open
opened: 2026-10-19
priority: P1
cost: M
refs: [5384]
---


Found by PR 5384's review (MINOR there). This bug is already on main: `tangent_arc` in `crates/profile/src/path.rs` is shared by `tangent_arc_to` and `arc_onto`.

**The defect.** `tangent_arc` spells the quarter-tangent `x = across / (|d| + along)`. When the target is nearly BEHIND the tip, `along → −|d|`, and the denominator cancels. Its true value is about `|d|·(π − δ)²/2`, but its rounding error is about `ulp(|d|)`, so `x`, and with it the stored sweep, loses most of its digits. The centre and radius (`ρ = |d|²/(2·across)`) stay well conditioned. Only the sweep is wrong, so the stored arc does not end on its own end vertex. The arc's facts are registered (`Facts::Registered`), and `ConstructedProfile::validate` does not re-check them (D1), so at f64 the miss can pass validation silently.

**Measured.** The probe is `At(0,0) Toward(1,0) TangentArcTo(p) LineTo(Start)`, with `p` at chord 2.7 nearly behind the tip and the carrier radius R set by the offset:

| R | f64: miss of the stored arc's end | f64 `validate` | Interval |
|---|---|---|---|
| 1.36e4 | 3.4e-8 (the reviewer saw 2e-8 to 7e-8) | **Ok: silent**, a miss of 34× the default ε | sweep enclosure ~2e-11 wide; `validate` refuses (the span is undecided) |
| 1.37e7 | 4.4e-2 (reviewer: 0.07) | refuses: "whether a point falls inside an arc's span is undecided" (misleading) | refuses the same way |
| 1.37e8 | 2.7, a whole chord (reviewer: 11.6, winding flipped) | refuses: "within tolerance of a full circle" | sweep `[−2π, 2π]`; refuses |

The reviewer measured "tangential contact" as the Interval refusal for R ≥ 1e4.

**Fix shape (not taken in 5384).** The branch-free, cancellation-free spelling is `x = 2·across / (|d| · |u + d/|d||²)`, because `1 + cos δ = |u + d̂|²/2` and `|u + d̂|` is first-order in `π − δ`. The other route is `(|d| − along) / across` on the far side, but that branch needs a decided sign, which is a structure record. Either one changes the stored bits of every tangent arc (the spelling), or of every arc past a half turn (the branch). That moves goldens well beyond `profile`: sweep digests and stored-form dumps. So the fix wants its own unit, re-baselining those goldens and naming what moved, rather than a ride-along on 5384.
