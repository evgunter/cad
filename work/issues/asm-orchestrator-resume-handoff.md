---
id: asm-orchestrator-resume-handoff
kind: issue
title: ASM program — resume this line of work (orchestrator handoff)
status: closed
opened: 2026-08-12
closed: 2026-09-03
github: 430
refs: [414, 425, 561, 611]
---

## From GitHub issue 430

opened 2026-08-12, 2 comments.

> **STATE MOVED (2026-08-16): R1 is COMPLETE and this body's snapshot is historical.** Live state = the docs/ASM-LOG.md tail, always. Rolling state refreshes live in this issue's comments (first one dated 2026-08-16: R1 complete, A12/A13 ratified, R2-a dispatched, schema chain, dual-tally warning). The reading list below still applies.

Standing handoff for whoever picks up the ASM (assemblies) orchestrator role. Read first: memories/MEMORY.md (follow pointers — especially orchestration-model, agent-lane-operations, usage-limit-protocol), docs/ASM-PLAN.md, the docs/ASM-LOG.md TAIL (always the live state), docs/ASSEMBLY-DESIGN.md (A1–A11, all ratified).

## State at this writing (2026-08-12)

- MERGED: ASM-1 (#364), ASM-ROOTS (#383), ASM-2K (#381), ASM-2A (#414); ASM-3 discharged inside ROOTS+2K. A9/A10/A11 ratified.
- **PR #425 (ASM-2B) green; its blinded single review IS DISPATCHED (ordinal 34, v4 ladder, frozen head 07999509)** — if you inherit mid-review: the reviewer's report lands at cad-work/asm-2b-r1-report.md; adjudicate (including the deletion-vs-flip deviation it was asked to recommend on), fix pass via the implementer if needed, merge on green, row at merge, sweep lanes asm-2b + asm-2b-r1.
- Then: **ASM-4** (docs/ASM-4-SPEC.md, binding; block ASM-2 slot 3 = fable; L/structural pre-logged) closes R1 → finalize **docs/ASM-R2-SPEC-DRAFT.md** (R2-a solve, R2-b minting+planar verification — the program's first numeric-predicate unit) and open the R2 lane.

## Standing rules learned this cycle (beyond the memories)

- Briefs: v4 verdict ladder verbatim; cold clippy = CI scope AND `cargo clippy -p pncad-py --features python --all-targets -- -D warnings` AND the interval graph (the python lane bit three times); poll harness-backgrounded calls' output files with foreground reads (lost wakes endemic); kill only your own recorded PIDs, never pgrep pattern-matching.
- Tooling: local-scripts/ (new-lane, with-build-slot, clean-lanes, monitors). Session start: install + arm EVERY monitor (glob). Away-channel env: CAD_CHANNEL_SELF_TAG="(ASM orchestrator)", CAD_CHANNEL_BRANCH_PREFIXES="asm/,mngr/cad-assemblies-implement". Usage alerts: act only on your own account's (resolve from your agent dir), peer-revive with the flock rule.
- A/B: v4 + #409 (pre-log difficulty AND numeric|structural before the draw; results-off-file while dispatches are in flight; stratified 1:1 for L/XL from block ASM-3; dual blocks-of-two — next dual takes the banked SAME-MODEL slot per #405; sample numbers follow ordinals; check the dual stopping tally before any dual — it was 4-of-6, and the sixth qualifying dual's recorder must notify Ev explicitly).
- Design conversations on their own asm/* branches, never the orchestrator branch; sign-off via watched comment 👍; unsure fork-vs-elaboration → fork.

## Pending with Ev

- #414 resolver `Option>` shape (defended on that thread; change only on his ruling).
- AQ1 / AQ2 / AQ5 / AQ6 open by design; AQ2 turns pressing when the "accept updated version" DocEdit lands (natural home: with or right after ASM-4).

## Filed findings feeding later units

- #382 overlap-validation honesty (doc-truth half is cheap; detection is R2/R3 census work — R2-b's F1 row starts it).
- #415 geom-core tolerance_init env-sensitive red (kernel lane's triage).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

## Comments

**2026-08-16** — orchestrator:

(ASM orchestrator) Handoff state refresh (2026-08-16, supersedes the body's 2026-08-12 snapshot — the body's standing reading list still applies; ASM-LOG tail remains the live state):

- **R1 COMPLETE**: ASM-4 merged (#525, ordinal 40, D-2 amended at adjudication via #540). ASM-UPD merged (#549, ordinal 46, schema v10) — A13's pin-update door.
- **Ratified this cycle**: A12 (mate reading edges + non-body roots, #522, discharges AQ7 — raised and signed-off same-day), A13 (update granularity, #544, discharges AQ2).
- **Live pipeline**: R2-A binding spec ready (docs/ASM-R2A-SPEC.md; block ASM-3 slot 1 = opus, L/NUMERIC pre-logged) — **dispatch gates on M9-1 PR-2's merge** (#552, in review at ordinal 47; editor-core ContactClass re-export; their schema shifts to 11). R2-B spec finalizes on the same event (draft carries the recon addendum + coset table + M9-1 seam updates). Block ASM-3 slots 3-4 bank.
- **Cross-program state**: dual tally 5-of-6 (next qualifying dual's recorder notifies Ev explicitly — next dual ordinal is 48); schema chain v9=RESPELL, v10=ASM-UPD, 11=M9-1's at re-merge; #561 (tag-value pinning) adopted by LIB.
- **Orchestrator branch this session**: mngr/cad-asm-2 (away-channel prefixes asm/,mngr/cad-asm-2,mngr/cad-assemblies-implement). #414 resolver shape settled (Arc confirmed on-thread, Ev accepted).
- Pending with Ev: AQ1/AQ5/AQ6 open by design (AQ6 waits for C7-era tables).

**2026-08-18** — orchestrator:

(ASM orchestrator) **Superseded by #611** for the live pointer: the ASM program's ratified v1 scope (R1+R2) is now code-complete, and #611 carries the current resting state, the priority queue (TESS-SPLIT → exit walk + demo → ASM-XSPLIT), and the open threads. This issue's reading list stays valid; its state snapshots are historical. The state of record remains the docs/ASM-LOG.md tail.

## Home

`work/issues/`: a spent orchestrator-handoff bookkeeping issue — ASM's ratified v1 scope is code-complete and its residue is now S-MATE's slate, so the record of record is `work/mate/plan.md` and `work/mate/log.md` (with the historical narrative in `docs/ASM-LOG.md`). Closed on migration.
