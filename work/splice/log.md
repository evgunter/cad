# SPLICE — the log

## 2026-10-25 — opened

Opened by a refold approved in chat, before any orchestrator started on
ORBIT or SECT. Twelve rows moved by `git mv`, ids and bodies unchanged,
each with a dated Moved note: the gate deletion from ORBIT; the sites it
waits on from ROOTS and GAUGE; the crossing rung from REACHHOLD and the
spline join from HONE, both un-parked (their parks waited for a first
producer of spline edges between faces, and every loft and sweep is
one); two touch and walk rows from CLEAVE (P3, taken on request); one
touch row from APEX; two flush rows from SEAMRANK; two GAUGE test rows
that go live with the deletion. Every row is P0 or P1: the work here is
what blocks the klein, lily and teapot joins. 29 budget points.

## 2026-10-25 — orchestrator picks up the track

The SPLICE orchestrator (`session_016P4GehXjm4s8N46wtAg2FB`, a cloud
session) holds the track from here.

**Overlap with INTENT / D10.** SPLICE's work is kernel-side (operand
edges, crossings, the join, the flush readings) and does not touch
dependency, placement or intent, so the D10 refactor hold
(`d10-one-way-to-say-intent-is-unbuilt`, DESIGN.md "State of the
build") does not hold any row here. Two seams to watch:
- D′3 (PR 5636, `intent/dprime3-assemble-product-slot`, unmerged)
  edits `crates/editor-core/src/names/emit_union.rs`, on this track's
  ground. Its change is additive (`repeated_member_view`) and does not
  touch `Flush`, so it does not collide with the two flush rows; the
  flush unit, dispatched last, takes whatever main carries then.
- Stage 4 E and F are built: booleans glue on a Zero verdict and the
  boolean declare surface is retired. So no lane here declares a
  contact on a boolean. Where a row's prose says "a declared `Rest`
  contact would have to accept" (the klein rims, in
  `delete-the-boolean-operand-edge-gate`), it is read as a coincidence
  decided from values under `[coincidence]`.

**Dispatch.** Lanes run as their own cloud sessions: this container is
4 vCPU and builds one heavy cargo job at a time, and every unit here
seeds topo. Three run in parallel:
- `splice/gate`: the sites typed, both sector walks' NURBS
  departure, and the gate deleted, in one PR (the sites are only
  reachable, and their rows only red, once the gate is gone; PR 3984's
  attempt took the same shape). Tier: **dual**, because it is new
  incidence under every boolean (plan, "Review posture"). The two
  split-side P1 rows (`section-area-skips-…`,
  `split-section-boundary-…`) sit behind the SPLIT gate
  (`splitting::classify::gate_operand`), not this one; the lane
  measures whether the boolean reaches them and takes them only if it
  does.
- `splice/touch-doors`: `the-touch-and-cross-doors-…` and
  `a-touch-beside-a-spline-…`, both in `carrier_touch` /
  `carrier_cross`. Tier: single, full (certified distance bounds).
- `splice/carrier-walk-spline`: `carrier-walk-has-no-crossing-row-…`.
  Tier: single, full (a certified crossing count).

Held for the gate: the crossing rung, then the spline join, then the
two flush rows.

Lane sessions: `splice/gate` `session_01P78fY3Hgsq13g359bgcNLP`;
`splice/touch-doors` `session_01V5dB8mxDac8zAFBWSEmdT7`;
`splice/carrier-walk-spline` `session_019eib6yHkmVaSGLLzEVtvhK`.

## 2026-10-25 — the SECT seam gets a row

Agreed in chat: SECT gets its own orchestrator rather than SPLICE moving
rows across. SECT's ownership ruling forbids a stopgap, and the arm is
built on SECT's critical path. Filed SECT's
`a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm`, parked on
SECT step 1 and on `delete-the-boolean-operand-edge-gate`. Lily wall 8
stays this track's first demo witness.

## 2026-10-25 — PR 5719 (carrier walk) reviewed

Single full review (`session_01WJhm39zKSMKSiECpcoahqv`, on c2e7a187):
MERGEABLE, no MAJOR. Every claim held, and the reviewer measured each
one. The spiric differential against main is byte-identical (2400
rows). A random-spline crossing fuzz ran 5680 queries per scalar with
0 wrong, and goes red 149 times with the monotone gate removed. The fix
list went to the lane: route the window net through
`SpanLocate::restrict_window`, drop zero-width end pieces, move the
on-point to mid-piece, adopt the fuzz as a row, and fix three stale
prose sites.

**Class noted at adjudication:** a hand-rolled homogeneous window net
(`span_row` divided per channel) beside the certified per-scalar door
`SpanLocate::restrict_window`. The new copy bypassed the door's
argument contract and its [0,1] clamp at Interval. The one other
`span_row` caller outside geom-core is `derivative.rs`. Any lane that
needs a spline window's Bézier net reads it through `restrict_window`.

## 2026-10-25 — PR 5715 (touch doors) reviewed: NOT-MERGEABLE

Single full review (`session_01TKt5dMsxyqRwbeLAunJLcm`, on f4fcf1ec).
It found two MAJORs from one root cause, both shown by rows that are red
on the head and green on main (branch `splice/review-5715`, 1499d787):
- `NurbsCurve3::accel_upper_bound` is a per-open-span sup of C″, so a
  spline with a C0 interior knot (multiplicity ≥ p, which clamped-v1
  admits) reads `accel = curvature = 0`;
- the PR's newly wired second-order bounds then certify across the
  corner. `off_face` answers OffFace for a real crossing of the face,
  and `curve_pair::walk` (CPAIR) misses a spline crossing a line.

Main is sound today because the bound was `None` for a spline there.
The fix list went to the lane: the bound is either window-scoped and
non-finite across a non-C¹ knot, or the piece walks split there. Also:
a curvature floor that is not definitely positive gives a non-finite
bound; one home in `geom` for the point-vs-span reach walk, now on
its third copy (`Curve3::clear_of`, `walk::near_pieces`,
`ArcPieces::clear`); and the cell-door pin gets its own looseness
guard. The re-review runs on the fix pass's head.

**Classes noted at adjudication:**
- **A second-order (Taylor) bound read across a tangent discontinuity.**
  Any consumer of `accel_upper_bound` / `accel_hi` / `curvature_hi`
  assumes C¹ on the piece it bounds. A lofted operand with a polyline
  or C0 section reaches these doors once the operand gate is gone. The
  lane sweeps the consumers; a symbol sweep cannot see a site that
  assumes smoothness without naming these bounds.
- **The point-vs-span reach walk is copied**, not shared: one
  `geom_core::bisect` driver, three loop bodies. The lane gives it one
  home and lists the `census/curved.rs` and `shell.rs` reach sites.

## 2026-10-26 — the demo joins' first refusal has its own row

