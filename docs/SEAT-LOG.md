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

## ORDINAL 1001 CLAIMED — SEAT-2 dual (2026-09-01)

SEAT-2 (PR 1521, the kernel query seat) implementation delivered and
green on the drawn lane (default, eps = 1e-12; run 33554063477; one
earlier red was the binding census correctly catching the unrosterd
`query` prelude name — fixed by rostering). The unit's first lane
died at dispatch on a usage limit with nothing pushed (~23h annotated
gap); the re-dispatched lane delivered whole. v6 dual dispatches on
frozen head 5b269616: parity byte 9 → **R1 fable + R2 opus**.
Difficulty M was logged pre-draw. Full row recorded at merge.

## SEAT-2 MERGED (2026-09-01, PR 1521)

The kernel query seat landed: topo owns the materializers, the EXACT
kind predicates and the relocated decided door; select_where
delegates with behavior pinned unchanged — both reviewers' own
execution probes found zero disagreements. Dual outcome: converged
substance (the retired premise surviving in shipped prose; the
citation hand-off list wrong both directions), R2 grading it MAJOR
where R1 graded MINOR — no unilateral, pair counts toward the
twelve. One R1 method correction (background waiter → foreground,
rule 2b). Fix pass took all 8 union items; 15 surviving named
helper scans re-authored; 5 new topo door-contract rows; issue 1527
filed (DatumValue unit-normal enforcement design). Full row:
MODEL-AB-LOG SEAT2 (ordinal 1001, sample #94 at merge). Register
sugar note, consumer-gated, recorded here: no edges_matching(body,
pred) one-call door exists; KERNEL-VERBS:465 now states the
materializer+filter truth.

Next: SEAT-3 (the flush detector at the body seat, retiring the
issue-757 producer gap) dispatches on the block's next slot.

## ORDINAL 1002 CLAIMED — SEAT-3 dual (2026-09-02)

SEAT-3 (PR 1531, the flush detector at the body seat) implementation
delivered and green (lane=default ASKED via trailer, eps drawn; run
33571617460; the one neutral check is main's inherited freecad
render-drift baseline, re-rendered driftless). The cylindrical
measurement answered YES — the Rest ladder already verifies curved
rungs; the detector widening is a reported fork, not taken. v6 dual
dispatches on frozen head 1cf1c377: parity byte 41 → **R1 fable + R2 opus**. Difficulty M was logged
pre-draw. Full row recorded at merge.

## SEAT-3 MERGED (2026-09-02, PR 1531) — WAVE 1 COMPLETE

The flush detector landed at the body seat and the issue-757
producer gap is retired: BooleanDeclarations has a geometric
producer, the two hand-declarer twins are gone, and the anti-twin
rule holds through the one shared verify chain (now stated
correctly — the dual's top finding was the module prose naming the
wrong link). The cylindrical widening is measured, one identifier
away, and deliberately homed at issue 1537 rather than taken. Both
review lanes independently executed the mechanism (round-trips,
band sweeps, differential twins) and could not break it. Full row:
MODEL-AB-LOG SEAT3 (ordinal 1002, sample #97 at merge).

Wave 1 (design §1) is complete: the band drop (SEAT-1), the query
seat (SEAT-2), the flush producer (SEAT-3). Every §1 acceptance in
VERB-SEAT-DESIGN §6's sketch that Wave 1 owns is met; the demo
frictions the montage recorded at the kernel seat are retired at
their sites.

Next: the SEAT-4 spec (docs/SEAT-4-SPEC.md — the Verb substrate
with the blend pair; VS-Q1/Q2/Q5 elaborated from the ratified
recommendations; deviations from those recommendations, if any,
are Evan-gated). SEAT-4 is block SEAT-B1's last slot; its merge
publishes the block-close record and the draw byte.

## ORDINAL 1003 CLAIMED — SEAT-4 dual (2026-09-02)

SEAT-4 (PR 1547, the Verb substrate carried by the blend pair)
implementation delivered and green (lane=interval ASKED via
trailer, eps 1e-6 drawn; run 33591161974). No Evan-gated stop
triggered — the three ledger answers land as recommended; one
deviation (the compound-Bounds allowlist row for verbs/run.rs) is
flagged for retroactive review per the self-merge convention. Pin
evidence includes a differential of the digest suite against
extracted main. v6 dual dispatches on frozen head 6f4fdea6: parity
byte 40 → **R1 opus + R2 fable**. Difficulty L was logged pre-draw.
Full row recorded at merge; block SEAT-B1 closes at that row.

## SEAT-4 MERGED (2026-09-02, PR 1547) — BLOCK SEAT-B1 CLOSES

The Verb substrate landed: crates/verbs owns the canonical blend
vocabulary, editor-core lowers both blends through one generic
wire_blend with behavior pinned end to end (tags, wire format, and
a provenance-extended evaluation digest differentially reproduced
on the merge base). Zero MAJORs from either review arm; the fix
pass extended the digest rather than softening its claim, with
red-first evidence for all three planted mutations, and fixed the
BlendVerb wart the Shell compiler-experiment exposed. The
compound-Bounds allowlist row is flagged for Evan's retroactive
review. Full row + the SEAT-B1 block-close record (draw byte 85):
MODEL-AB-LOG SEAT4 (ordinal 1003, sample #103 at merge).

Next: SEAT-DV (the issue-1527 ruling — DatumValue unit-by-
construction) opens block SEAT-B2; then SEAT-5 (the boolean
migration) and SEAT-6 (the ParamSource channel, with the issue-1372
handoff note to VERBS at dispatch).
