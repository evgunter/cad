# EVAL-4 — `declare_all` and `rem_apply` stop dropping `applied.maintenance`: the accepted edit travels whole (spec)

**Program:** EVAL (`work/eval/plan.md`, unit 4). **Item, one unit:**
`work/eval/D367.md`.
**Track:** E — the CIW/CHROME posture: one implementer lane, one style
review with claims to falsify, a fix pass, record-at-merge. No A/B draw.
**Correctness arm:** the Python-side funnel test is the guard and it must
stay green with the second copy of the declare body gone (§Review claim 2).
**Branch:** `eval/4-accept-funnel`. **Difficulty:** S–M.

## The claim

**An accepted edit is an `Applied<P>` — the new document, the edit record
and the cluster maintenance the edit performed — and a door that hands a
caller only the document has thrown the maintenance away** (`Applied`'s
own doc, `crates/editor-core/src/edit.rs:1179`–`:1195`: the record "rides
the accepted edit … what the record adds is VISIBILITY"). PR 1503 gave
`pncad-py`'s `Doc` one swap point, `accept(applied)`
(`crates/pncad-py/src/py/doc.rs:~218`), so the held document and its
maintenance move together. Two `editor-core` doors still drop it:

1. `names/flush.rs:425` `declare_all` (and `declare` over it, `:409`)
   returns `(applied.doc, id)`. Its one production consumer, `pncad-py`'s
   `declare_findings` (`doc.rs:~258`), already refuses to use it FOR THAT
   REASON — its comment says so — and carries a second copy of the door's
   body (`declare_node` + `insert_node`) to keep its mirror honest. The
   drop has cost a duplicate already.
2. `refactor.rs:1439` `rem_apply` (the closure inside `split`) keeps
   `applied.doc` and `applied.record.minted` and drops
   `applied.maintenance` for every remainder edit; `SplitOutcome`
   (`:530`) carries `remainder_edits` and `part_edits` but no
   maintenance for either document, and the comment at `:1508` records
   that automatic maintenance DID act on the remainder ("appended it
   instead"). `refactor.rs` is FIX's by announced seam
   (`work/eval/program.md` keep_out); the announcement is the
   orchestrator's log line and this unit's PR.

The fix shape is PR 1503's, one level down: the door returns the
acceptance whole and the caller accepts it at its one swap point.

## What lands

1. **`declare` and `declare_all` return `(Applied<P>, RecipeNodeId)`.**
   The id stays a checked value (`DeclareError::NoMintedId` keeps its
   arm); the document and the maintenance travel inside `Applied`. Every
   Rust caller (`crates/editor-core/tests/seatfw_curved_flush.rs:220`,
   `lib_sel2_flush.rs:221`, `:287`, and the sweep's list) takes
   `.0.doc` where it took `.0`.
2. **`pncad-py`'s `declare_findings` goes back through
   `pncad::select::declare_all`** and `self.accept(applied)`, and its
   comment explaining why it could not is deleted — the premise is gone
   (Q4). One body for the declare doors, the same `DeclareError` arms
   through the same `declare_err`. `pncad-py` is LIB's ground for the
   `.pyi` and the Python surface; this change touches no Python-visible
   signature, so it is EVAL's by the same rule that let PR 1503 edit the
   file — say so in the PR body, and if the implementer finds a stub or
   `.pyi` line that must move, STOP and report rather than edit it.
3. **`split` keeps what its edits performed.** `rem_apply` extends a
   `remainder_maintenance: Vec<ClusterMaintenance>` and `SplitOutcome`
   carries it beside `remainder_edits`; the part's edits get the same
   treatment (`part_maintenance`) if any `apply` on the part side can
   perform maintenance — the implementer reads `part_edits`'s
   construction and says which, with the reason at the field. **Before
   adding either field, survey `split`'s callers** (`rg 'refactor::split|
   split\(' crates demos --type rust`) and state in the PR body what each
   does with the outcome: a caller holding a maintenance mirror that
   swaps `outcome.remainder` in by hand is the staleness PR 1503's funnel
   exists to make unspellable, and the new field is what lets it accept
   honestly; a caller with no mirror needs nothing and the field is still
   right, because the outcome is the record of what the refactoring did.
   The item's own warning binds: do not assume a stored mirror exists
   to go stale — find it or say there is none.
4. **`InlineOutcome` (`refactor.rs:~548`) is checked for the same drop**
   and fixed in this unit if its edits go through an `apply` that can
   perform maintenance; otherwise the PR body says why not.
5. Comments state the invariant (an accepted edit travels whole; the
   caller's swap point is where the document and its maintenance change
   together), never the history of the drop.

## Sweep

The class is "a door that destructures `Applied` and keeps less than all
of it". Shape grep, hit list with dispositions in the PR body:

```
rg -n 'applied\.doc|\.doc,|Applied<' crates/editor-core/src crates/pncad-py/src crates/viewer/src demos --type rust
rg -n 'apply\(' crates/editor-core/src --type rust -g '!edit.rs'
```

A hit in DOCM's files (`persist/*`, `program.rs`, `doc.rs`, `edit.rs`,
`node.rs`, `resolve/*`, `mate*`, `assembly.rs`) or the viewer is reported
with what it keeps and what it drops, never edited. State what the pattern
cannot match (an `Applied` bound to a name other than `applied` and
destructured by pattern).

## Review

One style lane, `docs/prompts/reviewer-style-lane.md` by path. Claims:

1. No door in `editor-core` returns a document from an accepted edit
   without its maintenance: run the sweep shaped differently and name
   every survivor.
2. `pncad-py`'s funnel test
   (`test_assembly_author.py::test_last_maintenance_describes_the_last_accepted_edit_at_every_door`)
   still runs and passes with the declare doors through the sugar —
   confirm the Python suite ran on CI (the change filter's `RUN_PNCAD_PY`
   line), since a green over a skipped python job is the standing hazard.
3. `SplitOutcome`'s new field is populated by every `rem_apply` call
   (count the calls, `:1448`–`:1526`, against the extends) and a row
   exists that goes red when one is missed — a split whose remainder
   edit joins two clusters, asserting the join appears in the outcome.
   If the implementer argues no split edit can perform maintenance, that
   argument is checked against `apply`'s maintenance arms, not accepted
   from the PR body.
4. Q4: the deleted `pncad-py` comment was the only thing citing the old
   premise (grep "drops"/"maintenance" in `pncad-py/src`).
5. Q6: every disclosed not-this-unit (an `InlineOutcome` left alone, a
   caller left holding a mirror) has a file on a slate, not a sentence.

## Records at merge

`work/eval/log.md` entry with the FIX seam announcement; `D367` `closed`
with `pr:`; this spec deleted per `docs/DOC-LEDGER.md`.