The gate lane measured, on PR 5723, that klein wall 3, lily wall 8 and
teapot wall 3 all refuse at their contact once the gate is gone. In
each, an analytic edge of one operand is read against a NURBS wall of
the other (`CurvedBooleanUnsupported { kind: Nurbs }`). No row owned
that: the crossing rung composes along a NURBS edge, and SECT's new row
is the face × face section. Filed
`an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane` (P0 H,
design open). The plan's "lily wall 8 needs no curved section" still
holds, but lily also needs this row. Designers weigh it once 5723 is
in review.

## 2026-10-26 — PR 5719 (carrier walk) merges

The fix pass (77e42f67, 732ff87f) answered the review:
- each piece's net now goes through `piece_controls` over
  `restrict_window`, and `SpanNet` is gone;
- roots are cut only at knots strictly inside the window;
- the contact on-point is mid-piece (refusals at 0.9·z dropped from
  1135 to 30 of 3000);
- the review's fuzz is adopted as `spline_arc::fuzz::random_splines_at_*`.
  Its first CI run found an oracle miscount, not a row one: an
  8000-step polyline missed a narrow rational excursion. The oracle now
  bisects to 2e-4 chords, and 11,981 + 2,395 rays came back with 0
  wrong.

Orchestrator's read of the delta (no second review: the delta changes
certified decisions only within the items asked). One comment-only
drive-by: a history line in the fuzz header removed. Merged on green
`gate ok`.

## 2026-10-26 — PR 5723 (the gate) goes to review

Green at b0860ea4 (main b6588d2c merged; the render moved no cell,
since all three walls refuse before and after, at different sites).
Tier: dual, letter M (both main rows cost M). Arm drawn at dispatch:
`/dev/urandom` byte 169, mod 3 = 1, so **SEQUENTIAL**. One review now
(`session_01XsRqThr5DVNjqpXVWs9hj7`, dispatched 01:28Z); a second runs
on the fix-pass head only if this one raises a MAJOR.

## 2026-10-26 — PR 5715 (touch doors) re-reviewed: one MAJOR, a pin

Re-review of the whole unit (`session_01YLh5gFkY6a5rytfMtr81zz`, on
9fbe827e). NOT-MERGEABLE on one MAJOR, a slow-set red the PR gate does
not select: a new test-side `edge_box(` caller missing from
`boxes::tests::every_door_that_reads_a_box_is_inventoried`'s pins.
Every soundness claim held:
- the crease rule is exactly `m ≥ p`;
- the near-crease bounds are tight to about 1e-12 at both scalars;
- `point_walk` reproduces the old walk bit for bit;
- the cornered rows read a real meeting.

The fix pass goes to the lane:
- the pin;
- the reviewer's negative-floor row, re-written rather than
  cherry-picked, since its commit message names a model (at f64 the
  `max(floor, 0)` clamp is load-bearing: a spline floor reads negative
  on 205 of 4000 random nets);
- an Interval creased-touch row for the `is_finite_length(bend)` guard;
- the crease cost filed on this slate. A creased spline, which includes
  lofted nets, loses the second-order clear, so a `PIECE_BUDGET`
  refusal is expected back once the gate is gone.

The orchestrator reads the delta.

**Class noted at adjudication:** a slow-set row that inventories the
tree is not run by a PR that changes what it inventories. Evidence
added to `ciw/a-diff-that-moves-corpus-ids-runs-none-of-the-slow-set-id-pins`.
From here on, SPLICE implementer briefs say to run the touched crates'
slow set (`cargo nextest run -p <crate>`) before reporting green.

## 2026-10-26 — PR 5715 (touch doors) merges

The second fix pass (d8d52d0d) answered the re-review:
- the boxes inventory pin;
- the reviewer's negative-floor row, re-written without the commit;
- an Interval creased-touch row, red without the `is_finite_length`
  guard;
- the cornered meet row asserts exactly one meeting;
- the prose for Interval's straddling floor.

Filed on this slate:
- `a-creased-spline-gets-no-second-order-clearing` (P1 M). The lane
  measured no crease in the klein loop's or the lily sheath's NURBS
  edges (all interior multiplicity 1);
- `the-touch-floor-reads-a-curved-arc-apart-from-the-pieced-arc`
  (P2 M), the fourth point-against-arc walk, left apart because folding
  it would move certified decisions.

After main (with PR 5719) merged in, the cell-door pin moved to
(121, 407), with `wrong` 0: the in-plane walk now crosses the spline
edge and the touch floor clears the cells beside it. The S4 guard
still reds under an always-clear floor.

The lane's topo slow set had one red, the lattice discard pin, which is
main's: it was bisected to PRED 5724 meeting FLUSH 5706, and is filed
in `work/issues/` (PR 5735). The orchestrator read the delta and merged
on green `gate ok`.

## 2026-10-26 — C5 gains the curve × face clause (PR 5727)

Ev took the converged design (fork log row 198) after asking about
dimensionality. Answered on the PR: a span × face meets in isolated
points; A and B differ only in eliminating the curve parameter; and a
boundary meeting is non-generic, so it is a coincidence. Ev's reply:
"if it is actually for the point then it's fine".

Filed on this slate:
- `a-span-is-read-against-a-faces-boundary-edges-first` (P0 H);
- `a-point-has-no-place-in-a-nurbs-faces-trim` (P1 M);
- `the-side-of-a-spline-face-is-read-by-no-dihedral` (P1 M);
- `curved-boolean-unsupported-names-no-edge` (P3 E).

The crossing rung is now an arm of the same table. SECT's row got a
correction: klein's join pair is cylinder × NURBS, tangent along the
rim.

Lily wall 8's path is now:
1. the boundary-first reading;
2. the dihedral side reading;
3. the two flush rows.

No interior arm is needed for it.

## 2026-10-26 — PR 5723 (the gate) merges

