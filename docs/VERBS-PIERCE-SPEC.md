# VERBS-PIERCE — the curved pierce/split substrate (one PR)

The unit CYLCYL PR-B's opening measurement created (adjudicated
2026-08-26, addendum in `docs/VERBS-CYLCYL-SPEC.md`): the
cylinder-union refusals come from the CROSSING layer, not the
join — so the germ arms (CYLCYL's parallel-axis + Steinmetz,
SPHSPH's circle, and every later exact rung) all wait on the same
two unwired doors. Shared substrate, one PR, branch
`verbs/pierce`. Difficulty logged pre-dispatch: **M**. #347's
union half is the consumer of record; the arms themselves are the
NEXT unit, not this one.

## The two doors (as measured on #1044's opening table)

1. **`PointSplitCarrierUnsupported` — Circle-edge splitting.**
   Splitting an edge at an event point is wired only for `Line`
   carriers (exact point parameter). The Circle arm is the
   deliverable: an exact split parameter on the f64 lane, and on
   the interval lane whatever honest form the carrier supports —
   the PR-B lesson binds: `atan2`/branch-cut forms that need
   ordering are not available on `Real`; a bracket/subdivision
   derivation (or a typed interval-lane refusal with the reason
   at the site) beats a lane fork. Both halves of the split
   inherit descriptions/pcurves per the existing Line-split
   shape; census + validity pinned both lanes.
2. **`CurvedPierceUnsupported` — the pierce event path.** A rim
   circle definitely piercing a partner wall must produce the
   split-and-ring-insert the Line pierce already performs. Reuse
   the existing pierce machinery with the Circle split from door
   1; mint nothing new geometrically (the event point comes from
   the existing section/route answers — this unit routes and
   splits, it does not intersect).

## Fences

- **No join arms.** After this unit the four measured CYLCYL
  cases must reach the JOIN layer and refuse THERE (typed, naming
  the absent arm) — the doors move one layer down, honestly. The
  arms unit then flips them green.
- No sphere/cone work beyond what the split door shares by kind;
  no Steinmetz, no `SectionConic` widening.
- The D10 extent-certificate posture (PR-A) is not touched.
- STOPS for adjudication if the ring-insert half needs machinery
  beyond the existing Line-pierce precedents.

## Acceptance

- The coaxial-boss case (PR-B's table row 3) passes the split and
  refuses at the join, typed.
- Parallel-equal-r and coaxial-equal-r (rows 1–2) likewise reach
  the join layer.
- Planted reds: a Circle split at a poisoned/degenerate parameter
  refuses typed; the interval lane's posture pinned (build or
  honest refusal, both directions per the two-arm pattern where a
  measured constant gates).
- Existing boolean suites bit-identical; the #1044 conservatism
  rows untouched.

## Lane obligations

`docs/prompts/implementer-discipline.md` binds. No Co-Authored-By
trailer (blinding). Lane-private PR draft. Merge origin/main
before opening; confirm CI runs STARTED (a CONFLICTING PR gets no
run, silently — verify one fires); note the drawn point; watch to
completion; cancel detached timers before the final report; do
not merge.

## Door 2 STOPS — the in-lane measurement (awaiting adjudication)

Door 1 shipped. **Door 2 fires the §Fences STOP** ("if the ring-insert
half needs machinery beyond the existing Line-pierce precedents"), and
the four measured rows are not one family:

- There is no Line-pierce-into-a-CURVED-face precedent to reuse:
  `curved_face_arm` refuses every definite crossing of a curved face,
  Line carriers included. The pierce path that exists is plane-only.
- The ROOTS are only half-missing, and the first statement of this
  addendum got it wrong. `solid_contain`'s `cast_ray` (:818-885) solves
  the certified ray × cylinder quadratic, generic over `T: Decide`,
  trimmed by `point_on_wall_in_face` — so **LINE × cylinder crossing
  parameters already exist** and are reusable. What does not exist is
  the CIRCLE × curved-surface root: the residual there is a degree-2
  trigonometric polynomial, a quartic in `tan(θ/2)`, and nothing in the
  repo solves it (`conic_plane_crossing_roots` is plane-only;
  `implicit.rs` offers enclosures and no roots). "The event point comes
  from the existing section/route answers" still has no answer to read
  for the circle case — surface×surface sections take no curve.
- The pierce ring (`vtxfac.rs`, Delta 3) needs the pierced face's
  PLANE and outward normal and refuses without one; its transient
  chord is a straight `line_between`. A pierce into a cylinder wall
  has no ring lane whatever the roots say.
- **Two of the four rows are not pierces.** Both are undeclared
  VALUE-COINCIDENT contacts, and they are coincident in DIFFERENT
  places. `coaxial-equal-r`'s rim circle lies on the partner WALL
  carrier. `coaxial-stacked`'s binding coincidence is not the
  rim-on-wall sentence at all — the two cylinders meet cap to cap, so
  the contact that must be declared is the pair of **coincident cap
  DISCS**, a plane × plane rest; the wall carriers happen to agree as
  well, but they are edge-adjacent, not the contact. Plain `union`
  passes `BooleanDeclarations::none()`, and CONTACT-DESIGN C2/C4 forbid
  inferring either gluing at any ε, so no arms unit flips them —
  their honest destination is the declaration ladder, not the join.
  The acceptance sentence "rows 1–2 likewise reach the join layer"
  rests on a premise the ratified contact design denies.

Only `parallel-equal-r` (circle × wall) and `steinmetz` (line × wall)
are genuine pierces, and their costs are NOT equal. **`steinmetz` needs
only the ring lane** — its line × wall roots exist at `cast_ray`.
`parallel-equal-r` needs the ring lane and the circle × wall quartic.
The ring lane is the shared blocker and the reason both stop.

## What door 1 turned out to include

The split had a ROUTING half the spec did not name, and it carried a
silent wrong answer: `contfp` decided a `Circle` boundary edge by its
CHORD, and a cap rim's chord is the disc's own diameter — so every
event on the diameter, strictly inside the cap, was reported `OnEdge`.
That is what #1044's table measured as the coaxial-boss row's door.
One layer below, `point_in_loop` reads a loop as the polygon through
its vertices, and a cap loop has two vertices: **every interior point
of a cylinder cap answered `Out`**, so a box driven through a cap
unioned as two disjoint solids with the overlap counted twice
(7.003185307179585 against a truth of 6.643185307179586). Both are
closed: `Circle` boundaries take their exact arc rows, and a loop of
arcs of one circle is read by its radius — the planar analog of
`curved_face_containment`'s iso-bounded class.

**What is left is not an "honest remainder", and calling it one was
wrong.** `point_in_loop`'s own contract is a planar POLYGON (line
carriers, the F5 regime), so falling back to it with an arc-bearing
loop is a call outside its stated domain — a contract violation, not a
conservative default. The class is **arc-bearing non-disc loops**, not
"loops that mix arcs and lines": a LENS cap is two arcs of two
different circles, all-arc, and equally broken. Both #1068 reviewers
measured silent wrong bodies at that shape (half-disc cap
3.321592653589793 against 3.231592653589793; half-cylinder cap
3.266592653589793 against 3.204092653589793; lens likewise). The unit
therefore GATES it: an arc-bearing loop with fewer than three
vertices — where the polygon has zero area and the error is
demonstrated — refuses typed
(`BooleanError::ArcLoopContainmentUnsupported`). Arc loops with three
or more vertices keep the polygon walk: a slot (4) and a rounded
rectangle (8) were measured correct, and that is the whole evidence —
unproven, not proven. Issue #1076 owns the general arc-aware ray
parity that retires both.
