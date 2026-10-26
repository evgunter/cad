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
