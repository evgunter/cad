# BLEND-8 — the ladder rim phase retires only a source key (spec)

**Program:** BLEND (`work/blend/plan.md`, unit 8). **Item:**
`work/blend/ladder-rim-phase-may-retire-a-new-split-key.md`.
**Track:** kernel change — the standard v6 unit (binding spec, drawn implementer
arm, cross-model dual review, union fix pass, record-at-merge; §Review).
**Pre-draw fields, logged before the draw:** difficulty **S**, task-class
**STRUCTURAL**.

- **S** — the fix is the `split_rim` guard shape already in the same crate
  (`blend/open/ruled.rs:298`–`:340`), applied at one site. What the unit is
  paid for is Phase 1: whether a public door reaches the orientation at all,
  measured across every ladder fixture in the tree, so the row that pins the
  fix is a real body and not a story.
- **STRUCTURAL** — no numeric decision; a key-provenance rule and a census.

## The claim

**`Retired` names source keys, and the ladder rim phase can violate that.**
In `crates/sweep/src/blend/surgery.rs::rim_phase`, step (2) (~`:2515`) splits
each rim vertex's meridian `m` and names the piece still touching the rim
vertex the UPPER remnant; step (6) (~`:2746`) retires that upper piece with
`rec.dead.edges.push(mr)`. `Body::split_edge` (`crates/topo/src/split.rs:145`)
keeps the parent key on one of the two pieces by a rule that depends on the
edge's stored direction, so when the parent key stays with the LOWER piece the
upper one is `created.new_edge` — a FRESH key — and the phase pushes a fresh key
as a retirement. The totality walk's direction (b) ("every retirement names a
SOURCE key", `test_support::assert_naming_totality`) reads it as a violation,
and `editor_core::names::emit_blend` (`:242`–`:254`) builds a retired-set entry
no row can match. Every shipped row runs the other way round — every
revolve-minted meridian's `he_plus` starts at the rim vertex — so the branch is
UNMEASURED, not known-dead.

The ruled band's `split_rim` already handles the same shape: it reads the
survivor's provenance off `meridian_remnants`, records the survivor as a
fragment of the ORIGINAL source, and retires the source key ONLY when the
dying piece IS the source (`if near == source { rec.dead.edges.push(source) }`).
That is the fix shape here, and the unit's claim is that the ladder phase then
records the same facts whichever way the meridian runs.

**Ratified and not re-litigated:** `BlendNaming`'s vocabulary
(`blend/naming.rs`: `meridian_splits`, `meridian_remnants`, `dead`, what a
retirement IS); the document layer's reading of it (`emit_blend.rs`'s module
docs); the ladder walk's steps (1)–(6) otherwise.

## Phase 1 — find the orientation, or show no public door reaches it

Two questions, both answered by measurement and reported as a table in the PR
body before any code changes.

1. **The census.** For every ladder fixture the tree carves today — the die
   pips (`m5_pr12_die.rs`, `slab ∖ ball`), the `slab ∪ ball` boss
   (`fillet_h4_concave_rim.rs`), the dome (`test_support::dome`), the
   `sphere_zone` rim pair, the repaired boss's dome rim (unit 6's fixture;
   use it at the merge base as a body even if its carve refuses there),
   `hemisphere_on_flat_base`, `bowl`, `domed_cavity` — record, at each rim
   vertex, whether the meridian's `he_plus` STARTS at the rim vertex or ends
   there, and which piece `split_edge` keeps the parent key on. Read the
   rule off `split.rs`, not off the outcome. A fixture where `he_plus` ends
   at the rim vertex is the witness and Phase 2 uses it.
2. **If the census finds none**, try the doors that could mint one, in this
   order, and record each: (a) the boolean with the ball placed so the pole
   points the other way (a pip cut from BELOW a slab's underside; a boss
   grown downward) — the boolean re-mints the sphere's seam meridian and its
   direction is the sphere's own; (b) `revolved_about_y` of the pole-touching
   profile authored in the OTHER traversal order, and a partial revolution
   with a negative sweep — the lowering's meridian direction is the
   profile's; (c) `sphere_zone` with its bore on the other side. Stop at the
   first witness. If none of (a)–(c) produces the orientation, say what
   fixes the direction at each door (cite the line) — that is the finding,
   and the orchestrator decides whether the branch is unreachable by
   construction (then the fix lands with a `debug_assert!` stating the
   invariant and NO fixture is faked through a private path).

