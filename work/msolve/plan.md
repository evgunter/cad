# MSOLVE — the mate solve's correctness residue (plan)

**STATUS: OPEN (2026-09-04).** Successor to S-MATE, which closed while
its residue was still being measured. Live state is `log.md`'s tail and
the item files beside this plan, never this file.

Branch prefix: **`msolve/`**. Away-channel tag `(MSOLVE orchestrator)`.

## Why this is not DOCM

DOCM took `mate.rs` and `mate/*` into its `paths` at S-MATE's exit, and
its charter is *"the editor-core document layer — the persisted recipe
vocabulary, the `DocEdit` set, document identity, and the frames and
selectors the viewer and the mate tool consume"*. That is **custody of
the files**, and it is the right home for the document-layer questions.

What is left over is not that. It is whether the solve computes the
right pose — a question about assembly semantics, answered by measuring
what a document evaluates to, with no document-vocabulary content at
all. Ev's steer, and the reason this program exists rather than four
items on DOCM's slate.

**The overlap is real and unresolved.** Both programs' globs name
`mate.rs` and `mate/*` until DOCM cedes them or declines to. That is
announced, not assumed — and it is exactly the state
`scripts/work.py territory` is blind to
(`work/issues/territory-cannot-see-a-path-two-programs-both-claim`), so
it will not warn anyone.

## The slate, in dependency order

1. **`mate-solve-is-transform-blind`** — the gate, and the only live
   defect here. `fold_pair` builds each mate's coset from authored
   alignment data and never reads the evaluated body, while
   `wire_transform` composes its map on afterwards. Measured through
   ordinary doors: **string-identical solved frames with and without a
   `Node::Transform`**, `fault = None`, `product = Ok`, mated faces 10
   apart — a green document with a gap where the author declared
   contact. Characterization rows are on main (PR 1773) and are written
   to go **red when this is fixed**; their header says the fix deletes
   them rather than updating them.

   The fix shape, from the measurement: a `derived_offset` **sibling
   that walks the input chain and composes every pose-bearing node's
   map**, not just the pattern's.

   Two blind spots the measurement stated, and the first is worth
   closing before anything else: a `Prismatic` mate along +z may absorb
   the translation in its free direction, which would make this
   **class-dependent and intermittent** rather than uniform; and
   rotation is untested (the probe is translation-only with angle
   exactly 0, chosen for exact arithmetic).

2. **`nested-pattern-mate-heads-refuse`** — RULED IN by Ev on PR 1731,
   both halves: the member vocabulary extends through
   identity-transparent nodes, nested patterns and pattern-of-transform
   alike, as one walk. **Gated on (1)** — extending without composing
   the transform's map converts one silent wrong answer into two,
   because `DanglingHead` is currently the only guard on the patterned
   twin of the silent half.

   Measured cost: `Member.copy` grows from `Option<(RecipeNodeId, u32)>`
   to a chain, loses `Copy`, and it is the `BTreeMap` key for `by_pair`
   and `edge_of` and the thing the spanning tree selects its edges by.
   Owes loop-closure rows for a nested member that nothing in the suite
   has today.

3. **`mate-dangling-head-is-a-catch-all-that-reports-a-false-cause`** —
   `derived_offset`'s `# Errors` defends its catch-all with *"the
   pattern node's own evaluation names the underlying cause in its own
   voice"*. Measured false: the mate fault **poisons** the document, so
   that node never evaluates and the cause appears nowhere. Predates
   the unit that found it; already mis-labels the decided-zero case.
   The proposal on file is one variant carrying the evaluation layer's
   typed refusal verbatim, closing the catch-all.

4. **`mate1-sweep-inferred-a-remap-from-a-refuted-reachability`** — a
   correction owed to a sweep report other units may be reading:
   issue 1405 inferred a remap requirement from a reachability AQ8's
   addendum had already refuted. Closed by exhaustion on PR 1749 (318
   cut sets). No code; the record is the deliverable.

5. **`aq8-skip-half-is-cited-as-ratified-and-is-not`** — the "(b) SKIP"
   half is cited around the tree as ratified and lives only in a
   test-file comment and a commit message. Ratify it or demote the
   citations; it is load-bearing for PR 1749's argument.

## The first [ev] question

**Where the extended vocabulary is written down.** `docs/MATE-1-SPEC.md`
carried the A11 rider and was deleted at merge per `docs/DOC-LEDGER.md`,
and no `crates/*/README.md` picked it up. So the rule Ev has just
extended has no ratified home to extend. That is a question before it is
a unit.

## Review posture

Inherited from FIX, where it was measured over eleven units: **one style
review per unit, plus a correctness arm where a unit moves a kernel
answer rather than its rendering.** (1) and (2) both move kernel
answers. No A/B rows; the band above is claimed for bookkeeping only.
