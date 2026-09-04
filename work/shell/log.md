# SHELL log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/shell/plan.md`. A/B band 2300–2399
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose SHELL section is the
charter this plan restates. Opens at VERBS' exit. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `shell-curved-clearance-consumer` from `work/issues/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## Opened ahead of VERBS' exit (2026-09-04)

Ev's direction (in-chat, 2026-09-04): pick up the `shell` track as its
orchestrator now; VERBS stays live on its Wave-2 remainder (CYLSPH in
review, RIMCAP PR-1 open, 1031B in its dual, CONE and C5ARMS PR-2
uncut). The charter's "dispatches at VERBS' exit" is superseded;
`docs/MODEL-AB-LOG.md`'s banding note that SHELL draws no ordinal
before that exit is amended in the same commit.

**Re-homed from `work/verbs/`** by header edit and `git mv` (ids
unchanged): `shell-needs-shellnaming-birth-channel`,
`shell-of-hollow-body-thicken-every-boundary`,
`shell-offset-three-followups`, `mint-offset-ignores-cone-mirror-nappe`,
`transform-rigid-refuses-approx-face`, `tour-hollow-tube-scene`,
`shell-curved-wall-clearance-window` (parked on M10-5, which has
MERGED — the park is stale and is lifted with item 6's conversation).
`tier3-approx-regrid-per-face-cost` stays PERF's.

**Territory** moved in both `program.md`s: SHELL takes
`topo/{shell,replace_face,transform,offset_together}.rs`,
`geom-brep/{offset,offset_meters}.rs` and `sweep/tests/verbs_shell*.rs`.
`offset_axial.rs` stays VERBS' while VERBS-RIMCAP is open — measured:
PR #1674 (`verbs/rimcap-1`) rewrites 485 lines of it; none of VERBS'
or SEAT's open PRs touch the files moved here. A hazard for the first
unit: SHELL-1 changes the shell doors' return type, and RIMCAP's own
tests call `topo::shell`; whichever lands second merges main and takes
the `.body` reads.

**This session runs in a cloud container**, not on Ev's box: no
monitors, no build-slot mutex, no away-channel script. Lanes are git
worktrees under `/home/user/shell-lanes/<lane>/` with their own
`CARGO_TARGET_DIR` under `/home/user/shell-lanes/<lane>-target/`
(never the orchestrator's checkout, never a shared target); the box
is 4 cores / 15 GB, so ONE heavy cargo job at a time and hosted CI is
the gate. A cold `cargo build -p sweep -p topo --tests` was timed at
opening to price local iteration (result in the SHELL-1 dispatch
entry).

**First unit cut: SHELL-1** (`shell-needs-shellnaming-birth-channel`),
spec `docs/SHELL-1-SPEC.md`, branch `shell/1-naming`. Pre-draw
difficulty **M**, task class **STRUCTURAL** (a record filled at the
doors' own steps plus a mechanical return-type sweep over 25 caller
files). The `[ev]` conversation for item 6 (where the curved
wall-clearance gate lives) opens beside it rather than after items
2–5 — it is Ev-paced and its inputs (M10-5 merged, M10-7 in flight on
parameter-aware certification) are on the table now.

## SHELL-1 dispatched; the clearance-gate fork goes to Ev (2026-09-04)

Opening state-sync merged (#1730). Block SHELL-B1 drawn and recorded
branch-side on the orchestrator branch (fable at slot 2 ⇒ SHELL-1's
implementer is the OPUS arm); lane `shell/1-naming` at
`/home/user/shell-lanes/shell-1/cad`, own target dir, dispatched
against `docs/SHELL-1-SPEC.md`. A cold `cargo build -p sweep -p topo
--tests` on this box takes ~47 s, so local iteration is cheap.

**Branch layout, decided:** the orchestrator branch holds the
branch-side A/B records and is merged only when a block concludes;
anything that must reach main earlier goes on its own branch off
main (the `[ev]` PR below is cherry-picked onto
`shell/ev-clearance-gate` for that reason — the duplicate commit is
harmless at the block-end merge).

**`[ev]` #1737 opened** for item 6, `shell-curved-clearance-consumer`
(`needs_ev`), with `shell-curved-wall-clearance-window` re-parked on
it (its M10-5 park had lapsed). The question is measured in the item:
the curved gate is E7's self-intersection question asked of the
cavity clone; the engine's inner half is already body-level; no
scalar remap of a body exists, so the certified gate lives either in
`shell::<Interval>` with the engine's inner half moved below
editor-core (B, recommended) or in the driver's replay after
LIB-G17 (A). Waits for Ev's sign-off; not self-merged.

**Sequencing measured for units 2–4:** unit 2's nappe half touches
`offset_axial.rs` (`nappe_signed`), VERBS' until RIMCAP merges, and
its winding-rename half gains a fourth site from VERBS-1031B, so it
waits for both PRs; unit 3 rewrites `shell.rs` and waits for SHELL-1;
unit 4 (`transform-rigid-refuses-approx-face`) touches
`transform.rs` only and is the next spec to write.
