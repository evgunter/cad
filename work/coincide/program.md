---
id: coincide
kind: program
title: COINCIDE — the coincidence door and its lint after stage 4: what a row says, what it costs, where it runs
status: ready
opened: 2026-10-11
group: api/assembly
prefix: coincide/
tag: (COINCIDE orchestrator)
ab_band: 12900-12999
paths: [crates/editor-core/src/coincide.rs, crates/editor-core/src/coincide/*, crates/editor-core/src/checks.rs]
keep_out: [opened 2026-10-11 by INTENT's split (Ev approved the division in chat 2026-10-11) - the rows moved by git mv with ids and bodies unchanged, INTENT stays open and keeps the D10 stage chain (stage 3, stage 4 H and I, stage 5 C) and every row that waits on it, so a row here that turns out to need a stage 3 or stage 4 unit moves back to INTENT rather than waiting here, FLUSH holds the topo glue rows that left INTENT in the same commit, the door's ground is shared with INTENT (checks.rs, the at-rest census) and TALLY (eval/select.rs), so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled]
---

The coincidence door (`crates/editor-core/src/coincide.rs`, built by
INTENT stage 4 C+D and run at rest by PR 4670) and the
`unproven-coincidence` lint over it. This program owns the rows that
are about the door as it stands: what a row's recourse names, which
corpus rows it cannot close, its cost and cancellation, where it runs
(the wasm pump), and the lint's readers (Python, STEP import, the
containment check). None of them waits on a D10 stage still to build.
