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

## Stock-take after SHELL-8, and SHELL-9 cut (2026-09-08)

SHELL-3 still waits on PROPS' sign-hull unit (no PR; branch idle
since 01:11 UTC), and SHELL-4 on SHELL-3. Of the followups, two were
weighed and placed rather than cut: the three-quarter-turn cone
frustum's `TogetherEdgeDisagreement` is the cone-hyperbola class the
axial door already names in its `Line` arm and pins on a conical
wedge (an offset meridian cap cuts a cone in a hyperbola; the
kernel routes conics to rung 3 at C5 R1, permanently until a PR
moves it) — the item's premises corrected (the quarter-turn `wedge`
is a cylinder) and parked on the new design question
`offset-lane-has-no-conic-carrier`, Ev's fork, not a SHELL unit
(#2219); and the same-surface latitude-seam void refusal from
SHELL-7 was DIAGNOSED by a lane before any cut (branch
`shell/9-probe`, four rows, ~12 min): two defects, not one. The
collinear-cap drum fails inside `Body::revert` — a plane's normal is
negated with `u_ref` fixed, which mirrors the plane chart, and
neither the `Chart` images nor the cache rows on that plane are
transformed, so a same-plane `Chart` circle fails `ChartResidual` on
the reverted body — a class that reaches boolean subtract's
`revert(B)` on a split planar face, so TOPO's:
`work/topo/revert-does-not-mirror-plane-chart-images` (#2221). The
two-arc sphere fails because `shell` never runs the closing pcurve
mint the posture table says every producer runs after the void door
`Transfers` rows — `topo::mint_pcurves` on the assembled body makes it
tier-3 valid at `4/3·π(r−t)³`. That half is SHELL's, small and
exact, and is **SHELL-9** (`docs/SHELL-9-SPEC.md`, branch
`shell/9-closing-mint`): the verb runs the mint once before its
closing validate, the sphere row flips to its closed form, the drum
row stays refusing and names TOPO's item, and the whole corpus's
cache rows are diffed at the merge base and head (a row that moves
on a tier-3-valid body is a finding for TOPO's pcurve pass).
Rejected: making `insert_voids` `Maintains` (a posture-table change
in S-BOOL's and TOPO's files; the void door does not finish the body
it would mint). Alternative not taken: waiting idle for sign-hull —
the fix flips a closed-form row now and opens block SHELL-B3 on a
clean covariate (pre-draw S / STRUCTURAL logged here, before the
byte). Lane at `/home/user/shell-lanes/shell-9/`, private target and
scratch beside.

## SHELL-9 MERGED (2026-09-08, PR #2223 — ordinal 2306, sample #165)

`shell` and `shell_open` run the closing pcurve mint once on the
assembled body before the closing validate — the producer's half of
the posture table's `insert_voids` `Transfers` contract, which the
boolean and the revolve kept and the verb never did, so the reverted
cavity's rows rode into the result as they were. The two-arc sphere
shells to `4/3·π(r³−(r−t)³)`; the drum still refuses at the void
door, naming TOPO's `revert` plane-mirror item as its cause; the
refusal is `ShellError::Pcurve`, typed and — every committed fixture
says — unreachable. Every body that shelled at the true merge base
carries bit-identical rows, measured by the unit's instrument and
by both reviewers' corpora. Both reviews APPROVE-WITH-FIXES.
**Unilateral, R2, by execution (MAJOR — tally candidate with a
dedup caveat):** the closing mint LAUNDERS a wrong-content operand
row — a vessel with one face's row attached to another face's
half-edge fails tier 3 on its own and shells to a tier-3-valid body
— a class wider than the missing-row instance the PR disclosed (and
R1 noted at NOTE, unscheduled). No operand gate was added: the
convention is kernel-wide (thirteen producers spell the same closing
mint, none gates its operand's rows), so whether a producer should is
a posture-table decision — filed by the lane as
`shell-launders-a-stale-operand-row` with R2's rows as pins, and by
this orchestrator for TOPO as
`producer-closing-mint-is-a-convention-with-thirteen-copies`.
**Convergent:** the mint's position is not load-bearing (moved before
the partition, both lanes' suites stay green — said at the site with
what would pin it); the corpus instrument asserted nothing; two
spellings of one row dump; the header's accumulation; the
differential's base was the cut point, not the merge base (both
reviewers re-took it, same result); `validate_pcurves` is not
exported, so a consumer reads pcurve findings only out of tier 3's
error list. Spec premises this unit falsified, mine: the closed form
named the cavity's volume where the thin solid's was meant, and its
digits were the measured value, not the f64 closed form. Seams
announced at merge: TOPO (two doc lines in `pcurves.rs`), LIB
(`editor-core`'s and `pncad-py`'s exhaustive folds gain the arm —
the second appeared on main mid-pass and cost one red round).
Process: the unit was preceded by a diagnosis lane (no arm, ~12 min)
that split the item in two; the cut commit carries no orchestrator
trailer (an omission). K rows: none. Rubric idiom/tests/docs: not
scored by either reviewer.

Block SHELL-B3 slot 0 concludes; slots 1 and 2 remain (record
branch-side). SHELL-3 still waits on PROPS' sign-hull unit.

## SHELL-10 cut (2026-09-08)

The doors' remaining whole-body walks, SHELL-8's disclosed item made
concrete by SHELL-9's cost count (N + k + 1 whole-body mints on an
opened N-solid body): spec `docs/SHELL-10-SPEC.md`, branch
`shell/10-scoped-walks`, block SHELL-B3 slot 1 (the arm is in the
branch-side record; the pre-draw fields S–M / STRUCTURAL are logged
after the block byte — disclosed on the item, as every non-first
slot's are). Decision bound by the spec: each simultaneous door reads
exactly its scope — the partition built from the named solids'
shells, the pcurve pass over the scope's faces through an additive
`mint_pcurves_of` (TOPO seam), the closure check over the scope's
shells through an additive `validate_closed_of` or the per-shell
machinery that exists (never a second validator), with SHELL-9's
cache-row instrument and SHELL-8's body dumps as the two
differentials. Rejected: leaving the closure check whole-body as "a
read" — a read that refuses is a write to the caller. Alternative not
taken: idling until PROPS' sign-hull lands (branch idle since 01:11
UTC, no PR) — the block has two slots open and this is the last
in-fence kernel item with its evidence already built. Lane at
`/home/user/shell-lanes/shell-10/`, private target and scratch beside.

## SHELL-10 MERGED (2026-09-08, PR #2229 — ordinal 2307, sample #166)

The two simultaneous offset doors read their scope: the partition is
built from the named solids' shells, a moved face's solid is read in
two hops, `re_scope` rebuilds when aimed at a solid it does not hold,
and each door closes with `pcurves::mint_pcurves_of` over the scope's
faces (an additive TOPO entry sharing `mint_faces` with the
whole-body pass, which keeps the opening `clear()` that alone drops
rows on dead keys). The closure check could NOT narrow: the spec's
STOP fired — five of tier 1's thirteen passes count owners or
refcounts arena-wide and no per-shell entry exists — and the lane
filed it (`doors-still-read-the-whole-body-for-tier1`) and found,
beside it, that the attach layer's setters run a whole-body tier-1
`validate` as a postcondition on every write: 18 per scoped planar
call, 16 axial, a PANIC under the release profile's
`debug-assertions = true`, reachable through a public door on a
malformed out-of-scope solid. Both reviewers reproduced the count and
called it TOPO's own finding; placed by this orchestrator as
`work/topo/attach-postconditions-validate-the-whole-body-and-panic`.
Both differentials (SHELL-9's cache rows, SHELL-8's dumps) are empty
at the true merge base, on the unit's corpora and on both reviewers'.
Both reviews APPROVE-WITH-FIXES, no unilateral MAJOR in class code,
no tally candidate. **Convergent, and what the unit had to say
plainly:** the doors are still O(body) — the setter walks, three
whole-arena decide iterations, the clone and tier 2 — so the direct
door on one of N solids scales linearly with N on both trees and the
narrowing is invisible in cost; the unit's cost table was inside the
instrument's noise (1–7% spread, two rows slower on one lane's run)
and is withdrawn to the one separable row, the §2.4 STOP declared
undecidable by that instrument, and the reads account stated once in
`Scope`'s doc (R1 rated the doors' false "one whole-body read left"
sentence MAJOR — class doc). Also convergent: `mint_pcurves_of`
cannot hold the `Maintains` posture for a caller that kills
half-edges (two dead-key rows survive it, invisible to tier 3; the
contract is now stated true and the guard's blind spot named);
`re_scope`'s rebuild arm was unreachable and unpinned (kept, now
pinned); the doors no longer launder an out-of-scope half-minted face
(intended — two items' citations corrected). Spec premises this unit
falsified, mine: that the closure check could be narrowed without a
second validator; that a cost row could decide the STOP on this box.
Seams announced at merge: TOPO (`mint_pcurves_of`, a `DECLARED` row,
a `review_m1_pr5_internal::ALLOWED` row, the export). K rows: none.
Rubric idiom/tests/docs: not scored by either reviewer.

Block SHELL-B3 slot 1 concludes; slot 2 remains (record branch-side).
SHELL-3 still waits on PROPS' sign-hull unit.

## Second session close (2026-09-08, ~23:40 UTC)

Six units landed in this session — SHELL-5, 6, 7 (block SHELL-B1
concluded, SHELL-B2 opened), SHELL-8 (SHELL-B2 concluded), SHELL-9
and SHELL-10 (SHELL-B3 slots 0 and 1) — samples #159–#161, #162,
#165, #166; tally candidates: one at SHELL-6 (R2's, the per-chart
cone door) and one at SHELL-9 (R2's, the laundered operand row, with
a dedup caveat). **Block SHELL-B3 stays open at slot 2** (the arm is
in the branch-side record on the orchestrator branch); it fills with
the next SHELL kernel unit. What that unit is: **SHELL-3** the moment
PROPS' sign-hull unit merges (branch `props/sign-hull`, idle since
01:11 UTC, no PR), then SHELL-4. Nothing else in the fence is a
kernel unit ready to cut on its own evidence: the naming-record gap
(`shelled-result-does-not-name-the-wall-it-built`) has several viable
shapes and is Ev's to weigh in on before a spec; the conic carrier
(`offset-lane-has-no-conic-carrier`) is a C5 R1 fork for Ev; the
laundering posture (`shell-launders-a-stale-operand-row`) is TOPO's
posture-table decision; the dimension-audit and no-approx rows are
docs and test hygiene, not block slots; the followups' winding rename
is three owners' and waits on its announcements. Placed for other
programs this session: TOPO —
`tier-3-does-not-check-shell-roles-per-solid`,
`tier3-accepts-a-ring-outside-its-outer-loop`,
`split-edge-children-lack-pcurve-rows-on-curved-charts`,
`a-chart-spans-solids-after-move-shells-to-new-solid`,
`revert-does-not-mirror-plane-chart-images`,
`producer-closing-mint-is-a-convention-with-thirteen-copies`,
`attach-postconditions-validate-the-whole-body-and-panic`; BOOL —
`subtract-of-a-hollow-operand-files-the-island-under-one-solid`,
`boolean-mod-doc-links-a-feature-gated-variant`; issues —
`cone-nappe-is-decided-in-five-places`. Seams announced: S-BOOL
(`insert_voids`), PROPS (`classify_shells_of`), TOPO (`pcurves.rs`
doc lines, `mint_pcurves_of` and its registry rows), LIB
(`ShellError` arms in both exhaustive folds). The orchestrator branch
`claude/work-shell-readiness-y31rxk` equals main plus the SHELL-B3
draw and slot lines; a successor starts from it, reads this entry,
and cuts SHELL-3 into slot 2 when sign-hull lands (ask M10's
orchestrator for the co-review at dispatch, per plan item 5).

