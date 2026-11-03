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
- 2026-10-31 — **Seam (CARVE):** `a-span-ending-on-a-faces-rim-has-no-sector-clearance` is open again: its trigger, `a-half-turn-spine-sweeps-only-off-its-exact-tangents`, closed with PR 6138. The klein loop now sweeps its exact spine with no cap tilt, so re-measure its line's end against the rim. (CARVE orchestrator)
- 2026-10-31 — **The curved lane reads a spline span through the ring composite** (PR 6136, closing `a-spline-span-against-a-curved-face-is-not-dispatched-to-the-composite`). Review tier: single FULL. The span against a cylinder, cone, sphere or torus face was correctness-bearing: the divisor's lean, the far nappe, covers.
  - **Review** (frozen at `f6058492`): APPROVE-WITH-FIXES, no MAJOR. Owed item (c), the divisor as a pointwise factor, holds on all four kinds. The reviewer's three rows are on `splice/review-6136` @ `3737cea9`, merged into the branch.
  - **Adjudicated: fixed.**
    - Three guarantees no row could red: the meter's direction, the covered touch deferred, an end on the unadmitted side.
    - The `projected.rs` divisor docs, which stated the inequality backwards: now |f| ≤ d·divisor.
    - Degenerate geometry (`CompositeRefusal::FaceDegenerate`), typed `InvalidInput`, not a kernel invariant.
    - The tightened lily pin, traced at both ε.
    - The stale parity-guard note and the vestigial binding.
  - **Class: two copies of one policy had drifted** (Q1, sure). The nappe and trim policy now has one home (`place_crossing`): below the own nappe is `Unplaced`, the gap is `NappeGap` escalated located. The spline copy had sent the gap to the frontier. The composite refusal's typing also has one home (`composite_count`, `Counted`).
  - **The lane's calls, taken:** `Counted` as an enum; `off_face`'s reach as `Reach { Bounded, Unbounded }`; the bounds-allowlist claim verified (10 → 11).
  - **Filed:** `the-cone-nappe-gap-arm-has-no-reaching-row` (P3). Its arm has no reaching fixture: the root doors and the composite refuse first.
  - **Weights** are positive by construction (`NurbsCurve3::new`), as the hull argument needs.
  - **Lily wall 8's next stop:** `vtxfac::classify_vertex_on_face` (`a-vertex-piercing-a-spline-face-reads-no-outward-normal`, P1). It is being built on `splice/vertex-on-spline-face`, with the normal read at the sweep's own certified chart point. Its second-sheet question was ruled the placement read's gap and filed separately; no partial check is built.

## 2026-10-31 — PR 6026 merges: the interior-loop guard reads a NURBS face against a plane

Review tier: a concurrent pair (letter H: a new arm of the interior-loop
guard), then one re-review. I read the final pass myself. The review-log
row is `docs/dual-review-log/rows/PR-6026.md`.
- **W1′ (a component reaching its chart's boundary clears)** is now
  checked per face. A side clears only if its channel does not close and
  it is not collapsed (`chart_side_bounds`). R1 had raised it as a MAJOR
  by inspection; it is now closed.
- **The witness's pad** is a certified bracket on the wall, inside the
  branch's own tube window. It scales with ε and is tight (1–1.25× the
  true distance).
- **The pivot's f64 shift** is charged to the band.
- **The demo-tour reds** at 1e-6 (chaintol) and 1e-12 (the stem's torus
  KernelLimit) are identical at base and head, so they are not this PR's.

