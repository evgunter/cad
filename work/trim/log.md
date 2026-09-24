# TRIM log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/trim/plan.md`. A/B band 2500–2599
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose TRIM section is the
charter this plan restates. Opens when CURVED lands the rim arms. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `clearance-window-tightening-needs-chart-boundary` from `work/m10/`
- `interior-iso-curve-de-boor-extractor` from `work/issues/`
- `general-pcurve-face-props-and-tess-refuse` from `work/issues/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## Opened for dispatch (2026-09-04)

Same orchestrator as CURVED (branch `curved/orchestrator` carries both
programs' state-sync; unit branches stay `trim/<unit>`). The tracks
doc's gate "at CURVED's rim arms" was traced and found to name this
program's own file: the rim arms are `nurbs_iso_derive`'s in
`topo/src/pcurves.rs` (P-2, PR #1177: cap-rim arm widened, wall-seam
arm reverted), and the wall-seam arm's blocker is the de Boor
extractor — item 1 here. Ev confirmed in-chat (2026-09-04) that the
order is the orchestrator's call; the plan's §Opening condition and
§Order are revised accordingly: the extractor opens, the props/tess
lane measures behind it, the clearance-window description runs in
parallel. Riders on the Track Q files (`D36`, `S394`, `S83`, `D305`,
`fitted-magnitude-nan-schedule-parameter`) land with whichever unit
opens their file. First action after this PR: write the extractor's
spec (Fable) and dispatch it under the A/B protocol from band 2500.

## First dispatches (2026-09-04, later)

- **`docs/TRIM-3-SPEC.md` ratified (#1862)** — the chart-boundary
  description (PR-1, `topo` only) and the clearance-window seam (PR-2).
  Refuted on the way: cell dropping alone does not flip the L-cap row
  (the exhibit arm's 9-station lattice is a third consumer site);
  extruded bodies carry no stored pcurves (the description derives via
  `walk_loop`); a box fixes neither planar row; SHELL-3 moves the same
  functions (PR-2 sequenced by announcement; SHELL-3 not dispatched).
  Rulings §9: keep `ClearanceReport::windows`; new `chart_bound.rs`;
  the `WINDOW_TIGHTENING` const stops promising and the exact-region
  recourse gets its own item.
- **`docs/TRIM-1-SPEC.md` ratified (#1865, rulings #1876)** — the de
  Boor collapse extractor. Refuted: `an_interior_column_still_refuses`
  is not the row this unit flips (it is an arc-class row refusing at
  the schedule residual); the wall–seam revert cited a test's name as
  its reason; no `geom` primitive is needed. `Pcurve::IsoLine` gains
  interior columns (no new variant); rational class = weight nets
  separable by structure; riders S394 and
  `fitted-magnitude-nan-schedule-parameter` carried.
- **Block TRIM-B1** drawn branch-side (`trim/b1-block`): byte 43 ⇒
  fable at slot 1. Slot 0 = TRIM-3 PR-1 (Opus) on `trim/3-chart-bound`;
  slot 1 = TRIM-1 (Fable) on `trim/1-de-boor-extractor`; slot 2 =
  TRIM-3 PR-2 (Opus), opens after PR-1 merges with the seam announced
  to SHELL and M10.
- Both spec lanes were starved of the build mutex (1–2.5 h waits under
  load 25–40) and pre-registered their one measurement as the
  implementer's first act; see the CURVED log's operations note.

## Heads-up from TOPO (2026-09-05)

`work/topo/S331` (the vacuous green through `validate_pcurves`) sits on
TOPO's slate as a question about what at-rest validation may claim, but
its mechanism is `crates/topo/src/pcurves.rs:1229-1300` — this
program's file, beside `D36`. TOPO edits nothing there: a proposal
comes to this board first, after TOPO's opener lands, and the two
programs decide then whether the row moves here or lands by seam.

## S331 moved here (2026-09-05)

Superseding the heads-up above: with Ev's concurrence TOPO moved
`S331` onto this slate (`git mv`, id and body kept; the direction TOPO
proposes is in the item's tail). It is TRIM's to sequence, beside `D36`.

## Announced seam from PROPS (2026-09-05): `ssi.rs` and `ssi/certify.rs`, the free hull doors' consumers

The coeffs-window unit (`docs/PROPS-COEFFS-SPEC.md`, item
`work/props/coefficients-carry-their-knot-vector.md` — ruling A of the
Span sweep applied one level down) moves `geom-core`'s free `hull`
doors onto a `CoeffWindow` minted from `(KnotVector, coeffs)`. Three
consumers are on Track Q's ground: `ssi.rs::pcurve_windows` and the two
span loops in `ssi/certify.rs` — each mints one `SplineCoeffs` per
coordinate channel from the `kv` it already holds and takes the span
from it; the arithmetic is unchanged and pinned by a bit-identity
digest captured at the merge base. Mechanical, disclosed in the PR
body with the territory list. Signed (PROPS orchestrator).

Addendum (2026-09-05, PROPS orchestrator): the coeffs fix pass (#1992)
renames the mint the two `ssi` consumers call (`kv.coeffs(..)` →
`kv.with_coeffs(..)`) and adds one sentence at each mint-refusal arm
(`ssi/certify.rs` `box_chain`, `probe_tube_chart`) saying what the arm
returns and that it is unreachable by construction. No arithmetic.
Signed (PROPS orchestrator).

## TRIM-3 PR-1 merged (2026-09-07) — the first TRIM unit

PR #1911, ordinal 2500, sample #151; block TRIM-B1 slot 0 concluded.
The dual (R1 Fable, R2 Opus) both MERGEABLE-AFTER-FIXES; adjudication
on the PR (comment 5561699789); eleven union items all taken. Two
soundness fixes from review: singular chart joints refuse typed (R1's
wrong description on a pole-touching sphere face), and `assembled`
refuses an outer spanning more than the period (R2's 3τ lift). The
pair is EXCLUDED from the A/B tally under 3(e) (R1 killed by a Fable
429 and resumed). Class findings filed at adjudication:
`round-holes-get-no-chart-bound-benefit`; the spec gained an
amendments section (two-edge loops, singular charts, the metred hull,
the shared schedule). PR-2 (the clearance seam) is next on this item:
its opening act is announcing the seam to SHELL and M10 on the away
channel, and its consumer must re-chart every plane and read the
METRED hull. Operations: `chart_region::SCHEDULE_2D` became
`pub(crate)` for the new consumer — a visibility change on Track Q's
file, doc-only otherwise, recorded here as the announcement.

## TRIM-1 merged (2026-09-07)

PR #2095, ordinal 2501, sample #153; block TRIM-B1 slot 1 concluded.
The dual (R1 Fable, R2 Opus) both MERGEABLE-AFTER-FIXES and converged
on everything (adjudication comment 5564847666); eleven union items all
taken. Substantive fix: the seam class meters its fixed channel against
the chart's u domain (an out-of-domain `u` had certified through the end
span's polynomial extension). The P-2 body now mints and validates at
rest; TRIM-2's opening measurement is re-cut — on that fixture the
tessellation lane stops at `patch_bound::Degree1Crease`, not at any of
the six filed sites, so TRIM-2's spec must start from a fixture the
crease gate admits (a degree-2+ chart) or take the crease gate as its
first door. Filed at adjudication: `step-adopt-let-ok-iso-discards`
(`work/issues/`, EXCH's), `rational-gates-test-unit-weights-not-constancy`
(TRIM). Next on this program: TRIM-3 PR-2 (the clearance seam; announce
to SHELL/M10 first) and TRIM-2's spec.

## Resumed after a six-day outage (2026-09-13)

Two items landed on this slate from other orchestrators while this
session was blocked: `boundary-iso-doors-panic-before-they-can-refuse`
(DOOR: `boundary_iso_u/v` panic in the slice on a count-corrupt net
where their `# Errors` contract promises a typed refusal — E, this
program's `nurbs_iso.rs`; folded into TRIM-2's spec lane as a rider
candidate, or its own small unit) and `S351` (a citation watch on
`nurbs_iso.rs`'s placement rule, re-homed from CITE at its close; not
fired). TRIM-3 PR-2's seam announcement (2026-09-07) drew no objection
in six days: dispatched on `trim/3-window-seam` (TRIM-B1 slot 2, Opus).
TRIM-2's spec lane opened with the re-cut (the crease gate) as its
first premise.

