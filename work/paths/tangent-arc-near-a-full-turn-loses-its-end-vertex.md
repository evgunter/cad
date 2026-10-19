---
id: tangent-arc-near-a-full-turn-loses-its-end-vertex
kind: issue
title: paths: a tangent arc whose target is nearly behind the tip computes its quarter-tangent through a cancelling denominator; the stored arc misses its own end vertex, silently at f64
status: closed
closed: 2026-10-19
branch: paths/tangent-arc-near-full-turn
opened: 2026-10-19
priority: P1
cost: M
refs: [5384]
pr: 5403
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

## Outcome

Fixed in #5403 (`paths/tangent-arc-near-full-turn`).

- **The form.** `path::tangent_arc` chooses its quarter-tangent with
  `Real::select_le_zero` on `along`: `across/(|d| + along)` up to a
  half turn and `(|d| − along)/across` past it. The two are equal over
  the reals, and on its own side each is a sum of terms of one sign.
  No structure record is needed: the choice is a value operation, not
  a decision, and an `Interval` straddle takes the hull of two accurate
  arms. `via_quarter_tan` has the same shape and takes the same choice
  on `d₁·d₂`. Every arc up to a half turn keeps its bits and its
  symbolic forms. The branch-free `2·across/(|d|·|u + d̂|²)` is just as
  cancellation-free, and it was tried first. It rebuilt the stadium's
  half-turn forms, and the link study (`m10_9_pins_interval`) stopped
  certifying at its M10-9 scale: the mapped-source residual came out
  [0, 1.007e-9] against ε = 1e-9.
- **Oracle.** `scripts/oracles/tangent_arc_sweep_mpmath.py` reads
  `path::tests::dump_tangent_arcs_for_the_mpmath_oracle`: 1962 cases,
  turns 1e-12 to 2π − 1e-12, both windings, scales 1e-6 to 1e6, three
  directors, two tips. The worst `f64` sweep error is 3.6 units of
  2⁻⁵² (the single cancelling spelling: 5.7e16), and the worst end
  miss is 2.0 units of 2⁻⁵²·scale. `Interval` always encloses, and is
  at most 1.2e-14 wide (9.8e-15 near the full turn).
- **Refusal reasons.** `seg::classify`'s diameter clearance is now
  spelled `2r·cos²(|Δθ|/4)/(1 + sin(|Δθ|/4))`. `1 − sin` rounded to
  zero, so at R = 1.37e8 `f64` read `NearFullArc` for a clearance of
  3.3e-9 m, which is inside the band. Both scalars now escalate on
  `arc_diameter_clearance` with that margin. A target decided ON the
  departure line behind the tip keeps its kind, `DegenerateArcChord`,
  as PATHS-DESIGN rules. A new `behind` field makes its message say
  the true thing ("on the departure line behind the tip"), where it
  used to say "the endpoints are within tolerance of each other (chord
  2.7 m)". At R = 1.37e7 and the default ε, `f64` escalates on
  `arc_span` with a margin of one ulp of 2R. That is the scene's f64
  resolution, a true reason. At R = 1.36e4, `Interval` escalates
  `arc_span` at the shared vertex: filed below.
- **Moved.** The stored sweep of every tangent and `Via` arc past a
  half turn. One corpus field: `shared coverage corpus 8`'s half turn
  now stores `−π` exactly, where it stored one ulp past it. The plate's
  walk ledger (the clearance's forms), and the link's past-the-ceiling
  `frozen`, 556 → 557. No decision moved, and nothing in viewer moved.
- **Rows.** `tests/tangent_arc_near_a_full_turn.rs`: the three radii
  by both verbs, the `Via` arc near a full turn, and a counterexample
  search (shape 1) over near-behind targets. All are red on the
  cancelling form at each ε row, and green after.
- **Filed.** `a-center-arc-near-a-full-turn-loses-its-end-vertex`
  (P1, measured: a `Center` arc at R = 1.37e6 misses by 2e-4,
  silently), `a-hairpin-line-fillets-half-tangent-cancels` (P2,
  derived) and `a-near-full-arcs-span-reads-a-chord-scale-apex-at-interval`
  (P3).

## Fix pass (single FULL review: APPROVE-WITH-FIXES)

The review falsified two claims above. The select did NOT keep arcs up
to a half turn form-identical: the plain walk turns a `Select` into an
opaque atom. It also kept a second spelling of Δθ beside
`bulge_from_via`'s `atan2`. The fix pass replaces the form:

- **One home** (`sugar.rs`). It reads an arc's sweep as an angle:
  - `split_sweep(across, along) = 2·atan2(across, along)`: the tangent
    arc, read by `tangent_arc_to` and by `arc_onto`, which runs it in
    reverse;
  - `split_turn`: a line×line fillet sweeps its corner's turn, read off
    the legs, not off the short chord `t2 − t1`;
  - `sweep_about`: the angle between the radii, wound as
    `sgn·(π + atan2(−sgn·(va × vb), −va·vb))`. This covers `Center`
    arcs, fillet side runs and arc-carrier fillets, and closes the P1
    `a-center-arc-near-a-full-turn-loses-its-end-vertex`.
  - `Via` lowers its carrier from tan(θ/4), and `tan(sweep/4)` is at
    its pole near a full turn: measured at R = 1.36e4, its Interval
    landing was 4.8e-10 wide. So `Via` and `bulge_from_via` read
    `split_quarter_tan`, the select between the two half-angle
    quotients, from the same split.
- **Oracle.** 3390 cases: tangent arcs, and `C` rows for arcs about a
  centre. The worst f64 sweep error is 3.4 units of 2⁻⁵². Interval
  always encloses, at most 1.1e-14 wide. On the reviewer's generator
  (78 678 rows) there are no violations. The only full-width enclosure
  is at the cut, where `across`'s enclosure straddles zero, and the
  departure-line gate refuses those inputs first.
- **Symbolic tier.** The link certifies further: 3.029e2·ε → 3.437e2·ε,
  because a half turn's literal-zero `along` folds through rule D's
  half-π fold. The bracket's refusal moves 7.624e2·ε → 7.768e2·ε, and
  the pad's ceiling holds. The pad's fillet sweeps are `atan2` atoms,
  so 56 of its decisions move from symbolic to numeric.
- **`behind: bool` dropped** (PATHS-DESIGN's ruling: two types over a
  tag). `DegenerateArcChord`'s message names both of its cases, for
  either verb, with a recourse. The separate-variant question is filed
  as `degenerate-arc-chord-names-two-cases-under-one-kind`.
- **Filed**: `a-stored-arcs-end-is-never-checked-against-its-authored-vertex`
  (P1, the class).
