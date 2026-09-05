---
id: coefficients-carry-their-knot-vector
kind: unit
title: The coefficient↔knot-vector pairing is length-only
status: closed
opened: 2026-09-05
closed: 2026-09-05
branch: props/coeffs-window
refs: [span-carries-its-knot-vector]
pr: 1985
---



## The residue

`span-carries-its-knot-vector` closed the span↔structure pairing: a
`Span` borrows the `KnotVector` it is a proof about, a `CurveWindow`
borrows its curve and a `SurfaceWindow` its surface, every
span-restricted door reads everything from that one borrow, and both
`admits` predicates are gone because the state they tested is
unrepresentable. It did **not** close the pairing one level down, and
this row is that one.

**Where it lives, exactly: `geom-core`'s free `hull` functions.** They
are the only doors left that take a coefficient array *beside* a span
rather than reading it through a borrow:

| Door | `crates/geom-core/src/spline/hull.rs` |
|---|---|
| `span_hull(coeffs, span)` | :123 |
| `span_hull_rational(coeffs, weights, span)` | :193 |
| `derivative_span_hull(coeffs, span)` | :296 |
| `sup_norm_bound_span(coeffs, span)` | :339 |
| the private `span_indices(coeff_len, span)` they all route through | :108-113 |

`span_indices` refuses `coeff_len != span.knots().control_count()` and
nothing else; `span_weights_positive` (:172) is the same check on the
weights.

**What it catches:** an array of the wrong *length*. That is exactly the
bound that keeps the window `[index − degree, index]` inside the array,
so nothing here can index out of range and no public door here panics
(D9).

**What it does not catch:** which curve the coefficients came from. The
review lane measured the three shapes the retired `admits` used to
refuse, all at equal control count, and all three are answered finitely
by these doors:

- **(a)** same degree, different interior knots — the disclosed shape;
- **(b)** a span whose index is **empty** in the vector the coefficients
  belong to (`admits == false` at the merge base);
- **(c)** a span of a **different degree** (`admits == false` at the
  merge base) — the sharpest, because the basis row that would pair with
  it is a different length from this vector's.

The exit that matters is `sup_norm_bound_span(coeffs, span) <= eps`, the
C2.2 honesty limb: a same-length foreign array yields a *finite* bound
over data that was never bounded.

**What is NOT in scope, and this is what changed:** `geom`'s curve and
surface doors. `CurveWindow::{eval_in_span, ders_in_span, …}` and
`SurfaceWindow::{eval_in_span, ders_in_span, ders3_in_span}` read their
control points and weights from the curve or surface the window borrows,
so they have no coefficient argument to mis-pair. The count relation
survives at `NurbsCurve::new` and `NurbsSurface::new`, once, at
construction — and there it is load-bearing: it is what makes every
window's `first_control + j` a construction fact.

Pinned as behaviour, so closing it reds the rows rather than passing
unnoticed:

- `crates/geom-core/tests/span_hull_window.rs::the_coefficient_pairing_is_still_length_only`
- `crates/geom/tests/curves/span_window_pairing.rs::the_free_hull_doors_relate_coefficients_by_length_alone`
- `crates/geom/tests/curves/span_window_pairing.rs::the_free_hull_doors_answer_every_shape_the_guard_refused` — shapes (b) and (c), adopted from the review lane's probe
- `crates/geom/tests/surfaces/span_window_pairing.rs::the_constructor_relates_the_net_to_the_vectors_by_count_alone` — the constructor half, falsifiable in both directions

