---
id: kevs-fan-merge-needs-a-re-describing-kill-door
kind: issue
title: "kev's fan merge re-bases carriers and no precondition can refuse it: the kill needs a re-describing door"
status: open
opened: 2026-09-14
parent: S93
refs: [S93]
---

## What

`S93` asked both fan-rebasing operators to stop leaving a re-based edge
carrying a certificate against a point that is no longer its endpoint.
`Body::mev`'s fan site now does: it re-certifies the moved run in its
plan phase and refuses `EulerOpError::RebasedCarrier`. `Body::kev`'s
fan merge does not, and this row carries why, with the measurement.

## The defect, unchanged

`kev(he)` kills `end(he)` and hands that vertex's surviving fan to
`start(he)`. Every merged member keeps the `EdgeCurve` it was certified
with, which pins `carrier(t₀)` (or `carrier(t₁)`) to the DEAD vertex's
point. Where the two vertices' points differ, each merged edge
describes a locus that no longer ends where the edge does. Tier 1 does
not constrain it; tier 3 reports it at rest; `split_edge` and
`set_edge_curve` refuse on such an edge, and `seqgen`'s `split_site`
filter exists to route around exactly that (`crates/topo/src/seqgen.rs`,
the "Why the re-certification" paragraph — narrowed by S93 from two
causes to this one).

## Why the same gate could not land here

The gate that works for `mev` is a precondition: refuse before
mutating. Applied to `kev` it refuses the fan merge's live callers,
because they kill **mid-surgery** and re-describe the merged edges at
the end of the door, so the promise the precondition would need to see
does not exist yet at the call.

**The measurement, re-taken honestly by S93's fix pass.** The first
version of this row said `cargo test -p topo` was green with the gate
on `kev`. That was false, and both S93 reviews caught it; the commit
that claimed it wired the gate in a shape (`kev` taking a `Tol`) that
does not compile the crate's own tests, 76 call sites of one-argument
`kev`. Re-taken on `topo/s93-rebased-carriers` with the gate wired into
`Body::kev`'s plan phase as
`self.certify_rebased_run(&fan, self.resolve_vertex_point(v)?, Tol::witness())?`
— the row's own shape with the band taken internally instead of through
a signature change, so the tests compile:

- `cargo test -p topo`: **14 failures of 703** lib rows (`tests/all.rs`
  and the doctests stay green). Every one is
  `RebasedCarrier { … ResidualExceeded { check: EndpointStart | EndpointEnd } }`
  raised out of `kev`. They stand at FOUR distinct coordinates, and
  only one of them is a composite door:
  - `crates/topo/src/seqgen.rs`, `apply`'s `Kev` arm — the generator's
    **walk**: `generator_is_deterministic_for_equal_decisions`,
    `selection_is_pinned_over_a_fixed_stream_set`,
    `issue_60_kef_roundtrip_on_coincident_ring_twins`,
    `validate::tests::random_op_sequences_stay_tier1_valid_at_every_step`,
    `validate::tests::random_sequences_then_teardown_validate_vacuously`,
    `review_m1_pr4::seqgen_generates_every_op_kind_and_every_site_shape`;
  - `crates/topo/src/seqgen.rs`, `roundtrip`'s `SplitEdge` arm — the
    two-op inverse `kev` then `set_edge_curve`:
    `random_op_sequences_hold_all_properties`;
  - `crates/topo/src/seqgen.rs`, `teardown`:
    `teardown_handles_the_genus_one_acceptance_body`,
    `review_m1_pr4::genus_two_double_hole_body_tears_down_to_nothing`;
  - the fixtures that kill on a merged fan directly:
    `euler_kill::tests::kev_mirror_case_merges_fan_onto_the_valence_one_survivor`,
    `euler_kill::tests::isomorphic_smoke_check_for_the_module`,
    `euler_kill::tests::the_merged_fan_keeps_a_carrier_its_endpoint_left_and_split_edge_refuses_on_it`,
    `review_m1_pr4::kev_mirror_has_no_single_op_remake`,
    `review_m1_pr4::same_face_bridge_edge_kef_refuses_and_kev_kills`.

  So **the cost is at the composite doors AND inside `topo`'s own
  generator**, which kills fans in its walk, in its `split_edge`
  roundtrip inverse and in its teardown, each with no surgery scope
  open. Any shape that makes plain `kev` refuse has to answer the
  generator too.
