---
id: mate-memo-key-does-not-carry-the-solve
kind: issue
title: A mate's memo key omits the solve, so a mate can read Ok in the evaluation that blames it
status: open
opened: 2026-09-04
refs: [1461]
---

Found by CHROME's badge-attribution lane, which hit it hard enough to
need a second commit to work around.

**The evaluation is internally inconsistent.** A mate node's memo key
does not carry the solve, so a cluster that breaks around an *unedited*
mate never re-runs that mate. Its row therefore reads `Ok` in the very
evaluation whose fault names it.

Observed twice in one lane, on documents built through ordinary doors:

- author mate A, evaluate, then author mate B that breaks the cluster —
  A's row reads `Ok` while every instance in its cluster reports the
  refusal;
- two mates on one pair — the fault text says *"mates 3 and 4 cannot
  both hold"* while node 3 reads `Ok`.

**Why it is worth a file rather than a note.** It is not a display
problem. The solve records one `MateFault` against every instance in
the cluster and every mate holding it together
(`crates/editor-core/src/mate/solve.rs:838-856`), and those faults name
the offending mates in public fields — so a consumer that trusts the
blame will point a user at a green row. CHROME's fix had to corroborate
every blame against the evaluation before using it
(`crates/viewer/src/tree.rs:337-339`: take the first blamed mate the
run *agrees* is `Failed`, else keep the row's own failure). Without
that guard the naive read points at an `Ok` row **and drops the
message**, which is strictly worse than the defect it was fixing.

That guard is a viewer-side workaround for a kernel inconsistency. It
should not have to exist, and the next consumer of `MateFault` will not
know to write it.

Sibling already recorded in the tree:
`crates/viewer/tests/review_gui4_r1.rs:548` — "the memo answers for
every instantiate node, so the removed part is never noticed". Same
shape: a memo key that omits a dependency the answer depends on.

**Not CHROME's.** `editor-core`'s eval memo and mate solve are outside
this program's `paths`, and its `keep_out` fences the mate vocabulary
explicitly.

Signed: (CHROME orchestrator)

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/msolve/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.