## Announced seam from TOPO (2026-09-13): `pcurves.rs` read, and possibly one helper, with the split-edge unit

TOPO's `split-edge-children-lack-pcurve-rows-on-curved-charts`
(branch `topo/split-edge-pcurve-rows`) makes `Body::split_edge`'s
children carry pcurve cache rows. The lane reads `mint_pcurves`,
`mint_pcurves_of` and the row types in TRIM's `crates/topo/src/pcurves.rs`
end to end; if the mint needs a helper that splits one cached row at a
parameter, that helper lands in `pcurves.rs` by this seam — one
function, its doc, its rows — and the PR names it. No other edit
there. Signed (TOPO orchestrator).

## Announced seam from TOPO (2026-09-13): one helper in `pcurves.rs`, `split_cache`

TOPO-B2 slot 1 (`split-edge-children-lack-pcurve-rows-on-curved-charts`,
PR #2531) closes `Body::split_edge`'s missing child rows by CARRYING the
parent half-edges' rows across the split rather than by documenting the
gap. The mechanism is one new `pub(crate)` helper in this program's
file — **`split_cache`** — plus the `SplitRows<T>` alias that names its
answer: it reads the parent's stored row, re-derives the face's chart
window as the hull of that face's stored chart boxes (the
self-referential way `mint_face` builds it and `validate_pcurves`
re-builds it), and re-certifies the parent's image over `[t0, t]` and
`[t, t1]` through `PcurveCache::certify`. Read-only, called from
`split_edge`'s plan phase, so a refusal arrives with the body untouched.

Two smaller edits in the same file, both statements rather than
arithmetic: `Posture::Carries` joins the `staleness_posture` vocabulary
and `split_edge`'s `DECLARED` entry moves onto it (its old entry, "the
one primitive that makes a row stale in CONTENT rather than by key", is
no longer true of it), and the module docs' posture section gains a
`Carries` bullet while the `Neither` bullet loses `split_edge`. Nothing
else in the file moves: `mint_pcurves`, `mint_pcurves_of`, `mint_face`,
`walk_loop`, `chart_boundary` and `validate_pcurves` are untouched, and
no derivation lane's `PcurveFittedLane` bound changes.

