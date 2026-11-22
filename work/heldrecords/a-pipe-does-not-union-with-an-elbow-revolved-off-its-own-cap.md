---
id: a-pipe-does-not-union-with-an-elbow-revolved-off-its-own-cap
kind: issue
title: A pipe does not union with an elbow revolved off its own end cap: the union refuses a recorded contact as undecided at a margin of -7.07
status: open
opened: 2026-11-21
priority: P0
cost: H
---


Found by PROFILE's elbow probe (2026-11-21), run on hosted CI as the
throwaway PR 7280 (run 44889924284, `test (app)`,
`editor-core::zz_probe_elbow`), for Ev's question whether a hollow
cylinder's end loops can be revolved into an elbow
(`work/profile/a-faces-loop-or-closed-trace-cannot-become-a-profile-loop.md`).
Filed here because the refusal is about a recorded contact
(`crates/topo/src/boolean/refusal_routes.rs`, `Coincide::Contact`,
"whether a recorded contact holds along its witness"); re-home it if
another program's charter fits it better.

## Measured

A pipe: `circle((0,0), 1.0)` and `circle((0,0), 0.6)` extruded 2. The
same two circles drawn on its end cap (`editor_core::draw_on_face`,
`OnFace::nearest()`), revolved a quarter turn about a line in the cap's
plane (`axis_through = (3, 0)`, `axis_heading = −π/2`, or `(−3, 0)`
and `+π/2`). The elbow builds. `Node::Union` of the pipe and the elbow
refuses, in all eight cases where the revolve builds (either cap,
either sign of the angle, either side), at ε = 1e-9, 1e-6 and 1e-12:

> the Boolean op refused: whether a recorded contact holds along its
> witness is undecided: margin -7.0685834705770345e0 lies past the
> ambiguity band (1e-9, 1e-8) — a decided sign this decision cannot
> use. Recourse: move the parts so they clearly meet or clearly stand
> apart there

## The defect

The two bodies share the whole annular cap, and their walls meet
tangent along both rims: this is the plain case of building onto a
body's own face and joining the result. The margin is not near the
band: −7.0686 is −9π/4 to the printed digits, a length or an angle
measure decided far on the wrong side, so the reading is asking the
wrong question of this contact (or reading it at the wrong place), not
losing a close call. The recourse cannot be followed: the parts meet
by construction.

Banded P0: a normal verb broken on normal geometry. It also gates
`work/profile/a-faces-loop-or-closed-trace-cannot-become-a-profile-loop.md`:
once a face's loop can be a profile loop, the elbow the person makes
by two clicks and a revolve is this union.

Not traced: which contact record, which witness, and why the margin is
that value. Not measured: an extrude off the same cap unioned with the
pipe (the same shared cap, with the walls coincident and not only
tangent); a solid rod and its elbow (no inner rim).

The probe's source is on the branch `profile/probe-elbow` until PR 7280
is closed; its construction is the four lines above.

## 2026-11-22 — Traced: the glue door's tangency table, on the rim, at a neutral point

Reproduced on main (`3,0`/`−π/2` and the other three building poses
at End; the probe of PR 7280 run locally) with the refusal text
unchanged. Instrumented, not committed.

**Which pair, which locus, which reading.** The outer wall of the pipe
(a `Cylinder`, r = 1) against the outer wall of the elbow (a `Torus`,
R = 3, r = 1), glued at `boolean::glue` → `glued_tangency` →
`verify_tangency` (`crates/topo/src/boolean/mod.rs`). No closed-form
line exists for the pair, so the locus is the rim circle the two
faces share (`rim_wedge::shared_rim`), t ∈ [0, 2π]. The table is
`contact_verify::tangency_reading` (`Claim::Tangency`) →
`tangent_pass`, row (4), the second-order margin: at sample i = 2,
t = π/2, p = (0, 1, 2), `κ_rel` = 0 exactly, the span bound
`kappa_drift` = 14.137 (= 4.5π), arm = 1, so
`Margin::sagitta(|κ_rel| − drift, arm)` = (0 − 4.5π)·1²/2 = −9π/4 =
−7.0686. Decided `Negative`, not `deferred` (the at-sample reading
`Margin::sagitta(|κ_rel|, arm)` is an exact Zero, not Positive), so it
falls to `k_stats::gate_rejection(SECOND_ORDER, …,
SizedPass::Positive)` — "a decided sign this decision cannot use" —
which `verify_tangency` raises as `Coincide::Contact`.

**Why κ_rel is zero there.** Across the rim the cylinder is straight
(normal curvature 0 along its axis) and the torus's normal curvature
along the sweep is cos θ/(3 − cos θ) at the rim point (cos θ, sin θ,
2): it changes sign at θ = ±π/2, where the torus's normal (±y) is
perpendicular to the revolve plane. Every elbow off a round tube has
these two neutral points; it is the G1 tube chain's neutral
meridians (`crates/sweep/tests/mate7a_torus_rest.rs`,
`the_g1_tube_chain_refuses_at_the_glue_door`, pinned refusing by
PR 6079). So the margin is not a close call read wrong; the table
asks for a jet-determinate tangency (`κ_rel` definitely positive),
and a smooth seam between a straight tube and a bend never has one.

**The next wall, behind it.** With row (4) skipped for
`Claim::Tangency` (experiment only), the same unions refuse
`TangencyUnsupported` from `Uncertified::SagUnreached`: along the rim
circle the span bounds are `residual_sag` = `gap_sag` = 3.99 at 9
samples, quartering per halving, ≈ 1e-3 after `REFINE_HALVINGS` = 6,
against a band of 1e-9. The sampled touch cannot certify a curved rim
whose surfaces vary along it, though both faces end on that circle by
construction.

**The neighbours.** A solid rod (`circle(1)` alone) and its elbow:
the same refusal, same margin. An extrude of the cap's ring off the
same cap, unioned with the pipe: builds (the walls are one carrier,
glued `Rest`/continuation, not `Tangent`).

**Why this is a design question, not a fix.** Ratified text pulls two
ways. The topo README's C4 `Tangent` row (the glue door's table)
requires "a definite second-order κ_rel … An in-band κ_rel, or an
exact zero at an isolated point, escalates" — written by PR 6079
(2026-10-30, not an `[ev]` PR), which replaced the earlier clause
that *bridged* exact zeros at isolated points ("a G1 tube chain's
neutral meridians"). The MATE-7 ruling (`docs/MATE-7-TANGENCY-DESIGN.md`,
Ev 2026-09-01) and tier 3 put jet-determinacy on the cusp arm only:
a π rim is the smooth-seam zip with no second-order condition. Options
seen: (A) a shared rim glues on its structure (identity, first-order
parallelism, the routing's wedge π), the sampled touch and second
order kept for loci derived from values; (B) keep the table, grow
exact span bounds for a circle carrier and bridge an isolated
`κ_rel` zero on a π-routed rim only; (C) keep refusing, with a
recourse that is true.
