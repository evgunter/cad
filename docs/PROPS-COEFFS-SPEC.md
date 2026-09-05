# PROPS coeffs — coefficients carry their knot vector: the free `hull` doors move onto a window

**Binding at dispatch** (PROPS program; the item is
`work/props/coefficients-carry-their-knot-vector.md` — read it in full,
and `work/props/span-carries-its-knot-vector.md` (closed) whose ruling A
this unit applies one level down; difficulty logged at spec: **L**,
task-class **STRUCTURAL** — no arithmetic moves). Read
`docs/prompts/implementer-discipline.md` in full and
`crates/geom-core/README.md` §SPLINE-DESIGN S1. Branch
`props/coeffs-window`, cut from `main`.

## The ruling (the item's "shape of a fix", answered)

**Option (a), in the structural form the curve half took.** A
coefficient array is a proof about the knot vector it was fitted or
composed against, so it travels with that vector, and a span of that
vector is taken FROM the pair — never beside it. Concretely, in
`crates/geom-core/src/spline/hull.rs` (or a sibling module if `hull.rs`
is better left to the arithmetic):

- `SplineCoeffs<'a, E: CertifiedEnclosure> { knots: &'a KnotVector, coeffs: &'a [E] }`,
  minted ONLY by `KnotVector::coeffs(&self, coeffs: &[E]) -> Option<SplineCoeffs<'_, E>>`,
  `None` when `coeffs.len() != self.control_count()` — the one check the
  free doors carry today, done ONCE at the mint (D2-addendum row 1 at
  the mint; the doors themselves have no refusal left). The rational
  twin `KnotVector::coeffs_rational(&self, coeffs, weights) -> Option<..>`
  carries the weights beside (the same count check, and the weight
  positivity check `span_weights_positive` makes today — decide whether
  positivity is a mint-time refusal (`None`) or stays a per-span poison
  as now; state the reason at the door).
- `SplineCoeffs::span(index) -> Option<CoeffWindow<'a, E>>` and
  `span_at(t)` — the `CurveWindow` mints (`crates/geom/src/curves/nurbs.rs:666-690`)
  transcribed; `CoeffWindow` holds the `SplineCoeffs` and a `Span<'a>` of
  ITS vector, so "a span of another vector against these coefficients"
  has no spelling. Hand-written `Debug` printing the borrow as an
  address (the Span unit's item I).
- The span doors become `CoeffWindow` methods reading everything from
  the borrow: `hull()` (was `span_hull`), `hull_rational()`
  (`span_hull_rational`), `derivative_hull()` (`derivative_span_hull`),
  `sup_norm_bound()` (`sup_norm_bound_span`). The whole-domain doors
  that take `(kv, coeffs)` beside each other are the SAME relation one
  door over and become `SplineCoeffs` methods: `domain_hull()`,
  `domain_hull_rational()`, `derivative_coeffs()`,
  `derivative_domain_hull()`, `sup_norm_bound()`,
  `sup_norm_bound_rational()`. `span_indices` and `span_weights_positive`
  are deleted with the check they made. **No free function in `hull.rs`
  takes a coefficient array after this unit.**
- Bit identity is the receipt: every method body is the free function's
  body with the reads rerouted through the borrow — the same operations
  in the same order. Capture a digest at the merge base through the
  retired spellings (the Span unit's `span_bit_identity` shape: a corpus
  of non-rational and rational scalar splines, degrees 1–4, interior
  multiplicities, every span, both `f64`-bracket and `Interval` lanes,
  the derivative doors and the whole-domain doors), pin it, and show it
  unchanged at the head.

## Consumers (all of them; each mechanical)

- `crates/geom-brep/src/ssi.rs:~1063` `pcurve_windows` and
  `crates/geom-brep/src/ssi/certify.rs:~544,~636` — `coords[k]` from
  `ring_coords()` beside `kv = p.knots()`: mint one `SplineCoeffs` per
  coordinate channel OUTSIDE the span loop, take `.span(index)` inside.
  **TRIM's ground (Track Q, PCURVE P-2's successor) — announced by this
  spec; the orchestrator posts the seam.**
- `crates/geom-brep/src/props/quad.rs:~636` `bspline_range_hull` and the
  `derivative_coeffs` closures at `:~693,~1174,~1224` — PROPS'.
