---
id: ring-2-red-rows-that-are-not-re-pins
kind: issue
title: Four of the RING-0 dry run's red-row classes are not re-pins: a consumer's claim stops being true
status: open
opened: 2026-09-21
priority: P2
cost: D
refs: [H5]
---


## What

H5 ruling 2 settles the ordinary case: a certified bound that gets
tighter re-baselines, cause named per file, no pinned number is a
contract. The RING-0 dry run (`scalar/ring-0-dry-run`, `RingInterval`
as a newtype over `DInterval`) went 22 red across five crates, and
**16 of those are exactly that case** — every pinned bound that moved
moved tighter, 436 tighter and 0 looser over the 960-row coefficient
corpus, and every `vpad`/`apad` in the sweep reporting tables
shrank.

**What the red set is conditional on.** The dry run KEEPS the ring's
refusing guards on `hull` and `clamped_to` (it forwards them to the
backend's `intersection`/identity rules but leaves the ring's refusal
in front, disclosed in RING-0's §4 sweep). That is where the six
`from_bounds` re-mints and every domain clamp in the tree live, so a
RING-2 that drops either guard gets a different red set than the one
below, not a subset of it. Same for `from_certified`: the dry run
returns NaI, and a decorated bracket there changes which reads are
hazards at all (`ring-nan-poison-is-load-bearing-at-unguarded-reads`
§"Two sites that are conditional").

Six rows, in four classes, are a different thing. Each is a
consumer whose CLAIM stops holding, so re-cutting its table would
restate a falsehood.

**(1) A certified bound no longer dominates an f64 sample of the same
expression — 3 rows.** The ring padded one ulp per operation
unconditionally; the backend carries exactness witnesses, so a
residual that is mathematically exact now certifies at or near `0`,
below the rounding error of the test's own `f64` evaluation.

- `geom/tests/curves/review_m5_pr2_e2e.rs` — "clean sample 4.44e-16
  escapes the bound 0e0": the clean fit's bound collapses to exactly
  zero.
- `geom/tests/curves/hull_circle_rehearsal.rs` — sampled plane
  residual 9.99e-16 against a bound of 9.42e-16.
- `mesh/src/nurbs_cert.rs`, `probe_multiplicity_p_minus_one_is_
  covered_and_dominated` — sampled `uu` 10.2873760185735481 against
  certified 10.2873760185735463.

  "The certified bound dominates dense `f64` sampling" was never a
  theorem: the bound encloses the REAL value, and the sampler's own
  rounding is outside it. It held only while the ring's padding
  exceeded that rounding. RING-2 has to decide what these three
  assert instead — the natural form is that the sampled value lies
  within the bound widened by the sampler's own error bound.

**(2) A structural exactness a gate rests on — 1 row.**
`geom-brep`'s `props::quad::tests::q9_the_outer_rules_order_is_
pinned_by_refinement` asserts that an EXACT quadrature rule answers
the same integral bit for bit at round 0 and round 2, with a
quarter-ulp gate. Under the newtype the two midpoints differ by
1.11e-16 on 0.325, because the backend's exactness witnesses fire
asymmetrically across the two chord counts while the ring's
unconditional pad was symmetric. The distinction the row exists to
draw — an exact rule against a converging one — is still the right
one, so the gate must be RE-DERIVED at the new arithmetic rather
than widened until it passes
(`memories/output-stability-as-justification.md`).

**(3) The `0 · inf` corner, as a test's witness — 1 row.**
`geom-core/tests/certified_door.rs`'s
`ring_poison_is_reached_by_arithmetic_not_only_by_construction` uses
`[0,1] * [0,inf]` as its witness that arithmetic can mint poison. The
backend answers that corner `0` by convention and keeps `Dac`
(RING-0's `mul-zero-times-infinite` allowlist class), so the witness
dies. `[1,2] / [0,0]`, the other entry in the same array, survives —
but it survives as the EMPTY set, whose endpoints are NaN, so after
cut (ii) that file's only surviving row does not witness the shape
this whole swap turns on: a refusal carrying REAL endpoints. RING-0
added `a_backend_refusal_can_carry_real_endpoints` to the same file
for exactly that (`[-2,-1] / [0,5e-324]` → `[-inf, -1.797e308] Trv`,
and `([-2,-1] / [-1,1]) * [0,0]` → a finite `[0,0] Trv`); RING-2
inherits it and should keep it green rather than re-deriving it.

**(4) The sign clamp's own claim — 1 row.**
`geom-core/tests/review_m5_pr2_scratch.rs`'s `lane_sign_clamp`
asserts BOTH arms of the clamp: a product of two same-signed
enclosures has `lo() >= 0.0`, and a product of two opposite-signed
ones has `hi() <= 0.0`. Cut (ii) turns the clamp off by ruling, so the
lane asserts a rule the arithmetic no longer makes. **The arm the dry
run actually reproduced is the opposite-sign `hi`**:
`2.2250738585072014e-308 * -1.8669573922462645e-308` gives
`hi = 5e-324` where the clamp gave exactly `0`. The same-sign `lo` arm
moves the same way (`1.902e-308 * 1.902e-308` gives `lo = -5e-324`),
so both arms die, but a reader looking for the failure by name should
look at `hi`. Whether the backend SHOULD adopt the clamp is a question
cut (ii) answered "no"; if that is revisited, this lane is the
acceptance for it.

## Disposition

RING-2's. Filed so the four survive the deletion of RING-0's item:
the dry-run table lives in that item's `## Closed` section and goes
with the program's directory.
