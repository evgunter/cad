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
