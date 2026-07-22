---
name: Boolean consumer findings (M3 PR 5)
description: Demo-tour findings as the first outside consumer of union/subtract — the silent cookie-cutter wrong-component defect, extrude-operand DanglingDescription refusal, coplanar/touching refusals; repro recipes
type: knowledge
---

# Boolean consumer findings (M3 PR 5, branch ev/demo-boolean-tour, 2026-07-21)

Found while building the demo tour's boolean leg (`demos/tour/src/bool_bodies.rs`,
which demonstrates all of these live with exact numbers). Operands are extruded
axis-aligned boxes with edges re-described as chord lines (see finding 2).

1. **RESOLVED by PR 5's merged tree (verified in the demo run, 2026-07-22)**:
   the silent wrong-component defect is gone — the same repros now return the
   exact volume on R1's working orientations ({+z, −x, −y}: pocket subtract
   V = 7.979192 exact) and refuse typed `SeamOrientation` on the refusing half
   ({−z, +x, +y}) per the R1 record in `docs/M3-LOG.md`. The die demo is
   promoted; its self-promoting guard now sits on a −z pip awaiting PR 5.5.
   Original finding, kept as the historical record:
   **Cookie-cutter seams are SILENTLY WRONG** (fail-loud violated). When the
   seam ring closes within a single face of an operand, the finish stage keeps
   the wrong component and it passes the tier-1/2 gate. Repros (exact):
   - pocket: `[-1,1]^3 ∖ [-0.17,0.17]^2×[0.82,1.25]` → returns the inverted
     cut-out fragment, V = −0.020808 (want 7.979192), kind Seamed.
   - through-pillar: same cube ∖ `[-0.17,0.17]^2×[-1.25,1.25]` → V = −0.2312.
   - boss/inset-leg union: tabletop ∪ leg overlapping into the underside →
     returns only the leg-outside piece (V = 0.09464 instead of 3.45464).
   - flush-stacked overlap union (shared side planes) → V = 2.675516 (want 2).
   Only caught by a volume oracle — tier 2 doesn't check global sense and
   tier 3 isn't in the op's gate. Blocks any pocket/boss feature (e.g. a die).
   Multi-face seams (two-brick style) are exact.

2. **Extrude output is not boolean-consumable as-is**: `extrude` describes
   edges as `Intersection{s1,s2}`; the ops' carve/merge drop faces those
   surface keys reference → `Merge(InputNotClosed{DanglingDescription})`.
   Workaround: re-describe operand edges via `EdgeCurveSpec::line_between`
   (demo `booleans::normalize_edges_to_chords`). The ops should remap or
   re-describe operand descriptions themselves.

3. **Touching/coplanar unions refuse** (now PR 5.5 territory, typed):
   flush-stacked touching boxes → `Join(UnpairedLooseEnds{8})`; corner-flush
   shared-plane leg → `Join(RingHoming(Escalated{point_in_loop_boundary}))`;
   knife through a two-shell Voided body → RingHoming at first; on the
   post-fix-pass tree (2026-07-22) that cutaway refuses
   `JoinDesync("neither section loop's regions hold a classifiable vertex")`.

4. **Working today**: multi-face seam union/subtract (exact volumes), nested
   `Voided` (two shells, V exact), disjoint assembly. `Voided` bodies
   tessellate/mass-property correctly (void shell subtracts).

Post-fix-pass reconciliation (2026-07-22, this PR): the die assert fired as
designed (its pip was on working +z) and the die is promoted — live R1
orientation matrix + 3-pocket compose (V = 7.937576 exact), self-promoting
guard retargeted to a −z pip for PR 5.5. open_box's pure interior-top cutter
now succeeds (V = 2.368 exact). Finding 2's chord-normalization workaround is
STILL IN PLACE — whether PR 5's extrude-operand description remap made it
unnecessary was not re-tested; check when PR 5.5 unblocks the pure variants.
