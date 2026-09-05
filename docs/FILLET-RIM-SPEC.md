# FILLET-RIM — `rim_of`, the rim selector (spec)

**Program:** FILLET (`work/fillet/plan.md`), unit `no-public-rim-arc-selector`
(`work/fillet/no-public-rim-arc-selector.md`; Ev's ruling on PR 1735: option
1). **Track:** kernel change — the standard v6 unit (binding spec, drawn
implementer arm, cross-model dual review, union fix pass, record-at-merge;
§Review below). **Pre-draw fields, logged before the draw:** difficulty **S**,
task-class **STRUCTURAL**.

- **S** — one door, one error enum, one module, consumers already written
  against the shape (`sweep::test_support::rim_arcs_at` and its callers).
- **STRUCTURAL** — the door reads stored tags and stored carriers EXACTLY
  (§Phase 1 is what makes that honest); no band, no margin, no funnel. If
  Phase 1 shows a band is needed the unit STOPS and is re-logged.

**Territory note.** `crates/topo/src/query.rs` is SEAT's (the kernel query
seat, `docs/VERB-SEAT-DESIGN.md` §1). FILLET enters it by announced seam
(`work/seat/log.md`); the door follows the module's own conventions and
adds no vocabulary beyond itself.

## The claim

**A rim is named by any one of its arcs.** Every consumer that fillets a solid
of revolution needs the rim WHOLE — every arc a chart seam split it into, no
more (a co-surface seam meridian refuses `TangentialEdge` at margin zero) and
no less (a strict subset stops at a seam vertex and refuses `SeamVertex`) — and
every consumer hand-rolls the same scan to get it. The door:

```rust
pub fn rim_of<T: Real>(body: &Body<T>, edge: EdgeKey) -> Result<Vec<EdgeKey>, RimError>
```

returns the closed rim the given arc belongs to: every edge of `body` whose
certified carrier is the SAME circle and whose two sides lie on the SAME TWO
SURFACES (surface keys, so several faces of one surface across chart seams
count as one side), in carrier order starting at `edge` and running in the
carrier's positive parameter direction (D9: deterministic, and
`rim_of(b)` is a rotation of `rim_of(a)` for any two arcs of one rim — a row
asserts it). Co-surface seam meridians can never match: their two sides are
one surface, and a rim's are two.

Refusals, typed, in `RimError` beside the door:

- `NotAnArc { edge, kind }` — the seed's certified carrier is not a circle
  (or there is no certified carrier: say which).
- `CoSurface { edge, surface }` — the seed's two sides are one surface: a
  seam meridian, not a rim edge.
- `NotOneRim { arcs, gap }` — the matched arcs do not tile the full circle
  without gap or overlap and shared vertices between consecutive arcs (the
  partial revolve's open rim is the honest instance; `gap` names the
  parameter at which the tiling fails). A partial set is never returned.
- `NotIntact(EntityId)` — a dangling key or an unreadable reference on the
  way; the `sweep::blend` `not_intact` shape.

Nothing here decides: same circle means the stored carriers' `center` and
`radius` bit-equal and `axis` equal or negated (the same set of points);
same surfaces means equal `SurfaceKey`s. That is a tag read, the module's
EXACT class ("total tag reads, no funnel, no margin"), and it is honest
exactly when the bodies consumers hold store one rim's arcs on one carrier
— which Phase 1 measures rather than assumes.

## Phase 1 — measure before touching anything

For each body class a consumer actually holds, take every rim and read its
arcs' stored carriers and side surface keys; record in the PR body whether
the arcs are bit-equal on `center`/`radius`, whether `axis` agrees up to
sign, whether `u_ref` agrees (it need not; say so), and whether the side
surface keys pair up:

- a full revolve, seam-split rims (`test_support::revolved_about_y`, the
  waisted lantern of `blend_seam_split_rim.rs:539`, the lily lantern in
  `demos/tour`);
- the same after `merge_coplanar_faces` (the repaired pole body,
  `blend1_r1_probes.rs::p4_…`);
- one-edge rims (`test_support::dome`, `sphere_zone`);
- boolean-made rims (the die's pips, `m5_pr12_die.rs`; the pierce ring
  fixtures) — carriers minted by the boolean, not by a revolve;