**A finding for this board, filed in the same PR**:
`validate-pcurves-never-recertifies-a-face-it-finds-incomplete` — the
pass skips its re-certification and continuity passes for the whole
face when any half-edge is missing a row, so a stale stored row on an
incomplete face is accepted unmeasured. Measured on the item's own
fixture. Neighbour of `S331`, and distinct from it: that row is about a
CLEARED face, this one about an INCOMPLETE one.
Signed (TOPO implementer lane, `topo/split-edge-pcurve-rows`).

## The seam as built, and a second finding (2026-09-14, PR #2531 fix pass)

The announced helper landed with two changes to what was announced,
both inside the same seam and both narrowing what this file spells
twice:

- **`split_cache` takes the edge's two half-edges, not one**, and
  derives the face's chart window **once per face** instead of once per
  half. `SplitRows<T>` is gone: a one-use alias for
  `Option<(PcurveCache<T>, PcurveCache<T>)>` named nothing the tuple did
  not.
- **The window has one home in this file.** `stored_rows(body, face)`
  walks a face's loops once and returns each loop's cycle with the hull
  of the chart boxes its stored rows carry; `validate_pcurves`'s passes
  0 and 1 now read it instead of walking the same loops and hulling the
  same boxes themselves, and `split_cache` reads the same function. The
  findings `validate_pcurves` reports, and their order, are unchanged.
  `mint_face`'s window is deliberately NOT this one — it hulls the
  images it is deriving, none of which is stored yet.
- **`Posture::Carries` was dropped.** The posture walk only
  distinguishes `Maintains` from everything else, so a fourth arm no
  walk can read was a comment wearing an enum's clothes (and it made the
  module docs' "three postures exist" false). `split_edge` is declared
  `Transfers` — the posture that moves each row onto the key that now
  carries what it says — with its note saying what it restricts.

**One line in `crates/geom-brep/src/pcurve_cache.rs`, also this
board's**: `Pcurve::chart_box`'s `IsoArc` arm said its whole-segment box
is tight "at the full span, which is the only span any mint asks for".
A restriction of a stored row to one child of a split asks for a
sub-span, so the sentence is false at this head; it now says a sub-span
gets the conservative box. The arm's arithmetic is untouched.

**A second finding for this board, filed in the same PR**:
`iso-derivation-arms-assume-an-edge-spans-the-charts-whole-domain` —
`nurbs_iso_derive`'s two rim arms map an edge's whole carrier interval
onto the chart's whole `u` domain, so `mint_pcurves` refuses on any
body whose spline-chart wall edge has been split, while minting the
same body unsplit. Measured on `sweep::loft_body` prisms; the fixtures
are committed as `crates/sweep/tests/split_edge_loft_charts.rs`, whose
rows pin the refusal as the current, filed behaviour.
Signed (TOPO fix-pass lane, `topo/split-edge-pcurve-rows`).

## Announced seam from TOPO (2026-09-14): pcurve rows under `revert`, with the revert unit

TOPO's `revert-does-not-mirror-plane-chart-images` (branch
`topo/revert-mirrors-chart-images`) makes `Body::revert` transform
the `Chart` images and the pcurve cache rows of faces on a reverted
plane with the frame (`(u, v) ↦ (u, −v)`). If the row transform lands
in TRIM's `crates/topo/src/pcurves.rs` it is one function by this
seam, its doc and its rows, named in the PR; otherwise the rows are
dropped and re-minted through `mint_pcurves_of`, which the PR says.
No other edit there. Signed (TOPO orchestrator).

## Announced seam from TOPO (2026-09-14): two doors beside `shift_branch`, and the posture docs' fourth position

TOPO-B2 slot 2 (`revert-does-not-mirror-plane-chart-images`) closes
`Body::revert`'s unmirrored plane charts by RE-STATING every datum in
a plane's chart coordinates under the reflection its frame undergoes
(`v_ref = normal × u_ref` negates with the normal, so `(u, v) ↦ (u,
−v)`). Two doors land in this program's `crates/geom-brep/src/pcurve_cache.rs`
by this seam:

- **`Pcurve::mirror_v`**, beside `shift_branch` and in its shape — the
  image under `(u, v) ↦ (u, −v)`, exact in every variant (a sign flip on
  the `v` coefficients; a NURBS image's control net negated in `y`
  with knots, weights and parameter untouched, rebuilt through
  `NurbsCurve2::new` exactly as the shift does). Its own `impl<T:
  Real>` block, because it needs no `SpanLocate`.
- **`PcurveCache::mirrored_v`** — the same certified cache with its
  image mirrored and the certificate VERBATIM, the `with_remapped_surfaces`
  argument: the mirrored image on the mirrored chart evaluates to the
  same 3-D points (bit-identical up to the sign of a zero, which a
  distance squares away), so every number the run would produce again
  is the number it produced.

Two edits in `crates/topo/src/pcurves.rs`, both prose: the posture
section gains a paragraph for `revert` — a fourth position, outside
the guard's walk (the door takes `&self`), carrying every row key for
key with the plane faces' rows mirrored — and `insert_voids`'s
`DECLARED` note stops saying the reverted rows go stale in content.
No arithmetic in this file moves; `mint_pcurves`, `mint_face`,
`walk_loop`, `validate_pcurves` and `split_cache` are untouched.

