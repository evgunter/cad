---
id: ciw
kind: program
title: CIW — hosted CI, workflows and scripts, and what no gate reads
status: ready
opened: 2026-09-03
group: infra/ci
prefix: ciw/
tag: (CIW orchestrator)
ab_band: 1500-1599
paths: [.github/workflows/*, local-scripts/*, .claude/hooks/*, scripts/apt-install.sh, demos/*.sh, demos/*.py, scripts/check-*.py, scripts/criterion-emit.py, scripts/doc-gate.sh, scripts/check_admesh.sh, scripts/check_step.sh, scripts/k_probe_sweep.sh, scripts/tess_budget_cut.sh, scripts/tess_budget_sweep.sh, crates/pncad/src/prelude.rs, crates/topo/src/review_m1_pr5_internal.rs, crates/viewer/src/vocab.rs]
keep_out: [scripts/gates/* is GUARD's (work/guard/program.md), tools/* is INSTR's (work/instr/program.md), scripts/ci-filter.py is S-TCOST's, scripts/work.py is the tracker's own and changes only with work/README.md, CI build knobs (profile/cache) stay S-TCOST's rule — measured in-unit or not at all, what a main push re-gates is an [ev] ruling, UNREAD (claims and references no gate reads) merged in on 2026-11-02 and kept its fences - a new gate is a new file under scripts/gates/* which is GUARD's directory - announce it on GUARD's log and build it on GUARD's harness (lib.sh) rather than a private reader, a checker that is an arm of scripts/work.py lint is META's ground - build it there by announced seam and announce it on META's log and the row stays here, crates/*/tests/* and crates/test-utils/* are S-TINT's and S-TCOST's, crates/pncad/src/prelude.rs and crates/topo/src/review_m1_pr5_internal.rs and crates/viewer/src/vocab.rs are claimed only because three rows' guards land there and are shared with whichever programs own those crates - a row here edits them only for the guard it adds, the reference rows read citations in work/ and crates/*/src and docs/doc-ledger/* and the façade censuses in crates/pncad/tests/all.rs and the Python binding census without claiming them, docs/doc-ledger/ notes are written by the program being swept at its exit and this program only adds a check that their recovery commands resolve - it does not rewrite notes beyond the one-line pointer fixes its own rows name, a stale citation found in another program's item file or plan is routed to that program and not fixed across the fence (META's one-file-one-item rule), run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled]
---

**The workflow half**: `.github/workflows`, the jobs that gate a PR and
the nightly that holds the rest, and what a reader of a red run can
tell from it.

The per-PR gate is sized for latency (`ci.yml`); everything else runs in
`nightly.yml`. `work/ciw/latency-cut.md` carries the selection and its
evidence, and a change that adds work to the per-PR gate argues against
it.

Ev put this band low by name — *"it is low priority to improve tooling
in order to cause ci to fail less / main to be red less often"* — and
most of the slate is that. The rows that are not about redness are about
a PR that is silently UNGATED rather than red (a head with no merge ref
gets no run), and a red inherited from main that is not attributed to
the merge that caused it. The P3 rows came from BLIND and MIRROR when
those tracks left the tracker: instruments that pass while blind, which
is Ev's medium band (tooling that prevents a SILENT bug).

**The guards that do not exist yet** (UNREAD's half, merged in on
2026-11-02 when the budget came to count only P0, P1 and unbanded rows
and both tracks fell under it: `docs/doc-ledger/unread-merges-into-ciw.md`).
Every one of these rows is something the tree states that no gate, test or
CI job reads, so it goes stale without anything going red:

- **claims** — a measured number, a census's membership, a compile-fail
  proof, a table's reasons, a probe-gated pin, a formatting promise;
- **references and surfaces a human wrote** — a `file.rs:NNN` citation
  in `work/`, a test name in a doc comment, a `work/<program>/<id>`
  path in a crate comment that rots the day the row moves, a
  doc-ledger recovery command nobody runs, a public method neither
  façade census can see, authored lines folded into a merge commit.

It is the workflow half's question asked of the whole tree: *"what goes
red if this stops being true?"*, where *"nothing, and here is why"* is a
complete answer for one row. A row in either half that is an UNGATED
claim rather than a red one belongs to the same sitting, and two pairs
of rows are one finding seen from each half (`work/ciw/plan.md`).

Charter and unit order: `work/ciw/plan.md`; narrative in `work/ciw/log.md`.
