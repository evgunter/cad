# FILLET-SPLIT — the open bands leave `surgery.rs` (spec)

**Program:** FILLET (`work/fillet/plan.md`), unit
`surgery-module-holds-four-surgeries`
(`work/fillet/surgery-module-holds-four-surgeries.md`; Ev's ruling on PR
1916: a move with no design implication needs no ask). **Track:** kernel
change — the standard v6 unit (binding spec, drawn implementer arm,
cross-model dual review, union fix pass, record-at-merge; §Review below).
**Pre-draw fields, logged before the draw:** difficulty **S**, task-class
**STRUCTURAL**.

- **S** — a file move: no function changes body; the gate's file list and
  the ledger entry are re-scoped, not extended.
- **STRUCTURAL** — nothing decides differently; every carve bit-identical.

## The claim

`crates/sweep/src/blend/surgery.rs` (~4 900 lines) holds four surgeries as
titled sections: the plane–plane open band with its trihedral corners (the
blank phase), the closed-rim LADDER, the closed-rim ANNULUS with its
hostless struts, and the RULED open band with its transverse cut-off — plus
the refusal constructors, the plan structs, the ring check, the one `kef`
door and the description pass. Both FILLET-H7 reviewers (Q8) named the
honest shape: the two open bands in their own files behind one
ratification. The compound bound `T: Decide + Bounds` is allowlisted for
`blend/(battery|build|surgery).rs` (`scripts/gates/bounds-allowlist.sh`,
the "M5 PR 12" entry in `crates/geom-core/src/real.rs`'s
`bounds_allowlist` ledger); moving ratified code into new files re-scopes
that entry's file list — the seam is the ratified thing, the file list is
its spelling (Ev, PR 1916).

## Phase 1 — measure before touching anything

Read `surgery.rs` end to end and record its section map with line ranges;
for each section, what it reads from the others (plan structs, `kef_minted`,
`attach_contact`, `flank`, `cap_incidence`, `seam_split_param`, the refusal
constructors, the description pass). The shared items are the seam the
split must leave in one place; if any open-band section reaches into a
closed-rim section's private state (not through a shared helper), STOP and
report — the split is then a refactor, not a move, and this unit does not
do refactors.

## Phase 2 — the change

1. `blend/open/planar.rs` (the blank phase and the trihedral corners) and
   `blend/open/ruled.rs` (the ruled band and the cut-off), or
   `blend/{open,ruled}.rs` if `open/` earns nothing — the lane picks the
   shape Phase 1 supports and says why; `surgery.rs` keeps the shared seam
   (plans, the `kef` door, `attach_contact`, the description pass) and the
   two closed-rim walks, and its header's tour names where the open bands
   went.
2. `scripts/gates/bounds-allowlist.sh`'s "M5 PR 12" line lists the new
   files beside the old; the `real.rs` ledger entry gains one sentence: the
   re-scope, its date, PR 1916 as the ruling — NOT a new entry (this is a
   move of ratified code, not an extension of scope).
3. Every `pub(super)`/`pub(crate)` visibility change the move forces is
   listed in the PR body; no item becomes `pub`.
4. Prose: every pointer into `surgery.rs` by line or section (`docs/`,
   `crates/*/README.md`, `work/`, test-file headers) re-pointed; the
   Row-4 `unreachable!` rule and the one-`kef`-door census
   (`review_fillet_t_r1_probes`) hold across the new files — widen the
   census to the directory.

## Constraints, binding

- Every existing carve bit-identical to the merge base — the dump over
  every corpus (incl. the ruled row FILLET-T adopted).
- No function body changes; `git diff --color-moved` shows moves, and the
  PR body states every non-move hunk.
- No new metered predicate; nothing decides.

## Acceptance

Phase 1's section map; the two files with `surgery.rs` shrunk to the seam
and the closed-rim walks; the allowlist and ledger re-scoped (one line
each); the dump identical; the visibility list; hosted CI green (full
matrix; the discipline and allowlist gates included).

## Out of scope

Any change to what the surgeries do; the closed-rim walks' own split
(they stay); D325/D326's content (FILLET-T's, landed first — this unit
branches after T merges).

## Review

v6 dual on the frozen head, claims to falsify:

- **C1** Every existing carve is bit-identical to the merge base.
- **C2** Every hunk is a move (`--color-moved`); the PR body lists every
  non-move hunk and each is a visibility or `use` change.
- **C3** The allowlist re-scope is exactly the old seam's files plus the
  new ones and nothing else; the ledger says re-scope, not extension; the
  gate script's self-test still catches a compound bound in a file outside
  the list.
- **C4** No pointer into `surgery.rs` by line or section is left stale
  (`docs/`, READMEs, `work/`, test headers); the one-`kef`-door census
  covers the new files.