**One finding filed on TOPO's slate, named here because its middle
answer is this file's**:
`revert-leaves-a-periodic-charts-loop-wrap-mid-chain` — with the plane
images mirrored the drum's reverted cavity reports exactly
`NegativeVolume`, but the two-arc sphere's still reports the
`LoopDiscontinuity` SHELL-9 measured, because the forward walk's
one-period wrap is parked at a closure the reversed loop no longer
has. Whether the continuity pass should accept a wrap anywhere on a
closed loop is the question that lands here.
Signed (TOPO implementer lane, `topo/revert-mirrors-chart-images`).

## Fix pass on the announced seam (2026-09-14, PR 2542): one door, no fourth posture, one row filed here

The two doors above landed as ONE affine door and its two callers.
`Pcurve::map_affine(point, vector)` in `pcurve_cache.rs` carries every
chart-space coefficient through an affine map of the chart given as
its action on points and its linear part on vectors — a NURBS net
through `NurbsCurve2::map_points`, the tree's infallible door for a
pointwise map of a validated net, so no arm can refuse and the two
`RevertError` variants minted for the refusal are gone. `mirror_v`
is that door with the second channel negated; `shift_branch` is that
door with the first channel of points translated and vectors left
alone, its behaviour unchanged (its rows are the pin; the
`.ok()?` on a re-validated `NurbsCurve2::new` it carried was the
same unreachable arm). `PcurveCache::mirrored_v` is infallible with
it; the certificate-verbatim argument now has one home, on
`Pcurve::mirror_v`, and the doors point at it. The posture docs no
longer call `revert` a fourth posture: it is a `&self -> Self`
producer outside the guard's walk, said as such, with the dead-key
exception stated there rather than only at the call site. Filed on
this slate, because the guard is this file's:
`pcurve-posture-guard-is-blind-to-body-producing-doors` — every
body-returning producer's row posture is prose checked by nothing,
and `revert` was the one that was wrong. Signed (TOPO fix-pass lane).

## Announced seam from TOPO (2026-09-14): two posture notes with the loop-re-parenting unit

TOPO's `loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart`
(branch `topo/loop-reparenting-rows`) makes `kfmrh` and `ring_move`
honest about the pcurve rows of the loop they move. In TRIM's
`crates/topo/src/pcurves.rs` it re-states the two doors' notes in
`staleness_posture::DECLARED` to say what each does with those rows;
no other edit there. Signed (TOPO orchestrator).

## The seam as built (2026-09-14, PR 2549): four posture entries, a third door, and two sites left open

The unit announced above closes the case where a door
moves a whole LOOP between faces on different surfaces: the rows on
that loop keep their keys and change which CHART they are stated in,
and where the target face does not `chart_mints` — a plane, or a NURBS
placeholder — `validate_pcurves` skips the face and says nothing. The
fix is a `Decide`-bound limb in this program's territory only by its
prose: `Body::drop_rows_on_chart_change` lands in
`crates/topo/src/euler_ring.rs` (TOPO's), compares the two faces'
surface KEYS, carries every row when they agree and drops the moved
loop's rows when they do not. It derives nothing, so no bound moves and
`geom-brep` is untouched.

In this program's `crates/topo/src/pcurves.rs` the edits are prose
only, and no arithmetic in the file moves — `chart_mints`,
`mint_pcurves`, `mint_face`, `walk_loop`, `stored_rows`, `split_cache`
and `validate_pcurves` are all untouched:

- `staleness_posture::DECLARED` moves **four** entries from `Neither`
  to `Transfers` — `kfmrh`, `mfkrh`, `mfkrh_plug` and `ring_move` —
  each with the note that says which surface key decides. The item
  named two doors; the class sweep found `mfkrh` is the third (its
  `mfkrh_plug` sugar always changes chart, because a placeholder is
  always a fresh key), so four entries move rather than two.
- The posture section gains a paragraph for the three doors, and the
  `Neither` line stops reading "the Euler operators, the kill ops, ring
  surgery" — ring surgery is no longer in that bucket. The same
  paragraph names the two sites the sweep measured and did NOT close
  (`mef`'s moved run, `kef`'s remnant — half-edge runs rather than
  loops), filed on TOPO's slate as
  `mef-and-kef-move-half-edge-runs-between-charts-and-leave-their-rows`
  and pointed at from there.

Nothing in this seam changes what `validate_pcurves` measures. One
behaviour change worth this program knowing about: a loop moved onto a
DIFFERENT MINTING chart used to leave rows that the pass re-certified
and refused (`Certify` per row); after the fix those rows are gone and
the face reads as one the minting pass has not run on, which the pass
is silent about by design. The refusal is not lost so much as made
unnecessary — the body no longer holds the wrong row — and the two
rows in `crates/topo/tests/loop_reparenting_pcurve_rows.rs` that
measure it say so at the assertion. Signed (TOPO implementer lane,
`topo/loop-reparenting-rows`).

## The seam widened by one function, and three prose repairs (2026-09-14, PR 2549's fix pass)

Both blinded reviews of the head above converged on four things inside
this program's `crates/topo/src/pcurves.rs`. None of them moves any
arithmetic in the file; `chart_mints`, `mint_pcurves`, `mint_face`,
`walk_loop`, `split_cache` and `validate_pcurves` still compute what
they computed.

**The seam is one function wider than announced.** The fix adds
`pcurves::loop_rows` — **the one per-loop rows walk**: given a loop, the
half-edges a pcurve row can be keyed on, as a three-way `LoopRows`
(`Cycle`, `NoCycle`, `Corrupt`) so a caller states its disposition
instead of re-deciding it. `stored_rows` now calls it (its own
let-else is gone, its behaviour unchanged in all three arms), and so
does the loop-re-parenting doors' drop in TOPO's `euler_ring.rs`. The
reason it lands here rather than beside the doors is this program's:
the question "which rows does this loop have" is `validate_pcurves`'s
question, and a second spelling of it is how a door and the validator
came to disagree about a face's rows without either being able to see
it. The third and fourth copies — `shell.rs`'s `rename_loop_surface`,
which discards differently, and the outer
`once(outer).chain(rings)` chain at 33 sites — are NOT folded in here
and stay on TOPO's
`a-faces-loops-are-walked-by-hand-in-thirty-four-places`.

**Three prose claims this change made false, repaired.** The header's
"there is no invalidation machinery and none is needed" (there is now,
per door); the header's "the tier-3 pcurve pass catches a stale row
LOUD … so the posture is fail-loud, not silent-wrong" and
`Posture::Neither`'s "safe because the tier-3 pass catches the
consequence loud" (the pass reads an INCOMPLETE face and a face whose
rows no longer certify, and is silent about a complete face on the
wrong chart and about any face `chart_mints` refuses — the same
paragraph that PR added names `mef`/`kef` as leaving rows the pass
never reports). `Posture::Transfers`'s own doc named only the graft and
the split, neither of which is what the four new entries do; it now
covers a row whose key never moved but whose chart did, so the four
entries sit under an arm that describes them. `set_face_surface`'s
entry claimed the pass "re-certifies against" a surface swap, which is
true only where the new surface mints; corrected, with the measurement
filed on TOPO's slate as
`set-face-surface-leaves-a-complete-face-certified-against-the-chart-it-left`.