The operand edge gate is gone: a body built by loft or sweep now enters
every boolean, and each site refuses a spline or spiric edge typed. The
review (sequential arm, no MAJOR, so it was the unit's only review)
found no wrong reader. Fix pass `7b654b1100`:
- the census pins its builds, on fresh-keyed bodies;
- `face_of` is carried on the refusal;
- the refusals name the operand's own edge;
- the emit e2e obligation and the `NoSoundBox` finding moved to the
  crossing-rung row;
- the teapot's contact edge is the belly's meridian seam.

The dual-review row is `docs/dual-review-log/rows/PR-5723.md`.

The lane's slow sets found 16 rows red on bare main, none of them this
PR's. A nightly triage lane took them
(`session_014vYCa4XAZmPgeKRwfJrvFt`, run 40274924857).

Next on this track: `a-span-is-read-against-a-faces-boundary-edges-first`
(P0 H), lily wall 8's first step.

## 2026-10-26 — lily wall 8's first two steps dispatched

- `splice/boundary-first` (`session_01FNsFT2VKj6yRjpeBz5nY9P`):
  `a-span-is-read-against-a-faces-boundary-edges-first`. This is the
  curve × face table's skeleton, its ends, and its boundary pass through
  `curve_pair`; a shared rim glues at the coincidence door. Tier: dual,
  letter H, so a concurrent pair (the table's first pass is new
  incidence under every boolean, and the table's shape is hard to change
  later).
- `splice/spline-dihedral` (`session_01PXJ7h4STwjpmR9pXfHA6KH`):
  `the-side-of-a-spline-face-is-read-by-no-dihedral`. Tier: single,
  full.

Held until boundary-first lands, since they build on the table:
- the interior arm (`an-analytic-edge-…`) together with the converse
  rung (`nurbs-edge-crossing-rung-…`), as one brief;
- point-in-face for a spline chart;
- the two flush rows, which lily reaches after the side reading.

Lane briefs now ask for the touched crates' slow set before green, with
every red split against `origin/main`. Editor-core is the exception:
its full slow set takes hours on a 4-vCPU box, so lanes run only the
rows their diff can move.

## 2026-10-26 — PR 5764 (spline dihedral side) reviewed: one MAJOR

Single full review (`session_01QgwBdDPBpAwPsCKJFCbcHZ`, on 39e05242):
NOT-MERGEABLE. The MAJOR, shown by a red row, is a face trimmed along
an interior C0 knot line of its carrier. The foot lands on the knot,
the derivative is evaluated on the span convention's side, and the
door reads the other half's normal, deciding a wrong side.

The sign convention held under mutation. The other findings: the κ
fold and `FootUndecided` are unpinned; the walk direction is a bare
`Vec3` held by convention; `splitting::rules::edge_wedge` is an analytic
copy of the door; and there are three spellings of "into the face =
n × t". All of it went to the lane. The same tier re-reviews the fix
pass.

**Class, the second instance in one day:** a derivative read across a
spline crease (an interior knot of multiplicity ≥ degree). The first
was PR 5715's curvature bound across a C0 curve knot; this is a surface
normal at a foot on a C0 knot line. One statement of where a spline has
no derivative (`KnotStructure::crease_runs`) must be consulted by every
reader that takes a derivative at a point on a spline. The
boundary-first lane was told while still building.

## 2026-10-26 — PR 5764 re-reviewed: a foot on the wrong sheet, down-rated

The re-review (`session_01JTtugdpZZVE2CKi7NHrnKA`, on ecfc622d)
confirmed the crease fix. It raised one MAJOR, shown by a red row: a
hairpin carrier folds back over itself within the band, `project`
returns a foot on the other sheet, and the door decides a wrong side.
The class is "a foot on another sheet of the carrier": the door never
reads the face's trim, which C5's curve × face clause requires. The fix
is point-in-face for a spline chart
(`a-point-has-no-place-in-a-nurbs-faces-trim`), which is unbuilt.

**Orchestrator's call (unilateral, logged): down-rated to a scheduled
row rather than blocking the PR.** Nothing calls the door yet. Its
contract will say the foot is decided on the carrier, not the trim.
The wiring row is parked on point-in-face, and the hairpin fixture
becomes point-in-face's witness. The alternative was holding the PR
until point-in-face lands. That buys nothing while the door has no
caller, and it leaves the fixes below unmerged.

Fixed now, because they make false decided answers:
- an `OffFace` decided at an irregular (zero-speed) foot;
- an irregular chart point that escalates only by accident.

Also in the fix pass:
- the `EdgeWalk` public mint narrowed;
- the seam-continuity over-escalation and the foot-reader class
  (`.project(` then `.ders(`) filed.

The orchestrator reads the delta.

## 2026-10-26 — PR 5768 (boundary-first) dual-reviewed

Concurrent pair (letter H), both on 98ee1373, dispatched together at
11:29Z with identical briefs:
- R1 `session_01QGkdu4DiX62671SWzvo6iZ`, evidence `splice/review-5768-r1` @ c4e4fad;
- R2 `session_01DAybgkUR3a16QpYg3izStL`, evidence `splice/review-5768-r2` @ b27f4c5c.

Both read APPROVE-WITH-FIXES, and both report no glimpse of the other.

**Correspondence pre-note** (DUAL-REVIEW-PROTOCOL rule 7):
- Bilateral:
  - `twin_edge`'s validity premise is assumed, not checked
    (`validate.rs` defers clearance);
  - `twin_edge`'s gate survives mutation;
  - the `MintedRule::Along` / `read_lying_in` arms and the Along split
    are unreached;
  - an ellipse `param_near` may misplace on the new split path;
  - `twin_edge` swallows kernel faults;
  - the shared-run predicate and the span closure are duplicated;
  - `route()` is a table, not a dispatch;
  - the `boundary_spans` precedence changes.
- Unilateral R1:
  - `BoundaryCrossing::Clear` conflates Along;
  - "readings disagree" is a frontier refusal, not an invariant;
  - the coincidence door is unbuilt;
  - `census::boundary_crossings` is a third reader;
  - the sweep row cannot tell the paths apart.
- Unilateral R2:
  - the vertex reading has no row that reds without it;
  - C5's ends-first order;
  - `twin_edge` deciding a Clear span from topology as a possible C5
    departure.
- Tally candidates: none. Neither review raised a MAJOR.

The union fix list went to the lane. The orchestrator reads the
fix-pass delta. **Open to Ev:** whether `twin_edge` (deciding a Clear
span's interior from y's topology under the boolean's
non-self-intersection precondition) is consistent with C5's "only what
the boundary leaves undecided is read against the face's interior", or
needs a sentence in C5.

## 2026-10-26 — PR 5768 (boundary-first) merges

The fix pass `76d5bbcfd5` answered the dual pair's union. The
orchestrator read the delta: no second review, since the pair is
concurrent and raised no MAJOR. Dual-review row:
`docs/dual-review-log/rows/PR-5768.md`.

The curve × face table now exists in geom-brep, with its boundary
pass. Where the demos stand:
- lily wall 8: reaches the converse arm (a blade NURBS seam against the
  sheath's planar base cap);
- teapot wall 3: reaches `PointSplitCarrierUnsupported` on the spout's
  NURBS seam (`a-meeting-on-a-spline-boundary-edge-has-no-split-parameter`);
- klein wall 3: unchanged, behind the cap tilt.

Next on lily's path: the table's interior arm together with the
converse rung, as one brief. `twin_edge`'s fit with C5 is put to Ev in
chat. The code is kept, and its premise is stated as the boolean's
non-self-intersection precondition.

## 2026-10-26 — PR 5764 (spline dihedral side) merges

The second fix pass, `49e19569`:
- the door's contract now says the foot is the CARRIER's, and a caller
  may consume the side only once the foot is placed in the face; the
  wiring row is parked on point-in-face, with the hairpin fixture as its
  witness;
- a regularity check (`irregular_reach`) runs before any side, so a
  foot near an irregular point (a kink or a pole) escalates `Crease`;
- `EdgeWalk`'s public mint takes an `EdgeHalf`.

Filed:
- the zero-speed foot in geom's `project`;
- seam continuity (P3);
- the foot-reader class `a-derivative-is-read-at-a-foot-no-one-placed`.

The orchestrator read the delta. It merged after main's RESTREAD red
was fixed (ATREST PR 5779).

## 2026-10-26 — the converse arm dispatched, stacked

`splice/converse-arm` (`session_01GJUH1SbY5bRWRdPiwDHdvq`):
`nurbs-edge-crossing-rung-is-the-ring-composite`. This is the
curve × face table's spline-span × analytic-face arm, with the edge's
control-hull box, the end-to-end crossing row, and the crease rule.
Tier: dual, letter H.

It is **stacked on `splice/boundary-first`** (PR 5768: reviewed,
accepted, held only by main's RESTREAD red). Stacking was chosen over
waiting, because lily wall 8's next refusal is exactly this arm and
5768's table is final. The PR targets main and merges main once 5768
lands. The interior arm (`an-analytic-edge-…`) follows on the same
table after this one, rather than in parallel, so two lanes don't
collide in the new module.

PR 5764 (spline dihedral) and PR 5768 are both read and accepted, and
both wait only on RESTREAD's main red
(`work/restread/main-red-coincident-wedges-overlap-refuses-flush-copies.md`).

## 2026-10-26 — two rulings for the converse-arm lane

- **Witness 3** (the far loft prism against the `[5,6]^3` brick)
  stops past the hull at two more refusals. One is
  `sphere_extent_scan`'s NURBS re-gate. The lane scopes it
  reach-first, in ops.rs, on this PR, as an announced seam. The other
  is `point_in_solid`'s `KindUnsupported`. A box exit rides this PR
  only in its sound form: q outside the selection's overall certified
  box. "Outside every face's box" is unsound, since a box solid's
  centre is outside every face slab yet inside the solid. A row must
  pin that. If the sound form does not build the far case, the exit is
  filed on the owning slate and the rows are re-pinned.
- **The end-to-end row** stops at `sectors.rs`'s `sector_face::resolve`,
  which has no NURBS arm. The lane pins a tripwire there, adds a
  sweep-level witness, and files
  `the-sector-walk-reads-no-nurbs-face-at-a-crossing-vertex`, which
  carries the owed ranking row and the spline-crease caution.
- 2026-10-26 — **Seam (CLEAVE):** filed `main-red-the-teapot-north-star-test-expects-the-old-spline-face-refusal` (P1 E) on your slate. Since the boundary pass, the python north-star teapot test expects the old spline-face refusal text and gets `PointSplitCarrierUnsupported`. The python suite runs only on PRs that touch `pncad-py`, so CLEAVE's PR 5544 was the first to see it red. (CLEAVE orchestrator)

## 2026-10-26 — PR 5789 (the converse arm, spline span × plane) merges

A concurrent dual review (letter H), frozen at `a2cec4f985`. Both
reviewers: APPROVE-WITH-FIXES. Both fuzzed the root counting against a
dense oracle, 3000 and 364 cases, with 0 mismatches. Bilateral
findings:
- an in-band pass read clear by sign alone, against C5's "an in-band
  decision escalating and never pruning" (r2 MAJOR, r1 MINOR);
- the crease rule coded twice, each half masking the other;
- re-pins matching the variant only;
- the subdivision ladder hand-rolled beside `geom_core::bisect`.

r1 alone: the knots-f64 ratchet hidden behind a funnel, and
`uncrossable` read eagerly ahead of the boundary pre-pass. C5 already
decides the in-band case, so it was fixed to the contract rather than
put to Ev.

The fix pass is `36bb222376`; the orchestrator read the delta.
Clearance is now the residual metered past the band. The crease rule
has one home, in metres. The walk is `geom_core::bisect`. The baseline
was raised honestly from 219 to 221. The exit is lazy. Re-pins name
their face. `2174c44c5f` re-pins the M7-8 disjoint union, which now
builds its assembly through the arm.

Added to the curved-dispatch row:
- the off-trim routing (r1);
- the chart frame's rounding (r1);
- the metering divisor's region, the hull grown by the band
  (orchestrator).

Next on lily's path:
- the spline-face door's interior arm
  (`an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane`), where
  lily wall 8 now stops;
- the sector walk's NURBS arm (P0, filed by this unit).

## 2026-10-26 — two lanes dispatched: point-in-face and the sector walk

- `splice/point-in-face` (`session_01UeAbQJ2zE9Jn1YEQa2tvML`):
  `a-point-has-no-place-in-a-nurbs-faces-trim` (P1 M). Letter M, draw
  byte 48, 48 mod 3 = 0: a holdout, so a concurrent pair. It unblocks
  the parked wiring row and teapot wall 3's pierce placement, and the
  interior arm places its meetings through it.
- `splice/sector-walk-nurbs` (`session_01E1iVdA3wv8cshkHCxPcy6D`):
  `the-sector-walk-reads-no-nurbs-face-at-a-crossing-vertex` (P0 M).
  Letter M, draw byte 59, 59 mod 3 = 2: sequential. It owes the wavy
  loft's end-to-end row. The brief binds the crease class and reuse of
  `dihedral_side`'s regularity and crease readings.

The two keep out of each other's files (`contain.rs` and
`sectors.rs`/`sector_face.rs`). The interior arm
(`an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane`, P0 H),
where lily wall 8 now stops, goes next, once point-in-face lands.

## 2026-10-27 — PR 5802 (the sector walk's NURBS arm) merges

Letter M, sequential. One full review at `5ec4b34ddf` gave
APPROVE-WITH-FIXES, with no MAJOR and no claim broken. Its four
curved-chart rows passed, and mutations of the end choice, the sense
fold, the half-edge, the crease check and the regularity check each
went red. There were three MINORs:
- the tripwire lost its face check;
- no boolean row reaches the new refusals;
- the PR's affine corner row is blind to the end choice.

The fix pass is `b2c0a9ba68`, and the orchestrator read the delta.
The face check is restored. The refusal mapping is pinned on the
shared walk's fixture, and the doc says why no boolean reaches it
today: a corner's own knot refuses at the crossing rung first. The
control-net reach has one home, `geom_brep::control_net_reach`.

The wavy-loft union now passes the sector walk and stops at the held
pierce (`a-spline-wall-cannot-say-whether-it-holds-a-pierce-point`,
P0, filed by this unit). The point-in-face lane measured that this
stop does not move with PR 5805: the wall's trim has no pcurve on a
split piece mid-boolean (`LiftGap`). That finding is recorded on the
row.

**Process note.** PR 5789's lane did not run sweep's slow set, though
it touched sweep tests. Five unfiled sweep and pncad-py slow-set reds
on main surfaced through this lane's run. They are triaged on
`splice/slow-set-reds`. Every SPLICE brief now names sweep's slow set
explicitly.

## 2026-10-27 — PR 5805 (point-in-face for a spline face) merges

Letter M, holdout (byte 48), so a concurrent pair, frozen at
`d4b983c13a`. Both reviewers: APPROVE-WITH-FIXES.

R1 raised one MAJOR, shown by a red row: on a closed chart each loop
was lifted on its own, so a hole one period from its outer loop read
In. R2 raised no MAJOR and did not exercise closed charts. Blinded
coding (byte 16) tallies R1's MAJOR, which makes this an **M-tier
miss**. The short readout to Ev rides an `[ev]` PR.

Bilateral: the Out rule, the parity half-open rule and the caps had no
red row, and the reading drew on two period sources.

The fix pass is `6b5c472181`, plus `5086a8a8ec` for an ε-scaled row.
The orchestrator read the delta.
- Rings are lifted into the outer loop's cover by one decided whole
  period.
- A square tube closed in u, with a seam face and a hole, is built at
  rest as the witness.
- One period source; clearance through the metric door; caps
  witnessed.
- Seam and pole completeness is filed P2
  (`a-spline-faces-trim-reads-nothing-at-a-closed-seam-or-a-pole`).

The wavy-loft union still stops at the held pierce, because a split
piece has no pcurve mid-boolean. The parked wiring row is unparked.

The lane had not run sweep's slow set in its first pass; it ran it
before reporting. Its nine reds are all main's, already filed or fixed
in PR 5814.

## 2026-10-27 — the interior arm dispatched

`splice/interior-arm` (`session_01EdpJs5Doyx3iN4jJo1CRrf`) takes
`an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane` (P0 H,
letter H, a concurrent pair). This is where lily wall 8 now stops.
- It builds C5's analytic span × spline face arm by enclosure or
  Krawczyk, and places each meeting through PR 5805's point-in-face.
- The brief binds the crease class (four finds now), feet on another
  sheet, ε-scaled rows, and sweep's and topo's full slow sets.
- PR 5814 (main's slow-set reds and the past-every-reach axes) is on
  its fix pass. PR 5821 is the `[ev]` short readout for the M-tier
  miss at DR-353.

## 2026-10-27 — PR 5814 (main's slow-set reds; the exit's axes) merges

Five slow-set rows were red on main and filed nowhere; the lane
bisected each.
- **Two are SPLICE's, from PR 5789.** Its past-every-reach exit read
  the reach hull on world axes, so 6 of 13 poses refused. It now reads
  along planar face normals plus the anchor→point axis. The lofted
  union check's re-pin now names the concave wall.
- **The rest are other programs'.** FLUX (PR 5734), CLEAVE (PR 5546)
  and QUAD (PR 5741, found while running topo's set) are filed on
  their slates with bisects.

Letter M, sequential (byte 202). The one review was APPROVE-WITH-FIXES
with no MAJOR. The exit is sound for any direction. The planar-normal
candidate had no red row; the reviewer's probe adopted. One fix pass,
`a289dc31`.

The root cause is a process gap: hosted CI runs no slow set, and
PR 5789's lane skipped sweep's. Every SPLICE brief now binds sweep's
and topo's full slow sets.

## 2026-10-27 — PR 5837 (the capped box's probes scale with ε) merges

PR 5814 adopted the reviewer's capped-box probes, but at fixed 1e-6 m
offsets. Those fall inside the band at ε 1e-6, and the nightly runs the
slow set at that ε, so it went red. RESTREAD filed the red on our
slate.
- The near-face probes now stand off by the cap's fit plus 4·escalate.
- Both of PR 5814's mutations still turn the row red at all three ε,
  verified locally.
- Hosted CI does not run this slow row at the ε rows, so the local runs
  are the evidence until the nightly.
- The RESTREAD item is closed with PRs 5814 and 5837 named.

The orchestrator read the diff, which is test-only, so it got no
separate review. This is the third ε-unaware probe today (5805, 5814,
and 5826's re-check). Every brief now binds ε-scaled geometry.

## 2026-10-27 — PR 5826 (the interior arm, analytic span × spline face) merges

Letter H, a concurrent pair, frozen at `b2728f422d`. Both reviewers:
NOT-MERGEABLE-AS-IS, on one shared MAJOR. The chart-edge-near-end skip
pruned in-band floor boxes, so a tangency, or a transverse crossing on
a closed seam, returned `Ok([])` at all three ε. That broke C5's
"never pruning". The MAJOR is bilateral, so nothing is tallied.

The fix pass is `0271d125cd`; the orchestrator read the delta.
- The skip is deleted. A span ending on an open chart's edge now
  refuses, which is recorded.
- One crease home, `Cuts::touch`; `crease_apart` is gone.
- Krawczyk pieces live in `geom_brep::krawczyk`, and `meet3` fails
  loud on a disjoint enclosure.
- An arm-level sheet-exactness row was added, and the v-f ends read in
  space are filed.

What it changes on the demos:
- Lily wall 8 was premised wrong. It is (Nurbs, Nurbs):
  `a-spline-span-against-a-spline-face-has-no-interior-arm`, now lily's
  next blocker.
- Klein wall 3 moves to a Crease refusal on a G1 rational-arc joint,
  filed as `a-meeting-on-a-g1-arc-joint-of-a-spline-face-refuses-as-a-crease`.
- Teapot wall 3 is unchanged.

Next: the spline × spline interior arm (lily). The held-pierce split
pcurves (the wavy loft) and the G1-joint crease (klein) also remain.

## 2026-10-27 — the spline × spline arm dispatched

`splice/spline-span-arm` (`session_01EdpJs4Ez9ZNEZk53hwLNSN`) takes
`a-spline-span-against-a-spline-face-has-no-interior-arm`, re-banded
P1 → P0 because it is now lily wall 8's blocker. Letter H, so a
concurrent pair.
- The lane generalises PR 5826's `(t, u, v)` arm over the curve side
  rather than copying it.
- The brief binds the crease class on both curve and surface, the
  sheet-exact placement, ε-scaled rows, and the full slow sets.

## 2026-10-27 — PR 5855 (the spline × spline arm) merges; lily moves

Letter H, concurrent pair, frozen at `015e27412e`. Both reviewers:
APPROVE-WITH-FIXES, no MAJOR. Both re-derived the new end bound
(`krawczyk::solution_reach`) and found it sound, and every swallow probe
was reported or refused at three ε.
- Bilateral: the end-box gates had no red row, and an end on a span
  crease refused needlessly.
- The fix pass is `4742bb0286`. Every gate mutant now reds a per-PR
  row. An end on a crease is the end's. `SpanCarrier` pairs a curve
  with its sheet.

**Lily wall 8 moves past every face reduction.** It now stops at the
result's seam re-description: `classify_dihedral` reads no spline, on
the shared rim between two NURBS walls. That is filed as
`the-results-seam-between-two-spline-walls-is-read-by-no-dihedral`,
and it is next on lily's path. Klein and teapot are unmoved.

## 2026-10-27 — the spline seam reading dispatched

`splice/spline-seam-dihedral` (`session_01EdpJs76R3BtUnnQPDh3NwV`) takes
`the-results-seam-between-two-spline-walls-is-read-by-no-dihedral`. It
is re-banded P1 → P0, as lily wall 8's next stop.
- Letter M; draw byte 36, and 36 mod 3 = 0, so a holdout: a concurrent
  pair.
- The brief: each face's normal is read at the edge's pcurve parameter
  on that face (sheet-exact, as PR 5802's sector walk), not at a foot.
  It reuses `dihedral_side`'s regularity and crease guards. The arm is
  read from certified curvature bounds.

## 2026-10-27 — Ev ratifies C5's ladder of evidence (PR 5860)

Ev asked for C5's curve × face clause to be made general rather than
given a `twin_edge` exception. PR 5860 rewrites it as one ladder, each
rung reading only what the rungs before it left undecided:
1. topology, by incidence, under the boolean's validity precondition,
   which it names as assumed;
2. the span's ends;
3. the face's boundary edges;
4. the face's interior.

Ev approved it ("5860 looks good!"). Only the wording changed.

## 2026-10-28 — PR 5868 (the spline seam reading) merges; lily's rim reads Transverse

Letter M; draw byte 36, a holdout, so a concurrent pair, frozen at
`06e486b9af`. Both reviewers found the same MAJOR, so nothing is tallied
and nothing is missed (row `PR-5868`). The MAJOR: the arm `1/κ̄` over
the whole chart is the loose direction (`implicit.rs:309-314`), and a
wall flat near the seam with a far bend read a true corner Smooth.
- Fix pass 1 made the arm a region-local fixed point, `chart_arm`.
- A single re-review at `abf40f1d12` found the class still open.
  `chart_arm` is a certified lower bound with unbounded slack: a flat
  wall at uneven speed reads arm 0.00375 against 1.0, and a refused cell
  near an apex shrinks it to its distance.
- Fix pass 2 (`833da0303c`) brackets the decision. The wedge is read at
  hi, the spline arm taken as the reach. Transverse also needs
  `sin θ · lo` decided, and anything else escalates at the arm. So a
  lower bound's slack costs an escalation, never a class. The
  orchestrator re-ran the seam rows at three ε, and the "wedge at lo"
  mutation reds both ported counterexamples.

**Lily wall 8:** the rim reads Transverse, decided at lo. She stops at
the rim's certification, whose cause moves with ε:
- at 1e-9 and 1e-12, `PlaneNurbs Limb ImageMiss`;
- at 1e-6, `PlaneNurbsHullValue` (the sweep row) or the pair's
  transversality (the tour).
That is SSIEDGE's `plane-nurbs-certificate-bound-does-not-refine-with-eps`,
and it is lily's next stop.

Filed by the lane:
- `the-chart-curvature-bound-over-reads-a-flat-chart-at-uneven-speed`
- `the-seams-chart-arm-builds-every-cell-of-the-chart-per-read`
- `a-smooth-seam-between-spline-walls-has-no-second-order-reading`
- `the-shared-rim-routing-exempts-a-spline-face-by-kind`
- `a-seam-on-a-creased-spline-chart-reads-no-arm`

**The at-rest main red.** During the fix passes, demo-tour's chain
gallery read 10 at-rest findings against its pin of 196. That was
PR 5853's change: its CI skipped demos, and main's push CI runs none.
The orchestrator reproduced it on main at `2d0d7307`, and the lane filed
it as `atrest/main-red-the-gallery-chain-pins-196-at-rest-findings-after-pr-5853`.
Main re-pinned the chain at `0f4502051b`, so the row is ATREST's to
close.

**Outside SPLICE's ground:** the lane added a new curve-pair
touch-window fuzz seed (`0xdcf91059414eafd4`) to CPAIR's open row
`a-touch-window-fuzz-seed-contradicts-the-dense-distance-at-1e-12`. It
turned up in the lane's slow sets.

## 2026-10-28 — the rim's exact image dispatched

Lily wall 8's next stop is the rim's certification. `nurbs_pair_limbs`
reads each chart's degree-1 image fitted through feet, though the rim is
an exact `IsoArc` row of each wall. Filed as
`a-rim-between-two-spline-walls-is-certified-against-a-fitted-image-not-its-exact-one`
(P0, lily's blocker), and dispatched on `splice/rim-exact-image`.
- The precedent is `plane_nurbs_incidence`'s `exact_arc_rim` arm, so
  this is the same arm on the NURBS-pair lane, not a design fork.
- SSIEDGE's fixed-schedule design row stays open for edges with no
  exact image.
- The ground is shared with `isotail`, `select` (edge_nurbs.rs,
  certify.rs), `ssiedge`/`ssiarith`/`ssimarch` (ssi/certify.rs) and
  `pcert`/`pctail` (pcurve_cache.rs). All of them are ready with no live
  lane, so the seam is announced on the PR.
- Letter H (a certificate's soundness), so a concurrent pair.
- The brief binds the full slow sets of geom-brep, topo and sweep, plus
  demo-tour's three ε rows, which CI does not run (lesson from PR 5868).

## 2026-10-28 — PR 5907 (the rim's exact image) merges; lily reaches limb 3

Letter H, a concurrent pair frozen at `c40a01aa44`. Both reviewers:
APPROVE-WITH-FIXES, no MAJOR, so nothing is tallied (row `PR-5907`).
Both re-derived the mixed certificate's containment and found it sound,
and both found that the fallback launders nothing.

The reviewers disagreed on a fact: does a rim interior to both charts
certify at limb 3? The lane settled it by measurement. Limb 3's
`TubeNotOneArc{Undecided}` is decided by where the carrier lies against
the domain-cut box, in two shapes:
1. an arc along a chart's boundary row;
2. a carrier ending on both charts' domain sides.
Span, ε and the sub-arc count only decide whether the carrier reaches
limb 3 at all.

**Lily wall 8:** limbs 1–2 now pass on the exact images, so she reaches
limb 3. She has both shapes, and shape 2 (her end on a corner of both
charts) fires first.
- Shape 1 is `ssiedge/a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary`,
  re-derived in this PR.
- Shape 2 overlaps `nurbs/a-pair-walk-end-at-a-domain-corner-refuses-undecided`,
  a P4 that says it has no consumer. Lily is one.
- Both stand between lily and limb 3. The routing is the orchestrator's
  next step.

Filed by the lane:
- `ssiedge/a-tube-whose-graph-margin-straddles-zero-refuses-as-a-decided-zero`
  (P1): undecided is read as decided. This is lily's tour stop at default ε.
- `splice/lily-wall-8-at-1e-6-reads-the-rims-crossing-angle-in-band` (P1).
- `issues/main-red-two-tori-subtract-refuses-on-tangent-carriers-not-on-no-section-classification`.
  SECT bisected it to its own PR 5869 (not 5886) and is fixing it on
  `sect/two-tori-subtract-red`. Its PR moves the row into `work/sect/`.

The lane also gave both lanes one `ArcRimRefusal`, behind one mapping
door. The orchestrator re-ran the arc-rim, any-carrier and on-pair rows
at three ε, and the exact-never-used mutation reds 5 of 6 rows.

## 2026-10-28 — limb 3's two shapes claimed; a design fork weighed

Lily's next stop is limb 3's one-arc walk (PR 5907's log entry). Its
two rows move into SPLICE:
- `ssiedge/…a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary`
  (P0). SSIEDGE is ready with no live lane.
- `nurbs/…a-pair-walk-end-at-a-domain-corner-refuses-undecided`. It is
  re-banded P4 → P0, because a blocker inherits the band of what waits
  on it, and lily is the consumer the row said did not exist.

These are moves with no design implication (CLAUDE.md), so they are
done and logged.

How to read a solution set that runs along a cut box's face, or ends
on two faces at once, is a design fork. Candidates include a box
search over the charts' extension, an edge reading, or the topology
the operands already state. One Opus and one Fable designer weigh it
under `docs/DESIGN-FORK-PROTOCOL.md`, blinded (byte 123) on
`analysis/design-fork/limb-three-boundary`. No lane is dispatched
until they are reconciled.

## 2026-10-28 — PR 5938 (the tube's side tripwire) merges

A single FULL review: APPROVE-WITH-FIXES, with no MAJOR (row `PR-5938`).
The two-chart tube now names its `Undecided` as `AlongSide` or
`EndOnSides`, as a rename of a failed walk only. The reviewer's probes
up to ε/4 inside the row never reached an `Ok`. The single-chart twin
stays `Undecided`: past PR 4012's side arm, its in-band readings are
indeterminate.

Lily's sweep row pins `TubeNotOneArc { AlongSide }`. The structural
rim (PR 5959, in flight) is what moves her past it. With limb 3 stubbed
she stops at the interior-loop guard, and the first pair it refuses is
Plane × NURBS (GERMTAIL's `nurbs-plane-section-has-no-component-arm`,
P3), before the NURBS × NURBS arm
(`no-certificate-excludes-a-closed-loop-interior-to-two-spline-faces`,
P1, filed here). Both are routed once the structural rim lands.
## 2026-10-28 — limb 3's fork reconciled; to Ev

Three rounds. The designers converged:
- lily's rim is a structural selector under D2 (PR 5216), owing no
  limb 3;
- the boolean's `describe_edges` must recover the selector, not map
  `Transverse` to a witness;
- a carrier along a domain side refuses by name.

The ratified-text change goes to Ev as an `[ev]` PR. C1/C2's limb-3
wording (PR 5652, 2026-10-25) is later than D2's ruling (PR 5216,
2026-10-16), so the two conflict in letter. Both designers recommend
that C2 open on a witness-selected carrier.

The near-rim exclusion belongs to the boolean's interior-loop guard. It
has no NURBS×NURBS arm and refuses spline pairs. SPLICE files that row.

SELECT owns the selector type (unit 1) and import's recovery (unit 6).
The boolean-recovery unit is in no program's list yet. SPLICE takes it
once Ev rules, and asks SELECT's blessing on the PR.

## 2026-10-28 — Ev approves the limb-3 re-wording (PR 5933)

Ev: "nice catch, sounds good!". C2 now opens on a witness-selected
carrier, and a structural selector is certified per surface, owing no
limb 3. Fork-log row 208 is completed: B's first recommendation, as
converged. Next is the boundary-row recovery in the boolean's
`describe_edges` and the split's `finish`, which SPLICE takes.

## 2026-10-28 — lily's next stop filed on SELECT; the tori red closes

`select/the-boolean-recovers-no-structural-selector-for-a-rim-known-in-advance`
is filed at P0. It needs SELECT's unit 1, the selector type, and SELECT
has nothing dispatched. Ev was asked whether to start a SELECT
orchestrator or have SPLICE take the units. SECT fixed the tori main
red in PR 5925, so its issue file is closed.

## 2026-10-28 — SPLICE takes the structural selector's lily slice

Ev declined a SELECT orchestrator, since SELECT has one P0 and no P1:
"you can take it yourself if you like".
- `select/the-boolean-recovers-no-structural-selector-for-a-rim-known-in-advance`
  moves to SPLICE (P0).
- SPLICE builds the slice of SELECT's unit 1 (the selector type, with
  per-surface certification) that it needs: the boundary-row structural
  selector and the boolean's recovery.
- SELECT's umbrella row
  (`onpair-has-no-incidence-lane-on-a-spline-adjacent-pair`) and its
  other units stay SELECT's.

Dispatched on `splice/structural-rim`. Letter H (a type every two-surface
edge reads), so a concurrent pair.
## 2026-10-28 — SPLICE splits along its seams

The load was 91/30. Ev asked whether to split the track, keeping in
SPLICE the threads its orchestrator should continue. It split four ways,
by mechanism, with the rows moved by `git mv`, ids kept and a dated
Moved note on each:
- **SPLICE** (20/30 plus the structural-rim row in flight): lily wall
  8's path. The limb-3 rows, lily at 1e-6, the census main red.
- **SPLICECARRIER** (20/30, 5 rows): a NURBS carrier has no parameter
  inverse. The join, the split parameter (teapot wall 3), the two flush
  readings, the touch floor.
- **SPLICEFACE** (26/30, 10 rows): a spline face read on its own sheet.
  End placement, a foot no one placed, closed seams and poles, creases
  (klein wall 3's G1 joint), the curved-face dispatch, the zero-speed
  projection, the held pierce.
- **SPLICESEAM** (25/30, 13 rows): the seam reader's residue after PR
  5868, and the doors that still read a spline by kind.
The three new tracks are `ready`, with no orchestrator.

## 2026-10-29 — two main reds filed on their owners (the face_normal fold census)

PR 5959's lane found topo's slow-set row
`face_normal::tests::no_source_file_folds_the_bit_by_hand` red on
main, and the orchestrator reproduced it there. It has six hand folds
of a face's sense bit, from two sources. Filed on their owners:
`tess/main-red-trimmed-rs-folds-the-face-sense-by-hand` (PR 5920) and
`clear/main-red-injective-rs-folds-the-face-sense-by-hand` (PR 5955).

## 2026-10-29 — PR 5959 merges: the structural rim; lily stops at a tangent rim

PR 5959 adds D2's selector to `Intersection`, with one structural form,
a shared row, certified per wall with no limb 3. The boolean recovers
it for a rim that is a domain-side row of two spline walls. It had a
concurrent review pair (letter H), three fix passes and two single
re-reviews; the row is `docs/dual-review-log/rows/PR-5959.md`.
- **Pair, bilateral MAJOR:** the walls' crossing was read at the 7
  CERT feet only, so walls tangent between feet certified.
- **Re-review 1, MAJOR:** the whole-row cover's 1-ulp nudge skipped a
  tangency on a chart far from 0.
- **Re-review 2:** APPROVE-WITH-FIXES.

The whole-row bound found lily's walls tangent at two points of one
rim (u ≈ 0.3024 and 0.6977), confirmed independently by sampling the
walls' derivatives. Lily wall 8 now refuses at the rim at every ε and
never reaches the interior-loop guard. The guard's spline arms went to
a design fork. Its `[ev]` PR 5990 also asks Ev what SPLICE's witness
becomes.

Filed: `doors-carrying-a-shared-row-have-no-row-at-rest` (P2) and
`no-row-certifies-a-shared-row-at-interval` (P3).

## 2026-10-29 — PR 6040 merges: the two geom-core census reds

Review tier: a single FULL review, because the unit fixed soundness
reads. The review (frozen at 889bc2da) was APPROVE-WITH-FIXES with no
MAJOR.
- Five reads are fixed: `spline_face` `Path::at`/`tangent`,
  `offset_meters::cell_distance`, `limb2_offset_sup`,
  `blossom::whole_bezier_span` and `enclose::first_order_continuous_in`.
  Each fixed read has a row that reds when its fix is reverted.
- `four::along_side` now asks `is_certified`. The reviewer showed its
  "renames only" disposition was untrue: it could move an `Undecided`
  refusal to `AlongSide`.
- `chart_arm` refuses a non-finite `p`. A NaN `p` used to answer
  `Some(reach)`.
- One plant for a refused point bracket,
  `geom_core::test_support::refused_point`.
- `clearance.rs` and `injective.rs` are pinned at main's counts. Their
  unsound reads are filed on CLEAR, each with its line:
  `clear/clearance-lifts-trv-brackets-and-reads-their-ends` (P1).

I read the fix pass myself, with no re-review.
## 2026-10-29 — PR 5990 merges: the guard's spline arms, and `[one-way.readings]` re-phrased

Ev approved the design for the interior-loop guard's spline arms, and
ratified the W5 stage after a probe settled the factual crux.
`[one-way.readings]` is re-phrased so that stages always run together
may jointly prove one rule (fork row 217). Ev also approved re-spelling
lily's leaf as one loft, with a stem graft as the new wall 8, now in
flight.
- Filed `w5-loop-free-stage-before-the-plane-nurbs-whole-reading` (P1),
  to be built after PR 6026.
- Re-banded P0 → P3: `a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary`
  and `a-pair-walk-end-at-a-domain-corner-refuses-undecided`. Neither
  has a consumer.

## 2026-10-29 — PR 6042 merges: lily's leaf is one loft, and wall 8 is the leaf grafted onto the stem

Ev approved this change on PR 5990, question 3. Review tier: a single
STYLE review (a scene change, re-baselines and tracker re-points),
APPROVE-WITH-FIXES. I read the fix pass myself.
- The long leaf is one loft. It is authored from a root on the lower
  stem's axis: the sheath curls out of the stem, and the blade arches
  over from where the sheath ends.
- Wall 8 unions the stem and the leaf as they are authored, and they
  cross transversally. It refuses `EdgeCarrierUnsupported
  { CurvedCrossing }` (a leaf NURBS seam against the stem's torus) at
  1e-9 and 1e-6. At 1e-12 the stem's frontier stands it down.
- SPLICE claims lily's next stop from SPLICEFACE:
  `a-spline-span-against-a-curved-face-is-not-dispatched-to-the-composite`.
  Behind it is the NURBS × torus section, filed on SECTARMS.
- The old two-loft graft stays as the sweep row's witness that a rim
  tangent at two points refuses.
- The two SPLICECARRIER flush rows go back from P1 to P3: no live
  consumer remains.

## 2026-10-30 — the curved lane reads a spline span (branch `splice/spline-span-curved-crossing`)

- Closed `a-spline-span-against-a-curved-face-is-not-dispatched-to-the-composite`.
  The curved lane now dispatches a spline span to the ring composite
  against a cylinder, cone, sphere or torus. Two of the three owed
  items are built: the chart frame is formed in certification
  arithmetic, and an unsettled count goes through
  `carrier_touch::off_face` on both lanes, which now reads a plane. The
  third, the divisor's region, rests on a gradient premise that does
  not hold, because the divisor is a factor of the form at each point.
  A row now pins that.
- Lily wall 8 re-pinned at 1e-9 and 1e-6 on `CurvedBooleanUnsupported
  { operand: B, kind: Nurbs }` from the vertex-on-face pass. Filed
  `a-vertex-piercing-a-spline-face-reads-no-outward-normal` (P1).
- 2026-10-31 — (SECTARMS orchestrator) **Coupling note.** `[ev]` PR 6121 (SECTARMS's P0) proposes one section arm for a NURBS face against any implicit surface, `φ = f∘S` over the knot rectangle. It generalises the `(Plane, Nurbs)` arm PR 6026 is adding to `pair_section`, and it proposes building `w5-loop-free-stage-before-the-plane-nurbs-whole-reading` as the plane instance of one pre-march regularity sweep, keeping the ratified home and order (fork row 223). Nothing is asked of 6026. If Ev approves, the W5 row's shape may move: please read 6121 before building W5.
- 2026-10-31 — (SECTARMS orchestrator) **Coupling note on PR 6026.** SECTARMS's cylinder × cylinder unit (branch `sectarms/cylinder-pair-on-the-value`, stage 1) lands the component-with-ends type the approved designs share (forks 224 and 226): `ComponentShape::Open { ends }` carrying the rims reached per operand (`[ChartReach; 2]`, your shape), a cut end and node ids. It is meant to subsume 6026's `Open { reaches }`, so 6026's three construction sites and its W1′ reader become one-line rewrites. The unit also adds a region to `pair_section`'s inputs, where 6026 adds a march door (`pair_section_with`). One combined door struct would avoid two signature changes. Is that agreeable? If SPLICE prefers to land 6026 first, say so on SECTARMS's log and the unit will adopt 6026's shape instead.