## The tier-versus-engine row arrives from M10 (2026-09-13)

M10 closed at its exit sweep (`docs/DOC-LEDGER.md` sweep 13) and
`symbolic-tier-and-clearance-engine` came here by header edit and
`git mv` (id unchanged): `editor_core::measure::MinClearanceLane` is
the one lane trait the E12 symbolic tier cannot replay, because
`min_separation` is written at `geom_core::Interval` CONCRETELY in
`crates/editor-core/src/clearance.rs` — this program's file — and
`topo::Body`'s `SlotMap` arenas admit no scalar remap. **SHELL-3 is
the same question from the other end**, so the row belongs on that
unit's slate.

**The co-review pointer above has moved.** The entry that cut SHELL-3
says to "ask M10's orchestrator for the co-review at dispatch"; M10
has no orchestrator now. The tier side is SYM's (`work/sym/`) and the
`clearance.rs`/`measure.rs` seam is PROPS' — ask those two.

## Announced seam from TOPO (2026-09-13): one comment in `shell.rs` with the ring-nesting unit

TOPO's `tier3-accepts-a-ring-outside-its-outer-loop` (branch
`topo/tier3-ring-nesting`) adds the ring-inside-outer decide to tier 3.
In SHELL's `crates/topo/src/shell.rs` it corrects ONE comment — the
role assignment's "the disjointness check below and tier 3's windings
are what verify it", which the placed row shows overclaims — and
nothing else: whether `shell_open`'s glue adopts the validator's
decide as a second precondition is SHELL's call, and the lane reports
rather than does it. `encloses` stays where it is. Signed (TOPO
orchestrator).

## Announced seam widened (2026-09-14): a second comment in `shell.rs`

The ring-nesting unit's fix pass adds ONE more prose site in
`crates/topo/src/shell.rs` and still no code. The module header's
sentence *"the invariant is stated once more at rest by tier 3's check
9 (`ValidationError::RingMeetsOuter`)"* named one variant where there
are now two, so a paragraph beside it states check 9's other half
(`RingOutsideOuter`, with `RingNestingUndecided` for the pair it cannot
certify) and what shapes that half reaches.

The `(host, guest)` comment itself was also corrected, because the
sentence this lane first wrote there was FALSE on its shape: both
blinded reviews installed the SHELL-5 R1 mutant and found that
`shell_open` refuses an inverted pick with `ShellError::Corrupt` from
the naming record's `ring_rows` walk, before the verb's closing
`validate_geometric` is reached at all. The comment now says that —
the record builder is what refuses an inverted pick through this verb
today, check 9's nesting half is what makes the class loud at rest and
elsewhere, and nothing in the verb relies on the arm.

Whether the glue should adopt the nesting decide as a second
precondition is still SHELL's call and still untouched; `encloses`
stays where it is. Signed (TOPO, the ring-nesting lane).

## Announced seam from TOPO (2026-09-14): one clause in `shell.rs` with the revert-wrap unit