- extrude's hole rims (`extrude.rs`, a cylinder through a plate).

**Stop clause.** If any class stores one rim's arcs on carriers that are NOT
bit-equal, the exact door would refuse a real rim `NotOneRim`, and the door
needs a decided comparison with a band — a different unit (NUMERIC, a
`Margin`, a funnel row). Stop at the report; file the class as an issue
naming the producer that mints unequal carriers (that producer is the defect
the corpus should not have, and the finding is the unit's yield).

## Phase 2 — the change

1. `rim_of` and `RimError` in `crates/topo/src/query.rs`, exported where
   the module's other doors are; doc comment states the contract above,
   present tense.
2. `sweep::test_support::rim_arcs_at` (`test_support.rs:211`) becomes a
   fixture-selection scan that finds ONE arc at the radius and station and
   returns `rim_of` of it — same signature, so its callers (six suites) do
   not change; its doc loses the "no public door" sentence. Any other
   hand-rolled scan `rg -n 'Curve3::Circle' crates/*/tests demos --type
   rust` finds that selects a rim by radius/centre becomes a call — hit
   list and disposition in the PR body.
3. `FILLET3_SEAM_VERTEX_RECOURSE` (`crates/sweep/src/blend/mod.rs`) names
   the door: "request the rim whole — `topo::query::rim_of` on any one of
   its arcs hands you every arc the seam split it into". The composed pin
   that follows this recourse (FILLET-E2's row and
   `review_blend1_r2_probes.rs::the_seam_vertex_recourse_is_true_at_every_site_the_tag_fires`)
   follows it THROUGH the door.
4. Rows (a new suite in `crates/topo/tests/`, plus the consumer rows above
   in `sweep`): the seam-split rim returns both arcs from either seed, in
   the stated order, and the two results are rotations of each other; the
   one-edge rim returns `[edge]`; the repaired pole body's plane-hosted rim
   returns both arcs (faces differ, surfaces do not); a seam meridian seed
   refuses `CoSurface`; a straight edge refuses `NotAnArc`; a partial
   revolve's rim refuses `NotOneRim` with the gap at the wedge's end; a
   dangling key refuses `NotIntact`; the result feeds `fillet_edges` and
   carves (the end-to-end row the item was filed for); one row at
   `Interval` (`--features interval`). Determinism: two calls, identical
   `Vec`s.
5. `pncad-py`: if the bindings expose `topo::query`'s doors
   (`rg -n 'edge_adjacent_matches|all_edges' pncad-py`), `rim_of` joins
   them with a Python row; if not, out of scope and said so.

## Constraints, binding

- No band, no margin, no sampled geometry — an exact door or the stop
  clause.
- No other `topo::query` door changes; no `names/` vocabulary (a rim's own
  NAME is SEAT's / Track V's question, not this unit's).
- Every consumer keeps its claim by name; the six suites' rows are unchanged
  in what they assert.

## Acceptance

The Phase 1 table; the door with its four refusals, each with a row; the
consumer sweep's hit list; `rim_arcs_at` a call; the recourse sentence
followed through the door by the composed pin; hosted CI green, which since
2026-09-04 gates both compile modes on every run with nothing asked for — the
`CI-Config: lane=interval` this clause used to require is deleted; count
twelve `test (…)` jobs and say so.

## Out of scope

Naming rims (`names/`); the fillet's own doors; `merge_coplanar_faces`; any
decided comparison of carriers (the stop clause's issue).

## Review

v6 dual on the frozen head, claims to falsify:

- **C1** The door is exact: no decide, no band, no float comparison other
  than bit-equality, anywhere in it (read the code; grep the module).
- **C2** Phase 1's table is true of the tree: re-read one body per class.
- **C3** `rim_of(b)` is a rotation of `rim_of(a)` for every pair of arcs of
  every rim in the corpus fixtures; the order is the stated one.
- **C4** Every refusal is reachable through the public door with a real
  body (execute each); none returns a partial set.
- **C5** The recourse sentence, followed literally through the named door,
  carves (execute it on a convex seam-split rim).