- `cargo test -p sweep`: **129 failures out of 1394**, every one of them
  the fillet/blend verb refusing. 118 carried the new error, at exactly
  two sites: `site: "annulus closure kev"` (92) and
  `site: "rim closure kev"` (26), both in
  `crates/sweep/src/blend/surgery.rs`. The remaining 11 are downstream
  rows that build a fillet first. (Both S93 reviews reproduced this
  half exactly; it is carried forward unchanged.)
- Both sites sit inside `blend_surgery`'s surgery scope, which ends by
  running `attach_contact` over the `Described` list and then
  `topo::mint_pcurves`, and asserts `topo::validate_closed`. So the
  blend's OUTPUT is coherent and only its intermediate state is not —
  the door re-describes the merged edges after the surgery, and a
  precondition on the operator cannot see that it is going to.

Ordering does not rescue the caller either: any spec the merged edge
could carry BEFORE the kill must end at the vertex that is about to
die, and any spec it needs AFTER must end at the survivor, so no
sequence of today's doors expresses "merge this fan and re-describe
it". The same wall stands one level down: `split_edge`'s two-op inverse
(`kev` then `set_edge_curve`, `seqgen::roundtrip`) is the same shape.

## The landing shapes

Three, and the measurement above constrains each of them differently.
Deciding between them is a later unit's; what this row fixes is the
list and what each costs.

### (a) The surgery-scope switch

Refuse in `kev` when no scope is open, carry inside one, and
re-certify the moved runs at the scope's CLOSE. That is
`Body::tier1_sweep_is_mine`'s shape exactly
(`crates/topo/src/surgery.rs`): a door that has undertaken to sweep at
its end suppresses the per-op check, and the sweep at the close is what
answers for the intermediate state. Ev's ruling on PR 2305 is the
precedent — a composite door owns its intermediate state and pays at
the close.

**What the generator does to it.** Eleven of the fourteen `topo`
failures above are `seqgen`, with no scope open at any of them: the
walk (`apply`), the `split_edge` roundtrip inverse, and `teardown`. So
(a) makes the generator's kills refuse unless the generator opens a
scope around each of them — at which point the fuzz no longer sweeps
per-op, which is the property (a) rows exist to check
(`random_op_sequences_stay_tier1_valid_at_every_step`). It also needs a
new close-time sweep over "every run this scope re-based", which no
scope tracks today.

### (b) A tier-3 postcondition at the blend door

Leave `kev` alone and make `blend_surgery`'s close say what it already
believes: the output is coherent. Its closing
`debug_assert_eq!(topo::validate_closed(&body), Ok(()))`
(`crates/sweep/src/blend/surgery.rs`) is **tier 2**, one tier too low
to see a stale carrier at all — a carrier missing its own endpoint is
tier 3's check, and tier 2 passes it. Raising that one assertion to
the geometric tier turns the blend's re-description from a promise into
a checked one.

**What the generator does to it.** Nothing — and that is the limit:
the eleven `seqgen` sites are not behind any composite door, so plain
`kev` keeps the defect everywhere outside the blend, and `split_edge`
keeps refusing on merged edges. (b) reports; it does not prevent.

### (c) `kev_describing`

