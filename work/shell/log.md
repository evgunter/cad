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

## SHELL-2 cut and dispatched beside SHELL-1 (2026-09-04)

Unit 4 of the plan order, pulled forward because it touches only
`transform.rs` plus one lane method in `geom-brep/pcurve_cache.rs`
(TRIM's file — one method, four arms; noted for TRIM) and does not
wait on anything in flight. Spec `docs/SHELL-2-SPEC.md` and the item
are on the unit branch `shell/2-transform-approx` (state-sync rides
the unit's PR). Shape chosen and why: 57 caller files, four of them
generic doors, so the certifier joins the lane `transform_rigid`
already binds rather than adding a bound that cascades. Seam with
M10-7 (#1725) recorded in the spec: it adds `Sym` impls to the same
trait; whichever lands second adds the arm. Block SHELL-B1 slot 1 =
OPUS per the draw. Two implementer lanes now share this 4-core box;
briefs say so and ask for narrow build targets.

## Both units delivered; two duals dispatched (2026-09-04)

**SHELL-2 delivered first** (PR #1758, head `b58274d8`, interval lane
asked for, green): the lane on `PcurveFittedLane` as specced, four
explicit arms, `geom::NurbsSurface::map_affine` as the net door,
`ApproxSurface` retired for `ApproxLaneUnsupported` /
`ApproxRecertify`. **The spec's fixture premise was wrong** (mine):
the OFF-C lofted prism cannot be moved at all — its wall seams carry
`Curve3::Nurbs`, refused by the same door (issue record 1346) — and
no Approx-faced body in the tree is both movable and tier-3 clean
(the lane filed `no-approx-faced-body-is-both-movable-and-valid` with
three measurements). Rows 1/2/3/6 landed on a new `box_with_approx_cap`
fixture reading tier 3 as a finding-set difference. Also wrong in the
spec: the proposed `NurbsPlaceholder` wording and the KERNEL-VERBS
row (none exists). Lesson for the next spec: measure the fixture
MOVES before naming it as the acceptance body.

**SHELL-1 delivered** (PR #1756, head `f59f021e`, interval lane asked
for, green): `Shelled<T>` + `ShellNaming` / `RimNaming` /
`ShellRetired`, 142 call sites in 22 files re-spelled (the spec's
"25 files" counted files, not sites), `ring_edges` ⊆ `inner_edges` by
construction (the spec's edge partition was unsatisfiable as written
— corrected by the lane to the true statement), the counterbored drum
refuses `OpenFacesDisconnect` before any rim so a holed extrusion
stands in for the second-hole fixture, and `KfmrhResult::killed_shell`
is the one retirement the record does not carry (flagged for LIB's
emitter). **SEAT's census delivered:** all 142 sites pass a
compile-time literal for `tolerance` (140 × `1e-6`, 2 × `1e-9`);
nothing derives it — recorded for `shell-doors-take-tolerance-beside-tol`.

**Duals dispatched** concurrently on this box (four review lanes,
`-j2` each, the shared-box note recorded as a method note applying to
both arms of each pair): SHELL-1 = ordinal 2300 (byte 99 ⇒ R1 fable,
R2 opus), SHELL-2 = ordinal 2301 (byte 96 ⇒ R1 opus, R2 fable);
claims on main via #1760; briefs hashed under the lane-private
`ab/briefs/`. Implementer wall-clock: SHELL-2 ~1 h 15 m, SHELL-1
~1 h 47 m, no restarts.

## Both duals adjudicated; fix passes dispatched (2026-09-04)

**SHELL-2 (ordinal 2301):** R1 opus APPROVE-WITH-FIXES 0/5/5, R2
fable APPROVE-WITH-FIXES 0/4/3. Convergent: the certificate-agreement
assertion is vacuous at the 1e-9 fixture and rests on a false
sentence (`hull_sup` is NOT a rigid invariant — measured 7.4e-9 under
an oblique rotation; `on_locus_max` is), the window rule has three
copies and `recertify_approx` lacks it (a narrowed-window body passes
tier 3 and refuses at the map), and the fixture's stated reason is
wrong (Chart+IsoLine re-description IS accepted; the one real wall
is check 7's cache need). Unilateral: none demonstrated at MAJOR.
Fix pass: ten items, implementer-inherited.

**SHELL-1 (ordinal 2300):** R1 fable APPROVE-WITH-FIXES 1/2/4, R2
opus APPROVE-WITH-FIXES 2/2/2. Convergent MAJOR: `holes.1` is
documented as a source key and is a result key on every revolve cap
(the spec's own site table carried the wrong key space — mine).
**Unilateral MAJOR, tally candidate:** R2's "the ring rows'
correspondence is unpinned" — a source column rotated by one leaves
all 41 shell rows green (class test-gap, demonstrated by a surviving
mutant); R1 pinned the exact correspondence in its own audit but did
not raise the shipped rows' gap. Fix pass: eleven items,
implementer-inherited.

**Method notes (both arms of each pair equally; no relaxation):** the
briefs named a verdict term v4 §2 does not define ("MERGEABLE WITH
FIXES" for NOT-MERGEABLE-AS-IS) — every reviewer used the doc's
terms; the box hit ENOSPC during three of the four reviews (four
review lanes plus two 5–12 GB implementer targets on a fixed
allowance) — each lane reclaimed only its own tree; SHELL-2 R1 lost
its local C9 check to it and relied on CI's clippy/k-lint rows. Rule
adopted: reclaim a lane's target the moment its report is in, and
drop implementer `incremental/` before dispatching a dual.

**Class findings given a home:** the STEP writer has no
`OFFSET_SURFACE` printer (filed to EXCH:
`approx-face-has-no-step-printer`); `mesh::tessellate` refuses an
`Approx` face without caches (filed to MESH:
`tessellate-refuses-approx-face-without-caches`); a naming record
that re-lists the graft map with the columns swapped and drops its
lookup shape is plausibly `BooleanNaming`'s shape too (both SHELL-1
reviewers, Q1) — logged here for LIB/S-BOOL, not filed: the shell
record gains lookup doors in its fix pass, and whether the boolean's
should is that record's owner's read.
