# BLEND-15 — an escalation's recourse is routed by one rule with one fall-through (spec)

**Program:** BLEND (`work/blend/plan.md`, unit 15). **Item:**
`work/blend/escalation-recourse-dispatch-has-three-homes.md`.
**Track:** kernel change — the standard v6 unit (binding spec, drawn implementer
arm, cross-model dual review, union fix pass, record-at-merge; §Review).
**Pre-draw fields, logged before the draw:** difficulty **S**, task-class
**STRUCTURAL**.

- **S** — two Display tables (three until BLEND-12 lands; it retires the
  third) made to agree on the unknown name, and a roster row per crate; no
  numeric question.
- **STRUCTURAL** — a design statement about where recourse prose lives;
  taken here as disposition (2) with the honest fall-through.

**Territory note.** `crates/sweep/src/blend/mod.rs` is BLEND's;
`crates/profile/src/{path.rs, validate.rs}` are S-BOOL's glob, edited by
announced seam (`work/bool/log.md`, at dispatch). **Follows BLEND-12**: it
starts from the tree BLEND-12 leaves (the fillet arm on
`PathError::Escalated` keyed on the predicate name through one map in
`validate.rs`; the `EscalationSite::Fillet` arm retired).

## The claim

**Two `Display` impls route a recourse by matching an escalation's predicate
NAME, each with its own table and its own answer on an unknown name.**
`BlendError::Escalated` (`crates/sweep/src/blend/mod.rs`, `match
source.predicate`) names the hole on an unknown name — the F6 gap sentence,
"no recourse is recorded for predicate …; this is a gap in the error table,
not advice to act on". `PathError::Escalated` (`crates/profile/src/path.rs`)
asserts a category on an unknown name — "path junction classification:
{source}" — which is false for every non-junction key, and after BLEND-10
and BLEND-12 the arm carries three families (the stored-form eight, the
fillet nine, the junction keys) whose ORDER decides which sentence a shared
name would get. Adding a predicate obliges an author to remember every
table; nothing enforces it.

**The decision, taken by the orchestrator (2026-09-13): disposition (2).**
`geom-core` does not carry user-facing prose (its `decide` names are a
K-corpus vocabulary, not sentences), so the tables stay in the crates that
own the sentences; what becomes ONE rule is the fall-through and the
obligation: (a) every Display that routes by predicate name answers an
unknown name with the honest gap sentence (the F6 shape, one constant in
`geom-core`'s error vocabulary or re-exported from `sweep::blend` — decide
by who may depend on whom and say why), never a category assertion, never
silence; (b) each crate carries a roster row that enumerates every
`decide("…")` name its `src` decides (read through `test_utils::source`,
the reader-census pattern BLEND-10 used) and asserts each is either in that
crate's table or on an explicit "no recourse" list with a reason — so a new
predicate cannot land silently in the fall-through; (c) the dispatch order
inside `PathError::Escalated`'s arm is stated at the site and pinned by a
row that a name in two families would red. Disposition (1) (one table
beside the band) is not taken: recourse prose is the door's, not the
scalar's.

**Ratified and not re-litigated:** the F6 gap sentence's shape; BLEND-10's
stored-form arm; BLEND-12's fillet arm; the predicate names.

## Phase 1 — measure before touching anything

The roster: every `decide("…")` name in `crates/sweep/src` and
`crates/profile/src` (the two crates with a routing Display), and for each
which table carries it, which sentence, or that it falls through — and
what the fall-through renders today. Names that appear in both crates'
tables with different sentences are listed. One table in the PR body
BEFORE Phase 2 begins.

## Phase 2 — the change

1. The gap sentence homed once; `PathError::Escalated`'s unknown-name arm
   renders it (the category assertion goes); `BlendError::Escalated`'s arm
   cites the home.
2. The roster rows (`crates/sweep/tests/recourse_roster.rs`,
   `crates/profile/tests/recourse_roster.rs`; aggregated; subject-named):
   every decided name is routed or explicitly unrouted with a reason; the
   dispatch order pinned.
3. **The mutant**, in the PR body: add a `decide("fillet_new_gate", …)`
   nowhere routed → the roster row red; swap two families' order → the
   order row red.
4. **The doc trail**: the item's three-homes count corrected to what the
   tree has after BLEND-12; `docs/KERNEL-VERBS.md`'s recourse sentence if it
   names the tables.

## Constraints, binding

- **Rendered text changes only on the unknown-name arms**; every routed
  sentence renders as before (dump the rendered Display of every refusal
  the profile and sweep suites produce at both SHAs and diff).
- **No new predicate; no behaviour change at any door.**

## Acceptance

- The Phase 1 roster; the one gap sentence at both arms; the roster rows;
  the order row; the mutant table; the rendered-text differential clean
  except the unknown-name arms; hosted CI green.

## Out of scope

Moving sentences between crates; `COINCIDENCE_RECOURSE`'s wording at
junction keys (the class item stays open); the FFI tag census unless a
`PathError` shape moves (it should not).

## Review

v6 dual on the frozen head; claims to falsify (verbatim to both reviewers,
with `docs/prompts/reviewer-style-lane.md` by path):

- **C1** Every routed sentence renders as before (re-run the rendered-text
  differential).
- **C2** The roster is complete: find a `decide` name in either crate's
  `src` the roster row does not see (a name built by `concat!`, a name in
  a macro, a name decided in a `#[cfg(test)]` module).
- **C3** An unknown name renders the gap sentence at BOTH arms and nowhere
  asserts a category or falls silent (construct one through a test-only
  predicate and read the text).
- **C4** The dispatch order is pinned: a name in two families reds the
  order row (plant one).
- **C5** No third routing table survives anywhere in `crates/*/src` (sweep
  for `match source.predicate` and its equivalents, incl. `editor-core`'s
  renderers).
