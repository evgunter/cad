# VERBS-SHELLFIX — the two teapot-found shell defects (two PRs)

Wave-3 aftercare: the defects #1078's demo found in the shipped
verb, both dual-verified on independent fixtures at ordinal 100.
Branches `verbs/shellfix-1`, `verbs/shellfix-2`. Difficulty
pre-logged: PR-1 **L**, PR-2 **M**. The issues carry the measured
record — read BOTH in full before either PR; the review corrections
appended to each are part of the record (the first characterization
of each was wrong in a way the fixtures corrected).

## PR-1 — #1082: `shell_open`'s rim on revolved bodies

The validated-wrong-body class (highest priority): tiers 1–3 bless
a body whose designated face carries its cavity counterpart's
boundary re-labelled as an interior ring — sharing the outer
loop's axis vertex, overlapping its seam edges — and CDT refuses.

1. **The class is re-scoped by the review — build to the corrected
   class**: "any designated face whose cavity counterpart's
   boundary cannot become an interior-disjoint ring of it". Two
   failure shapes, BOTH in the acceptance: the axis-touching
   half-disc (the D-loop), and the annular mouth whose correct rim
   is TWO disjoint annuli — a face SPLIT `kfmrh` cannot express.
   The one-face revolved-tube fixture (`the_seam_split_is_not_the
   _mechanism`, adopted at the fix pass) proves the seam-split
   story was NOT the mechanism; do not rebuild around it.
2. The fix direction is the opened arm's rim construction: where
   the counterpart's boundary is expressible as interior-disjoint
   ring(s), mint them correctly; where it needs a split, either
   perform the split through existing doors (M6-1 discipline; STOP
   for adjudication if that needs new surgery machinery) or refuse
   typed naming the shape — a refusal is acceptable, a validated
   wrong body is not. The teapot's pot MUST end opened (the
   scene's planted reds flip) OR the refusal must be typed and the
   scene's walls updated honestly — state which the geometry
   forces.
3. **The validator learns the invariant**: a ring sharing a vertex
   or overlapping an edge with its face's outer loop is a tier-3
   refusal (the B-rep violation both reviewers identified). This
   is the net that turns the whole class loud forever — planted
   red on the ordinal-100 anatomy fixtures.
