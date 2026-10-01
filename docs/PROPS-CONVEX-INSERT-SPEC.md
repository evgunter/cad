# PROPS convex insertion — the width `insert_once_ring` gives away

**Binding at dispatch** (PROPS program; item
`work/props/f64-refinement-inside-an-enclosure-has-five-more-sites.md` —
read it in full, this unit closes its FIFTH site only). Difficulty
logged at spec: **H / NUMERIC**.
**Review tier: DUAL** (`memories/orchestration-model.md`), so the pair
is an experiment row and its method is `docs/DUAL-REVIEW-PROTOCOL.md`'s.
Reason, recorded here so the call is not invisible: the correctness
question is an interval-arithmetic SOUNDNESS argument whose failure mode
is a bound that is too TIGHT — an unsound certificate in a kernel whose
whole job is certified enclosure, and the kind of error that reads as an
improvement. Its blast radius is every certified bound in the tree.
Read `docs/prompts/implementer-discipline.md` in full. Branch
`props/convex-insert`, cut from `main`.

## What this is

`crates/geom-core/src/spline/compose.rs`, `insert_once_ring`, the
window arm:

```rust
let alpha = (up - Interval::point(knots[i]))
    / (Interval::point(knots[i + p]) - Interval::point(knots[i]));
out.push(coeffs[i - 1] + (coeffs[i] - coeffs[i - 1]) * alpha);
```

**The site is SOUND and that is not in question.** `alpha` is already
an outward-rounded ring quotient, so this is not the rounded-ratio
defect the item's spelling 2 describes. What it gives away is WIDTH:
the lerp form reads `coeffs[i - 1]` twice, so an interval coefficient's
own dust enters with coefficient `1 + α`, and a fold of insertions
multiplies it per step. `to_bezier_spans` inserts to FULL multiplicity,
so the fold is `p` deep per interior knot and that is where it costs
most.

Measured already, and not to be re-derived from scratch: TESS-2 recorded
**355 ulps against 16** for the convex form over 30 insertions, and the
quarter cylinder's structurally-zero `S_vv` at **8.8e-11 against 7.0e-13**
— 126x.

**Two ENCL rows are parked on this fix**, one of them P0, and ENCL
measured the downstream and declined to take the site:
`offset-fit-at-tight-eps-refuses-every-curved-nurbs-chart` (the saddle
wall certifies at the DEFAULT eps where it refuses today; `bowed`
certifies at 1e-12) and
`a-rigid-map-still-refuses-the-bowed-approx-fixture-at-eps-1e-12` (0 of
93 refusals, and the bound becomes frame-invariant). Their numbers are
appended to our row by PR 3272.

## The fix, and the one way to get it wrong

The convex form is `β·c_{j−1} + α·c_j`, which reads each coefficient
once.

**Derive `β` from the knots, NOT as `1 − α`.**

```
α = (u − knots[i]) / (knots[i+p] − knots[i])
β = (knots[i+p] − u) / (knots[i+p] − knots[i])
```

`1 − α` in interval arithmetic is a different quantity: it inherits
`α`'s outward rounding and then adds its own, which gives back some of
the width the change exists to save and — more importantly — makes the
soundness argument go through a subtraction instead of directly. Each
of `α` and `β` outward-rounded from the knots separately encloses its
own true ratio, which is all the enclosure needs; they are NOT required
to sum to exactly one, and a lane that "fixes" `β` so they do has
misunderstood the invariant.

## The claim that has to be established, not assumed

**The result still encloses the true inserted coefficient.** This is
the whole unit. State the argument, and then TEST it rather than
resting on it:

- the true coefficient is `(1 − λ)·c_{j−1} + λ·c_j` for the exact
  rational `λ`;
- `β ∋ (1 − λ)` and `α ∋ λ` by outward rounding from the exact knots;
- so the interval combination contains the true value for any
  `c ∈ coeffs`.

The failure mode to hunt is a bound that comes out TIGHTER THAN TRUTH.
Test against exact rational arithmetic the way TESS-2 did for the
patch-bound instance: build cases where the exact inserted coefficient
is computable, and assert containment — not just that the new width is
smaller. **A narrower bound is the expected outcome and therefore not
evidence of correctness; containment is.**

## Rulings

- **Bounds are expected to TIGHTEN, tree-wide, and that is a
  re-baseline, not a cost** (discipline §3). Re-baseline with digits
  and say what moved.
- **A bound that GROWS is a finding, not a re-blessing.** The convex
  form should not widen anything. If something widens, stop and report
  with the case; do not absorb it.
- **Reproduce ENCL's two measurements** rather than citing them: the
  saddle wall at default eps and `bowed` at 1e-12, and the 0-of-93
  rigid-map figure. If either does not reproduce, that is the finding
  and ENCL needs to hear it — say so and I will carry it.
- **This unit closes site 5 of five. Do not take the others.** Sites 1
  and 2 (spelling 1, `f64` refinement then lift) and the two `quad.rs`
  sites (spelling 2, rounded ratio) are different defects with
  different fixes. The item stays open with site 5 marked closed.
- **Sweep for the SHAPE** (discipline §5): other ring-lerp combinations
  that read an endpoint twice. Put the hit list and its disposition in
  the PR body, one line per hit, fixed or not-this-unit-and-why.

## Posture

- eps posture: none — no band, comparand or predicate name moves. Say
  so.
- Verification is hosted CI (discipline §2), and **a green PR is not a
  green nightly**: this change moves certified bounds, so the eps rows
  and the slow set matter. If you judge a nightly-only row directly
  relevant, run it or dispatch the nightly on your branch and say so.
- **8-core 9 GB box, machine-wide build mutex**: read
  `memories/agent-lane-operations.md` §Build concurrency, wrap heavy
  cargo calls in `local-scripts/with-build-slot.sh`, pass no `-j`,
  never two batteries at once. Own `CARGO_TARGET_DIR` outside the
  worktree. Never end a turn with background work running, and do not
  sleep on a CI wait.
- Review: **DUAL**, in the concordance experiment.
- **Landing: the item gets `pr:` and `status: review`. DO NOT MERGE.**
  No `Co-Authored-By`, no `CI-Config:` trailer, no empty commits.

## Acceptance

The convex form in place with `β` derived from the knots; the
containment claim argued AND tested against exact rational arithmetic,
including cases chosen to break it; a red-first row that shows the
width the lerp form gives away and goes green; the tree-wide
re-baseline measured with digits and every moved bound moving in the
tightening direction, with any widening reported rather than absorbed;
ENCL's two measurements reproduced; the shape sweep with its hit list;
`work.py lint` clean; hosted CI green.