Next: `w5-loop-free-stage-before-the-plane-nurbs-whole-reading`. Read
SECTARMS's `[ev]` PR 6121 first: it may make W5 the plane instance of a
general regularity sweep. SECTARMS's cylinder × cylinder unit adopts
6026's shapes.
- 2026-10-31 — **A span ending tangent on a face's rim is cleared by its sector departure** (PR 6157, closing `a-span-ending-on-a-faces-rim-has-no-sector-clearance`, released by CARVE's PR 6138). Review tier: single FULL (a departure leaning the wrong way clears a crossing).
  - **Phase 0** gave a witness at all three ε. Klein wall 3's neck seam line starts at a rim vertex. The boundary pass reads it `Hit::End`, and `isolate` then refused, because the end sits on the arc-joint crease.
  - **Built:** `Pair::sector`. An end on a chart's domain edge is read over a one-sided state box, by mean value with the partials hulled over the window: the Clarke hull, valid across a C0 crease. The face's inward departure is weighed against the line's departure.
  - **Review** (frozen at `a081c2eb`): APPROVE-WITH-FIXES, no MAJOR. The algebra holds, and so do the crease hull and the signs in all 8 orientations. A strictly one-sided derivative window would be unsound, and the code does not use one.
    - γ (the rim's lean) was unpinned. The reviewer's bending row reds under γ=0, so γ is load-bearing. Near the walk's floor scale the row accepts only a refusal, never a clear.
  - **Class, a third time today: a demo pin loose enough that the stop moves unseen.** Klein's `|dir.x| > 0.1` admitted at least 4 edges. It is now pinned by the edge's endpoints and the face's crease geometry. (The same shape as 6136's lily pin and 6152's S11.)
  - **Fixed:**
    - the shared `mag`, `midpoint` and `dot` replace the local copies;
    - the redundant arguments are dropped;
    - the foot is read once;
    - a miss continues the ladder;
    - a `|g|` prefilter runs before any `deriv_box`;
    - the docs say the reach past an end is conditional.
  - **Filed:** `an-end-on-a-rim-inside-its-carrier-reads-no-sector` (P2), for the composite, the analytic root doors and trimmed rims. `ssi/four.rs` `EndOnSides` is the same class; its evidence was added to `a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary` rather than filed twice.
  - **Klein's next stop:** edge 4v1, the flare cone seam, crosses the loop's half tube on its u=½ double knot. CARVE owes it (`carvetail/the-skin-cuts-an-arc-wall-at-each-sub-arc-joint`), and the evidence is added there.
- 2026-10-31 — **A vertex piercing a spline face reads the face's outward normal** (PR 6152, closing `a-vertex-piercing-a-spline-face-reads-no-outward-normal`). Review tier: single FULL (the face sense, creases and sheets were at stake).
  - **Fork on the second sheet, ruled by the orchestrator** (no ratified text moves). The normal is read at the chart point the sweep itself certified: `ChartMeeting.uv`, or `spline_region::read`'s In point, carried in `ContactAcc::charts`. It is never a projection foot.
    - A second in-trim foot within the band is the placement read's own gap: `spline_region::read` returns the first foot it finds. It is filed as `spliceface/the-spline-placement-read-certifies-no-second-in-trim-foot` (P3, H), and cited at the site.
    - Rejected: a partial check here, which would be a second, weaker predicate for the same fact (`[one-way.readings]`), and an embedding-beyond-band ruling nobody asked for.
  - **Review** (frozen at `f2681f2c`): APPROVE-WITH-FIXES. The door, the gate, the sense path and the lever are sound.
  - **MAJOR, coverage:** no kernel row read the sweep's carriage of the chart point. One source was read by nothing at all, and lily wall 8, the only guard, runs nightly only. Fixed: one kernel row per source, each red when its source drops the chart.
  - **Class: a demo-only guard on a per-PR-gated change is no per-PR guard.** The same shape again: demo pins loose enough that a stop moves unseen (this PR's S11, 6136's lily pin, 6157's klein pin).
  - **Fixed:**
    - The lever's refusal near a crease is now its own `PierceLeverUnread { cause }`. The patch bound's over-refusal of an integral chart with any crease is filed as `encl/the-patch-bound-refuses-an-integral-chart-with-a-crease-anywhere` (P3).
    - **Class:** `face_normal.rs` was exempt from the hand-fold lint, so every door added there went unguarded. The exemption is narrowed to its tests.
    - The door names a stale face, a non-spline carrier and a placeholder chart, rather than one `Ok(None)`.
    - The crease reading is kept.
    - There is one chart-point normal door, `geom_brep::chart_point_normal`, which `dihedral_side` now reads too.
    - There is one placement door.
    - The Approx text now says what is true.
  - **The lane's calls, taken:** S6 (newtype) and S7 (comparing two uvs) were not done; S7 would be the forbidden weaker predicate.
  - **Lily wall 8's next stop:** `Euler(PcurveMint { face 3v1, SplineChart })` at 1e-9 and 1e-6, `vtxfac::ring_vertex_in`. It is filed as `a-pierce-of-a-spline-face-cannot-mint-its-ring-vertex` (P1, M).
  - **Main reds** found on the way: six editor-core interval rows, hidden by the nightly's cancelled full suite. Filed on GREEN (PR 6156).
- 2026-10-31 — **W5: the plane × spline pose's loop-free sweep, read by the interior-loop guard** (PR 6150, closing `w5-loop-free-stage-before-the-plane-nurbs-whole-reading`). Built per `[ev]` PR 6121 as the plane instance of C5's pose classification: hull, boundary pass, regularity sweep. Review tier: single FULL, then a re-review (a bound leaning the wrong way would clear a real loop).
  - **The lane corrected the brief.** "Every cell excluded or regular ⇒ loop-free" is unsound, because an excluded cell can hold the very extremum a loop encloses. The sweep therefore also requires every non-regular cell to be joined to the rectangle's boundary through cells excluded on the same side. It declines on any interior crease.
  - **First review** (frozen at `5e0132e8`): NOT-MERGEABLE-AS-IS. Two MAJORs, demonstrated, with one root: the guard read the sweep before C5's stage-1 coincidence call and stage-2 boundary pass.
    - A wall within the band everywhere cleared.
    - A boundary graze cleared.
    - Main refuses both as tangent.
  - **Class: a stage built in front of the ratified stages skips their decisions.** Any later stage-3 build, such as SECTARMS's NURBS × implicit arm, runs the hull and the pass first.
  - **Fixed, in the ratified order:**
    - the hull decides apart, coincident (a D10 record) or in band;
    - a door refuses a collapsed side not decided clear over its whole row;
    - C3's own pass runs once, before the sweep, and is carried to the whole reading;
    - the sweep runs on the one shared cell recursion (`SweepDuty::Regular`), its decisions named through `k_stats`, with a union-find join.
  - **Under the ratified order:** exact grazes refuse by name, and at 1e-6 sides lying in the plane read as coincident. That is the ratified order working, not a regression.
  - **Re-review** (frozen at `7eed17a4`): APPROVE-WITH-FIXES, no MAJOR.
    - Regularity holds under the levered slope: inflating the lever 10⁶× moves no loop row.
    - The sweep alone would read a long thin in-band wall as loop-free; the guarantee lives in the hull and the pass, and an asserting row pins it.
  - **The last pass:**
    - the door decides clearance over every row point, through one collapsed-side reading;
    - `Reached` cannot be empty;
    - `CoincidentSurfaces` maps in one place;
    - poison refuses at the hull whether or not a lane is handed in;
    - probes are moved off the hull thresholds;
    - `Want::Far` is pinned per ε.
  - **Accepted, not done:** the hull's heights stay in the scalar's own arithmetic, certified at `Interval`, as every analytic pose margin is. Requiring certified bounds would ripple through the generic boolean, to `Dual`, which has none.
  - **Filed:** `the-plane-spline-sweep-declines-a-loop-free-dome-beside-a-saddle` (P3, completeness). Krawczyk and Hessian isolation at `Declined::Floor` is SECTARMS's completion.
  - **Found on the way:**
    - `holds_the_ssi_lane` compared neither of two fn-pointer fields, a gap latent on main and hidden by a type alias; fixed.
    - A first review session stalled at clone, because a short SHA as `source_revision` fails with `ref_not_found` and surfaces no error. Every dispatch now passes the full SHA.
  - **Timings:** the classification costs 8–90 ms a pair. The whole readings it skips run 0.2 s for lily's caps, and for the blade cuts 1.7–8 s at 1e-9, 0.3–1.8 s at 1e-6 and 7–36 s at 1e-12.
- 2026-10-31 — **A pierce of a spline face mints its ring vertex** (PR 6166, closing `a-pierce-of-a-spline-face-cannot-mint-its-ring-vertex`; lily wall 8's next stop). Review tier: single FULL.
  - **Phase 0** mapped four answers:
    - (a) a chord-free primitive: an eleventh operator;
    - (b) deferring the face's rows: the face goes rowless mid-pipeline, so the trim reads go blind;
    - (d) the fitted lane: the chord is off the sheet, so it cannot certify;
    - (a′) a door that runs `mev` then `kemr` with the chord's rows never minted.
  - **Orchestrator's constraint on (a′):** `[topology.operators]` is a closed set of ten, so the door must be a composition of the existing `mev` and `kemr` (preconditions, deltas in order, minting order), with only the row plan done once (C4). A door with its own mutation would need Ev. It is documented "not an Euler operator", as `ring_move` is.
  - **Review** (frozen at `bc8e8732`): APPROVE-WITH-FIXES.
    - The door IS a composition: the Body Debug is byte-equal to `mev` then `kemr`, slot versions and free heads included.
    - MAJOR: nothing pinned it. A mutant mutating on its own passed every PR row and the net-delta postcondition, and its returned keys were equal. The reviewer's whole-arena equivalence rows now pin it.
  - **Class: a "composition of existing operators" claim needs a whole-arena equivalence row; a comparison of keys misses it.**
  - **Fixed:**
    - one `mev_fan_gate` shared by the fan `mev` and the door, where there had been a hand copy;
    - per-step delta asserts, replacing a hand-written net literal;
    - the door is listed in euler.rs's module header;
    - lily wall 8's pin names both faces by identity (the class from 6136, 6152 and 6157 again).
  - **Unified:** spline faces had gone through the door while analytic faces stayed on the chord, because `chord_join::join_lone_ring`'s keys-only `kfmrh` relied on the chord's site mint clearing an analytic face. That door now takes `kfmrh_minting`, and every chart goes through `ring_vertex`, so a curved analytic face keeps its rows as found. The P3 filed for it is closed here.
  - **Lily wall 8's next stop:** at 1e-9 and 1e-6, `GermSectionSpiric`, plane × torus (the stem's torus wall against the leaf's lower cap), at `join::frame_refusal`. Covered by `sectsite/c5-plane-torus-cone-cylinder-arms`, with the evidence added there.
  - **Filed:**
    - `sectarms/a-nurbs-wall-against-a-plane-has-no-germ-frame` (P2), the lens-wall kernel row's stop;
    - `cleave/a-fate-cut-into-a-complete-spline-face-refuses-spline-chart` (P2).
- 2026-11-01 — **The interior-loop guard reads two spline faces through the pair lane's whole reading** (PR 6171, closing `no-certificate-excludes-a-closed-loop-interior-to-two-spline-faces`, with SSIEDGE's `the-nurbs-pair-boundary-pass-reports-no-side-or-corner-region` claimed and closed). The design is `[ev]` PR 5990's (fork row 217). Review tier: DUAL, letter H, concurrent pair (row `PR-6171`).
  - **Phase 0:** the pair lane's Side and Corner hand-in was ratified in C3 but never built, so this unit built it on SSI ground. The topo side recovers shared rows through `side_row_certifies`, factored out of `IntersectionDraft::shared_row` so that the guard and the seam read one test, and shared corners through a named `decide_magnitude`.
  - **Dispatch slip:** the first dual dispatch, at 01:15Z, was withdrawn about a minute in, because the lane had read my 19:22Z phase-0 additions only about 6 h late, and a push was pending. **Class (friction):** a lane's inbox can lag behind a long build by hours. Before freezing a head for review, have the lane confirm it has read every message since its last report.
  - **The pair** (frozen at `6ea83b58`): both APPROVE-WITH-FIXES, no MAJOR. Every class-a attack refused rather than cleared, at all three ε.
    - Bilateral headline: no row guarded the cover's zero-free proof. Each reviewer had a surviving mutant.
    - Unilateral (R1): `four::covered` was unbounded in total work (a stall of over 20 min under a mutant), and the corner prose overclaimed.
    - Tally 0.
  - **The corner and side prose went to Ev:** `[ev]` PR 6188. Ev asked whether the weaker claim is a problem. The answer: not for soundness, since the guard needs only "no loop hides in the cell or cover", and a gap costs a refusal at worst; the text matters to builders who would rely on it. Ev approved ("the re-wording sounds good!"), merged `a3b01e33`.
  - **Fix pass** (`99d18281`): a claim-1 row red under both mutants; `covered` given a total budget; one refusal classification with incapacity named apart from tangency; one row-piece reader; one `KnownPair`; known rows taken at construction; the guard refuses a held spline pair; an NN pole row.
  - **Main reds found on the way**, all already filed:
    - `quad/lofted-concave-prism-pad-pin-moved-at-1e-6`;
    - SPLICESEAM's 1e-12 lofted-cylinder pin, which now passes (evidence added there);
    - a 1e-12 fuzz seed of `ssi::certify` torus limb 2, reproduced on main and added to `ssiarith/a-torus-limb-twos-sampled-residual-overshoots-its-span-bound-by-rounding`.
  - **Filed:** `the-interior-loop-guard-has-no-spline-by-curved-analytic-arm`.
- 2026-11-01 — **Sitting handed back: SPLICE's P0 and P1 rows are done** (program `ready`). Merged this sitting:
  - 6026: the plane × spline guard arm;
  - 6136: a spline span read through the ring composite;
  - 6150: W5, the plane × spline pose's regularity sweep in C5's ratified stage order;
  - 6152: a vertex on a spline face reads its normal;
  - 6157: the rim sector clearance;
  - 6166: the ring-vertex mint as a composition of `mev` and `kemr`, every chart through it;
  - 6171: the NURBS × NURBS guard arm, with the pair lane's Side and Corner hand-in;
  - `[ev]` 6188: C3 Side and Corner wording, approved by Ev;
  - tracker PRs 6146, 6147, 6156 and 6167.
  - **Where the demos stop now:**
    - lily wall 8: a plane × torus spiric germ (`sectsite/c5-plane-torus-cone-cylinder-arms`);
    - klein wall 3: the flare cone seam on the loop tube's u=½ double knot (`carvetail/the-skin-cuts-an-arc-wall-at-each-sub-arc-joint`).
    - Neither is SPLICE ground.
  - **The tail for a successor:**
    - P2: `the-interior-loop-guard-has-no-spline-by-curved-analytic-arm` (H; this pairs with SECTARMS's NURBS × implicit build, so coordinate there first), `an-end-on-a-rim-inside-its-carrier-reads-no-sector`, `doors-carrying-a-shared-row-have-no-row-at-rest`.
    - P3: the rest.
  - **Classes logged this sitting** (in the entries above):
    - a stage built in front of ratified stages skips their decisions;
    - "composition of existing operators" needs a whole-arena equivalence row;
    - a demo-only guard on a per-PR-gated change is no per-PR guard;
    - demo pins loose enough that a stop moves unseen (four times);
    - a loop-free test must account for the extremum's cell.
  - **Friction, the memory candidates for Ev:**
    - a short SHA as a session's `source_revision` fails `ref_not_found` silently;
    - a lane's inbox can lag a long build by hours, so confirm a lane has read every message before freezing for review.
- 2026-11-02 — (tracker sweep) **SPLICECARRIER, SPLICEFACE and SPLICESEAM merged back into SPLICE.** The track budget now counts only the dispatchable P0, P1 and unbanded rows (Ev, in chat, 2026-11-02), so the four tracks of the 2026-10-28 split measured 1, 11, 6 and 7 points, 25 against 30 together. They are one thread with four witnesses: a spline-edged operand entering every boolean. Their 30 rows (5, 11 and 14) moved here by `git mv`, ids and bodies unchanged, each with a dated Moved note. `program.md` keeps SPLICE's prefix and tag, adds the three tracks' paths and charters as threads, and the plan's slate is restated by thread. Their `program.md`, `plan.md` and `log.md` are deleted (`docs/doc-ledger/splice-successors-merge-back.md`); their logs held only the opening entries. (tracker sweep)
- 2026-11-03 — **Sitting opened** (program `active`, `splice/orchestrator`). All ten P0 and P1 rows dispatched at once, as six lanes, each its own cloud session from main `d1ac6881d4` (this container has 4 vCPU and a one-wide build mutex). Review tiers, called at dispatch:
  - `splice/far-union-volume` takes `main-red-the-lofted-cylinders-far-union-builds-the-wrong-volume-at-1e-6`. Letter M. Single FULL: a wrong answer whose cause is unknown, so believing the fix takes more than reading it.
  - `splice/spline-join` takes `joining-a-spline-carrier-is-unbuilt`. Letter H. DUAL, concurrent pair: it builds a spline carrier's run-on, the mechanism the carrier thread's other rows read. The lane reports a phase 0 (the producer that reaches the door, and the run-on's shape) before it builds.
  - `splice/split-parameter` takes `a-meeting-on-a-spline-boundary-edge-has-no-split-parameter`. Letter M. Single FULL: the parameter is carried, not recomputed, but the split sits under every boundary meeting.
  - `splice/held-pierce` takes `a-spline-wall-cannot-say-whether-it-holds-a-pierce-point`. Letter M. Single FULL. Phase 0 chooses between `pcurves::split_cache` and `pcurves::pcurve_of` for a split edge's rows mid-boolean, and reports before building.
  - `splice/crease-window-bound` takes `a-creased-spline-gets-no-second-order-clearing`. Letter M. DUAL, drawn: byte 17, 17 mod 3 = 2, so SEQUENTIAL. A bound that leans the wrong way clears a real touch.
  - `splice/kind-reads` takes the five E rows as one bundle: `a-spline-rider-reads-as-lying-in-no-plane-face`, `section-area-skips-a-spiric-or-nurbs-section-edge`, `split-section-boundary-curved-arm-untested-past-the-edge-gate`, `the-curved-arm-reads-a-spline-faces-side-through-the-dihedral-side-door` and `main-red-certified-endpoint-census-drifted-in-geom-brep-after-6171`. Single FULL: the curved arm's side door is new incidence in the sweep, so the bundle is not style-only.
  - **Coupling inside the slate:** `split-parameter` and `kind-reads` both edit `topo::boolean::reduce` (the point split and `curved_face_arm`: different functions, independent). `spline-join` and `split-parameter` both stand where the carrier has no parameter inverse; the split carries its meeting's parameter and asks the carrier for none, so neither waits on the other. Each brief names its siblings.
  - **Coupling outside it:** open PRs on shared ground are SECTARMS 6412 and 6405 (NURBS × implicit, `geom-brep`'s section arms) and ONEHOME 6371 (the finished-body constructor at every door). Lanes merge main whenever it moves.
  - **Lane sessions** (briefs on the never-merged `splice/briefs` branch, `briefs-1103/`, at `775995c2`): `far-union-volume` `session_013QoYWVNJT5aTjLGexvXdTF`; `spline-join` `session_013QoYWUjsTYxdx9tQYTyz52`; `split-parameter` `session_013QoYWWbRWy29ip68UMPcVp`; `held-pierce` `session_013QoYWYaHccnvdG5CNNzB8q`; `crease-window-bound` `session_013QoYWV4tTdSw7rPbkyV7rT`; `kind-reads` `session_013QoYWYBygkbMBizHt4Akzq`. The orchestrator is `session_013QoYWWB1aYPFTSw3rVyfKM`.
- 2026-11-03 — **Phase 0 rulings** (07:25Z check-in):
  - `crease-window-bound`: **parked, lane closed**, no review (so no dual-review row; the drawn byte 17 is void). The lane stopped on its own: DESIGN [topology] (`[ev]` PR 5913) leaves no carrier creased at rest, and TESSSIZE's `the-crease-arms-retire-once-no-carrier-carries-a-c0-knot` says to delete crease arms, not generalise them. The row is parked on it, with the lane's measurement. So are this slate's other two crease rows, `a-meeting-on-a-g1-arc-joint-of-a-spline-face-refuses-as-a-crease` (P2) and `a-seam-on-a-creased-spline-chart-reads-no-arm` (P3), which that row names. **Class (dispatch):** I dispatched a row a ratified ruling had already retired; the row itself carried no pointer to the ruling. A dispatch reads `git log -S` on the row's own mechanism in DESIGN.md first.
  - `kind-reads`: row 4 (`the-curved-arm-reads-a-spline-faces-side-through-the-dihedral-side-door`) is overtaken. `curved_face_arm` routes a spline face to `reduce::spline_face`, which reads no parent face's side, and the sheath now stops at the rim certificate (`NotTransverse`). It closes in the lane's PR with that measurement. `dihedral_side` stays, as the door `the-shared-rim-routing-exempts-a-spline-face-by-kind` would read. Rows 1, 2, 3 and 5 build as planned, reusing existing refusal variants where one means the same. The census drift has grown from four files to sixteen.
  - `held-pierce`: the premise was wrong. The rowless half-edges are not split pieces (`split_edge` already restricts rows); they are the sweep's zero-length `MevNull` placeholders, which hold no row by the `pcurves` rule. Ruled: the spline trim skips a placeholder as the analytic reader does, through ONE lift with a scaffolding policy and one placeholder predicate, not a second lift. Past it, the wavy loft stops at `GermFrameUnsupported { Nurbs, Plane }`, which is SECTARMS' `a-nurbs-wall-against-a-plane-has-no-germ-frame` (P2): the reproducer goes there, and the owed end-to-end row travels with it. The `placed: true` half gets a bounded attempt at a reaching input, or a filed row.
  - `spline-join`: no public boolean reaches the door on main. The public producer is `Body::split_edge` then `Body::join_edges` (`join_door_is_whole`). Approved design: the joined edge keeps the kept edge's carrier, and the far parameter is carried from the killed edge when the two carriers are content-equal. There is no re-knotting, no concatenation and no NURBS inverse; a crease at the joint cannot arise (PR 5913). Ruled that the carry is the first rung for EVERY carrier kind, not a NURBS arm, with `param_near` and the period as the fallback for distinct carriers of one locus. A different-carrier spline join stays a typed refusal.
- 2026-11-03 09:12Z — **Two reviews dispatched**, each a single FULL review with its claims on `splice/briefs` (`briefs-1103/review-<pr>.md`):
  - PR 6429 (`far-union-volume`), frozen at `163d03d9dc`, reviewer `session_013QoYWX7M6dgenfwQjjNigK`. The lane's finding is that the far union is built right and the test read it wrong. The 3.24e-5 excess is the lofted operand's own quadrature excess, on its two NURBS walls, inside the walls' certified bracket (±4.16e-3 each at 1e-6). The fix is test-only: each volume band adds both readings' brackets, and both pins come out. The 1e-12 pin of `the-lofted-cylinder-operand-runs-out-of-quadrature-budget-at-1e-12` was already stale on main. The review's main claim to falsify is whether the widened band can still go red on a wrong build, and on a ballooning bracket.
  - PR 6437 (`kind-reads`), frozen at `9816e38269`, reviewer `session_013QoYWX1QfaL1rw1xRtJNsJ`. Rows 1–3 and 5 are built, reusing `EdgeInFaceGap::Curved` and `RunEdgeCarrierUnsupported` (docs widened). The one new variant is `SplitFinishError::Undescribed`. The split now reads `classify_seam`, from one home in `topo::pcurves`. Row 4 is closed as overtaken. The census drift was 16 files, attributed to CLEAR, SECTARMS, ONEHOME, DOORS, TOUCHREC and SPLICE PRs. Filed: `charttail/cell-region-clear-of-singular-reads-a-spline-chart-as-having-no-singular-point`.
  - **Main reds the kind-reads lane met** in topo's slow set (reproduced on main) are both already filed: `boxes/main-red-the-box-door-inventory-misses-three-edge-cover-readers` and `flush/main-red-a-needle-cavity-12eps-channel-escalates-at-bool-contact-vertex`.
- 2026-11-03 09:40Z — **PR 6437 review** (`session_013QoYWX1QfaL1rw1xRtJNsJ`, frozen at `9816e382`): NOT-MERGEABLE-AS-IS, with two MAJORs, both demonstrated red at all three ε (evidence `splice/review-6437` @ `438b150b`). Rows 2–5 hold.
  - **M2, the root:** `slit`'s new arm read a spline carrier "not on" a surface from one sample off it. That refutes only "the whole carrier lies on it". `holding` asks whether the carrier lies in the face along SOME piece. A degree-1 net lying in x = 0 over half its span read off, and `sectors::closed_reach` turned the refusal it used to give into an `Off` answer.
  - **M1:** `holding` returned "in no face" before any refusal when the edge's ends were not named, even with an edge-on-face record naming the pair.
  - **Fix pass sent:** a certified-off reading over the whole span (an interval residual on the pieces, or the control hull outside the band for a plane), or else unread and refused under a distinct variant; a result type saying on / certified off / unread, in place of a three-meaning `Ok(None)`; style drive-bys; a third sweep pass shaped at wrong-answer arms. The same reviewer reads the fix pass.
  - **Class (logged at adjudication):** a sampled refutation answers a question about the WHOLE of a curve, never about a PART of it. A line or conic cannot lie on a surface along part of its length; a spline can. So every "does it lie in / touch along" reader that was sound for analytic carriers by sampling is unsound for a spline. Sweep: the lane's third pass.
- 2026-11-03 10:07Z — **PR 6429 review** (`session_013QoYWX7M6dgenfwQjjNigK`, frozen at `163d03d9`): APPROVE-WITH-FIXES, no MAJOR (evidence `splice/review-6429` @ `296d5a57`).
  - **The build is confirmed right by execution.** Every operand face reads bit-identical in the union. The fixture's wall is an exact rational cylinder. The midpoint's excess falls as the resolution tightens (3.2e-5 at 1024·ε, 2.5e-8 at 1·ε), so it comes from the reading.
  - **MINOR:** widening the band by the pad lets a cap displaced 500ε through at 1e-6 (mutants M3 and M5 survive there; every ε row kills them at 1e-9). The fix pass tightens the reading to a stated resolution, asserts the pad against the band, and gives "within band plus pad" one test_utils helper (it had three spellings).
  - **Fix pass:** the orchestrator reads it. No re-review.
  - **Class (logged at adjudication):** a red volume row fixed by widening its band by its bracket gets easier as the bracket degrades. The fix is a stated resolution with the pad asserted, never band + pad alone.
- 2026-11-03 10:36Z — **PR 6441 (`split-parameter`) review dispatched**: single FULL, frozen at `309625f4`, reviewer `session_013QoYWXwyZpYo6QXxGy5BgS`. The PR carries the boundary pass's own parameter (`Boundary::Meets.on_edge`) through the pierce to the point split, on any carrier. Teapot wall 3 now passes the split. It stops at `HeldPierceUnbuilt { pierced_kind: Sphere }` at the default ε and 1e-12, which is the `held-pierce` lane's site, and the lane has been told; and at `SplineCrossingUnsettled { why: Floor }` at 1e-6, filed as `teapot-wall-3-at-1e-6-floors-the-meridian-against-a-spout-wall` (P2). Also filed: `a-point-the-pre-pass-places-on-a-spline-boundary-edge-has-no-split-parameter` (P2) and `issues/a-conic-arcs-span-reach-passes-its-whole-turn-lever` (P3, a fuzzer seed red on main). A `[render]` run is owed after review, because the montage caption's refusal text moved.
- 2026-11-03 11:05Z — **PR 6435 (`held-pierce`) review dispatched**: single FULL, frozen at `37e8fec5`, reviewer `session_013QoYWXGdT2dSKo5kikmzqX`. One loop lift with a `Scaffolding` policy, and one `topo::null::is_placeholder`, read by the analytic and spline region readers alike; the spline trim is the one `Skipped` caller.
  - **Two stops move.** The wavy loft ∪ box and the saddle cut by its tangent plane both now stop at `GermFrameUnsupported { Nurbs, Plane }` (SECTARMS).
  - **Teapot wall 3**, on a scratch merge with PR 6441, moves to `GermFrameUnsupported { Sphere, Nurbs }`, SECTARMS' P0 `a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm`. Whichever of 6435 and 6441 lands second re-pins `teapot::wall3_pinned`.
  - Filed: `a-spline-wall-holding-a-pierce-point-has-no-operand-that-reaches-it` (P2).
- 2026-11-03 11:26Z — **PR 6429 merged** (`2e1d9ce7`). **PR 6441 review** (`session_013QoYWXwyZpYo6QXxGy5BgS`, frozen at `309625f4`): APPROVE-WITH-FIXES, four MINORs and no MAJOR (evidence `splice/review-6441` @ `1c17acdf`).
  - **The claim that no analytic edge reaches the held branch was false.** The lane's probe covered topo, and the analytic rows live in sweep. The reviewer logged 222 hits there, all inside certification, 18 of them moving a child interval by 1 ulp. The vertex never moved.
  - `held_on`'s same-edge guard survived a mutant, so the fix pass makes it structural: a held parameter carries the key of the edge it was read on.
  - The kernel row could not tell the two parameters apart, so it gets distinct ones.
  - No frame moves, so no `[render]` is owed.
  - **Fix pass:** the orchestrator reads it.
  - **Class (logged at adjudication):** a "no existing input reaches the new branch" measurement must run where the inputs live. Here they were in another crate's tests, which the module doc cited without naming the crate.
- 2026-11-03 11:53Z — **PR 6435 review** (`session_013QoYWXGdT2dSKo5kikmzqX`, frozen at `37e8fec5`): APPROVE-WITH-FIXES, two MINORs (evidence `splice/review-6435` @ `1adb6f28`). The change is correct under every attack, including a placeholder at each joint across a closed chart's seam. The two halves of `is_placeholder` ("the edge is null", "its ends are one point") each survived a mutant, so rows are owed for both.
  - **Ruled (API):** the public `Body::loop_lift` stays strict, because finished bodies hold no null edges and no public caller would ever vary the policy. The `Skipped` lift gets a crate-private door.
  - The skip's exactness rests on the mint's identity joint elements (`euler.rs`), so the docs cite that.
  - **Fix pass:** the orchestrator reads it.
- 2026-11-03 — **PR 6429 merges** (`far-union-volume`), closing `main-red-the-lofted-cylinders-far-union-builds-the-wrong-volume-at-1e-6` and `the-lofted-cylinder-operand-runs-out-of-quadrature-budget-at-1e-12`. The far union was built right. Its test read a quadrature midpoint against a band three orders tighter than the bracket.
  - The first pass widened the band by the bracket. The review let a 500ε cap displacement through that, so the fix pass measures at a stated resolution (1e-6 m) and asserts each bracket within the band. Every survivor dies at every ε; the floor is a 3e-6 m displacement.
  - The orchestrator read the fix pass; there was no re-review. The test-only diff gives `topo::test_support` a `volume_at` and an `assert_volume_near`.
  - The 1e-12 row went from 159 s to 29 s.
  - Filed: `tint/volume-rows-hold-a-midpoint-within-a-band-plus-an-unbounded-pad` (11 sites), plus evidence on `issues/a-tilted-plane-cut-through-a-tube-measures-1e-7-off-the-closed-form`.
  - Main reds the lane met, all already filed: BOXES' inventory, FLUSH's needle cavity, and SPLITDOOR's `the-cone-fuzz-draws-a-cone-bounding-no-one-material-at-a-seed`.
- 2026-11-03 — **PR 6441 merges** (`split-parameter`), closing `a-meeting-on-a-spline-boundary-edge-has-no-split-parameter`. The orchestrator read the fix pass; there was no re-review.
  - A held parameter is now a `carrier_cross::Held` minted only by the boundary read, answering only for its own edge; a mismatch at the split refuses `ClassificationInvariant`.
  - The kernel row tells the line's parameter from the arc's.
  - The PR body states the analytic 1-ulp movement.
  - `sheets_part`'s held arm is reached 63 times on analytic rims, and a spline row for it is filed (P3).
  - `teapot::wall3_pinned` derives its corners from the authored stations.
  - No frame moves.
  - Teapot wall 3 passes the split. The `held-pierce` lane re-pins it on its next stop (6441 landed first).
- 2026-11-03 12:14Z — **PR 6434 (`spline-join`) dual review dispatched**: letter H, so a CONCURRENT pair on the frozen head `fd976ed601588ad07e8ea84826f57b32befe04d7`, with an identical brief (`splice/briefs` `briefs-1103/review-6434.md` and `review-common.md`). R1 is `session_013QoYWUqVJ5KZ3vgavTAENm` (evidence `splice/review-6434-r1`); R2 is `session_013QoYWWUaG2SNpMwyoPGr64` (evidence `splice/review-6434-r2`). The protocol is `18e21edce5`. Neither reviewer received a relaxation.
  - **The PR:** a uniform "carried" rung in `edge_join::joined_spec`. A killed edge on a content-equal carrier, at an equal parameter at the vertex, gives the joined far parameter, with no inverse and no period. Period and `param_near` stay as the fallback.
  - **What moved:** a split pair's joined interval is now the parent's exactly. On a spiric that is 1 ulp from `param_near`.
  - **Filed:** `a-spline-join-beside-another-carrier-has-no-far-parameter` (P2, no producer).
  - **Claims the pair carries:** orientation and overlap of the carried interval; the parked tier-3 crease check (`measuredoor/tier-3-…`), which means a creased carrier can still exist at rest today.
- 2026-11-03 13:15Z — **PR 6434 dual review** (frozen at `fd976ed6`; R1 `session_013QoYWUqVJ5KZ3vgavTAENm`, evidence `splice/review-6434-r1` @ `e0e3a9b5`; R2 `session_013QoYWWUaG2SNpMwyoPGr64`, evidence `splice/review-6434-r2` @ `f75cf19e`). Both APPROVE-WITH-FIXES, no MAJOR, so the tally is 0. Neither found a wrong carry, a wrong arc or a weakened certificate. The blinding byte for the coding is 62.
  - **Correspondence pre-note.** Bilateral:
    - the content check is only partly pinned (R1's mutant drops a circle's frame, R2's a net's knots; both survive every row), MINOR for both;
    - `same_carrier` is a hand census hosted in topo, and belongs beside `Curve3` (style);
    - `carried_far`'s doc argues at length (style);
    - non-f64 scalars never carry (NOTE).
  - **Unilateral R1:**
    - the rung's Circle and Ellipse wiring is unpinned, and the "what moved" note was incomplete: on main a split pair joined back off the parent's interval on circles (15/63 fractions) and ellipses (3/63). MINOR, demonstrated by a grid row;
    - the PR's closed fixture carries interior C⁰ knots, against PR 5913 (NOTE, demonstrated);
    - the `join.rs` lead was unscheduled (NOTE).
  - **Unilateral R2:** the rung order is written in three proses; degree is compared on its own track; a closed spline's seam vertex is unreached (style and NOTE).
  - **Tally candidates:** none.
  - **Fix pass:** the union. Carrier content equality moves to `geom`, destructuring every field, with one twin per field. Both reviewers' rows are adopted. A C¹ closed fixture replaces the creased one. The `join.rs` lead gets a row. The orchestrator reads the fix pass.
  - **Class (logged at adjudication):** a content comparison written as a hand list of fields silently ignores a new field, and twins that vary one field per kind cannot catch the omission. The fix is a comparison that destructures every field, homed beside the type.
- 2026-11-03 13:19Z — **A new P0 arrived from main**: `a-spline-face-has-no-subtract-or-intersect-lane` (H). It was filed by the curved-demo-joins bookkeeping session (`work/issues/curved-demo-joins-build.md`), together with a P0 re-band of the split-parameter row as teapot wall 3's blocker. ∖ and ∩ refuse any spline face at the revert roster (`reduce::revert_arm_exists` has no `Nurbs`), and klein wall 4 stops there.
  - **Dispatched** as `splice/nurbs-revert-roster` (`session_013QoYWUJw29zhMKEVRtVAc9`) from main `c89d725d`, with a required phase 0: why the roster exists, a ∖/∩ census over the union's spline fixtures with `Nurbs` admitted, and a staging proposal. Letter H, so DUAL, concurrent pair.
  - **PR 6441's state-sync head conflicted with main** on that re-band, and so got no CI run, silently (the known CONFLICTING-no-run shape). Resolved by keeping both sections. The demo-joins row's lint warning was its cue to re-measure: teapot wall 3's stops are now `HeldPierceUnbuilt` (default, 1e-12) and the 1e-6 bore-wall Floor, which is re-banded P0 as a blocker of that row.
  - Before that, the gate went red on time alone: `viewer sketch_on_face::a_boss_sketched_on_a_side_lands` ran 1.032 s against the fast set's 1.0 s. Filed on TCOST, `a-boss-sketched-on-a-side-runs-at-the-fast-sets-one-second-bound` (P4). The integration cannot re-run jobs (403).
- 2026-11-03 13:50Z — **Friction (logged): the orchestrator's own tracker PR conflicted a unit in its merge window.** PR 6441 was green and ready. Merging tracker PR 6450, which appends to `work/splice/log.md`, made it conflicting on GitHub, whose merge ignores the union driver, and so cost a third full CI run (about 18 min). A unit's state-sync, and its log line, go on only at the merge, and the orchestrator holds its own log-touching tracker PRs while any unit is in its merge window. (PR 6441 also met a 1.03 s timing red and a conflict with another session's re-band today.)
- 2026-11-03 14:10Z — **PR 6441 merged** (`b0164da8`) after its fourth CI run. Teapot wall 3 passes its seam split. The `held-pierce` lane re-pins it on its next stop.
- 2026-11-03 14:38Z — **PR 6437 fix pass in** (head `21dd9210`, CI green). `Placed::{On, Off, Unread}` replaces `Option<bool>`. A spline reads `Off` only by its control hull, and is otherwise unread, refused as a new `EdgeInFaceGap::CarrierUnread`. M1 is fixed, and `closed_reach`'s refusal is restored. The lane kept the ends-plus-midpoint check, which is in the M2 class, because dropping it refused two builds that ship today. The whole-unit re-review, by the same reviewer (`session_013QoYWX1QfaL1rw1xRtJNsJ`) on `21dd9210`, judges that point first.
  - Also found: `undescribed_ending` ends a user-geometry crease with KERNEL_DEFECT_ENDING. The evidence went onto `topo/merge-faces-undescribed-ending-is-a-typed-kernel-defect`, whose planned `unreachable!` repair would turn the refusal into a panic.
  - Filed: `contacthold/a-tangent-locus-run-between-schedule-stations-is-read-through-no-face`.
  - The row-3 trigger for a spline edge is `parting/the-plane-split-reads-no-crossing-of-a-spline-edge`. No open row retires the split's refusal of a spline face.
- 2026-11-03 14:50Z — **PR 6437 re-review** (`21dd9210`): NOT-MERGEABLE-AS-IS, one MAJOR, demonstrated at all three ε (evidence `splice/review-6437` @ `42d76461`). The kept ends-plus-midpoint check reads a straight spline keel that crosses a U face's notch as "in no face", and it ships with no refusal. It ignores the crossing records that the line path cuts at. M1 and M2 are closed.
  - **Ruled:** an unread carrier refuses wherever any record relates it to the face. The only exemption is a structural one (an end shared as a vertex of one body), shown sound and pinned. Without such an exemption, the two builds the check kept alive refuse, are re-pinned, and get a filed P1. The same reviewer reads the delta.
  - **Class (again):** a sampled read kept to preserve a build is the M2 class under another name. "It broke two builds" is output stability, not soundness.
- 2026-11-03 17:12Z — **Revert-roster phase 0 ruled.** With `Nurbs` admitted on a scratch tree, 40 cases × 6 runs at three ε built nothing, wrong or right, and every ∖/∩ refusal matched ∪'s exactly. Klein wall 4 moves to wall 3's crease stop. A read-only audit found that every spline sign path folds `Face::sense`.
  - **Ruled:** admit every partner kind at once. Delete the second list (`revert_arm_exists`, `PairRefusalSite::RevertRoster`), re-wording any ratified clause that names it as a consequence and showing it. Owed:
    - one kernel row per sign-reading spline path on a reverted face (`sense = false` reaches them for the first time under ∖);
    - the probe as a ∖/∩ census guard.
  - **Class (logged):** a second hand list of one fact. The revert roster was the union's roster less one kind, for a reason the code had outgrown.
- 2026-11-03 — **Seam (ONEDOOR): `[topo.homes]` is now held per PR** by `scripts/gates/engine-leaves.sh`, merged in PR 6458. Ev ratified the rule on PR 6426; it is in `crates/topo/README.md`.
  - **What the gate refuses** in `crates/topo/src`, outside an engine directory:
    - naming an item declared under `boolean/` or `splitting/`, counted by name however it is reached;
    - one engine naming the other;
    - a crate-visible item inside an engine;
    - any construct binding a new name to an engine item (an `as` rename, a `type` alias, `use crate as`, `#[macro_use]`);
  - Each is counted over a shrink-only allowlist, `scripts/gates/engine-leaves.counts`.
  - **What it means for your open branches:** a unit that adds such a site reds on its PR, with "Move the code; do not raise the count". The signature test says where the code goes: no `Body` → geom-brep; a `Body` reading or construction naming no engine type → the crate root; names an engine type → that engine.
  - A move that clears sites lowers the line in the same PR. `scripts/gates/engine-leaves.sh --lines FILE` lists a file's counted sites.
  - (ONEDOOR orchestrator)
- 2026-11-03 — Note from FRAME: the python row is red on main at `TestTeapot.test_both_joins_refuse_through_the_document`, which still expects `point_split_carrier_unsupported` after PR 6441. Filed on your slate as `the-teapot-north-star-row-still-expects-the-point-split-refusal`. FRAME's PR 6465 merges over it as an inherited red. (FRAME orchestrator)
- 2026-11-03 — **PR 6434 merges** (`spline-join`), closing `joining-a-spline-carrier-is-unbuilt` (P0). Dual-review row `PR-6434` (concurrent pair, tally 0).
  - **Fix pass:** carrier content equality is `geom::Curve3::same_content`, which destructures every field. Both reviews' rows are adopted. The closed fixture is now C¹. The `join.rs` lead is filed as `a-germ-site-is-footed-on-its-section-again`.
  - **Slow sets** on the fix head: sweep 3537/3537 (98 min), geom 551/551, topo 3902 plus the two known main reds.
  - **Correction:** the 12:14Z dispatch entry names the protocol `18e21edce5`. That hash came from this shallow checkout's graft. The protocol in force is `24b76fff0e`, which the row records; its text did not change in between.
- 2026-11-03 18:50Z — **Two rows arrived on SPLICE's slate from other programs** (main also retired `work/issues/` and moved `curved-demo-joins-build` onto SPLICE):
  - `the-teapot-north-star-row-still-expects-the-point-split-refusal` (P1, E), filed by FRAME. pncad-py's teapot north-star row went red on main because of PR 6441: the python row runs only when its subject is touched. It rides PR 6435, which moves teapot again, and the `held-pierce` lane re-pins it. **Class (again):** a stop that moves needs every pin of it re-read, including the python suite's; the tour's `wall3_pinned` was not the only one.
  - `a-rim-arc-lying-on-a-cylinder-refuses-as-a-curved-pierce`, filed by FRAME's `turned_onto` lane. It is klein wall 3's new stop, where the loop's cap-rim arc lies on the bulb's neck cylinder (the REST mate's rim). Banded P0 as a blocker of the P0 demo-joins row, cost M. Dispatched as `splice/rim-arc-on-cylinder` (`session_013QoYWU9Aeu63rM1pUrPe3f`) from main `27205d77`, with a required phase 0. DESIGN [coincidence] already rules that a coincidence is a margined verdict and a mate makes it structural, so the open part is the reading, not the semantics, and no designers are needed. Single FULL review. The demo-joins row still names klein wall 3's old crease stop; it is updated when this lands.
- 2026-11-03 20:06Z — **PR 6435 was green (`ea939a36`) but conflicts with main.** Main's face-loop-home refactor (`7c1c89b7`, `19c27a27`) re-spelt exactly the two sites the unit unified: `region::boundary`'s placeholder skip and `Trim::of`'s loop walk. Handed back to the `held-pierce` lane: keep main's loop walks, keep the one `null::is_placeholder` predicate, and adopt main's vertex-point door in it if that is the reading main now uses.
  - Earlier on the same PR: the python teapot north-star row is re-pinned and FRAME's item closed (pncad-py had gone red on main from PR 6441); engine-leaves pins lowered after the placeholder predicate moved.
  - **Friction:** this PR has met a main move at each merge attempt (three today). A unit that unifies two readers is exposed to any refactor of either.
- 2026-11-03 20:15Z — **Rim-arc phase 0 accepted.** The circle-on-cylinder reading works. The gap is `parents_distinct_from` reading a NURBS parent as `OutsideInventory`. Ruled: a one-way "distinct", needing one certified point off the analytic surface on every Bézier piece (the identity theorem on a rational patch), confined to that branch. Klein wall 3's next stop is `GermFrameUnsupported { Cone, Nurbs }` (SECTARMS).
- 2026-11-03 20:15Z — **PR 6437's second fix pass** (`593c3a91`): no exemption was proven, so four builds now refuse. Among them the lofted cylinder's far union with a disjoint brick (PR 6429 confirmed it correct today), refusing between one operand's own rim arc and its own other wall. Filed P1 by the lane as `an-unread-rider-related-only-by-its-own-vertices-refuses`. A disjoint far union refusing is P0 by the band definitions, so the delta read is asked to judge a same-operand exemption from each operand's at-rest validity BEFORE merge.
- 2026-11-03 20:20Z — **PR 6437 delta read** (`593c3a91`): APPROVE-WITH-FIXES. The notch MAJOR is closed at three ε, and the four regressions are typed refusals.
  - **The same-operand exemption** is judged sound in principle when read by lineage: both cells' split roots lie in one operand's input, the shared vertex is unfused, and no record names the pair. Shared-vertex topology alone is not enough, because fused vertices join the operands. It rests on each operand being at rest, which is unchecked for spline or spiric carriers: there is no entry validation, and the tier-3 check-9 arms are silent there.
  - **Ruled:** soundness first. 6437 lands with the refusals. The exemption is its own unit (`an-unread-rider-related-only-by-its-own-vertices-refuses`, re-banded P0 because a disjoint far union refusing is a normal verb broken), owing both the lineage reading and the missing check. It is dispatched after 6437 lands.
  - **Class (logged):** a sound refusal can regress a build another unit confirmed that same day (PR 6429's far union). The cost is real and is scheduled at the band it earns, never avoided by keeping the unsound read.
- 2026-11-03 20:38Z — **PR 6476 (`nurbs-revert-roster`) dual review dispatched**: letter H, a CONCURRENT pair on the frozen head `1bcc5148a904c10b98ae7f35b7fd1cb83f757418`, with an identical brief (`briefs-1103/review-6476.md` and `review-common.md`). R1 is `session_013QoYWVuWAyCF8EZweeT5ae` (evidence `splice/review-6476-r1`); R2 is `session_013QoYWV4a6xpVA2SyVsF4Vo` (`splice/review-6476-r2`). The protocol is `24b76fff0e`, and neither reviewer received a relaxation.
  - **The PR:** `Nurbs` is admitted to ∖/∩ for every partner kind. The second roster and `PairRefusalSite::RevertRoster` are deleted. There is a kernel row per sign-reading spline path on a reversed face, and the ∖/∩ census is committed as a guard. Klein wall 4 now equals wall 3's stop.
  - **Main reds the lane met:** pncad-py's teapot north-star row (fixed in PR 6435), and `editor-core m10_p_fence` at Interval, filed on SCALAR as `main-red-the-m10-p-fence-interval-digest-moved` (P1).
- 2026-11-03 21:03Z — **PR 6435 merges** (`1a20e372`, `held-pierce`), closing `a-spline-wall-cannot-say-whether-it-holds-a-pierce-point` (P0) and FRAME's `the-teapot-north-star-row-still-expects-the-point-split-refusal` (P1; pncad-py's python row is re-pinned on the join's germ frame, which clears main's python red).
  - The conflict with main's face-loop-home refactor kept main's loop walks. The one `null::is_placeholder` now reads vertex points through main's `resolve_vertex_point_key` door, so region.rs's re-grown inline copy is gone.
  - Main's new `a_faces_loops_are_walked_in_one_place` audit caught the adopted seam row walking loops by hand, and it is fixed.
  - Teapot wall 3's default and 1e-12 stop is SECTARMS' NURBS × sphere arm.
- 2026-11-03 — **PR 6435 merges** (`held-pierce`), closing `a-spline-wall-cannot-say-whether-it-holds-a-pierce-point` (P0).
  - The spline trim lifts past the sweep's placeholders through a crate-private door, with one `is_placeholder` predicate; the public `Body::loop_lift` is strict.
  - The orchestrator read the fix pass. Slow sets on `985e2f4c`: topo 3909 plus the two known main reds, sweep 3536/3536, the tour green at three ε.
  - The demo-joins row (`curved-demo-joins-build`, now SPLICE's own since main retired `work/issues/`) swaps this row for SECTARMS' NURBS × sphere arm as teapot wall 3's default and 1e-12 blocker.
- 2026-11-03 — **PR 6437 merges** (`kind-reads`). Closed: `a-spline-rider-reads-as-lying-in-no-plane-face`, `section-area-skips-a-spiric-or-nurbs-section-edge`, `the-curved-arm-reads-a-spline-faces-side-through-the-dihedral-side-door` (overtaken) and `main-red-certified-endpoint-census-drifted-in-geom-brep-after-6171`. `split-section-boundary-curved-arm-untested-past-the-edge-gate` is re-banded P3 and parked on PARTING's split-gate row.
  - **Three review rounds:** two MAJORs in the first (a sampled refutation; a record bypassed), one in the re-review (the kept midpoint check: the notch case). All are closed.
  - **Four builds now refuse instead of shipping on a sampled read.** Their way back is the P0 `an-unread-rider-related-only-by-its-own-vertices-refuses`: the same-operand lineage exemption, together with the missing operand at-rest check.
- 2026-11-03 21:50Z — **PR 6437 merged** (`5f6aaf16`), the fifth unit of the sitting.
  - **Dispatched** `splice/same-operand-rider` (`session_013QoYWV9rr7vjvRgLfvwNip`) from main `5f6aaf16`, for the P0 `an-unread-rider-related-only-by-its-own-vertices-refuses`. A phase 0 is required: are the four stops this case; what establishes each operand's at-rest guarantee for spline or spiric carriers, and whose it is to build; is `split_root` lineage reliable. The exemption and the guarantee ship together. Letter H, so DUAL, concurrent pair.