4. Acceptance: every planted-red row from the unit + both review
   probe suites flips or re-pins per its own instruction; the
   teapot scene updated per item 2; `probe_opened_vessel_cup`
   extended to check rings/genus/mesh (the review's "why nothing
   caught it"); box control bit-identical; existing suites
   bit-identical elsewhere.

## PR-2 — #1081: the oblique-junction re-anchoring

`plan_reanchors` (replace_face.rs:1466-1473) re-anchors a moved
face's boundary against the neighbour's UNMOVED carrier; at any
oblique junction the gap is exactly t·|cos θ| (the review's law,
verified on hexagon/bevel/kite) and the group door refuses.

1. The fix: when the neighbour is IN THE SAME GROUP (its own
   offset is being applied in the same `replace_faces_offset`
   call — the shell case), re-anchor against the neighbour's
   MOVED carrier. Neighbours outside the group keep the current
   posture (their carriers genuinely don't move). This dissolves
   the class for shell (every boundary face moves together) while
   changing nothing for the single-face door.
2. The tangent-adjacent third door (replace_face.rs:1325 — a
   mapped description whose surface's offset is not a rigid
   translation) is OUT of scope: it is real, separately
   documented at the fix pass, and its fix is the mapped-
   description transport family. Keep its refusal; note it.
3. Acceptance: the teapot's belly UN-SQUARES (the pot hollows with
   its real arc meridian — the scene's wall-1 flips and the
   register row retires); the ordinal-100 fixtures (hexagon,
   beveled box, kite, sphere-zone, cone-frustum, triangular
   prism, the partial-revolve wedge) all hollow with closed-form
   volume pins; the tangent bullet still refuses at its own door
   (differential); the #1048 acceptance corpus bit-identical.

## CORRECTION (2026-08-28, before any PR-2 code — the opening measurement)

**PR-2 §1's premise above is FALSIFIED and its fix must not be built.**
Measured on the ordinal-100 hexagon (`t = 0.02`) by the PR-2 lane
before writing code; the difficulty is re-logged at the re-scope
(PR-2a **M**, PR-2b **L**), pre-work, so the A/B pre-registration
stands.

**1. The refusing edge's two faces are BOTH outside the group.** §1
says to re-anchor against the neighbour's moved carrier "when the
neighbour is IN THE SAME GROUP (its own offset is being applied in the
same `replace_faces_offset` call)". That condition is never true on
this class: `shell` offsets ONE chart per call, and the edge that
refuses belongs to two OTHER charts.

```
group face        FaceKey(3v1)  plane n = (-0.866, -0.500,  0.000)
refusing edge     EdgeKey(2v1)  gap 0.010000000000000009  (= t/2)
  plus side face  FaceKey(4v1)  plane n = ( 0.000, -1.000,  0.000)
  minus side face FaceKey(2v1)  plane n = ( 0.000,  0.000, -1.000)
```

The gap is `t·|n_group · n_neighbour| = t·cos 60° = t/2`, which
confirms the review's law with the neighbour pair identified.

**2. Re-anchoring alone would produce a WRONG BODY.** Grant the door
full knowledge of the co-moving set and let the re-anchor succeed: the
moved VERTEX is still derived from one chart's rigid transport, and
`shell` visits the three charts at a corner in sequence, so the corner
accumulates `−t(n₃ + n₄ + n_cap)`:

```
original corner V       = [-0.100000, -0.173205,  0.000000]
true offset corner      = [-0.088453, -0.153205,  0.020000]
sequential accumulation = [-0.082679, -0.143205,  0.020000]
discrepancy             =  0.011547 m   on a 0.020 m wall

seq corner's signed offset from side3: -0.030   (want -0.020)
seq corner's signed offset from side4: -0.030   (want -0.020)
seq corner's signed offset from cap:   -0.020   (want -0.020)
```

30 mm of wall where 20 mm was asked for, at both oblique faces — and
no tier catches it: every loop stays simple and consistently wound and
the volume is whatever the wrong corner makes it. **`ReanchorOffCarrier`
is the gate PREVENTING that body and stays load-bearing** until the
simultaneous solve replaces it; removing it without one converts a
typed refusal into the validated-wrong-body class #1082 just closed.
A box is unaffected and always was — with mutually perpendicular
normals the accumulated sum satisfies `δ·nᵢ = −t` exactly, which is
why #1048's corpus never saw this.

**3. The repair is a SIMULTANEOUS offset**, not a re-anchor posture:
each moved vertex solved against ALL the moved surfaces meeting it,
each affected edge re-derived as the intersection of its TWO MOVED
surfaces. Split accordingly:

- **PR-2a (M)** — the door for ALL-PLANAR corners: a 3×3 solve per
  vertex, plane∩plane per edge. Acceptance: hexagon, bevelled box,
  kite and triangular prism hollow with closed-form volume pins (the
  `t·|cos θ|` rows become POSITIVE rows); a non-simple corner (valence
  > 3 inconsistent, or a singular 3×3 — coplanar-adjacent faces)
  refuses typed naming the shape; the box corpus bit-identical;
  `ReanchorOffCarrier` keeps firing for everything the door does not
  cover, curved corners included (differential rows). STOP again if
  the edge re-derivation needs intersection machinery beyond
  plane∩plane.
- **PR-2b (L)** — the curved corners through the C5 table
  (plane∩cylinder / sphere / cone per-corner solves). The sphere-zone,
  cone-frustum and partial-revolve-wedge fixtures land there, and so
  does **the teapot's belly, which is a sphere zone**. The pot's
  un-squaring is therefore honestly PENDING until 2b and must not be
  half-claimed in 2a.

## CORRECTION (2026-08-29, PR-2b's opening measurement)

**PR-2b's corner shapes above are wrong, and the door was built to the
measured ones.** Taken at head before any 2b code, by the probe that
ships as `crates/sweep/tests/sf2b_head.rs`.

**1. The dominant curved corner has TWO surfaces, not three.** The
split above says the curved corners are "the plane∩cylinder / sphere /
cone per-corner solves", carrying PR-2a's 3×3 frame across. Measured,
a full revolve's rim vertex is incident to exactly two distinct
surfaces:

```
sphere-zone vase   4 corners  2 [plane ∩ sphere]     + 2 axis poles
cone frustum       4 corners  2 [plane ∩ cone]       + 2 axis poles
drum               4 corners  2 [plane ∩ cylinder]   + 2 axis poles
bellied pot        2 [plane ∩ sphere], 2 [cylinder ∩ sphere],
                   2 [plane ∩ cylinder], 2 axis poles
partial wedge      4 corners  3 [cylinder ∩ plane ∩ plane]
                   2 corners  3 [plane ∩ plane ∩ plane]
```

Two surfaces determine a CURVE, not a point. What pins the vertex on
it is the revolve's seam, whose azimuth is conventional data (D2)
carried from the operand — the same law PR-2a applies to a line's
`t = 0` anchor. There is no `plane∩curved∩curved` corner anywhere in
the corpus, and none is built.

**2. The teapot's belly is not only a sphere zone.** The spec says
"the teapot's belly, which is a sphere zone". The bellied pot carries a
FOOT CYLINDER below it, so two of its four junction corners are
`cylinder ∩ sphere` — a pair `intersect::route` sends to rung 3. The
door answers it without the table and without marching: in the meridian
half-plane a cylinder is a line and a sphere is a circle, and that
meeting is a quadratic. Nothing widened the table; a torus, whose pair
has no arm at all, never reaches the door and keeps the table's own
refusal.

**3. Two latent defects, both found by fixtures rather than by
reading.** `geom_brep::offset_surface`'s cone arm is NAPPE-BLIND — the
apex slide moves the surface `+d` along its own normal on the `v > 0`
nappe and `−d` on the other — so the cone frustum GREW when asked to
shrink (cavity 0.001058 against an operand of 0.000895). And
`shell_open`'s rim LIFT transported a rim rather than solving it, which
coincides on a cylinder and is 6.2 mm wrong on a sphere; the lift now
takes the same simultaneous door with every other chart at distance
zero.

**4. The conditioning lever is the incident edges' ARC LENGTH**, not
their chord. A chart with one seam closes on itself and its chord is
exactly zero, which made every meter read `Zero` and called a perfectly
transversal corner degenerate. Measured on the revolved tube.

**5. Two more, found at the MERGE with main's description collapse.**
U2 restated a conventional locus as a chart IMAGE plus an authority
record. The per-face door carries an image forward under a constant `v`
shift because it keeps the edge's parameter WINDOW; this door re-solves
both endpoints, so an edge shortens and slides within its own chart and
a constant shift describes none of that — measured as a `ChartResidual`
refusal on the cone frustum's anti-seam. Every chart image here is
therefore `image: None`, which the spec's own docs define as the
REQUEST to derive the image from the carrier. And a DECLARATION is
re-authored in its own sketch plane rather than translated, which is
what lets a reshaping chart (a sphere's concentric arc) travel at all.

## Fences

- No SSI, no crossing-pipeline entry (the no-crossing pin stands).
- #1055's curved-clearance window and #1056's hollow-operand gate
  are untouched (their gates still fire).
- The single-face door's outside-group behavior unchanged.
- PR-1 STOPS for adjudication if the annular split needs surgery
  beyond existing doors.

## Lane obligations (both PRs)

`docs/prompts/implementer-discipline.md` binds. No Co-Authored-By
trailer (blinding). Lane-private PR drafts. Targeted local runs;
verify hosted coverage at the STEP level (the klint_row lesson —
a green job name is not evidence). Merge origin/main before
opening; confirm CI jobs actually RUNNING; note the drawn point;
watch to completion; cancel detached timers before the final
report; kill detached jobs whose evidence is superseded (the
#1085 rule). Do not merge.
