# BLEND-7 — a closed chain's junctions are judged against the links that touch them (spec)

**Program:** BLEND (`work/blend/plan.md`, unit 7). **Item:**
`work/blend/closed-chain-junctions-pair-with-a-rotated-link.md`.
**Track:** kernel change — the standard v6 unit (binding spec, drawn implementer
arm, cross-model dual review, union fix pass, record-at-merge; §Review).
**Pre-draw fields, logged before dispatch (the block was drawn 2026-09-07;
this unit took slot 1 by the 2026-09-08 reorder, recorded branch-side):**
difficulty **M**, task-class **STRUCTURAL**.

- **M** — the defect is a positional convention that drifted (one list built
  in walk order, read in ring order) and its fix is small; what the unit is
  paid for is the ground it opens: no suite has ever built a closed rim of
  more than two links, so the two closed-rim doors on N ≥ 3 crossings are
  unmeasured and Phase 1 may find a second refusal behind the first.
- **STRUCTURAL** — no numeric decision; a pairing invariant, rows that build
  three- and four-arc rims, and the `arcs.len() == 2` premise swept.

## The claim

**Every junction of a chain is judged between the two links incident to it,
and today a closed chain of three or more links is not.**
`crates/sweep/src/blend/battery.rs::walk_chains` (`:1196`) grows a run from a
seed forward through junction vertices, then backward, recording
`joints_fwd` and `joints_back`, and hands the chain
`junctions = joints_back.reversed() ++ joints_fwd` (`:1276`–`:1277`). When the
run closes, the closing vertex is recorded by whichever direction met it:
for a seed link `v0→v1` in a three-link rim the forward walk records
`[v1, v2]` and closes at `v0`, the backward walk then finds both links at `v0`
used and records `v0` as a back-joint, so the list reads `[v0, v1, v2]`
against the ring `[0, 1, 2]`. The G1 check (`:1408`–`:1412`) pairs
`junctions[i]` with `ring[i]` and `ring[i+1]`, i.e. `v0` with links 0 and 1 —
but `v0` sits between links 2 and 0. Every junction is then judged against
one link that does not touch it, and `pick` (`:1425`–`:1439`), choosing the
carrier end nearer the vertex, reads that link's FAR end. A two-link rim is
immune because both links touch both vertices, and two links is the only
closed rim any suite builds.