Put the census and the door table in the PR body. **This is the unit's first
deliverable.**

## Phase 2 — the change

1. **Step (2)** records provenance the way `split_rim` does: the survivor
   (lower piece) is a fragment of the source; if the meridian `m` was itself
   already a fragment (a second carve on the same cap — the ruled band's
   case; say whether the ladder can meet it and pin the answer), the source
   is read off the existing `meridian_remnants` row and that row retired.
2. **Step (6)** retires the source key only when the dying upper piece IS the
   source; a minted piece that dies gets no retirement row. Keep the
   `remnants` tuple carrying the source alongside so step (6) can tell.
3. **One spelling.** If the guard is now written twice (`split_rim` and here),
   hoist it into one helper in `surgery.rs` (or a shared module) that both
   phases call, with the `split_rim` doc paragraph moving to it. Two copies
   of a provenance rule is the drift the style lane exists to catch; say in
   the PR which way you went and why.
4. **Rows**, in `crates/sweep/tests/blend7_ladder_split_key.rs` (aggregated
   via `tests/all.rs`; fixtures from `test_support`): the witness carves
   tier-3 valid at its closed-form volume, and `assert_naming_totality`
   passes on it in all three directions; a row driving the naming through
   the document layer (`editor_core` — the `emit_blend` acceptance, the
   pattern of `blend_seam_split_rim.rs`'s birth/death row) so a fresh key in
   `dead.edges` would be caught where it bites; the existing rows' dumps
   unchanged (below).
5. **The mutant**, in the PR body: put the old `rec.dead.edges.push(mr)`
   back and show the witness row red on direction (b) and the document row
   red, and every existing ladder row still green — that is the item's
   "unmeasured" made measured.

## Constraints, binding

- **Every existing carve is bit-identical to the merge base** (`bitdump.rs`
  differential, both SHAs in the PR). The birth record for every existing
  fixture is unchanged too: dump `BlendNaming` (or its totality summary) at
  base and head for the census fixtures and diff.
- **No change to `split_edge`'s key rule** — `topo` is TOPO's; the phase
  adapts to the rule, it does not move it.
- **No orientation is decided from a sampled quantity**; provenance is read
  off keys and records only.

## Acceptance

- The census table and (if needed) the door table, from the merge base.
- A witness body through a public door, its carve row and its document-layer
  row green; or the measured finding that none exists and the orchestrator's
  re-scope recorded in this spec.
- The mutant table: the witness rows red, every existing row green.
- The bit-dump and birth-record differentials clean, with the two SHAs.
- Hosted CI green (full matrix).

## Out of scope

The annulus phase (`rim_phase_annulus` — its seam splits have their own
provenance rows; if the census shows the same shape there, file it as an
issue on BLEND's slate with the reading, do not fix it here); the ruled band's
`split_rim` beyond hoisting; anything in `emit_blend.rs` (EVAL's).

## Review

v6 dual on the frozen head; claims to falsify (verbatim to both reviewers,
with `docs/prompts/reviewer-style-lane.md` by path):

- **C1** Every ladder carve at the merge base is bit-identical at the head,
  and its birth record is identical too (re-run both differentials).
- **C2** The witness body reaches the orientation through a public door and
  the head carves it with every retirement naming a source key (re-derive
  from `split.rs` which piece keeps the key; do not trust the census).
- **C3** The mutant (the old push restored) reds the witness rows and nothing
  else.
- **C4** The provenance rule has one spelling in `sweep::blend`, or the PR
  says why two are kept (grep for a third: the annulus phase's seam splits).
- **C5** The census is complete over the tree's ladder fixtures (find one it
  missed).
