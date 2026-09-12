# D290 — the knot rescale is a `KnotVector` door; `on_carrier_domain` is rescale-then-lift

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-12).** Binds
the implementer of unit `D290`; deleted at merge per `docs/DOC-LEDGER.md`.
Read `docs/prompts/implementer-discipline.md` in full first. The item is
`work/scalar/D290.md`.

## 0. The finding this executes, and what the tree adds to it

`edge_nurbs.rs`'s `on_carrier_domain` (`crates/geom-brep/src/edge_nurbs.rs`)
is two operations spelled as one: a knot RESCALE of the fit's clamped
`0 → 1` knots onto `[t0, t1]` (structure, `f64`), and a scalar LIFT of the
control net through `T::from_f64`. It rebuilds the curve through
`NurbsCurve2::new(..).ok()`, so a count error that cannot occur on a
validated curve's own net is swallowed into `None` beside the knot
validation that can fail. The fix the row states: rescale the knots as a
`KnotVector` operation at `f64`, then lift with `NurbsCurve2::map_scalar`,
so the `.ok()` covers only the knot door.

**The rescale already has a second, private spelling**, and the two
disagree on one thing that matters:

- `crates/geom-brep/src/offset_fit.rs`, `rescaled_knots(kv, lo, hi)`,
  maps the interior knots affinely and **pins the two clamp runs to `lo`
  and `hi` exactly**, so the fitted surface lives on the base's own chart
  rectangle bit for bit.
- `edge_nurbs.rs` maps EVERY knot as `t0 + (t1 - t0) * k`. For `k = 1`
  that is `t0 + (t1 - t0)`, which is not `t1` in `f64` in general (an
  ulp off either way); for `k = 0` it is `t0 + (t1 - t0) * 0.0`, which is
  `t0` exactly only when `t1 - t0` is finite. So the image's domain can
  fall an ulp short of, or past, the carrier interval it is supposed to
  be expressed on.

This unit therefore closes a CLASS of two, not an instance: one door in
`geom-core`, both callers on it, the private copy deleted. The pinned-ends
behaviour is the correct one and is what the door does.

## 1. What this unit delivers

**The primitive**: a method on `KnotVector` in
`crates/geom-core/src/spline/knots.rs` — name it in the crate's own
vocabulary (`rescaled`, `on_domain`, or what reads best beside
`clamped`/`unit_segment`) — that returns the same clamped structure
affinely re-expressed on `[lo, hi]`:

- the two clamp runs are `lo` and `hi` **exactly** (assigned, not
  computed); every interior knot is `lo + (hi - lo) * k`;
- degree and knot count are unchanged, so `control_count()` is unchanged
  — say so in the doc, it is the fact the curve door below rests on;
- it REFUSES, typed, when `lo` or `hi` is non-finite or `hi <= lo`
  (a collapsed or reversed domain), naming the domain rather than
  whichever `clamped` clause happens to fire first — add a
  `KnotVectorIssue` variant if none says it; and it goes through
  `clamped` (or re-checks the same clauses) so that an interior pair
  collapsing under rounding to a multiplicity above the degree refuses
  through the existing clause rather than minting an invalid vector.
  Argue in the doc why the pinned ends cannot break the clamp clauses
  (equal source knots map to equal images; the map is monotone for
  `hi > lo`).

**The curve door**: `NurbsCurve2`/`NurbsCurve3` (the `nurbs_curve!`
macro in `crates/geom/src/curves/nurbs.rs`) gain the same-curve-on-another-
domain door — the rescaled knots with the net and weights carried
verbatim through `from_validated_parts`, with the count argument
`map_scalar` makes stated once more for this door: the rescale changes
neither degree nor knot count, so `control_count()` is unchanged. Its
`Result` is the knot door's and nothing else. Whether it also lands on
`NurbsSurface` (u and v) is the implementer's call: land it only if the
`offset_fit` consumer becomes simpler for it — that consumer builds a NEW
surface on rescaled knots from an interpolation, so it may only need the
primitive. Say which and why.

**The callers**:

- `edge_nurbs::on_carrier_domain` becomes: curve door at `f64` (the only
  `Result`, mapped as today to `PlaneNurbsRefusal::PcurveFit`), then
  `map_scalar(T::from_f64)`. No `.ok()` over a count check remains.
  Whether `PcurveFit` should carry the `SplineError` is TRIM's question
  and not this unit's — leave the refusal vocabulary as it is.
- `offset_fit::rescaled_knots` is deleted; its two call sites use the
  primitive.

**What must not change**: `offset_fit`'s knots are bit-identical before
and after (it already pinned the ends). `edge_nurbs`'s interior knots,
control net and weights are bit-identical; **its end knots move onto
`t0`/`t1` exactly where they were an ulp off** — that is the one
behaviour change, it is the correct one, and §3 says how to show it.

## 2. Comment style, docs

Comments state the invariant (implementer-discipline §4): the door's doc
says what the ends are and why the count is preserved; nothing narrates
the two former copies. `crates/geom-core/README.md`'s knot-structure row
lists the new door beside `clamped` if that table enumerates doors;
present tense only.

## 3. The pin

- **Ends are exact**: a row over a `(lo, hi)` pair for which
  `lo + (hi - lo) != hi` in `f64` (search for one; state it as a literal
  with the ulp difference asserted, so the row documents why the pin
  exists) asserting `domain() == (lo, hi)` bit for bit, degree and
  `control_count()` unchanged, interior knots equal to the affine image.
- **Refusals are typed**: non-finite `lo`/`hi`, `hi == lo`, `hi < lo`
  each refuse with the domain-naming reason; an interior collapse (two
  distinct interior knots whose images coincide at a tiny span) refuses
  through the multiplicity clause, not by panicking or by minting.
- **`edge_nurbs` unchanged where it should be**: the existing
  `edge_nurbs` and `plane_nurbs_limbs` rows stay green; if any committed
  value moves because an end knot moved onto `t1`, do not restore the
  old number — say in the PR what moved and why (implementer-discipline
  §3).
- **`offset_fit` bit-identical**: its suites unchanged; a debug digest
  or a direct knots-equality row over its fixtures before/after is the
  cheap proof.
- **The lift is the lift**: the lifted `NurbsCurve2<T>` evaluates to the
  `f64` curve's points (bit for bit at `f64`, as a bracket at
  `Interval`) — `scalar_lift.rs`'s `described_nurbs_lifts_as_its_payload`
  rows are the shape to copy, one row at this door.

## 4. Sweep

The class is "a `Vec<f64>` of knots derived from another `KnotVector`'s
knots by an affine map, then `clamped`". Sweep `crates/*/src` and the
excluded roots (`scripts/doc-gate.sh --print-roots`) for it and put the
hit list with a disposition per hit in the PR body. Known hits at this
spec's writing: `edge_nurbs.rs` and `offset_fit.rs` (this unit). Known
near-misses to list as not-this-unit with the reason: the derivative
knot slices at `patch_bound.rs` and `props/quad.rs` (`inner`, degree − 1
— a different operation, and `derivative_knot_slice` exists), the
`pcurve_cache.rs`/`pcurve.rs` builders (knots minted from a segment
count, not remapped). Say what the pattern could not match.

## 5. Fence

This program claims no paths. This unit reaches
`crates/geom-core/src/spline/knots.rs` and `crates/geom/src/curves/nurbs.rs`
(PROPS' territory), `crates/geom-brep/src/offset_fit.rs` (PROPS') and
`crates/geom-brep/src/edge_nurbs.rs` (TRIM's). The orchestrator announces
the seam in `work/scalar/log.md` and on the PR; the lane's job is to keep
the diff to those files plus their tests, merge `origin/main` immediately
before opening the PR, and re-run `python3 scripts/work.py territory
--base origin/main` so the PR body lists exactly the paths crossed.

## 6. Verification and report

Local: `geom-core`, `geom` and `geom-brep` suites at default ε (the
change is `f64` structure; the interval lane is the lift row above and
rides the gate). Hosted CI proves the rest — poll the run to conclusion
in the foreground and report the run id. Report ≤150 lines: the door's
name and shape, the ends argument, the `(lo, hi)` literal the exactness
row uses, the sweep's hit list and blind spot, what moved (if anything)
and why, deviations, findings filed outside the fence (with the row
names).