Measured (unit 1's review, PR 2123): the pristine three-arc cylinder's whole
raised rim refuses `ChainNotG1` with margin 0.75 at arm 0.866 (`sin 120° ·
chord`) through `run_battery` and `fillet_edges`; the two-semicircle rim of
the same body builds. Characterization row on main:
`crates/sweep/tests/review_blend1_r1_probes.rs::r1_a_three_arc_rim_refuses_chain_g1_at_a_junction_where_a_two_arc_rim_builds`.

**Ratified and not re-litigated:** the chain walk's structural rule (exactly
two requested links at a vertex is a junction; anything else terminates —
`walk_chains`' doc, C8), the predicate `fillet3_chain_g1` and its band, the
self-closed single-link arm (`:1447`–`:1462`), README A3-2's annulus with
`SeamCrossing`s and BLEND-1's closure index.

## Phase 1 — measure before touching anything

1. **The pairing, as data.** With the G1 check instrumented (locally,
   uncommitted), dump for the three-arc rim and a four-arc rim (a circle
   authored as four quadrant arcs, extruded) each junction vertex with the
   two links the check paired it with and whether each is incident; and the
   same for a two-arc rim and for an OPEN three-link chain (three box edges
   in a row — the open case must already pair correctly, show it). Table
   in the PR body.
2. **Past the check.** With the pairing corrected locally, run the whole
   three-arc rim and the four-arc rim through `fillet_edges` at radius 0.1
   on the raised rim of the extruded disc: which door `resolve_rim` reaches
   (the rim is the cap's whole outer cycle and the walls are several faces
   of one cylinder — the annulus with `SeamCrossing`s), whether it carves,
   the tier-3 verdict, the census before/after, the volume against the
   plane–cylinder closed form with `volume_pad`, and every refusal met with
   its site. Both material sides (the disc's raised rim is convex; a
   three-arc bore through a block is the concave twin). **This table is the
   unit's first deliverable and decides its shape.**

**Stop clause.** If the annulus door refuses an N ≥ 3 rim on a gate that is
about a different property (a seam-crossing walk that assumes two crossings,
a closure index that cannot hold three), stop at the report, file it, and
the orchestrator re-scopes — the pairing fix still lands alone with the
carve pinned as far as it goes.

## Phase 2 — the change

1. **The pairing carries no positional convention.** Preferred: `Chain`
   records each junction WITH the two link indices the walk found incident
   to it (`(VertexKey, usize, usize)` or a small struct), and the G1 check
   reads those; the `junctions: Vec<VertexKey>` field and its doc
   (`:304`–`:307`) change shape, and the one other reader (`mod.rs:938`, a
   doc sentence) follows. Acceptable alternative: keep the list and fix the
   order so `junctions[i]` sits between `ring[i]` and `ring[i+1]` for
   closed AND open chains, with the invariant stated at the field and a
   `debug_assert!` in the check that both paired links are incident to the
   vertex — so the next drift is loud rather than a wrong verdict. Say
   which you took and why.
2. **Rows**, in `crates/sweep/tests/blend7_closed_chain_junctions.rs`
   (aggregated via `tests/all.rs`; fixtures from `test_support` — home a
   `disc_of_arcs(n, r, h)` builder there if none exists): the three- and
   four-arc rims carve tier-3 valid at the closed form with `volume_pad`
   stated; the concave twin; an open three-link chain's junctions unchanged
   (bit-identical carve); the two-arc rim bit-identical; a row that would
   have caught the defect had it existed — every junction of a walked
   chain is incident to both links it is paired with, over every fixture
   the suite builds (the pairing exposed through `test-support` or
   asserted through the carve's success).
3. **The `arcs.len() == 2` premise** across the closed-rim suites
   (`fillet_h5_hostless_rim.rs:191, :351`, `blend_seam_split_rim.rs:255,
   :334, :401`, and whatever `rg -n 'len\(\) == 2' crates/sweep/tests`
   finds on rim arcs): each is either a genuine fixture fact (a full
   revolve splits at one seam — say so at the site) or a premise that
   excluded this defect (then the row gains the N ≥ 3 twin or its doc
   says which shape it pins). Table in the PR body.
4. **The characterization row** on main
   (`review_blend1_r1_probes.rs::r1_a_three_arc_rim_refuses_chain_g1_…`)
   flips to the carve: rename it to its claim, keep its body as the pin.
5. **The mutant**, in the PR body: restore the rotated pairing and show the
   N ≥ 3 rows red and the two-arc and open-chain rows green.

## Constraints, binding

- **Every carve that builds today is bit-identical to the merge base**
  (`bitdump.rs` differential, both SHAs). Two-link rims and open chains do
  not move.
- **No new predicate; `fillet3_chain_g1`'s margin and lever are unchanged.**
  What changes is WHICH tangents it is handed.
- **The walk's structural rule is untouched**; only the record it hands
  the check.

## Acceptance

- The Phase 1 pairing table and the door table.
- Three- and four-arc rims carving on both material sides at the closed
  forms; the pairing invariant rowed; the premise table.
- The mutant table; the bit-dump differential clean; hosted CI green.

## Out of scope

The annulus door's own limits on N ≥ 3 crossings beyond what Phase 1
measures (a refusal there is filed, per the stop clause); the ladder rim
(unit 8); the seam-vertex tag (A3-2).

## Review

v6 dual on the frozen head; claims to falsify (verbatim to both reviewers,
with `docs/prompts/reviewer-style-lane.md` by path):

- **C1** Every two-link rim and open chain carves bit-identically to the
  merge base (re-run the differential).
- **C2** The three- and four-arc rims carve tier-3 valid at their closed
  forms (re-derive the plane–cylinder closed form; the row's derivation is
  the implementer's).
- **C3** After the change, no junction of any chain the suites build is
  paired with a link that does not touch it (instrument it yourself; then
  construct a chain shape the suites do not build — a five-arc rim, a
  closed chain seeded from a different link — and check the pairing on it).
- **C4** The mutant reds exactly the N ≥ 3 rows.
- **C5** The premise table is complete: find an `arcs.len() == 2` (or an
  equivalent two-crossing assumption in the surgery's annulus walk) the PR
  did not list.