**One row filed on this program's slate**, by both reviewers'
finding: `validate-pcurves-cannot-tell-a-never-minted-face-from-an-emptied-one`
— the pass reads a face a door emptied exactly as it reads one never
minted, measured on three doors, an S331-shaped vacuous green beside
`validate-pcurves-never-recertifies-a-face-it-finds-incomplete`.
Signed (TOPO fix-pass lane, `topo/loop-reparenting-rows`).


## TRIM-3 PR-2 and TRIM-2 PR-1 delivered; the week's pacing (2026-09-14)

TRIM-3 PR-2 (PR #2554, the clearance seam) and TRIM-2 PR-1 (PR #2564,
the trimmed-region quadrature) both delivered green on the full matrix.
TRIM-3 PR-2 landed three of the spec's four consumer edits: the
`min_separation` half crosses the drive's flip-crossing divergence
census (seven M10-6/R2 drive rows lose their certified leaf) and is
filed rather than forced (`min-separation-tightening-crosses-the-drive`);
the negative-angle-revolve mutant has no e2e row because no revolve
replays at `Interval` over an ε box (filed). TRIM-2 PR-1's fixture
measurement mirrored §0 (the `General` seam on `u = 2`); the lune box
is the CHORD's frame, not the axes' (the axis-aligned box is only
second order per chord). TRIM-3 PR-2's dual dispatched (ordinal 2502,
R1 Opus, R2 Fable). **TRIM-2 PR-1's dual is HELD for next week's
budget** (the weekly window is at ~64% with six days left); it draws
ordinal 2503 and TRIM-B2 slot 0's arm (Opus) when dispatched.

## Announced seam from TOPO (2026-09-14): a reverse-parking helper, with the revert-wrap unit

TOPO's `revert-leaves-a-periodic-charts-loop-wrap-mid-chain` (branch
`topo/revert-reparks-the-wrap`) makes `revert` re-park a periodic
chart's wrap at the reversed closure. `walk_loop`/`loop_closes` in
TRIM's `crates/topo/src/pcurves.rs` are read; if the re-park needs a
helper there it is one function by this seam, named in the PR. Signed
(TOPO orchestrator).

## The reverse-parking seam, as landed (2026-09-14): no helper, one paragraph

The revert-wrap unit (`topo/revert-reparks-the-wrap`) needed no
function in `crates/topo/src/pcurves.rs`: the wrap is re-parked by
moving each curved loop's `Cycle::first` to its source predecessor
inside `Body::revert`, which puts the forward walk's closure joint at
the reversed closure with no row shifted — `walk_loop` and
`loop_closes` are read, unchanged. What this lane touched in TRIM's
file is ONE prose paragraph, the posture docs' "`revert` carries the
map" position, whose last sentences said the reversal does not
re-state the branch choice and leaves the wrap to the producer's
closing mint; they now say how the reversal re-states it and point at
the anchor bullet in `revert`'s module docs for the argument. Signed
(TOPO, the revert-wrap lane).

## TRIM-3 PR-2 dual adjudicated; the seam gate is satisfied (2026-09-15)

