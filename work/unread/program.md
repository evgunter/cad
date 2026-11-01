---
id: unread
kind: program
title: UNREAD — claims, references and surfaces the tree states that no gate reads
status: ready
opened: 2026-10-26
group: infra/ci
prefix: unread/
tag: (UNREAD orchestrator)
ab_band: 15700-15799
paths: [crates/pncad/src/prelude.rs, crates/topo/src/review_m1_pr5_internal.rs, crates/viewer/src/vocab.rs]
keep_out: [opened 2026-10-26 by GUARD's split and joined the same day by UNSEEN (META's cut of the cross-tree reference and surface checkers) - every row moved with its id and body unchanged, a new gate is a new file under scripts/gates/* which is GUARD's directory - announce it on GUARD's log and build it on GUARD's harness (lib.sh) rather than a private reader, a checker that is an arm of scripts/work.py lint is META's ground - build it there by announced seam and announce it on META's log and the row stays here, .github/workflows/* is CIW's - wiring a new gate or a probe-feature run into a job lands by announced seam, tools/* and docs/K-REPORT.md and docs/TESS-BUDGET.md are INSTR's, crates/*/tests/* and crates/test-utils/* are S-TINT's and S-TCOST's, the three paths claimed here are where three rows' guards land and are shared with whichever programs own those crates - a row here edits them only for the guard it adds, the reference rows read citations in work/ and crates/*/src and docs/doc-ledger/* and the façade censuses in crates/pncad/tests/all.rs and the Python binding census without claiming them, docs/doc-ledger/ notes are written by the program being swept at its exit and this program only adds a check that their recovery commands resolve - it does not rewrite notes beyond the one-line pointer fixes its own rows name, a stale citation found in another program's item file or plan is routed to that program and not fixed across the fence (META's one-file-one-item rule), run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled]
---

The guards that do not exist yet. Every row is something the tree
states that no gate, test or CI job reads, so it goes stale without
anything going red. Two halves of one kind of work:

- **claims** — a measured number, a census's membership, a compile-fail
  proof, a table's reasons, a probe-gated pin, a formatting promise;
- **references and surfaces a human wrote** — a `file.rs:NNN` citation
  in `work/`, a test name in a doc comment, a `work/<program>/<id>`
  path in a crate comment that rots the day the row moves, a
  doc-ledger recovery command nobody runs, a public method neither
  façade census can see, authored lines folded into a merge commit.

The question each row answers is *"what goes red if this stops being
true?"*, and *"nothing, and here is why"* is a complete answer for one.
Cut out of GUARD on 2026-10-26 when GUARD measured 43 points against
30; UNSEEN, cut out of META the same day, folded in the same day
(`docs/doc-ledger/unseen-leaves-the-tracker.md`). Charter and order:
`work/unread/plan.md`; narrative in `work/unread/log.md`.