- `crates/geom-core/src/spline/net.rs:~311,~318` (`diff_u`/`diff_v`
  through `derivative_coeffs`) — PROPS' (geom-core).
- `crates/geom/src/curves/nurbs.rs:~1342` (the weights' derivative) —
  PROPS'.
- `crates/mesh/src/chords.rs:~240` — **MESH's; announced; the
  orchestrator posts the seam.**
- Tests: `crates/geom-core/tests/{spline_hull,span_hull_window}.rs`,
  `crates/geom/tests/curves/span_window_pairing.rs`,
  `crates/geom/tests/surfaces/span_window_pairing.rs` and every other
  citer (`grep -rn "span_hull\|domain_hull\|derivative_coeffs\|sup_norm_bound" crates/*/tests`).

## Posture

- Red-first: the four rows the item names as the residue's pins
  (`the_coefficient_pairing_is_still_length_only`,
  `the_free_hull_doors_relate_coefficients_by_length_alone`,
  `the_free_hull_doors_answer_every_shape_the_guard_refused`,
  `the_constructor_relates_the_net_to_the_vectors_by_count_alone`) go RED
  at the type level and are REPLACED: shapes (a), (b), (c) of the item
  become `compile_fail` doctests with legal twins (the Span unit's
  convention: the code named is the one `rustc` emits at 1.97.0, stated
  as not verified by stable rustdoc; the twin is the honest half), plus
  a row that every window a `SplineCoeffs` mints answers the same as
  the whole-domain door hulled over that span's window; the mint's
  length refusal keeps one behavioural row. The constructor row stays
  (the count relation at `NurbsCurve::new` is load-bearing and is not
  this unit's).
- ε posture: none. No `CI-Config:` trailer.
- D2-addendum: the retired `span_indices`/`span_weights_positive`
  poison paths — row 0 where the state is now unrepresentable (a span
  of another vector; a foreign-length array at a door), row 1 at the
  mint (`None`); classify each retired poison route and say which
  consumer read it.
- Sweep obligation (discipline §5): the shape is *a coefficient array
  beside a vector or a span it was not minted against* — every
  `(&KnotVector, &[E])` and `(&[E], Span)` signature in `crates/*/src`
  (grep both orders and `RingInterval` slices), and every consumer that
  builds coefficient arrays from a curve and hands them to a door with
  a different vector (`refined(carrier)` in `certify.rs` — the coords
  and the kv come from the SAME refined curve; say so). Hit list with
  dispositions; what the pattern cannot match. `InteriorKnot` is the
  family's third member and stays crate-private (CERT-N3's decision):
  one sentence at its doc naming `SplineCoeffs` as the second precedent.
- Territory: `python3 scripts/work.py territory --base origin/main`
  reported in the body with every path's owner; `geom-core/src/*` and
  `geom/src/*` are S-CERT's until the exit walk (#1924) merges — the
  PROPS-1/Span precedent applies (announced, mechanical, disclosed).
- Review: standard v6 dual (block PROPS-B2 slot 0; ordinal claims at
  review dispatch). Reviewers' first target: bit identity across both
  lanes and every door; second: whether any consumer's coefficients
  were in fact minted against a different vector than the span it
  used (the mint would now refuse it, or the type would) — a live
  mis-pairing found is a MAJOR finding about `main`, not about this
  unit, and is filed.
- Landing: the item `status: closed` with a `## Closed` section (the
  ruling recorded); the README's "one pairing S1 leaves open" paragraph
  rewritten to the closed state (present tense; the family is closed,
  `InteriorKnot` the deliberate exception); `docs/DESIGN.md`'s
  companion row untouched unless its one-line summary is now false;
  the spec deleted at merge with its `## Per-merge deletion` section in
  `docs/DOC-LEDGER.md`; no `Co-Authored-By`; state-sync as the last
  commit; push early to `props/coeffs-window`.

## Acceptance

No free function in `hull.rs` takes a coefficient array; `SplineCoeffs`
minted only through `KnotVector::coeffs*`; `CoeffWindow` carries the
pair; the three shapes are `compile_fail` with twins; the bit-identity
digest unchanged from the merge base on both lanes; every consumer on
the new doors; the README clause updated; hosted CI green on the full
matrix.