PR #2554 (ordinal 2502, frozen head 8e53655d2): both arms
MERGEABLE-AFTER-FIXES (R1 Opus 3 MAJOR/5 MINOR/4 NOTE, rubric
3/3/3/3/5; R2 Fable 2 MAJOR/4 MINOR/5 NOTE, rubric 3/4/3/4/5).
Neither could make a `Holds` unsound through the shipped seam. Both
measured deviation 1's stated mechanism wrong (the diverging census
rows are the chart-boundary loop walk's, recorded by the interval
leaf and never by the f64 witness lane, which returns `None` — a
lane-split artifact on every box, not a box-dependent count); the
decision to leave `min_separation` untightened stands as the identity.
R2 alone refuted C3 by construction: an extruded `CircleSplit` with
phase −π/4 mints a NEGATIVE cylinder band (the walk pins from a
principal azimuth), and under either `[0,τ] ∩ hull` spelling the root
collapses to a sliver and a `Violated` placement reads `Holds` — the
spec's E7 mutant is live e2e; the shipped rule (`hu` verbatim) is
right and the residue file's premise is false. Three claimed mutant
kills (E5 full-turn root; E4 drop-on-indeterminate; E4 tight-by-0.05)
do not execute. Adjudication on the PR (comment 5675317215); sixteen
items, fix pass dispatched to the implementer on `trim/3-window-seam`.
Tally: R2's MAJOR-1 unilateral but guards/claims-class (no reachable
wrong output) — no candidate; both reviewer arms paused once by the
usage limit (3(e)). **Seam gate**: the announcement to SHELL/M10
(#1911 comment 5568210053, 2026-09-07) drew no objection through
2026-09-15 — a week's silence; PR-2 may merge after the fix pass.
Recorded, not fixed: `raw_hull` is the identity until a Fitted/General
pcurve reaches a window; `window_of`'s `Err` arm has no e2e fixture.

## TRIM-3 PR-2 merged (2026-09-15) — block TRIM-B1 concludes

PR #2554, ordinal 2502, sample #201; block TRIM-B1's last slot ({OPUS,
FABLE, OPUS} all executed; the block record folds to main with this
merge). Fix pass from the dual: all fourteen executable items taken,
two recorded as the coverage boundary; the headline row is R2's P6 —
an extruded `CircleSplit` at phase −π/4 mints a negative cylinder band
and both `[0,τ] ∩ hull` spellings mint a phantom `Holds` against it —
with the cylinder root rule lifted into `cut_root`. Honest negatives
kept in the PR body: E4 catches neither of the two mutants it used to
claim (E2/E3's pinned receipts do); `window_of`'s `Err` arm and the
loose-window column are unreachable on this tree (filed). **Seam
record**: the seam into `editor-core/clearance.rs` (SHELL/M10's file)
was announced on #1911 (comment 5568210053, 2026-09-07) with the
diff's shape; no objection or acknowledgement arrived through
2026-09-15; the merge proceeded on the week's silence per the plan's
gate — this entry is the on-repo record R2's NOTE-11 asked for.
Residues on the program from this unit: `clearance-window-cone-sphere-torus`,
`exact-region-cells-for-lower-bound-only`,
`min-separation-tightening-crosses-the-drive`,
`revolved-bands-reach-no-clearance-row`, `three-tables-of-the-chart-arms`,
`a-refused-chart-boundary-has-no-reachable-window`. `docs/TRIM-3-SPEC.md`
is fully delivered (PR-1 #1911, PR-2 #2554) and leaves `docs/` per the
ledger in the post-merge docs PR.

## Pacing under the weekly budget (2026-09-15)

Block TRIM-B1 concluded at #2554. TRIM-2 PR-1 (#2564, TRIM-B2 slot 0,
OPUS) waits for its dual until the account's weekly reset (2026-09-18
17:00Z; window at 83 %). TRIM-B2 slot 1 is the next dispatch after
that: `loft-seam-carrier-exact-knot-compare` is S-CERT's file (seam or
exit first), so the slot's leading candidate is TRIM-2 PR-2 (after
PR-1 merges; the S-MESH seam announced at dispatch) or
`boundary-iso-doors-panic-before-they-can-refuse` (E, single review).

## TRIM-2 PR-1 dual dispatched; PROPS seam announced late (2026-09-15)

