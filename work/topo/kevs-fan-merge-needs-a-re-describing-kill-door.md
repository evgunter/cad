---
id: kevs-fan-merge-needs-a-re-describing-kill-door
kind: issue
title: kev's fan merge re-bases carriers and no precondition can refuse it: the kill needs a re-describing door
status: open
opened: 2026-09-14
parent: S93
refs: [S93]
priority: P1
cost: H
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

## For Ev, on the open `[ev]` PR (TOPO, 2026-09-14)

The three landing shapes above are a choice about what a kill
operator owes: (a) makes plain `kev` refuse outside a surgery scope and
lets a door pay at its close (Ev's PR-2305 shape), at the cost of the
generator's eleven kills opening scopes or re-describing; (b) reports
at the blend door and leaves plain `kev` with the defect; (c) adds a
re-describing kill door and leaves plain `kev` refusing where carriers
would go stale. TOPO recommends **(a)**: it is the shape the kernel
already chose for tier 1's postcondition, it makes every public
operator honest, and the generator's cost is the generator's — it
was producing stale-carrier bodies all along and filtering around
them. Counter-argument: (a) needs a scope to track "every run this
scope re-based" and a close-time sweep over them, which no scope
carries today; (c) is smaller and keeps the generator green but leaves
two doors for one kill.

## Why (a) over (c) — elaboration for Ev (TOPO, 2026-09-14, PR 2527)

The obligation both shapes carry: after `kev` merges two fans at one
vertex, the surviving edges' carriers were certified against endpoints
that no longer hold, and something must re-certify them before the
body is observed. The two shapes differ in WHERE that obligation
lives.

**(c) puts it on the operator call.** A second public door
(`kev` that re-describes, taking or deriving the new carriers) beside
plain `kev`, which refuses where any carrier would go stale. Costs:
two public kill doors for one topological operation, and every caller
decides at the call site which one — a decision that needs exactly
the "does a carrier go stale here" computation the operator already
performs, so the caller either always takes the new door (plain `kev`
becomes dead weight) or duplicates the gate. The blend kills many
edges mid-surgery and re-describes ONCE at its door's end today; under
(c) it re-describes per kill, or is handed a door whose semantics are
"carry now, I will fix it later" — which is (a) without the scope that
holds the promise. The generator's fourteen rows switch doors either
way. And it is the two-homes class: two doors, one reading of the same
obligation.

**(a) puts it on the boundary the kernel already has.** A body is
observable at a door's close, and the surgery scope IS that boundary
(D1, PR 2305: tier 1's postcondition paid once per door at the scope's
close; the blend already opens one scope for the whole blend —
`crates/sweep/src/blend/surgery.rs`, "One surgery scope for the whole
blend"). Plain `kev` outside a scope refuses typed where a carrier
would go stale; inside a scope it carries and RECORDS the re-based run
on the scope; the scope's close re-certifies every recorded run and
refuses typed if one fails. One kill door. The blend pays once at its
close — what it does by hand today, made a mechanism. The generator's
kills either open a scope (test support) or accept the refusal, and
its cost is its own: it was producing stale-carrier bodies and
filtering around them. Cost of (a): the scope grows a list of
re-based runs and a close-time sweep over it. That list is the same
mechanism question 4's enforcement wants (a scope-close mint of
missing pcurve rows), so one scope-close obligation list serves both
rows. What holds the scope closed is the RAII guard (D9), not a
convention.

**(b)** reports at the blend door only and leaves plain `kev` with the
defect; smallest and weakest.

## Second elaboration for Ev (TOPO, 2026-09-14, PR 2527): (c) as the default `kev`

Ev: "if we can make the variant in (c), why can't we just make that
the default one?" — the definition of (c) is `kev_describing` above: a
kill that takes the merged fan's re-descriptions alongside the
half-edge, certifies each supplied spec against the endpoints the merge
WILL give its edge (through `certify_rebased_run`'s door), refuses
typed before any mutation, and writes topology and descriptions
together.

**Made the default it is one door**: `kev(he, redescriptions)` — for
each merged member, certify the supplied spec if one is given, else
re-certify the member's existing carrier against its new endpoint;
refuse typed if either fails; write together. An empty list is the
common case (every merged member's carrier still ends where the edge
does — the valence-one survivor, coincident points). That door has no
intermediate stale state at all: the obligation is discharged at the
call, which is the `mev` gate's own shape (refuse before mutating) and
D9 row 0's spirit (the stale-carrier state cannot be produced by a
door). No scope list, no close-time sweep.

**What (a) offered over it was deferral** — a composite door carrying
stale carriers mid-surgery and paying at its close — and the
measurement above says who would use it: the blend, and the
generator. The blend does not need it: its two `kev` sites already
compute the carriers they later hand to `attach_contact`, so under the
default they hand them to the kill instead. The generator's three
sites (walk, `roundtrip`'s `SplitEdge` inverse, `teardown`) have no
specs — but the roundtrip inverse is already the two-op `kev` then
`set_edge_curve` with a chord spec, which becomes one call; the walk
and teardown kills at distinct coordinates either supply the same
chord spec or keep the `split_site`-style filter they have today. So
the only caller that wanted deferral can pay at the call too, and (a)'s
extra mechanism buys nothing (a) alone needs. The earlier
recommendation leaned on a deferral no caller requires; Ev's question
is right.

**Revised recommendation: (c) as the default and only `kev`**, with
the signature change's cost stated: 76 one-argument `kev` call sites
in `topo`'s own tests plus the blend's two and the generator's three
(the row's own measurement), which is churn rather than design. The
fuzz's "tier 1 at every step" property is unaffected — it never
depended on carriers.

## Third elaboration for Ev (TOPO, 2026-09-14, PR 2527): the default, and signature harmony

Ev: "i didn't realize it would need a default... that's not great.
also how harmonious would the type signature there be with the other
operations?"

**The signatures as they stand.** The make-operators take a site, the
geometry of what they make, and a band: `mev(site, point, curve, tol)`,
`mev_line(site, point, tol)`, `mev_null(site, …)`, `mef(…,
FaceSurface, tol)`, `mekr(site, …, tol)`. The kill-operators take keys
only: `kev(he)`, `kef(he)`, `kemr(he1, he2)`, `kfmrh(f1, f2)` — a kill
creates no geometry, so it carries none. `kev`'s fan merge is the one
kill that CHANGES geometry (it re-bases every merged edge's endpoint),
which is exactly why it is the one kill with something to describe.

**On the default.** The list has no default in the valence-one case —
the merged fan is empty, so the list is exactly empty — but in every
other case an empty list means "keep every carrier and re-certify it",
which is a default in all but name. A total form (every merged member
listed as `Keep` or `Redescribe(spec)`, refused if one is missing)
removes the default at the cost of the common caller spelling out
`Keep` per member. Neither is the kill family's shape.

**The shape that keeps the family harmonious is the original (c): two
doors.** `kev(he)` stays keys-only like every other kill and REFUSES
typed, before mutating, where any merged carrier would go stale — the
S93 gate inside the kill, naming the members in its refusal — so plain
`kev` never produces the defect and never needs a list; and
`kev_describing(he, &[(EdgeKey, EdgeCurveSpec<T>)], tol)` is the kill
that takes geometry, shaped like `mev` (a spec and a band) for the
callers that re-describe (the blend's two sites, the generator's
roundtrip inverse). That is the variant-family pattern one operator
already has: `mev` / `mev_line` / `mev_null` are one operator with
three doors differing in the geometry argument. No default anywhere;
the kill family stays keys-only; the describing door reads like the
make-operator it mirrors.

**Cost.** The two blend sites and the generator's three switch to the
describing door or accept the refusal (the row's measurement); no
other caller's signature moves. **Recommendation: (c) as two doors**
— which is (c) as written above; the "default" reading was mine, not
the shape's.

## Ruled (2026-09-14, PR 2527)

Ev: "(c) sounds good then!" — two doors. `kev(he)` stays keys-only
and refuses typed, before mutating, where any merged carrier would go
stale, naming the members; `kev_describing(he, &[(EdgeKey,
EdgeCurveSpec<T>)], tol)` takes the merged fan's re-descriptions,
certifies each against the endpoint the merge will give its edge,
refuses typed before any mutation, and writes topology and
descriptions together — the `mev`/`mev_line`/`mev_null` variant-family
shape. The blend's two sites and the generator's roundtrip inverse
take the describing door; the generator's other kills supply a chord
spec or keep their filter. Kernel answer: a block slot; this row is
now the unit, and `S93` closes with it.
