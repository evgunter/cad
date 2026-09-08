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
unfilled (its arm is in the branch-side block record) — the next
dispatch takes it; SHELL-3's spec is a
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


## SHELL-1 dispatched; the clearance-gate fork goes to Ev (2026-09-04)

Opening state-sync merged (#1730). Block SHELL-B1 drawn and recorded
branch-side on the orchestrator branch (SHELL-1's implementer arm is
in its A/B row); lane `shell/1-naming` at
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

## Announced seam from PROPS (2026-09-05): a doc link in `offset_meters.rs`

The coeffs-window unit (PR #1985, merged `55d541ae5`) moved
`geom-core`'s free `hull` doors onto `SplineCoeffs`/`CoeffWindow`; one
intra-doc link in `crates/geom-brep/src/offset_meters.rs` was re-pointed
at the new door. Doc only, no arithmetic; announced here after the fact
because the unit's body omitted it (the review found it). Signed (PROPS
orchestrator).

## Second orchestrator session opens (2026-09-08)

Asked (in-chat) whether this program is ready to be started and, if
so, to prepare to orchestrate it. **It is ready**: every trigger the
close-out entry named has fired — M10-7 (#1725), VERBS-RIMCAP PR-1
(#1674) and VERBS-1031B (#1671) all merged 2026-09-04, and VERBS
closed the same day (walk ratified #1793, tracker retired #1799), so
`offset_axial.rs` joins the territory as `program.md`'s own keep-out
clause said it would (TOPO's `keep_out` still lists it as unowned —
a sentence for TOPO to drop; nothing of TOPO's touches it). SHELL-3's
one remaining input is a seam, not a blocker: PROPS' sign-hull unit
(`props/sign-hull`, dispatched 2026-09-06, resumed 2026-09-07 with
~300 uncommitted lines, no PR yet) retires the planar re-chart inside
`clearance.rs` (~130 lines: `in_plane_axis`, `chart_frame`, the
`chart_axis` fields, the witness `chart=` columns). M10-10 (#2100)
does not touch the file (measured from its file list). The draft
spec is updated to say the move lands after that unit.

**The opening session's branch could not be merged.** Its five files
beyond main (this log's four 2026-09-04 entries, the branch-side
block record, `docs/SHELL-3-SPEC.md`, and the two issues it filed to
EXCH and MESH) were carried onto this session's branch as a patch
after `git merge` produced conflicts in hundreds of unrelated files —
the branch has two merge bases with main (`67271506`, `61792d36`)
and its criss-cross history defeats the merge, not its content.
Nothing is rewritten: the old branch stays as it is, unmerged. The
block SHELL-B1 draw and slot record stay BRANCH-SIDE on this
session's branch (the PCURVE/GUI shape) and reach main when slot 2
concludes; everything else goes to main now on `shell/tracker-sync-2`.
The two issue files were never on main, so until now the EXCH and
MESH findings from SHELL-2's reviews had no durable home; they do now
(`work/exch/approx-face-has-no-step-printer`,
`work/mesh/tessellate-refuses-approx-face-without-caches`).

**Blinding exposure, disclosed.** Slot 2's arm was named in this
log's close-out entry, on main since 2026-09-04, and in the
dispatched entry the opening session kept branch-side. Both mentions
are redacted in this sync (git history keeps them, as with the
PCURVE and LIB-12 redactions); the exposure stood on main for four
days and is flagged on the exposure at slot 2's A/B row, whatever its
reviewers disclose.

**Sequencing decided (a recommendation, not a fork — Ev,
2026-09-06):** the next dispatch is the hollow operand
(`shell-of-hollow-body-thicken-every-boundary`) at block SHELL-B1
slot 2, because it is self-contained in `shell.rs` and does not touch
`clearance.rs`; the nappe home follows and opens SHELL-B2; SHELL-3
dispatches at sign-hull's merge and asks M10 for the co-review then.
The alternative — SHELL-3 now, letting whichever lane lands second
resolve a move against an in-file edit — was rejected as a conflict
nobody needs. The plan's unit order is rewritten to this.

**Environment:** a fresh cloud container (4 cores, 15 GB, ~29 GB
free), the same shape as the opening session: no monitors, no
build-slot mutex, no away channel; lanes are worktrees under
`/home/user/shell-lanes/<lane>/` with private targets beside them,
one heavy cargo job at a time, hosted CI the gate. No lane exists
yet; nothing is dispatched by this entry.

## SHELL-5 cut and dispatched (2026-09-08)

The hollow operand, the plan's next unit: spec `docs/SHELL-5-SPEC.md`,
branch `shell/5-hollow-operand`, block SHELL-B1 slot 2 (the arm is in
the branch-side record). Pre-draw fields M / STRUCTURAL, logged after
the block byte — disclosed on the item. The design decision the spec
binds, made here: the moved clone goes through `insert_void` WHOLE
(every clone shell is strictly inside the material, which is what the
reach decides and the planar gate establish), and a new ownership
re-partition op beside `movefac` moves each operand void and its
dilated twin into a new solid, paired STRUCTURALLY off the graft map —
no classification, no probe. Rejected: teaching `insert_void` to mint
solids (S-BOOL's door, and its contract is "no new solid"), and a
containment-classified distribution (a probe where the construction
already knows the answer). Lane at `/home/user/shell-lanes/shell-5/`
(a worktree of this checkout), private target and scratch beside it.

## SHELL-5 MERGED (2026-09-08, PR #2159 — ordinal 2302, sample #159)

The hollow operand thickens every boundary: one thin solid per operand
shell, re-partitioned out of the void door's graft by the new
`Body::move_shells_to_new_solid` (the TOPO seam announced at #2152),
`OperandAlreadyHollow` retired, `thickened` and `RimNaming::side` on
the record, and one `OffsetDoor` decision both the cavity and the lift
now read. Both reviews APPROVE-WITH-FIXES. **Convergent** (R2 MAJOR,
R1 MINOR, both by execution): the planar clearance gate read the
OPERAND's footprints, and an inward offset extends past a concave edge
by `t`, so two diagonally offset voids — or, pre-existing, a notched
single-shell operand, reproduced by both on the merge base — built
silently with crossing twins and a double-counted volume while the
module docs called the gate sound. Fixed in the pass by growing both
footprints by `t` before the separation decide (over-refuses
convex-edge pairs, never under-refuses; no legitimate fixture or tour
scene refuses), the sentence corrected, four reviewer rows flipped from
pinning the silent build to asserting the refusal. **Unilateral, not a
tally candidate (MINOR):** R2's cross-pairing mutant — each void paired
with the NEXT void's twin — survived all 39 rows, because every
acceptance observable (counts, per-solid roles, volume) is
pairing-invariant; R2's record-reading row kills it and is adopted.
Spec premises this unit falsified, mine: "no flux read decides which
shell goes where" was unsatisfiable for WHICH shell is outer (a solid
stores no outer designation; the pairing is structural as ruled — both
reviewers adjudicated the lane's one `classify_shells` read acceptable);
§1.4 named `movefac.rs` as unowned ground (TOPO's; corrected
mid-flight). Also mine: the dispatch's item 8 guessed a listing-desync
the op does check (R2 n6). Found by CI rather than by anyone's sweep:
LIB-G17's emitter on main folds `ShellError` exhaustively and named
the retired variant — a sweep is accurate at its merge base, and this
one was five days old by the fix pass; the lane folded the three new
variants (a LIB seam, named in the PR).

**Class findings given homes:** tier 3 has no ring-inside-outer check
(`work/topo/tier3-accepts-a-ring-outside-its-outer-loop`, R1's file,
placed by this orchestrator) beside the earlier
`tier-3-does-not-check-shell-roles-per-solid`; the axial door's
one-surface seam corner (`axial-door-refuses-a-one-surface-seam-corner`,
SHELL); `shell_open` on a multi-solid body
(`shell-open-on-a-multi-solid-body`, SHELL — "hollow, hollow, open" is
not three verbs today); the curved window on a hollow operand pinned
by a self-retiring row that reds when SHELL-4 lands; STEP refusing
every hollow body (`BREP_WITH_VOIDS`, EXCH's, pre-existing — the
verb's ordinary output is unexportable, named here for EXCH's board);
the lift's door ladder differed from the cavity's (R2 Q1, a class) —
measured harmless on the oblique prisms and factored into one
decision. Residue `shell-open-on-a-void-face-with-a-hole` closed on
R1's pillar row. Rubric idiom/tests/docs: neither reviewer scored
them; recorded as not scored.

**Block SHELL-B1 concludes** with this merge (ordinals 2300–2302,
samples #121, #122, #159); its branch-side record goes to main in this
sync. The next kernel unit draws block SHELL-B2.


## SHELL-6 cut (2026-09-08)

The nappe home, the plan's next unit: spec `docs/SHELL-6-SPEC.md`,
branch `shell/6-nappe-home`, block SHELL-B2 slot 0 — the block is
drawn AFTER this entry and the item's pre-draw fields (S–M /
STRUCTURAL-NUMERIC) are committed, so this row's covariate is clean.
Decision bound by the spec: the nappe is decided ONCE per face from
its corner stations (`nappe_signed`'s decide, moved) and every reader
— both offset doors, the apex-window gate, `ConeOffset::displacement`
— takes the decided value; `displacement` loses its per-point
`copysign`. Rejected: leaving the mint nappe-blind with a second
per-door turn (the shape 1199 found), and a per-point read (a third
authority). The winding rename stays on its item. Lane at
`/home/user/shell-lanes/shell-6/`, private target and scratch beside.

## SHELL-6 MERGED (2026-09-08, PR #2178 — ordinal 2303, sample #160)

The cone nappe has one home in the offset lane: `topo::offset_nappe`
— `face_nappe` (the face's two extreme corner stations, enforcing the
premise that every corner is on one nappe) and `group_nappe` (a
chart's faces agreed, refusing `NappeStraddles` typed on either
reading), read once per cone chart by both offset doors, the
apex-window gate (one decide on the near end) and
`ConeOffset::displacement` (its per-point `copysign` gone; the nappe
passed in). `nappe_signed` is deleted, `mint_offset`'s caller turns
`d` before the mint, and `d` at the per-chart door now means the same
geometric thing on both nappes — which moved one baseline
(`verbs_offd`'s apex-window crossing row asks for the inward `d` it
always meant; both reviewers re-derived it: the kernel was wrong
before, the row is right now). Both reviews APPROVE-WITH-FIXES.
**What the reviews corrected in the record, by execution:** the
per-chart cone offset is NOT unreachable — `ReanchorOffCarrier`
meters `|d|·sin α` against ε, so below `ε / sin α` the door builds,
and on the merge base it built a body that GREW on an inward request
(R2, a merge-base control: 0.000894822126266624 →
0.0008948221627270332). The defect this unit closes was live at ε
scale, not latent behind a gate as issue record 1199 and both of
#1180's review arms had it; above the threshold it was latent, and a
neighbour that could hold both rims refuses `NeighborPairUnroutable`
first (R1: `intersect::route` has no cone×{sphere, cone, torus} row).
Spec §2.1 is landed at the ε-scale operand (R2's row adopted).
**Unilateral, tally candidate (+1 pending the blinded coding):** R2's
MAJOR — the dropped §2.1 was landable and its premise false — class
test-gap, demonstrated by execution (R1 argued the same
unreachability from the routing table and did not find the
threshold); R2's second MAJOR (the live-defect record) traces to the
same fact and dedups with it. **Convergent:** the `replace_face.rs`
module header still stated the pre-unit contract; `offset_nappe.rs`
claimed to be "the one place" the nappe is read tree-wide (four
other predicates in three programs' files decide the same fact —
filed as `work/issues/cone-nappe-is-decided-in-five-places`); the
first-face-decides premise on both doors with the axial door writing
per chart and no agreement gate (now `group_nappe`); a tautological
assertion. Spec premises this unit falsified, mine: `ApexWindowStraddles`
never existed; the `NappeError` shape (the doors' own error type is
right); the sum of corner stations as the decide (a sum can say
`Opening` for a face with corners on both nappes — R2's fixture; the
extremes decide now). Two K rows moved and re-derived per the
runbook (zero rows in every committed baseline). `offset_axial_side`
adjudicated NOT redundant with `face_nappe` (it meters a corner
against the MOVED apex; the reason is at the site). Friction recorded
in the PR: the axial door's apex-reach refusal talks about a corner
on the axis; `ReanchorOffCarrier` offers no route to the door that
works. Rubric idiom/tests/docs: not scored by either reviewer.

Block SHELL-B2 slot 0 concludes; slots 1 and 2 remain (record
branch-side). Next on the plan: SHELL-3 at sign-hull's merge, and
the follow-ups item 1 (the curved-rim narrowing, a PROPS seam) as
the other candidate for slot 1.


## SHELL-7 cut (2026-09-08)

The one-surface corner, cut from SHELL-5's measured refusal: spec
`docs/SHELL-7-SPEC.md`, branch `shell/7-seam-corner`, block SHELL-B2
slot 1 (the arm is in the branch-side record; the pre-draw fields
S–M / NUMERIC were logged after the block byte — disclosed on the
item). Decision bound by the spec: a vertex all of whose faces lie on
one surface of revolution moves as a point of that surface — the
concentric move on a profile circle, the perpendicular foot on a
profile line — through one `Profile` method both the carried-datum
arm and the new arm call, with the azimuth carried as every seam's
is; the torus latitude seam takes the standard latitude rule at the
carrier mint. Rejected: a torus-only special case at the corner (the
sphere seam and the carried datum already spell the general rule),
and routing the seam through the two-surface machinery it is not.
Sequencing: SHELL-3 still waits on PROPS' sign-hull unit (no PR yet,
branch idle since 01:11 UTC); this unit touches `offset_axial.rs`
only, so it runs now.

## SHELL-7 MERGED (2026-09-08, PR #2200 — ordinal 2304, sample #161)

The axial offset door takes a one-surface corner: a vertex all of
whose faces lie on one surface of revolution moves as a point of that
surface (`Profile::image_of` — the perpendicular foot on a moved line,
the concentric point on a moved circle — one arithmetic the
carried-datum arm and the new arm both call), with the azimuth carried
as every seam's is; every same-surface LATITUDE seam (centre on the
axis, plane normal to it) takes one posture through one predicate
(`offset_axial_centre`, which folded two names for one fact), and the
seam edges' declared rotation re-authors as a `RevolvedPoint` of the
moved corner with the sketch-plane premise decided at the site. The
full-period torus shells, solid and hollow (SHELL-5's measured row
flipped to its closed form); the whole axial corpus is byte-identical
at the true merge base, reproduced by both reviewers on corpora of
their own. Both reviews APPROVE-WITH-FIXES, no MAJOR, no tally
candidate. **Convergent:** the module header filed a fixture-backed
arm under "unreached" and conflated "has a row" with "a door builds
it"; the `(Line, ≥1 meridian)` refusal was reachable by hand and
unpinned; the PR named the wrong merge base; the single-profile
concurrence meter is a tautology (the edge layer is the meter; a
vertex δ off its surface is snapped, bounded by δ); one quantity had
two decide names. **Unilateral, both R1, by execution (MINOR, not
tally):** the seam-posture class was wider than the torus — a
collinear wall vertex makes a door-built one-surface CYLINDER vertex,
and two cocircular arcs make a sphere with a latitude seam — every
seam arm but the torus's certified one posture. The fix pass swept
every seam arm (the table is in the PR) and found two shapes that
stop PAST the seam door: a collinear-cap drum refuses at void
insertion's graft re-certification and the two-arc sphere at the
assembled body's tier 3, both cavities tier-3 valid through the direct
door at their closed forms — pinned as rows, filed
(`void-insertion-refuses-a-cavity-with-a-same-surface-latitude-seam`),
not widened. Spec premises this unit falsified, mine: "every corner
is metered against every moved surface" (not for a one-surface
corner); §2.1's differential instrument as named. Filed by the lane:
`partial-cone-frustum-three-quarter-turn-refuses-edge-disagreement`
(R1's pre-existing find, now pinned) and
`offset-axial-predicates-missing-from-the-dimension-audit`; by this
orchestrator, for TOPO: `split-edge-children-lack-pcurve-rows-on-curved-charts`
(both reviewers, by execution). K rows: two names folded into one,
one new (`offset_axial_reauthor_plane`), none in the committed
baseline. Rubric idiom/tests/docs: not scored by either reviewer.

Block SHELL-B2 slot 1 concludes; slot 2 remains (record branch-side).


## SHELL-8 cut (2026-09-08)

The multi-solid operand, cut from SHELL-5's e2e friction: spec
`docs/SHELL-8-SPEC.md`, branch `shell/8-multi-solid`, block SHELL-B2
slot 2 (the arm is in the branch-side record; the pre-draw fields
M / STRUCTURAL were logged after the block byte — disclosed on the
item). Decision bound by the spec: shell is per solid by definition,
so a multi-solid body shells EVERY solid — gates, door choice, moves,
evidence, insertion and partition each per solid, the simultaneous
doors' coverage precondition relaxed from the body to the solid (a
corner belongs to one solid), and an additive N-ary void-door sibling
(`insert_voids`) using the graft's existing N-ary form. Rejected: a
designation of which solid to shell (no vocabulary for naming a
solid; a user who wants one solid has a one-solid body), and
extracting each solid into its own body (no such door exists and the
clone-and-graft shape already carries N solids). The `insert_voids`
sibling is an announced seam to S-BOOL. Sequencing: SHELL-3 still
waits on PROPS' sign-hull unit (no PR; branch idle since 01:11 UTC).

## SHELL-8 MERGED (2026-09-08, PR #2207 — ordinal 2305, sample #162)

`shell` and `shell_open` apply to every solid of a multi-solid body:
the three whole-body reads are per solid (roles through a new
`props::classify_shells_of` shell-subset entry, the offset door
choice, the simultaneous doors' coverage relaxed from the body to a
`Scope` of the solids a move set touches), clearance skips
cross-solid pairs (no material between two solids), one clone with
each solid's charts through that solid's door, a partition per
operand shell of every solid, and a designation on any solid with the
lift's solid read on the result so a void designation finds its
minted thin solid. `insert_voids` is the N-ary void door (`insert_void`
its N = 1 case, literally the same call chain — S-BOOL's announced
seam); `NotOneSolid` retired to `NoSolid`; `ChartSpansSolids` typed,
and reachable. Both reviews found the same MAJOR by execution: the
PR's byte-identity claim was false — at the true merge base the
`shell8_dump` corpus differs by 6 pcurve-sign rows on the UNSCOPED
solid, R1 attributing it by mutation to the lift's per-solid scope
(at base the lift's move set covered every chart of the result, so
its edge walk re-authored the other solid's edges) — so the spec's
§3 STOP fired by the letter and the PR said none did. Adjudicated an
improvement (the untouched solid is now untouched), re-taken on a
private target, and pinned by a row that goes red under the mutant;
the most likely cause of the first empty diff was the shared-target
hazard serving the other tree's binary again, this time with no
symbol to expose it. Convergent MAJOR ⇒ no tally candidate.
**Unilateral, R1, by execution (MINOR):** the roles read ran once
over the whole body when any solid was hollow — a spec §1.1
deviation the PR did not disclose, and a plain neighbour's escalation
would refuse a hollow solid's shelling; fixed per solid in the pass
(verdicts 0 / 2 / 2). **Unilateral, R2, by execution (MINOR):**
`ChartSpansSolids` is publicly reachable (a disconnecting `subtract`
leaves both fragments under one solid sharing surface keys, and
`move_shells_to_new_solid` re-homes one without re-minting) —
overruling R1's reading that the arm was unreachable and should be
`Corrupt`; the typed arm stays, R2's rows adopted, the producer side
filed for TOPO (`a-chart-spans-solids-after-move-shells-to-new-solid`).
Convergent MINORs: the `Scope` doc and the PR overstated "reads
nothing outside" (construction is a whole-body walk; the moves are
scoped) — corrected, and the remaining whole-body walks filed by the
lane (`shell-doors-still-walk-the-whole-body`); a class of stale
single-solid docs incl. the `Shelled::body` sentence spec §4 named;
`OperandOuterShells` now names its solid. Both reviewers' e2e seats
met the same two frictions — the record does not surface the wall it
built, and nothing names "the inner wall of the second part" after
two hollowings — filed by this orchestrator
(`shelled-result-does-not-name-the-wall-it-built`). Spec premises
this unit falsified, mine: "every producer mints a fresh surface per
solid" (the spec's justification for treating a spanning chart as
corruption); the differential's base as named. Process: the
implementer's agent was killed by a container restart after its
push (its PR body is its report) and resumed from its transcript for
the fix pass; one hosted run died in the artifact store and was
re-triggered by an empty commit because the lane's token cannot
re-run jobs — recorded, not to be repeated (an orchestrator re-run is
the right lever). Seams announced at merge: PROPS
(`classify_shells_of` in `props.rs`, a pure refactor with
`classify_shells` delegating), S-BOOL (`insert_voids`, announced at
cut). K rows: none new in the committed baseline. Rubric
idiom/tests/docs: not scored by either reviewer.

Block SHELL-B2 concludes (record on main with this sync). The next
SHELL kernel unit draws block SHELL-B3. SHELL-3 still waits on PROPS'
sign-hull unit (branch idle since 01:11 UTC).