`crates/topo/src/shell.rs`'s "The closing mint" paragraph said the
verb's final `mint_pcurves` is "also what re-parks a periodic chart's
loop wrap where the reversed walk needs it". `Body::revert` re-parks
the wrap itself now (each curved loop's anchor moves to its source
predecessor, so the wrap sits at the reversed closure — the anchor
bullet in `revert`'s module docs), so the clause was deleted; the
paragraph's claim that the closing mint discharges the graft's
`Transfers` row over the whole merged body stands as written, and
nothing else in the file moved. Signed (TOPO, the revert-wrap lane,
`topo/revert-reparks-the-wrap`).

## Announced seam from TOPO (2026-09-14, the revert-wrap fix pass): one doc clause in `transform.rs`

`crates/topo/src/transform.rs`'s sense-invariant note on `map_surface`
names the obligation an orientation-REVERSING map would have — flip
`sense` on every face. `Body::revert` now also moves every loop's
`Cycle::first` to its source predecessor (the anchor is where a
periodic chart's loop wrap is reported, so a reversed cycle keeps it
at the closure only if the anchor moves), and a mirror would reverse
every cycle the same way. The tripwire's sentence gained that clause
and a pointer to `LoopBoundary::Cycle` and `Body::revert`; no code in
the file moved (`det = +1` is still enforced upstream). Signed (TOPO,
the revert-wrap fix pass, `topo/revert-reparks-the-wrap`).

## Rows arriving from FIX, 2026-09-20

Ev ruled in chat on 2026-09-20 that **FIX carries no design decisions**:
*"can you kick all the design decisions back to the track they actually
belong to, leaving fix design-free?"* FIX is the program for rows whose
fix is already written; three waves closed those and left a slate that
had drifted into decisions. Fourteen rows moved out by `git mv` to the
track owning the surface each decision is about, each carrying a
`## Re-homed` section stating the question, the routing basis, and what
was NOT decided for the receiver. Routing was taken from
`python3 scripts/work.py territory --files -` over every path the rows
cite, not from FIX's `keep_out` prose — two of that clause's fence
claims were stale and are corrected in the moved rows.

**One row: `plain-transform-rigid-still-refuses-the-m7-8-class`.**
PR 2418 gave the kernel a door for a body it certifies at rest —
`topo::transform_rigid_via` with `geom_brep::plane_nurbs_limbs`, and the
mint-side twin `EdgeCurve::certify_via`. The plain `transform_rigid`, the
door a caller reaches for first, **still refuses that body typed** with
`CertifyError::Unimplemented`, and PR 2418 pins exactly that.

It lands on SHELL because `crates/topo/src/transform.rs` is yours by
`paths`, and because your slate already holds this row's siblings:
`transform-rigid-refuses-approx-face` (the Approx arm of the same two
matches, which FIX's `keep_out` named as yours) and
`no-approx-faced-body-is-both-movable-and-valid`.

**Three options, none pre-empted.** Ev was offered the ruling in chat on
2026-09-20 and sent it here instead: (1) a `transform_rigid_certified`
convenience door, which needs a compound-bound **ratification** in
`crates/geom-core/src/real.rs` — PROPS's ground, and Ev's call, not a
lane's; (2) signpost at the plain door, naming `transform_rigid_via`
there — check whether 2418 already did this before assuming it did not;
(3) decide the asymmetry is correct and say so once, which is 2418's own
argument and closes the row with a sentence.

**Why the parent item's prescribed fix is unavailable**, established by
2418 rather than assumed: raising the bound to `T: Decide + CertifiedBounds`
does not compile — `transform_rigid`'s caller chain runs through
`verbs::Verb`'s blanket impl to `evaluate::<Dual64>`, and no `Dual`
implements `CertifiedEnclosure` — **and** it would violate the
discriminator Ev ratified on 2026-08-29 and recorded at `real.rs:1140`:
*"the discriminator is that nothing generic calls this door"*.

Its sibling at the other door, `graft-recertifies-through-the-narrow-lane`
(`boolean/combine.rs`, `T: Decide`, reachability NOT established), went to
REACH in the same sweep. Whichever of you rules first, the other should
read that ruling rather than re-derive it.

Signed (FIX orchestrator).

## Announced seam from FIX (2026-09-21) — PR 2948

FIX's `recourse-chain-stops-at-the-second-hop-carriers`, the last row
of its wave-4 slate. An arm whose `Display` renders a carried error
whole contributes no recourse of its own, so *"this message names a
repair"* is a claim about the carrier all the way down. Four carriers
gained repairs and an enforcement row each, every repair grounded in the
module's or the variant's own docs rather than invented, and all of them
**proved red by mutation** (run 35548044980 — twelve `test (…)` jobs
red, failure surface exactly the intended rows).

**Your ground:** `crates/geom-brep/src/offset_meters.rs`, which
territory names as ENCL'"'"'s, OFFSET'"'"'s and SHELL'"'"'s together.
`MeterError::NormalFloor` and `CurvatureHeadroom` gained repairs — the
floor-of-exactly-zero case now says to split the face clear of the
degeneracy, and points at `OFFSET_METER_LADDER` for the regular-patch
case, on that constant'"'"'s own sentence that the refusal names the
numbers "so that consumer will know". `Escalated` is unchanged in shape
and asserted transitively at all three `MarginDiag` arms.

Signed (FIX orchestrator).

## 2026-09-25 — note from S-DUP: #3151 edited `offset_together.rs`

S-DUP's PR #3151 (merged) touched shell's ground in
`crates/topo/src/offset_together.rs`: `Scope`'s doc now states the
setters' cost as the code has it under `begin_surgery()` and cites
`topo::separation::SolidOwners` as the other owner index (they agree on
every tier-1 body; `separation::owner_index` reds if they stop), and
`scope_walks` absorbed the two rows of the deleted
`crates/topo/src/shell10_r2_probes.rs`. No behaviour of the offset door
changed. The row that asked for this is
`work/dup/two-spellings-of-the-face-to-solid-owner-index.md` (closed).

Signed (S-DUP orchestrator).

- 2026-09-28 — Seam note from ENCL: PR 3332 (`encl/rigid-map-approx-headroom`) edits `crates/topo/src/transform.rs`'s `map_approx`. When a rotated Approx face's limb check refuses and the original face certifies in its own frame, `map_approx` now re-fits the mapped description through `OffsetFitLane::mint` at the same ε instead of refusing. Every map that succeeded before still ships the image bit for bit. Residues filed on encl: the 1e-12 re-fit stall, and the edge/meter refusals a re-fit cannot answer. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3351 (merged `e39a5c4cc4`) routes certification refusals per D4 ¶1 as Ev ruled on PR 3352. Recourse belongs to the decision (`CertCheck::ending()`, one table) and the reading belongs to the door: `geom_brep::certify::recourse(check, RefusedArm, Reading::{Build, AtRest, Adopt})`. `CertifyError`/`PlaneNurbsRefusal` `Display` is now payload-only. Each door appends `ending(reading)`. The `certification: ` prefix is gone. `transform.rs`: `TransformError::Certify` appends the Build-reading ending. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3382 (merged `9bf495c768`) adds `geom_brep::recourse`, the one table for sized decisions. `Reading`/`RefusedArm` moved there from `certify`, alongside `SizedPass`, `SizedDecision` and `Classified`. certify and the offset meters both route through it. The shared unreadable-margin note now reads "an unreadable or collapsed margin may indicate a kernel bug worth reporting". `offset_meters.rs` (shared claim): meter refusals end in their decision's recourse; the Shell route's zero-floor row is 74 words. `transform.rs`: `Reading` path only. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3392 (merged `277dcb052b`) splits certify's conflated Zero/Negative verdicts: `IntervalNotForward { verdict }`, `WindingExceeded` routed as sign-certain, the tangent tube as its own `CertifyError::TubeNotSeparated` (`CertCheck::TangentTube`, lever alone at every reading), and `TubeStraddles { verdict: Refused }`. `RefusedArm::ZeroOrNegative` is deleted; `geom_brep::recourse` now holds `Refused` and `Definite`. A zero span stays a defect at every reading (Ev, e1600790f9). `offset_meters.rs`: `Refused` moved to `geom_brep::recourse` (import path only). (ENCL orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3390 (merged `719ef596a1`). The shell volume-sign decision is sized (`geom_brep::recourse::SizedPass::NonZero`, the offer valued at |m|/K). `ShellClassifyError` gains a `Straddles` arm and a `payload()` data view, and its `Display` ends in its decision's one ending. `recourse::UNREADABLE_MARGIN_NOTE` is the shared unreadable-margin sentence. `ShellError::Roles` now renders exactly one recourse through `ShellClassifyError`'s Display. The row `shell-roles-refusal-ends-with-no-recourse` was deleted as moot. (ENCL orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)

- 2026-09-29 — Seam note from ORIGIN: PR 3430 (Ev's re-ruling on PR 3412: a chart is body-wide) rewired the shell door to group each solid's own faces through `topo::chart_groups::ChartGroups`; `ShellError::ChartSpansSolids` retired, `ChartSenseMixed` and `OpenFaceChartPartial` now scoped to one solid, and `replace_faces_offset`'s `SharedSurfaceKey` to the group's solid. SHELL-8's disconnecting-subtract slab, moved to two solids, now thickens. (ORIGIN orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `sweep/tests/verbs_shell.rs`, `topo/src/offset_axial.rs`, `topo/src/offset_together.rs`, `topo/src/replace_face.rs`, `topo/src/shell.rs`. `offset_axial.rs`, `offset_together.rs` and `replace_face.rs` state each re-charted face's own bit (the door kept it before); `shell.rs`'s rim glue states the host's bit in its `mfkrh` spec and its post-hoc `set_face_sense` is gone; `sweep/tests/verbs_shell.rs`'s mixed-sense row states the inner wall's own `false` so the chart stays mixed. (TOPO implementer)
- 2026-09-30 — Seam note from TOPO: PR 3513 (branch `topo/every-escalation-names-its-decision`) adds `SectionError::RadiusEscalated`; `replace_face` maps it to `ReplaceFaceError::Escalated` as before. (TOPO implementer)

## Note from CLEAVE (2026-10-01)

`plain-transform-rigid-still-refuses-the-m7-8-class` moved to CLEAVE,
id unchanged, carried by `graft-recertifies-through-the-narrow-lane`
on `cleave/nurbs-lane`. CLEAVE's designer pair weighed the class across
both doors. The convergent answer is that `transform_rigid` reads the
NURBS lane from `AtRestPolicy`. That needs no compound-bound
ratification; the row's premise that it did was the row author's.
— (CLEAVE orchestrator)

## Third session opens; triage and the priority-seam cut (2026-10-06)

The track carried 66 points against 30, with six unpriced rows and
four on legacy `D`. Three read-only triage lanes checked all 27 live
rows against today's tree; the orchestrator re-verified each closure
before acting on it.

- **Closed, already fixed or duplicated:** `no-approx-faced-body-…`
  (legs 1–3 lifted; leg 4 is EXPORT's `approx-face-has-no-step-printer`),
  `shell-mouth-chart-designated-in-full` (8c9e6968 builds the mouth as
  one face), `void-insertion-refuses-…-latitude-seam` (both halves
  landed), `shell-offset-three-followups` (item 1 re-filed as
  `shell-open-refuses-a-curved-designated-face`, item 2 dissolved into
  `loop_winding.rs`, item 3 rides SHELF's door re-shape).
- **Re-banded:** `shell-launders-a-stale-operand-row` P0 → P1/M,
  re-scoped to the shell door, riding the inside-out-operand row (one
  `AtRestBody` unit); `shelled-result-does-not-name-the-wall-it-built`
  P0 → P3/E (`ShellNaming::inner_of` and `thickened` now name the wall);
  the clearance-footprint, lofted-wall-seam and replace-face-refusal
  rows priced P1; the antiparallel row P2/E.
- **Moved:** to CLEAR, `shell-curved-wall-clearance-window` (SHELL-4
  closes it) and `violated-witness-can-sit-off-the-trimmed-face`
  (P3/E; mostly fixed). To a new sibling **SHELF** (22 points, P2
  spine), the offset-door re-shape, the three `transform.rs` rows, the
  `replace_face.rs` hygiene rows, the E test/walk rows, the corpus
  hold-out (TCOST's ground; TCOST is at its ceiling), and the parked
  cone frustum. OFFSET was told its conic-carrier row is an Ev fork
  with no `design` flag.
- **Filed:** `shell-open-refuses-a-curved-designated-face` (P1/H,
  design), `shelf/replace-face-reads-an-approx-iso-as-u-fixed` (P3/E).

SHELL now carries 28 points, as the seven units in `plan.md`.
Pole-touching ball: the premise drifted with f28c201d, so its lane
measures before it fixes.

## Dispatch (2026-10-06, ~05:57 UTC)

The cut merged as PR 4109. Four implementer lanes run as their own
cloud sessions (this container holds one heavy build at a time), each
on its own `shell/` branch, each a **single full review** at review
time — every one of them carries a meaningful chance of a correctness
bug (a P0 measurement, a door type change, a gate's soundness, a
certified projection):

- unit 1 `shell/pole-ball` — the pole-touching ball, measure first;
- unit 2 `shell/operand-at-rest` — `AtRestBody` operand (two rows);
- unit 3 `shell/planar-gate-misses` — footprint arcs + antiparallel lever;
- unit 4 `shell/lofted-wall-seam` — certified NURBS/ellipse re-anchor.

Unit 7's designer pair (`shell-open-refuses-a-curved-designated-face`)
runs locally; blinding byte on `analysis/design-fork/shell-curved-designation`.
Units 5 and 6 wait for a lane: 5 follows 4 in `replace_face.rs`.

## Unit 7 weighed (2026-10-06)

The designer pair on `shell-open-refuses-a-curved-designated-face`
converged in round 1 on the main question: no kind gate, and the rim
takes its chart's form (a seamed band on a wrapping periodic chart, a
ring on a window). The decision and the spec basis are in the item. A
reporting sub-question crossed twice and then converged on its root
(the tier-3 verdict carries the violates/undecided class), which is
RESTFRONT's ground; the evidence went onto that program's row. No
ratified text moves and no `[ev]` PR: the recommendation is clear and
is an elaboration of D1's seam convention. Design-fork log: no row,
because nothing went to Ev. The blinding byte stays on its analysis
branch. Unit 7 dispatches after units 2 and 3 land, since all three
edit `shell.rs`.

## Reviews dispatched (2026-10-06, ~07:42 UTC)

Units 1 (PR 4111) and 2 (PR 4112) have delivered. Both are red only on
the inherited `pinch_faces_tessellate` ε = 1e-6 row, which is JOIN's,
was noted on its log, and does not block merge (annotated at merge).
Each gets a single full review as its own cloud session, posting to
its PR. Units 3 (PR 4115) and 4 (PR 4117) are still in their lanes;
4117's first red was its own (a `NurbsLane` field the census helper
did not compare).

## Unit 1 MERGED (2026-10-06, PR 4111)

The pole-touching ball no longer refuses. The item closes on asserting
rows; the PR has no kernel change. Single full review:
APPROVE-WITH-FIXES, no MAJOR. The fix pass:
- builds the ball from `test_support::ball_poled_y`;
- drops the decorative seam axis;
- corrects the stale two-arc prose;
- uses one tolerance rule;
- corrects the pinch item's cause to `ci.yml`'s `eps_extra` path rule.
The CI gap that rule leaves was already CIW's
(`a-new-test-file-outside-the-eps-crates-never-runs-at-the-extra-eps-rows-before-merge`),
so the lane added evidence there instead of filing a duplicate. Merged
over the inherited `pinch_faces_tessellate` ε = 1e-6 red, which is
JOIN's `pinch-tessellate-row-escalates-at-eps-1e-6` (filed here), as
the merge rules allow.

## Unit 2 MERGED (2026-10-06, PR 4112)

The shell doors take an `AtRestBody`. Single full review:
NOT-MERGEABLE-AS-IS on one MAJOR. `AtRestBody` derefs to `Body`, so
reverting the doors still compiled, and every "refused at the gate"
row tested `validate`, not the door. The fix pass:
- pins the three doors' operand types at compile time, verified by
  planting the revert (E0308 at all three);
- rewrites the gate rows' docs to say what they prove;
- records that no document reaches `UnfinishedOperand` through a Shell
  node;
- corrects the stale docs;
- files S7 as `shelf/shelled-result-discards-its-own-closing-verdict`.
S6 (finished fixtures) was skipped because it grows. Merged over the
inherited pinch ε = 1e-6 red.

## Unit 6 measured; the klein elbow is a design fork (2026-10-06, PR 4138)

The lane measured the decline and stopped at the design question, as
briefed. The lift's scope holds the cavity's meridian caps, translated
one wall off the axis, and `offset_axial::classify` refuses "a plane
parallel to the axis but not through it". So the scope is not axial
under the gate's definition, and this is not a gap in it. The together
door would need two new arms:
- the gate admits such a plane, either roster-wide or lift-only;
- the torus × meridian edge arm accepts a spiric old rim. An
  experiment showed it refuses `TogetherAxialEdge` one stage later.
PR 4138 records the measurement and the probe and merged on the
orchestrator's read (comments and item prose only, no kernel change,
CI green). The item stays open, re-priced H with `design: true`, for a
designer pair. `offset_axial.rs` is OFFSET's and CURVED's shared ground
too.

Reviews of PRs 4115 and 4117 were both APPROVE-WITH-FIXES; fix passes
are out.

## Unit 6 weighed (2026-10-06)

The designer pair on the klein-elbow lift converged in round 1, and
both rejected the item's framing. The together door's domain is
"revolves at rest", so it is not closed under its own output, and the
lift is that door applied to its output. They agreed on the fix:
- widen "axial" to every axis-parallel plane, everywhere;
- `classify` reads one surface;
- rim carriers come from the moved pair, with one plane×torus section
  home in `geom_brep::plane_torus_section`;
- a zero-offset spiric is unrepresentable.
No ratified text moves, so there is no `[ev]` PR. The spec basis is in
the item. Unit 6 now runs BEFORE unit 7, whose partial-revolve lift
depends on it. Its review tier is DUAL: the gate change re-routes many
bodies and is hard to reverse once rows are rebuilt on it.

## Unit 3 MERGED (2026-10-06, PR 4115)

The planar wall-clearance gate:
- reads arc-bounded footprints through the existing `carrier_ball`;
- decides facing on a levered drift, unioned with the old cosine
  window, with the gap taken short by the drift.

Single full review: APPROVE-WITH-FIXES on two MINORs (window narrowing
on tall parts; the drift correction unguarded). Both are fixed with rows
that go red under the review's mutants, and the fourth extent dispatcher
was replaced by reuse. Filed:
- the extent class (FLUX);
- vertex-only siblings (CHART, HONE, TANG);
- the rim_wedge cosines (HONE);
- the tilted residue (shell).

Merged over six failures that are red on main at ε = 1e-6:
- JOIN's pinch row;
- BAND's `bounds_census` roster row (filed here, P0);
- four CLEAVE split closed-form rows (filed by the orchestrator, P0).
## Unit 4 MERGED (2026-10-06, PR 4117)

Lofted and NURBS-walled bodies get past the wall-seam re-anchor. The
re-anchor is certified and seeded on spline carriers and in closed form
on ellipses. Single full review: APPROVE-WITH-FIXES, no MAJOR. The fix
pass:
- adds a seed-sensitive row (red under the reviewer's seedless mutant);
- makes the analytic arms explicit;
- re-measures |δ| over the new carrier kinds;
- uses one `NurbsLaneUnsupported` spelling within the door;
- corrects the refusal docs and the vase number.
Filed:
- the closed-carrier seam case, the three-spellings class and the
  spline-extension residue (shell);
- the zero-span "kernel defect" text, riding unit 5;
- the ellipse boolean `param_near` sites (HONE);
- the interior-row iso arm (ISO).
ENCL's rigid-map row, which was parked on the closed item, is re-parked
on the oblique-corner item that now gates it.

## 2026-10-06 15:20 — unit 6 dual review dispatched; demos main-red routed to PATHS

- Unit 6 lane report landed on PR 4151 (head `70b1ef611`). Dual review (H, concurrent) dispatched on that frozen head: R1 `session_01WniRc3DStGGngmTdm1tfTz`, R2 `session_0154Q262rVu2oCh4QejUS4p5`, identical briefs and seven claims to falsify. Protocol `713b017b7`; blinding byte 236.
- The demos job's four `eps_regression` reds on 4151 are a red main (klein findings pin 10 retired, certified-cells header moved), reproduced on clean main `364b8aefa`. Filed P0 for PATHS: `work/paths/demos-red-on-main-klein-pin-retired-and-certified-cells-moved.md` (suspect #3774; #4136 second).
- Unit 5 (PR 4163): `test` red on `122ac63b`; the lane is asked to name its rows.

## 2026-10-06 15:50 — incoming: BAND's unwitnessed-transport row goes to SHELF

BAND filed `offset-door-declared-transport-has-no-built-witness` (P2, E) on this slate. It is a coverage row: the declared-description transport arm in `replace_face_offset` lost its only witness when the lamina full revolve stopped minting a meridian. It is not on the active cut, so it is moved to `work/shelf/` beside the other follow-on rows.

## 2026-10-06 16:40 — unit 6 dual review adjudicated

Both reviews on `70b1ef611` returned APPROVE-WITH-FIXES with no MAJOR, and all seven claims held under execution. The demos and 1e-6 reds were confirmed inherited by both reviewers.

Correspondence pre-note:
- **Bilateral:**
  - undisclosed stored-bit moves (both MINOR, on different bodies);
  - the `Err(_) => t_old` swallow;
  - the stand-off spelled three ways in `plane_torus_section`;
  - stale "through/contains the axis" docs;
  - each beyond-spec arm pinned by exactly one row (same mutants, same rows).
- **Unilateral, R1:**
  - the door returns a strict-subset body when the cap–cap line enters the tube (MINOR, demonstrated, pre-existing, newly reachable through two calls);
  - `section_refused` names one cause for three;
  - the turn rule is written twice.
- **Unilateral, R2:**
  - two interpenetrating solids pass tier 3 when a void dilates past the cavity wall (NOTE, demonstrated, pre-existing);
  - the r1p1 row's `Err => println` arm (MINOR, by inspection);
  - the spiric lever uses the speed floor (unsure);
  - stale predicate cites;
  - CURVED-SPIRIC-DESIGN.md:281.
- **Tally candidates:** none, since no unilateral MAJOR was raised.

Fix list of ten items sent to the lane. Both pre-existing wrong bodies are to be filed P1 on SHELF.
- 2026-10-06 — From FUSE: demo-tour's Klein pin (`klein.rs:876`,
  findings entry 10) fires on main at every `eps_regression` row since
  PR 3774 (PATHS 5b). The FUSE 3953 lane bisected it. Filed as
  `work/paths/a-klein-wall-radius-pin-fires-on-main-since-paths-5b.md`
  (P0): the pin's own text says the entry has retired. Every PR that
  merges main is red on the `demos` job until it is resolved.

## 2026-10-06 18:27 — unit 6 MERGED; unit 7 dispatched

- **Unit 6 merged.** PR 4151 merged at `75e040f1`. CI on its fix-pass head `393dbb29` was fully green, including the demos job and the 1e-6 pass, since main had fixed both by then. The DUAL-REVIEW-LOG row DR-91 rode it as the last commit (`fb69ee74`), with tally 0, fair. The klein-lift item is closed. The implementer session is archived.
- **Unit 7 dispatched.** Item `shell-open-refuses-a-curved-designated-face`, branch `shell/curved-mouth`, session `session_01AYdrwh3ShH3k149P2Kq1ju`, DUAL tier. The spec basis is the item's `## Decided` section.
- **Unit 5.** The lane on PR 4163 was told to merge main, take 4151's `offset_axial.rs` as the base, and finish the S4 sweep.

## 2026-10-06 19:50 — unit 5 MERGED

PR 4163 merged at `45e35802`, after the single STYLE review and the lane's fix pass (`22de2fbd`, with 4151 merged in and S4 swept over `offset_axial.rs`). Every check was green. The carrier and its three riders are closed. The lane session is archived.

Units 1–6 are merged. Unit 7 (curved designated face) is in flight on `shell/curved-mouth`.

## 2026-10-06 21:15 — unit 7 dual review dispatched

- **Lane report.** Unit 7's report landed on PR 4191 at head `3332ebea`, with run 37528954027 green.
  - A pole-touching periodic designation opens to a seamed band.
  - A non-wrapping window becomes a ring.
  - `offset_distance` is now the lift's one home.
  - Three items are filed: the cone tip and the tangent dome both refuse in the sealed arm, and a band between two boundaries is left as this unit's residue.
  - The lane flagged its reading of the seam-keeping spec for review.
- **Dual review.** H, concurrent, dispatched on that frozen head with identical briefs and six claims:
  - R1 is `session_01AzyaddcRJcicphMd5V1AYF`;
  - R2 is `session_01A9qDF2o7DTnahmwynRC6TV`.

  Protocol `713b017b7`; blinding byte 74.

## 2026-10-06 22:20 — unit 7 dual review adjudicated

The dual review on frozen head `3332ebea` returned R1 NOT-MERGEABLE-AS-IS and R2 APPROVE-WITH-FIXES.

The kernel work held under both reviewers' probes:
- caps from 1° to 89.9°, at both poles and on re-posed bodies, each against its closed form;
- the planar merge-base differential, bit-identical;
- `demos/tour` 96/96.

Both reviewers accepted the seam-keeping reading as within the spec.

Correspondence pre-note:
- **Bilateral:**
  - the void-side seamed band builds but has no row, and one mutant direction survives (MIN/MIN, both executed);
  - `chart_read` misreads windows wider than π (MIN/MIN, both demonstrated);
  - `chart_read` is a second copy of `topo::chart::Chart`;
  - `re_anchored` is a third spelling of `split_specs`;
  - escalations are folded into shape refusals;
  - the interior-edge wrap test is a proxy (unsure, both);
  - the pncad-py docstring is stale;
  - `audit_record` has no void arm;
  - the lift's cone/nappe arm is reached by no built body.
- **Unilateral R1:**
  - **MAJOR**, executed with a red document probe: `emit_shell` names only `rim.rim`, so a seamed band cannot be evaluated from a document ("kernel bug" naming refusal);
  - the dead `seams.len()==1` arm;
  - the guard text claims more than it checks;
  - `RimNaming.ring` names a retired loop on a band;
  - the NURBS ApproxNesting text.
- **Unilateral R2:**
  - `offset_distance`'s Offset refusals have no row;
  - a second `periodic` rule;
  - stale planar premises at `shell.rs:272` and `:1696`;
  - `lift_to` reads `from[0]` alone;
  - the module header keeps growing.
- **Tally candidate:** R1's document-path MAJOR (unilateral, contract/API, demonstrated by a red probe, fair pair). Blinded coding is pending.

A ten-item fix list has been sent to the lane, and both reviewers are archived.

## 2026-10-07 03:20 — unit 7 MERGED; the 2026-10-06 cut is complete

PR 4191 merged at `8e4dd542`.
- **CI:** green on `906f806c`. The branch's last two pushes had triggered no CI run, and the merge of main restarted it.
- **Review log:** the dual-review row is DR-97, renumbered because main took DR-95 (PCERT) and DR-96 (BAND) first. Tally 26 (R1's document-path MAJOR); fair pairs that found a MAJOR, 47.
- **Item:** `shell-open-refuses-a-curved-designated-face` is closed. The lane session is archived.
- **Merge conflict:** `pcurves.rs`'s `chart_boundary` takes main's `chart_u_period`. Shell's own `chart_period` keeps the kind rule for its one reader. The local shell/chart/pcurve suites ran 700/700 on the merged tree.

DR-91 (unit 6, PR 4151) recorded protocol `713b017b7`. That was a shallow-clone misread: the last commit touching `docs/DUAL-REVIEW-PROTOCOL.md` is `7cb05367ef`, and this tracker PR corrects the cell. DR-97 carries the right hash.

All seven units of the cut are merged:
- 4111, pole ball;
- 4112, AtRestBody operand;
- 4115, planar gate;
- 4117, lofted wall seam;
- 4163, refusal text;
- 4151, klein lift;
- 4191, curved designated face.

`plan.md` now carries the next cut as units 8–12. Two items are P1: the tilted planar walls (a silent wrong body) and the lofted oblique corner.

## 2026-10-08 — next cut dispatched (units 8–10)

Ev said go.

- **Unit 8, tilted planar walls** (P1, M): implementer dispatched on `shell/tilted-walls`, session `session_01WTRinsGsb2d1yVCkEEazRW`.
  - Direction decided here: the gate stays a measured clearance. A facing pair outside the existing windows has the least distance between its offset footprints decided against the band. This is not a dihedral-threshold refusal, which would refuse sound wedges.
  - Review tier: single full review.
- **Unit 9, the lofted oblique corner** (P1, H): a design fork, weighed by a designer pair (one Opus, one Fable) on where a moved chart meets an unmoved non-plane neighbour.
  - Blinding byte 172, Opus=A (`analysis/design-fork/shell-lofted-oblique-corner`).
- **Unit 10, the face door at rest** (P2, M, with its P3 rider): a design fork, weighed by a designer pair on the public posture of the three offset doors toward an operand that is not at rest.
  - Blinding byte 188, Opus=A (`analysis/design-fork/shell-face-door-at-rest`).
- **Unit 10, face door at rest** (P2, M + rider P3, E): designer pair agreed (premise correction; doors stay construction steps; no ratified text moves, no `[ev]` PR); `## Decided` written; implementer dispatched on `shell/face-door-at-rest`, session `session_01BRLJT85zsBBDGVYTJgw2qV`.
- **Unit 9, lofted oblique corner** (P1, H): designer pair converged over two rounds (derive, do not transport; the one-door merge's timing was sequencing, taken as a follow-up); no ratified text moves; `## Decided` written; filed `offset-doors-are-one-door-with-a-held-distance` and `a-fitted-wall-has-no-section-with-a-moved-cap`; implementer dispatched on `shell/oblique-corner-derives` carrying ISO's interior-row item (ISO has nothing dispatched), session `session_01Qu1nNjtcPLYhgkdaLBQBic`. Dual review.
- **Unit 9 measurement** (2026-10-08): the derivation holds on the twisted loft (plane × NURBS section certifies, corners root on every seam); the vase's rational walls fail `plane_nurbs_limbs`' limb 2 by ~3e-3 m even on the exact row, and the skinned wall's weights drift one ulp. Orchestrator took option 1: build as decided for polynomial walls, the loft keeps its chart-on-cap rim on a rational wall, the vase's cap move refuses typed (Decided item 3); the limb-2 bound and the weight drift filed in the unit's PR.
- **Unit 10 review** (PR 4315): single review, no MAJOR; fix pass sent (`unreachable!` comment names check 7, not `ShellWinding`; stale `classify_shells_through` doc; reverted-wedge pin rows; the topo README row re-worded descriptively, since a general door rule on a ratified page would bind future work and wait for Ev).
- **Designer pair dispatched** for `a-fitted-wall-has-no-section-with-a-moved-cap` (byte 133 on `analysis/design-fork/shell-fitted-wall-section`).
- **Unit 10 merged** (PR 4315): fixes verified (check-7 comment, descriptive README row, sense-flipped pin rows — `revert` on a plane flips the normal, not the sense, so the lane built the sense-flip rows directly). Closed both SHELL items and FUSE's duplicate; filed `shell-refuses-a-finished-body-wearing-one-chart-both-ways` (P3).
- **Fitted-wall fork decided** (2026-10-08): designer pair agreed (a fitted face's section is its fit's; nothing composed). Not put to Ev: the C5 refusal is agent text and the change restores OFFSET-DESIGN's ratified "most delegate to the fitted NURBS". Re-priced P2: lofts still stop at the wall–wall seams; filed `a-wall-seam-between-two-fits-has-no-section` (P2) and `two-fits-sharing-a-smooth-seam-disagree-by-their-certificates` (P3). Lands after unit 9.
- **Unit 9 ruling** (2026-10-08): step 3 (the loft's at-rest rims as `Intersection`) dropped — it broke 44 sweep rows that build today (the plane × NURBS certificate refuses their at-rest rows: TubeNotOneArc, FootPointInconclusive, Interval structural parameters). Instead the door derives a tilted declared Chart edge between distinct surfaces by section and drops its declaration (within the pair's converged design). The certificate's at-rest refusals filed by the lane. Twisted loft now refuses at the first wall's fit, BudgetExhausted 4.14e-9 vs 1e-9.
- **Unit 11 dispatched** (2026-10-08): cone tip (NappeStraddles) and tangent dome (TogetherAxialCorner) on `shell/apex-and-tangent-corner`; single review.
- **Unit 8 merged** (PR 4311): single full review, no MAJOR; fix pass verified (conic docs, test docs, either crossing pair, own measurement name, adjacency residue filed). Item closed.
- 2026-10-08 — Seam note from ENCL (PR 4348, `encl/offset-cert-coefficient-norms`, in review): `geom-core`'s `spline::compose` tensor helpers gain public norm doors (`tensor::coefficient_norm_bound` now takes `[&[Interval]; 3]` and refuses ragged rows; new `coefficient_norm_sup` and `PatchSpans::cell_norm_sup`), and the offset certificate (`offset_fit.rs`, `offset_meters.rs`) reads its vector upper bounds (‖Y‖, M̃, the integral arm's chart speeds) off coefficient norms per D4 ¶2 instead of per-coordinate boxes. The bounds are tighter or equal, and stored offset numbers were re-baselined. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4367, merged): `offset_fit.rs`/`offset_meters.rs` folds that feed guards now use `geom_core::interval::max_bound`/`min_bound`, so a NaN reaches its guard. `PatchRegularity::sup` and `CellNormal::sup` are NaN on a refused cell (previously the last cell only). `Composite::cell_terms` is the one home of `cell_bound`'s guards. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL: dispatched `shell-wall-meets-curvature-reads-as-an-offset-distance` on `encl/shell-wall-curvature-recourse`. The fit meter's CurvatureHeadroom recourse will read in wall terms under the shell (`AsShelled` in `crates/topo/src/shell.rs`, your ground). (ENCL orchestrator)
- **Unit 12 dispatched** (2026-10-09): band between two boundaries on `shell/band-between-boundaries`; single review. Main's build break (#4363×#4372) fixed by #4381/#4380 (SHELL's duplicate #4384 closed); FUSE's demo-tour P0 closed (fixed by #4374).
- **Unit 11 merged** (PR 4356): cone tip (nappe reads non-apex corners; first body through `lift_to`'s cone arm) and tangent dome (nearest root, tie by the old corner's side, foot only at exact tangency — after the review's regression catch). Single review; one MAJOR fixed. Filed: per-chart apex window reaching its apex, opened dome ties at the lift, nearly-tangent refusal row, circle–circle pairs.
- **Unit 9 merged** (PR 4351, DR-115): a tilted declared Chart edge is derived by section and its declaration dropped; the apex window is re-checked on the derived rims (the dual review's one MAJOR). Closed the oblique corner, the torus rim, ISO's interior row and SHELF's cap-rim order. Two main merges for nextest slow-list and DR-number collisions (DR-113/114 taken).
- 2026-10-09 — Seam note from ENCL (PR 4377, merged): `AsShelled` renders the offset fit's curvature, BoundNotFinite and InvalidRequest arms in wall terms (`THINNER_WALL` const). `shell_open` refuses a non-finite thickness as `ShellError::Thickness` ("is not finite"). (ENCL orchestrator)
- **Units 13 and 14 dispatched** (2026-10-09): 13 is the fitted wall's section (`shell/fitted-wall-section`, dual review); 14 is the per-chart door's inverted body under unit 10's ruling (`shell/inverted-body-at-rest`, single review). They touch different files (C5 routing vs the door's callers and pins).
- **Unit 13 ruling** (2026-10-09): the lane built items 1–3 and found that the item's "expected outcome" did not hold. The twisted loft refuses at the iso-row guard (FittedBoundaryUnsupported) at ε ≥ 4.1e-9 and at the fit below that. The vase refuses at the cap's limb 2 before any seam. A moved fit's corners have no root on a derived spline section, so r1_lane0 moves from FittedBoundaryUnsupported to CornerSection. Ruled (a): keep the spec as built and pin what was measured. Also check that the guard's payload is honest, append the measurements to the downstream seam items, and file the corner gap (P2). Seeding corner roots is a separate unit.
- **Unit 14 merged** (PR 4403): the per-chart door's inverted body is closed by contract under unit 10's ruling. The audit found no finished-body path that skips `shell_open`'s closing gate. Pins: both door moves are Ok with a closed-form negative volume and tier 3 `RingOutsideOuter`; `shell` past half the wall refuses `NotValid`. Single review, no fixes. CI's red `test` was TANG's agreement-gate fuzz counterexample (from #4292); the lane filed it on TANG's slate as `chord-join-serves-lines-where-the-whole-turn-reach-refuses`.
- 2026-10-09 — Seam note from ENCL (PR 4395, merged): a limb refusing at the offset fit's mint is now `OffsetFitError::MintLimb`, ending in the kernel-defect ending; the at-rest `Limb` keeps "re-fit" (`geom_brep::offset_fit::LIMB_REFIT_RECOURSE`). The shell and the transform re-fit both reach `MintLimb`. (ENCL orchestrator)
- **Unit 12 merged** (PR 4391): `shell_open` opens a chart that wraps between two boundaries as two seamed bands. The second band takes one `HoleRim` row per region, and a new `RimNaming::seam_pieces` names the surviving seam piece. Single review: no MAJOR. The fix pass made `canonicalize_chart` finish its scan and route to the band arm only when both sides wind once; it also shares the pole/band helpers, reads the band arm's range through `along`, and updated the error doc and the editor-core naming assertion.
- **Unit 13 dual review** (PR 4404, frozen head 3fd92b2a1b, byte 112): R1 and R2 both returned APPROVE-WITH-FIXES with no MAJOR, so the tally doesn't move. The pair is fair: both disclosed the same glimpse of cargo build lines through a shared log-file name, with no findings seen. Next time, give each reviewer its own log path in the brief. The fix pass carries the union:
  - the loft rim's refusal hidden behind the seam;
  - s1 always the plane;
  - a curved-fit certify row, run per PR;
  - the variant, Display, module, O2 and O4 docs;
  - the citations, and the quad item's overlap with ISO;
  - one accessor for "Approx is its fit".
- **Next cut planned** (2026-10-09): unit 15 is the moved fit's corners (P2, H, dual), dispatched after #4404. Unit 16 is the tilted read's three P3 gaps (`shell/tilted-read-gaps`; M-tier, rule-1 byte 178, sequential). The wall seam waits for a designer pair after 15.
- **Unit 13 merged** (PR 4404, DR-126): a fitted face's section with a plane is its fit's. C5 routes plane × `Approx` over the fit; the edge stores `Intersection { plane, approx }` with the plane always first; certify reads the fit through `spline_chart()`. Each rim's verdict is pinned behind the seams. Dual review: no MAJOR, fair pair. The row renumbered twice (DR-122, then DR-124) as other programs merged while CI ran. A curved-fit certify row is in the slow set under the ≥ 1 s rule. Filed: the corner gap (P2), QUAD's trimmed-fit volume rule (P3), HONE's stale composition citation (P4).
- **Unit 15 dispatched** (2026-10-09): the moved fit's corners (`shell/fitted-corners`, H, dual review).
- **Unit 16 review 1** (PR 4467, sequential, head fd80908701): REJECT, one MAJOR (executed). The arc refinement ignores half-edge orientation, so a minus-oriented spline or spiric arc's cut has holes, and `walls_cross` reads a 0.06 m overlap as clear. Reachable on the bowl sector's caps, right there only by distance. Fix pass sent (orientation; minus-orientation rows; the speed ball intersected with the hull ball). A second Opus review is owed on the fixed head.
- **Unit 16 review 2** (PR 4467, head 1b400e68f4): APPROVE-WITH-FIXES, no MAJOR. Review 1's MAJOR is confirmed fixed. The reviewer could not build an adjacent-wall crossing through `shell` (105 shapes). Final pass sent: a symmetric Zero contact (the verdict depended on argument order), pins for the weight term and the hull ball, the over-coverage claim corrected, the interval enclosure, and the shared spiric ball. The orchestrator checks the delta and merges; no third review.
- **Unit 16 merged** (PR 4467, DR-133, M-tier sequential): the tilted read now accepts a Zero touch only at a shared vertex, reads edge-adjacent pairs less their joints, and refines spline and spiric edges on their carrier with the window run the way the half-edge runs. A contact on `L` is read from either side. Review 1 REJECT: one MAJOR, a minus-oriented arc read a 0.06 m overlap as clear. Fixed; review 2 APPROVE-WITH-FIXES with no MAJOR. Filed: `clearance-footprint-reads-an-arc-as-its-whole-carrier-ball` (P3). Note: the row was not quite the last commit; the lane's work-note commit landed after it, concurrently.
- **Unit 15 dual review dispatched** (2026-10-10 06:4xZ, PR 4472, frozen head 7b5120d42e, CI green): a concurrent Opus pair on one identical brief. Blinding byte 53. Each lane has its own worktree, target and log directory, so the shared-log glimpse from DR-126 cannot recur.
- **Wall-seam designer pair dispatched** (2026-10-10): `a-wall-seam-between-two-fits-has-no-section`, one Opus and one Fable on the same problem statement. Byte 242, committed to `analysis/design-fork/shell-wall-seam`. They run alongside unit 15's review, because the seam needs design before it can be priced.
- **Filed** `a-saddle-walls-offset-fit-stalls-short-of-the-default-eps` (P3, M, measure first). This is unit (e) of the wall-seam cut.
- **Unit 15 dual review** (PR 4472, head 7b5120d42e, byte 53: A = R2, B = R1).
  - R1: APPROVE-WITH-FIXES with 1 MAJOR. `incident_edges` levers a derived section's root decision by `extent_of` read at the old edge's parameters, extrapolated on the new domain. The value was measured (an arm 2–4.5× the chord) and the orchestrator verified the path at head.
  - R2: APPROVE-WITH-FIXES with no MAJOR. A held non-plane surface at a corner now refuses `CornerSection` (the lane's `Unsupported`) where the corner used to build. Traced only; the orchestrator verified the path.
  - Bilateral: the ε-end rule's doc scope, the seed taken from the domain end, the triplicated agreement predicate, and weak rows.
  - Rule 3: each lane saw the other's process listing, but no findings, so the pair is fair. The next brief says to poll by own PID only.
  - Ruling: fix the MAJOR, R2's MINOR and the ε-boundary row, plus the doc and style items, in one pass.
- **Wall-seam fork to Ev** (2026-10-10, fork-log row 106): `a-wall-seam-between-two-fits-has-no-section`.
  - The designer pair agrees on the final state, recorded in the item's `## Designed`:
    - the crease seam is a fit × fit section;
    - `shell` moves every chart through one general simultaneous door;
    - the iso-row arm narrows;
    - curved clearance is promoted to a gate;
    - D2 does not change now.
  - They split on one question, now an `[ev]` PR: whether the NURBS × NURBS arm is a seeded operation, with C5 rows stating seeded or complete, or one complete arm. It took three rounds: round 1 crossed over, round 2 returned both to their first positions, and round 3 held.
  - Unit cut, pending the ruling:
    - (a) narrow the iso-row arm (M; independent of the ruling);
    - (b) the general simultaneous door (H; can precede the arm against plane + fit pairs);
    - (c) the NURBS × NURBS arm, whose scope is set by the ruling;
    - (d) curved clearance as a gate;
    - (e) the saddle fit's reach at the default ε (measure first; no item yet).
- **Wall-seam fork ruled** (PR 4515, 2026-10-10). Ev chose B: one complete NURBS × NURBS arm, built as a `Section` handle with `branch_at(seed)` and `all()`. `shell` asks only `branch_at`; the boolean asks `all()`.
  - Before the ruling, Ev asked whether B could avoid computing work it throws away. Both designers answered yes. A's lazy variant always ran the proof.
  - The ruling is recorded in the item's `## Decided`, and `needs_ev` is cleared.
  - Unit (c), the arm, is scoped by it. Units (a), the iso-row narrowing, and (b), the general door, are unaffected.
- **Filed** `shell-moves-every-chart-of-a-solid-through-one-simultaneous-door` (P2, H). It is unit (b) of the wall-seam cut, the door half of PR 4515's designed state.
- **Unit 15 merged** (PR 4472, DR-138, H tier, concurrent pair). A moved fitted face bounded by planes now solves its corners: the held planes are rooted along the derived plane × fit sections (route 1).
  - The review's MAJOR is fixed: a derived section levers its roots at its own domain's extent.
  - A lane verdict on a derived section contributes no root.
  - One `gap_within_eps` helper holds the single ε read.
  - The orchestrator merged main twice before merging; the second merge kept both test modules in `replace_face.rs`.
  - Filed from the unit: SSIEDGE one-arc on a window edge, QUAD sub-range trim image, SHELL iso-row u-moving image.
- **Units 17 and 18 dispatched** (2026-10-10 09:37–09:41Z).
  - Unit 17: narrow the iso-row arm. Session `session_01RZJC9khmQTn8f8veyfg6rH`, branch `shell/iso-row-narrow`, M tier; rule-1 byte 37 (mod 3 = 1) puts it in the sequential arm.
  - Unit 18: the general simultaneous door. Session `session_01De5VWXHkEBqhmCXCzNUivh`, branch `shell/general-door`, H tier, dual review.
  - Both specs are in the dispatch prompts, as `## Decided` sections the lanes copy into their items.
