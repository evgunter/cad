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

## SHELL-1 MERGED (2026-09-04, PR #1756 — ordinal 2300, sample #121)

The `ShellNaming` birth channel is in: `Shelled<T>` from both doors,
the record written at the doors' own steps, lookup doors, six-arena
retirements. LIB-G17 (`Node::Shell`) unparks — its `blocked_on` was
the GitHub issue number 1202, now closed by this record; LIB's
orchestrator reads the PR body's row table. SEAT's
`shell-doors-take-tolerance-beside-tol` has its census. Both spec
premises this unit falsified (the edge partition, the hole rows' key
space) were mine; the lane reported rather than absorbed them, which
is the posture the discipline asks for.

## SHELL-2 MERGED (2026-09-04, PR #1758 — ordinal 2301, sample #122)

The Approx transform door is in on the lane the door already bound;
the fixture premise my spec named was false and the lane measured
the truth (no Approx-faced body is both movable and tier-3 clean;
issue filed in this program). Three walls a user meets after placing
such a part are recorded: mass properties and tessellation refuse
without caches (MESH issue), STEP has no printer for the kind (EXCH
issue). `transform-rigid-refuses-approx-face` closed.

## Session close-out on Ev's direction (2026-09-04)

Ev asked (in-chat) to start no new work and close out what was
running, usage limit near. State at close: SHELL-1 and SHELL-2
merged with their rows; block SHELL-B1 slots 0–1 concluded, slot 2
(FABLE) unfilled — the next dispatch takes it; SHELL-3's spec is a
DRAFT on the orchestrator branch (`docs/SHELL-3-SPEC.md`, not on
main), dispatch waits for M10-7 (#1725) and M10's co-review; SHELL-4
waits on SHELL-3; unit 2 (nappe home + winding names) waits on
RIMCAP and 1031B; unit 3 (hollow operand) needs a spec — its
consequence, one thin solid per boundary shell, is stated in the
plan. Review-lane worktrees and targets are removed; the two
implementer worktrees stay on their merged branches. The
orchestrator branch (`claude/shell-orchestrator-track-qxa7vk`) holds
the branch-side block record and this log; it merges to main when
the block concludes or at the next orchestrator's opening, whichever
first.

## Announced seam from PROPS (2026-09-05): a doc link in `offset_meters.rs`

The coeffs-window unit (PR #1985, merged `55d541ae5`) moved
`geom-core`'s free `hull` doors onto `SplineCoeffs`/`CoeffWindow`; one
intra-doc link in `crates/geom-brep/src/offset_meters.rs` was re-pointed
at the new door. Doc only, no arithmetic; announced here after the fact
because the unit's body omitted it (the review found it). Signed (PROPS
orchestrator).