Ev, in-chat: finish the open duals before the usage limit. Ordinal 2503
claimed (PR #2669); byte 136, parity 0 ⇒ R1 Opus, R2 Fable; frozen head
`0c7cc6637`; briefs stored with sha256; dispatched with the spiric pair.
**Gate**: `docs/TRIM-2-SPEC.md` §4 makes PROPS's acknowledgement of the
`quad.rs` / `props.rs::nurbs_face` seam a merge condition. The
announcement was owed at PR open (2026-09-14) and was not made — the
orchestrator's miss; posted now on #2564 (comment 5683759668) and on the
sign-off watchlist. PR-1 does not merge without the ack or Ev's ruling.

## TRIM-2 PR-1 dual adjudicated (2026-09-18)

R2 (Fable) killed once by a model-side 429 during the hold, resumed
after the reset; both arms MERGEABLE-AFTER-FIXES (R1 Opus 2 MAJOR/5
MINOR/5 NOTE, rubric 3/4/3/3/5; R2 Fable 1 MAJOR/5 MINOR/6 NOTE, rubric
4/4/3/4/5). The FLUX enclosure held under every fixture either arm
built; both re-derived the Newton–Cotes degree argument and reproduced
M1–M8 verbatim. Headline, R2 alone: the AREA rule reads whole-box
hulls and refuses ordinary curved charts as `DegenerateFace` (area
`[−0.549, 1.742]` against the rectangle lane's `[0.968, 1.125]` on the
same chart) — every shipped row lives on `g ≡ 1` where the area
machinery is inert; unilateral, code-class: a tally CANDIDATE, but the
pair is excluded under 3(e) (R2's 429). Converged: the chord polygon's
closure unchecked (a certified wrong flux on an open walk), the vertex
pad unexercised by any row, the "fixed at the entering round" reason
false, the monotone margin levered by the round. Adjudication on the PR
(comment 5734849876); thirteen items; fix pass dispatched. **Seam
gate**: PROPS has not acknowledged; spec §Amendments now carries the
week's-silence fallback (merge no earlier than 2026-09-22).

## TRIM-2 PR-1 fix pass verified; merge waits on the seam gate (2026-09-19)

Head `e38ca5d87`, run 35418382136 green (twelve `test` jobs at step
level, four render lanes, five k-lint rows). All thirteen items taken;
the headline (the area rule) is now the rectangle lane's own 2-D cell
rule per sub-chord with a second-order trapezoid term, at
`QUAD2_AREA_PIECES` — E1's area bracket, 2.3× the oracle's before, now
equals it; Q3's all-iso bit-identity held throughout. The walk's
closure is checked (`TRIM_OPEN_WALK`) and shared vertices are hulled
into one bracket; `TrimChord::len` is gone (the door derives it).
Five mutants red nothing and are stated at their sites, not claimed
(the closure gap pad, the sliver constant, the λ pad, the bisection
ladder, the weights' bracket); `mu−1` was the wrong mutant (closed
Newton–Cotes on an even count is exact one degree past its order) —
`mu−2` reds Q6/Q9. One row outside the fence moved with a written
reason: `bool4_material_containment`'s tier-3 row, whose spline-walled
bracket now passes check 7 through the trimmed lane and is refused at
the census for its face kind (the row's own doc had named the day).
Filed: `curved-trim-e2e-fixture-waits-for-a-producer`. **Merge gate**:
no PROPS acknowledgement through 2026-09-19; per the spec's Amendments
the merge proceeds no earlier than 2026-09-22 absent an objection.

## TRIM-2 PR-1 merged (2026-09-19) — block TRIM-B2 slot 0 concludes

PR #2564, ordinal 2503, sample #222. Ev, in-chat: PROPS is paused —
merge; so the spec's §4 acknowledgement gate closes on Ev's ruling
rather than the amendment's 2026-09-22 fallback (both recorded). The
merge carried main forward 63 commits (78 crate files) after the
verified fix-pass head, so the merged head ran CI before the merge.
Next in the lane: TRIM-2 PR-2 (tessellation; seam to TESS announced
at dispatch), TRIM-B2 slot 1.

## TRIM-2 PR-2 dispatched; TESS acked the seam (2026-09-19)

PR-1 merged at a833156d4 (sample #222). TESS acknowledged the PR-2 seam
within the hour (#2564 comment 5739504272): both arms' shape fine; one
ask — a domination row for `nurbs_tighten`'s `General` sup against
densely sampled UV speeds (via `nurbs_cert::tests::Domination` from
#2848 if on main) — folded into the brief; two in-flight TESS PRs on
the same files' test modules and the error enum noted; PROPS's
`rational_cells` hull finding passed on as not-this-unit's. TRIM-B2
slot 1 = TRIM-2 PR-2, pre-draw S / NUMERIC, arm OPUS by the block's
draw; brief stored (sha256 7ac2e42863…); lane `trim-2-pr2`, branch
`trim/2-tess`. The sign-off watchlist is empty: all three seams this
orchestrator owed are closed (TRIM-3's by silence, TRIM-2 PR-1's by
Ev's ruling, PR-2's by TESS's ack).

## TRIM-2 PR-2 delivered; dual dispatched (2026-09-19)

PR #2863 (head f46673cd6, run 35435586012 green): the two arms, E2
(108 416 positions vs the oracle's 143 360, a factor-of-two band —
deviation 2, the widened chart's face bound over its whole domain),
TESS's domination row hand-spelled (`Domination` not on main), M1
seen by that row and not by E2 (said plainly). Nothing moved in
goldens or renders. Dual: ordinal 2504, byte 185 ⇒ R1 Fable, R2 Opus.

## TRIM-2 PR-2 dual adjudicated (2026-09-19)

Both arms MERGEABLE-AFTER-FIXES (R1 Fable 1 MAJOR/3 MINOR/4 NOTE,
rubric 3/4/4/4/5; R2 Opus 2 MAJOR/6 MINOR/2 NOTE, rubric 3/4/3/4/5).
The certified sup is sound for the stated reason (both re-derived the
derivative net's scaling; convexity's slack shown on interior-maximum
fixtures; the rational refusal load-bearing at 5×). Headline,
bilateral by per-patch attribution: E2's committed rationale is false
— the widened wall's patch is bit-identical to the oracle's and the
whole 34 944-position deficit is the P-2 route's plane-restated wall;
the factor-of-two band (deviation 2) was bought for an effect that
does not exist and is vacuous under every sup mutant. Also bilateral:
the domination row's cubic leg is attained at an end coefficient
(R2's `endsonly` mutant leaves it byte-identical), two typed refusals
are dead by construction, the `trimmed.rs` header is stale, a third
copy of the hull fold. Adjudication on the PR (comment 5743420032);
eleven items; fix pass dispatched. Tally: no candidate (every
substantive finding bilateral); neither arm killed or paused — the
first clean pair this orchestrator has run since the weekly hold.

## TRIM-2 PR-2 merged (2026-09-20) — block TRIM-B2 slot 1 concludes; TRIM-2 delivered

PR #2863, ordinal 2504, sample #224. Fix pass from the dual: all eleven
items; E2 now asserts the per-patch equality (stronger than the spec's
"schedule's own ±") and deviation 2 is withdrawn; the domination row
has three measured legs; the dead guards are gone. The merged head
carried main forward 254 commits and ran CI before the merge. The
unit `general-pcurve-face-props-and-tess-refuse` is CLOSED —
`docs/TRIM-2-SPEC.md` is fully delivered and leaves `docs/` per the
ledger in the post-merge docs PR. Seam: merged on TESS's pre-dispatch
ack (TESS paused; Ev, in-chat). TRIM-B2 slot 2 (FABLE by the draw)
is the next TRIM dispatch: `boundary-iso-doors-panic-before-they-can-refuse`
(E) or `loft-seam-carrier-exact-knot-compare` if S-CERT's file is
free — chosen at dispatch.

## The cut (2026-09-20)

Ev, in-chat (see CURVED's log for the words): TRIM keeps what closes
this session — `boundary-iso-doors-panic-before-they-can-refuse` and
the P-2 spec's deletion at the walk — and **CHART** (`work/chart/`,
band 6200–6299) opens with the chart-side residue, 31 items from here
plus CURVED's three SSI drive-bys; the chord-count arithmetic class
moves to TESS's slate as its item names TESS the owner (a line on
TESS's log). Program and plan re-cut; the former order and residue
lists are in this plan's history. **Protocol v7 note**: TRIM-2 PR-2
(S / NUMERIC) was dispatched into the v6 dual on 2026-09-19, the day
v7 was recorded and before this orchestrator read it — under v7 it
would have run opus/opus outside the protocol; its row T2T is a
seam-day row and any readout spanning 2026-09-19 treats it per the
entry's item 4. The remaining E unit runs outside the protocol.

## The run doors' posture seam (2026-09-14): two prose paragraphs, two table notes, no function

TOPO's `topo/mef-kef-runs-carry-or-drop-rows` gives `Body::mef`'s
chord surgery and `Body::kef`'s unsplice the loop doors' answer one
level down: the RUN of half-edges each moves between two faces' loops
keeps its rows across one chart (`Body::same_chart`) and loses them
across two, through a run-level twin of the loop door
(`Body::drop_run_rows_on_chart_change`, in TOPO's `euler_ring.rs`; the
loop door now delegates to it). `pcurves::loop_rows` is read and not
edited: the run is what each op's plan phase already holds, so no
`run_rows` walk was needed in TRIM's file. What moved in
`crates/topo/src/pcurves.rs`:

- the module docs' `Neither` paragraph, which said `mef`'s run and
  `kef`'s remnant are "left saying the old face's chart" and cited the
  TOPO row, now says each door disposes of the run's rows and what
  keeps the two in the `Neither` bucket (`mef` mints two rowless
  halves; `kef` kills two whose rows outlive their keys);
- the `Posture::Neither` variant doc's "two entries here are known to
  leave rows in that blind spot" sentence, re-stated the same way;
- `staleness_posture::DECLARED`'s `mef` and `kef` notes say what each
  does with the moved run. Both entries STAY `Neither`: the minting
  operators' posture is Ev's question on the open `[ev]` PR and this
  unit does not decide it.

One finding filed on this slate while placing `kef`'s killed halves:
`pcurves-docs-claim-a-recycled-slot-can-read-another-half-edges-row`
(the stale-row consequence's recycled-slot arm is one `SecondaryMap`'s
version check forecloses). Signed (TOPO, the mef/kef run lane).

## The run doors' posture seam, fix pass (2026-09-24): `mef` and `kef` read `Transfers`

The dual on `topo/mef-kef-runs-carry-or-drop-rows` (PR 2603) found
`Posture::Neither`'s definition — "leaves the map exactly as it found
it" — false for two doors that now drop rows whose chart moved, and
`Transfers` literally describing them. So in `crates/topo/src/pcurves.rs`:
`mef`, `mef_chord` and `kef` move to the `Transfers` section of
`staleness_posture::DECLARED`, in a section of their own; the
`Transfers` variant doc names the run beside the loop, and says the
bucket is silent about rows a door never HELD (a minted half arrives
rowless — the minting posture, decided elsewhere, not by the entry);
the `Neither` variant doc loses its "dispose of those rows themselves
now" history; the header's per-door paragraph on the two doors is a
one-sentence pointer at `DECLARED` rather than a restatement of it
(`the-pcurves-module-header-restates-the-posture-table-below-it` is
the class and this does not grow it). The run door
`Body::drop_run_rows_on_chart_change` is gone: one predicate-free
primitive `Body::drop_rows` sits under every decision site, and the
header's two references to the doors follow it. Signed (TOPO, the
mef/kef fix pass).