and stated at `crates/geom-core/README.md` (SPLINE-DESIGN S1, "the one
pairing S1 leaves open").

## The third member of the family

`InteriorKnot` (`crates/geom-core/src/spline/knots.rs`) has the same
shape — a value proved interior to one vector's domain, carried without
that vector — and is **deliberately crate-private** for it (CERT-N3's
decision, not reopened here). Its two consumers use the type in opposite
ways, both argued at its doc. Any move that makes it public owes the
borrow `Span` now carries.

## Shape of a fix, not a plan

The span's answer was a borrow, and the curve and surface halves took
the same answer one level up — the door reads the net from the structure
it already borrows. The free `hull` functions are the case where that
does not obviously apply: their `coeffs` are not a curve's control net
but whatever coefficient brackets a fitting or composition pass
produced, and they are `RingInterval`/`CertifiedEnclosure` values with
no owner to borrow. Candidates when this is scheduled:

- (a) give the hull doors a *pairing* argument of the same shape — a
  `Coeffs<'a>` minted only against a `KnotVector`, which is `Span`'s
  trick applied to the payload;
- (b) an invariant-lifetime brand, rejected once for `Span` on ergonomic
  grounds (`span-carries-its-knot-vector`, option B);
- (c) accept it with the docs saying it is a decision.

Not urgent: no live caller mis-pairs, and the failure is bounded to a
wrong number rather than a panic.

## Closed

**Ruled (a), in the structural form the curve half took; landed as
ruled.** `crates/geom-core/src/spline/hull.rs` has no free function
taking a coefficient array. `SplineCoeffs<'a, E>` borrows the
`KnotVector` its array was fitted against (and optionally the weights),
minted only by `KnotVector::coeffs` and `KnotVector::coeffs_rational`,
where the count relation is checked once and a wrong length is `None`.
`SplineCoeffs::{span, span_at}` mint a `CoeffWindow<'a, E>` — the pair
beside a `Span<'a>` of ITS vector — and the doors are methods reading
through the borrow: `CoeffWindow::{hull, hull_rational, derivative_hull,
sup_norm_bound}` per span, `SplineCoeffs::{domain_hull,
domain_hull_rational, derivative_coeffs, derivative_domain_hull,
sup_norm_bound, sup_norm_bound_rational}` over the domain.
`span_indices` and `span_weights_positive` are gone with the check they
made; weight positivity stays a per-window check at the rational doors
(a value precondition on exactly the weights a window reads, where the
count is a pairing fact), argued at the module doc.

**The three shapes** (a), (b), (c) are `compile_fail` doctests on
`SplineCoeffs` with legal twins (`E0308`, `E0451`, `E0061` at 1.97.0,
read off `rustc`; stable rustdoc does not verify the codes). The four
rows this item named as the residue's pins went red at the type level
and are replaced: the mint's count refusal keeps one behavioural row
and every window a pair mints answers what the domain door hulls over
its window (`geom-core/tests/span_hull_window.rs`); a curve's
`ring_coords()` channels mint against its own vector
(`geom/tests/curves/span_window_pairing.rs`); the constructor row stays.

**Bit identity is the receipt:** `geom-core/tests/coeffs_bit_identity.rs`
— 960 default-lane rows (`f64` and `RingInterval` brackets) — and its
whole-file-gated twin `coeffs_bit_identity_interval.rs` — 480
`interval`-lane rows; six vectors at degrees 1–4 with interior
multiplicities up to the degree, every span, every door — captured
through the retired spellings at the merge base and unchanged at the
head; `geom`'s 1001- and 11,151-row span digests unchanged too.

**Consumers, all on the pair:** `ssi.rs` `pcurve_windows` and
`ssi/certify.rs` (one pair per coordinate channel outside the span
walk; coordinates and knots from the same curve — `refined(carrier)`
included), `props/quad.rs` (`bspline_range_hull` takes the pair; the
ladder and the `DerivTake` closures mint), `spline/net.rs`
(`diff_{u,v}_knots`), `curves/nurbs.rs` (the weights' derivative),
`mesh/chords.rs`. Every mint refusal these consumers carry is dead by
construction and answers what the retired door's poison answered.

**Sweep residue, reported in PR for the orchestrator to place:** the
same shape survives outside `hull` — `quad.rs`'s
`bspline_eval_ring(kv, coeffs, t)` / `bspline_eval_ring_in_span(coeffs,
span, t)` evaluators, `compose.rs`'s `to_bezier_spans(kv, coeffs)`, the
tensor nets beside two vectors (`TensorNet`, `quad.rs`'s grids,
`compose/tensor.rs`), and the knot-algebra plans' `(kv, weights)` — none
a hull door, each length-checked, none this unit's.

**Companion note:** `crates/geom-core/README.md` SPLINE-DESIGN S1 (the
"one pairing S1 leaves open" paragraph rewritten to the closed state;
`InteriorKnot` the deliberate exception), its row in `docs/DESIGN.md`'s
companion table. The spec is deleted and ledgered (`docs/DOC-LEDGER.md`,
"Per-merge deletion — PROPS coeffs' spec").

## Fix pass

The dual review's findings (APPROVE-WITH-FIXES, both arms) landed as
`props/coeffs-fixpass`. The pair is **two types**: `SplineCoeffs<'a, E>`
(knots + coefficients, minted by `KnotVector::with_coeffs`) carries the
nonrational doors and no other; `RationalCoeffs<'a, E>` (knots +
coefficients + weights, minted by `KnotVector::with_rational_coeffs`)
carries the rational doors and no other, `RationalWindow::hull_rational`
on its window — so a rational claim without weights and a nonrational
bound that would ignore weights are both unrepresentable (D2 row 0),
`compile_fail` rows (d) and (e) with twins, `E0599` read off rustc
1.97.0. Weight positivity stays per window. The triplicated
mint-then-difference helper (`net.rs`, `quad.rs`, `chords.rs`) has one
home, `KnotVector::difference_coeffs`, with the never-empty contract
stated once; `quad.rs`'s two spellings of the range hull are one. Mints
are verb phrases, the window's pair accessor is `pair()`; the six
silent accessors are cut to `SplineCoeffs::knots` and
`CoeffWindow::{pair, span, window}`. Doc rot and history phrasing fixed;
the body's false "exactly what the retired door answered" corrected
(the three arms are unreachable by construction and answer
equivalent-or-safer — empty chain, `None`, poison — stated at each
site). The sweep residue is filed as
`coefficient-vector-pairing-survivors`. Digests: the 960/480-row
coefficient digest and `geom`'s span digests unchanged; the dual's
3,403-row extended corpus and its type rows adopted
(`coeffs_bit_identity_ext.rs`, `coeffs_pair_identity.rs`). The
`docs/DESIGN.md` companion-row edit in #1985 was warranted and
undisclosed; disclosed in the fix-pass body. The territory tool's full
output for the fix pass is in that body (the unit's body under-reported
it: `quad.rs` is cert's until #1924, not PROPS').
