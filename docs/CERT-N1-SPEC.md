# CERT-N1 — Track N's scalar-lift lane (D240 + D241, with D242, D243, C24)

**Binding at dispatch** (S-CERT program, `docs/S-CERT-PLAN.md` §CERT-N;
difficulty logged at spec: **M**). Read
`docs/prompts/implementer-discipline.md` in full before starting. The
absorbed Track N table in `docs/SMELL-SCAN-2026-08.md` (§"Track N —
`geom`, and the spline and linalg substrate") and findings S33, S100 are
the primary specification; this document fixes which rows this unit
takes, in what order, and the fence. Branch `cert/n1-scalar-lift`.

## What this unit is, and is not

Track N has nine items. This unit takes the scalar-lift lane the table
says wants ONE lane — **D240 then D241** — plus the two small row-0
"yes" members re-homed here by Track T (**D242**, **D243**) and the
curve-side discarded-jet row **C24**, all inside `crates/geom/src/` and
`crates/geom-core/src/spline/`. It does NOT take **H2** (ADV; S99–S103 +
S116(b) — CERT-N2, its own lane because S99's widening changes what
`net::is_placeholder` answers at ~25 consumer sites) nor **S235 / D98 /
D31** (CERT-N3). Track T's `D320` (the `sweep/src/skin.rs:774` ladder)
is filed, not taken.

**Fence (file territory):** `crates/geom/src/`,
`crates/geom-core/src/{spline/,linalg/}`, plus their tests. The
per-variant ladders OUTSIDE this fence — roughly ten across `topo`,
`mesh` and test modules — are NOT yours to edit: you provide the lift;
you file the consumer rows (draft text in your report, with each
site's citation re-derived) on the owning tracks, and land without
them. Filing IS the handoff. `crates/geom-core/src/spline/net.rs` is
CERT-10's fresh `TensorNet` home — read its docs before touching
`spline/`; this unit lands after CERT-10 merges.

## The rows, in order

1. **D240 — the lift.** `Curve3<T>` and `Surface<T>` have no
   `map_scalar`/`lift`; four hand-kept exhaustive ladders live in
   `geom/src/{curves,surfaces}.rs` (`:818`, `:908`, `:653`, `:1057` at the
   finding's citation — re-derive), each **silently mapping `Nurbs(_)`
   to the placeholder rather than lifting the payload** (the substitution
   S33 named; Evan: "baffle me with how they ever happened"). Give each
   enum ONE generic lift that lifts the NURBS payload (control net,
   weights, knots through the scalar conversion) and cannot forget a
   variant (exhaustive match in one place). The dual and interval
   ladders differed only in the scalar conversion: one lift, the
   conversion as the parameter. **Red-first**: a row that lifts a
   described NURBS curve/surface and evaluates the lifted payload
   against the source — red on the old ladders (which hand back a
   placeholder). Decide and state what a placeholder lifts to (a
   placeholder — argued) and what a poisoned described net lifts to
   (never the benign placeholder — H2's ground, do not widen into it;
   just do not make it worse).
2. **D241 — the name.** `geom/src/scalar_lift.rs` is named for the job
   it declines (it holds the four leaf point/vec converters and says
   the ladders stay elsewhere); `lift_to_dual` (curves) vs `lift_dual`
   (surfaces) sit beside two unrelated `lift`s. With D240 done the
   module can be what its name says — home the lifts there, one naming
   scheme, and retire the four surviving ladders in the same PR (D240
   is the work; this row is the name; one lane).
3. **D242** — `ControlPoint::channel`'s two `unreachable!` arms
   (`geom/src/net.rs:63`, `:88` at the 2026-08-31 re-derivation) are
   row 0's own *yes* side: the trait is `pub(crate)` with two impls and
   two call sites, one already `for d in 0..P::CHANNELS`, so a
   `channels()` iterator deletes both arms with no public API change.
   Do it; the rustdoc precondition prose ("`d >= CHANNELS` is a caller
   bug") goes with the arms.
4. **D243** — `insert_once_ring`'s interior-knot guard
   (`spline/compose.rs:315` at re-derivation) announces a caller
   precondition a type could carry: an `InteriorKnot` newtype. Row 0's
   question is the PROPAGATION, not the arm: the newtype must reach the
   filter (`interior_knots`, strictly interior by the `KnotVector`
   invariant) and the extras-filtering caller, so the `unreachable!`
   disappears because the state is unrepresentable — not because it
   moved. If propagation reaches past the fence, stop at the fence and
   file. Note CERT-10's `TensorNet` sits in the same module tree.
5. **C24** — `NurbsCurve::deriv_in_span`/`deriv2_in_span` each run a
   full order-2 basis and discard (S32's class on the curve side):
   compute once, return the jet, or argue the cost is nil with a
   measurement. Not a soundness row; last, and droppable with the
   argument if the lane is long.

## Posture

- ε: lifts are exact structural maps (state it); `CI-Config: lane=both`
  (the interval lift is half the point) with the ε argument stated;
  three-ε local sweep on every new/changed row.
- Review: the program's standard v6 dual; D240 is where a wrong answer
  is reachable (a lifted payload that does not evaluate to the source)
  — say so in the body so the reviewers execute it.
- Landing conventions (repartition rule 3): the landing PR DELETES the
  closed rows (D240, D241, D242, D243, C24 if closed) and the closed
  findings' text (S33, S100) from SMELL-SCAN, member by member where
  partly closed; relocate standing rules into surviving text first;
  expect the merge conflict on SMELL-SCAN and take it.
- No `Co-Authored-By`; rows and findings spelled out; push early to
  `cert/n1-scalar-lift`; the gate runs when the orchestrator opens the
  PR; the discipline doc's lane rules apply in full.

## Acceptance

- One lift per enum, payload-lifting, exhaustive in one place; the four
  in-fence ladders gone; red-first digits in the body; `scalar_lift.rs`
  named for what it holds; D242's arms and D243's guard gone by
  unrepresentability; C24 measured.
- Consumer rows filed for every out-of-fence ladder (re-derived
  citations), and Track T's D320 named as the sweep copy's home.
- Sweep obligation: other per-variant ladders in the fence (a `match`
  over `Curve3`/`Surface` that maps `Nurbs(_)` to a placeholder for any
  reason); hit list with dispositions; state what the pattern cannot
  match.
- Deviations stated; D2-addendum classification for anything minted or
  retired (D242/D243 are row-0 deletions — say so).
