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