A kill that takes the merged fan's re-descriptions and certifies them
in the same step — `kev` with a `&[(EdgeKey, EdgeCurveSpec<T>)]`
alongside, or a named `kev_describing` — certifying each supplied spec
against the endpoints the merge WILL give the edge (through
`Body::certify_rebased_run`'s door, `geom_brep::EdgeCurve::recertify`),
refusing typed before any mutation, and writing the topology and the
descriptions together. Plain `kev` then keeps the merge only where
every merged member's carrier already passes the gate, and the two
blend sites pass the carriers they already compute for
`attach_contact`.

**What the generator does to it.** `roundtrip` and `teardown` would
have to supply a spec per merged member, and the spec they would need
is a re-description of a chord onto a different vertex — the derivation
the operator refuses to make and `set_edge_curve` exists to take from a
caller. So the generator keeps plain `kev` and keeps its skip, and (c)
buys the composite doors without buying the fuzz.

### Smallest

**(b)**, by a wide margin: one assertion's tier at one door, no public
surface, no caller change, nothing for the generator to absorb. It is
also the weakest — it makes the blend's existing promise checkable and
leaves plain `kev` exactly as it is. **(a)** is the only one that makes
the defect unreachable through plain `kev`, and its price is the
eleven `seqgen` rows above plus a scope that tracks re-based runs.
**(c)** is the largest: a new public door and two other programs'
callers.

## Receipt — every site that re-bases a half-edge run

Swept two ways, at the head `522b3fe57`.

**Pattern 1**: `MevSite::Fan` with `he1 != he2`, and every `.kev(`
call. 273 `MevSite::Fan` occurrences in the tree — **221 struts, 25
runs, 17 shorthand/prose, 10 doc lines** — and every one of the 25 run
sites is a test or a review probe in `topo`. (The original sweep said
267 at `42bfd53a0`; the count was not re-taken across the
merge-forward, and is 272 at the merge base `8b71ce295`.) The three
kernel run sites hide in the shorthand group — `splitting/insert.rs`,
`boolean/insert.rs`, `boolean/vtxfac.rs` — and all three go through
`mev_null`.

**Pattern 2**: a re-basing written as a direct `half_edge.start = …`
write, which pattern 1 cannot match. The original receipt claimed the
sweep "found none outside `mev_fan_execute` and `kev`". That is false;
`rg '\.start = '` finds nine more, and here is the hit list with its
verdicts:

| hit | verdict |
|---|---|
| `crates/topo/src/euler.rs`, `mev_fan_execute` | the S93 gate's own site |
| `crates/topo/src/euler_kill.rs`, `kev` | this row |
| `crates/topo/src/split.rs`, `split_edge` | **re-certifies** — the parent's minus half moves to the new mid vertex, and `certify_edge_spec` certifies both children before any mutation |
| `crates/topo/src/revert.rs`, the reversal map | **inert** — every half-edge is re-based to its own OTHER end, paired with the `he_plus`/`he_minus` and `next`/`prev` swaps below it; the edge's two endpoints are the same two points and only the traversal direction reverses (the module's bitwise-involution argument) |
| `crates/topo/src/boolean/combine.rs`, the key remap | **inert** — `map(&vertices, he.start)` rewrites a key into the destination arena; the same vertex, a different key, no point moves |
| `crates/topo/src/merge_faces.rs`, `tear_before_kev` | **test-only** (`#[cfg(test)]`) — an armed tear that deliberately corrupts before a kill |
| `crates/topo/src/review_d18.rs`, `crates/topo/src/review_m1_pr5_internal.rs`, `crates/topo/src/validate.rs`, `crates/topo/src/review_m1_pr2/atomicity.rs` (×2), `crates/topo/src/review_m1_pr3.rs` | **test-only** — deliberate corruption injections that tier 1 must then refuse |
| `crates/geom-brep/tests/review_m2_pr7_props.rs` (×2) | not a half-edge — a loop-sample struct's own field |

What pattern 2 still cannot match: a re-basing that writes through a
helper rather than the field (`link_half_edges` and friends re-link but
do not re-base, checked by reading each), and a re-basing expressed as
a whole-body rebuild (`revert` and `combine` are the two, both above).

| site | re-certifies / carries / refuses |
|---|---|
| `mev` fan (`euler.rs`, `mev_fan`) | **re-certifies and refuses** — the S93 gate, narrowed to what the move itself breaks |
| `mev_null` fan (`null.rs`) | **carries** — the new point is `plan.p_old`, a bitwise copy, so no endpoint moves and no certificate is touched; structural, no comparison |
| `kev` fan merge (`euler_kill.rs`) | **carries, unchecked** — this row |
| `kemr` (`euler_ring.rs`) | nothing to re-base: it kills an edge and re-anchors loops, no half-edge changes its start vertex |
| `split_edge`'s children (`split.rs`) | **re-certifies** — both children certify against the new endpoints before any mutation (`certify_edge_spec`), and the carried pcurve rows with them |
| `merge_coplanar_faces`' absorption (`merge_faces.rs`) | **refuses or carries** — its only vertex-killing step is `kev(from_rim)` behind `strut_tip`, i.e. a valence-1 far vertex, so no fans merge and no carrier moves |
| `revert` (`revert.rs`) | **inert** — see pattern 2 |
| `boolean::combine` (`boolean/combine.rs`) | **inert** — see pattern 2 |
| `kfmrh` / `ring_move` (`euler_ring.rs`) | out of scope here: they re-parent LOOPS between faces, not half-edges between vertices (TOPO-B3 slot 0's subject) |
