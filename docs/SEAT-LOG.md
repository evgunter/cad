# SEAT — the verb-seat program (log)

The tail of this log is the program's live status (CLAUDE.md).
Plan: `docs/SEAT-PLAN.md`. Design of record:
`docs/VERB-SEAT-DESIGN.md`.

## PROGRAM OPENED (2026-08-31)

`docs/VERB-SEAT-DESIGN.md` ratified and merged (PR #1388, Evan's
sign-off in-session; the conversation record is on issues #1345,
#1372 and the PR thread). This program executes it under the plan's
wave cut. Ordinal band **1000–1099** claimed in
`docs/MODEL-AB-LOG.md` in this same commit (next free after GAUTH's
900–999).

Orchestration posture for this program, recorded once: the
orchestrator runs in a remote container session — hosted CI is the
gate; implementer lanes are session subagents in isolated
worktrees, one heavy-build lane at a time; away-channel/monitor
arming of the persistent box does not apply. Dispatch protocol is
unchanged (A/B ledger, blinded v6 duals, prompts by path).

Next: SEAT-1 dispatch (difficulty logged pre-draw per protocol;
draw and arm recorded in the ledger at the row's merge, never
here).

## ORDINAL 1000 CLAIMED — SEAT-1 dual (2026-08-31)

SEAT-1 (PR 1399, the band drop) implementation delivered and green on
the drawn lane (default, eps = 1e-6; run 33424552083 — three earlier
reds on the same head were an Actions budget outage, jobs never
started, nothing repushed). v6 dual dispatches on frozen head
0b291b29: parity byte 18 → **R1 opus + R2 fable**. Difficulty S was
logged pre-draw. Full row recorded at merge per protocol.

## SEAT-1 MERGED (2026-08-31, PR 1399)

The band drop landed: the four doors derive Band::linear(tol) at
operation entry like their siblings, 421 call sites followed, the
spacer's friction (3) and diechamfer finding 4's Band half retired.
Dual outcome: ONE bilateral MAJOR (interval-cfg orphans redding the
lint-interval row — both reviewers executed it independently; fixed
with both feature graphs verified and the interval lane ASKED for
at the fix gate), verdict labels divergent at converged findings.
Pair counts toward the twelve; no tally candidate. Full row:
MODEL-AB-LOG SEAT1 (ordinal 1000, sample #77 at merge). Issues
filed at adjudication: 1408 (Band::new spelling class), 1409
(shell's tolerance: f64 + unguarded acceptance), 1410 (stale
citation class). Reviewer probes worth keeping were adjudicated
recorded-not-adopted this unit (the eps=1e308 arm probe — sibling
precedent, per-target CI cost).

Lesson banked for SEAT-2's brief: a compiler-driven cleanup is only
as wide as the graphs actually compiled — briefs now say “verify
under BOTH feature graphs” explicitly.

Next: SEAT-2 (the topo query module + select_where delegation)
dispatches on the block's next slot.
