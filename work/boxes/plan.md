# BOXES — the plan

The region a face or edge can occupy, decided in several places.

Opened 2026-09-20 by REACH's priority-seam cut; cut again 2026-11-02,
when the rows that do not ask the extent question went to ONEDOOR and
`signed-penetration-depth` to CLEAR (`log.md`).

## The diagnosis

Every P1 row here is one question — *what region can this face or edge
occupy?* — answered at a site that could not find the answer's home,
because none is named: `[one-way.places]` has no row for it. So the
fix for each row is not "pick the survivor" but "name the home and make
the next caller unable to miss it". The order follows from that.

## Order

1. **The torus window onto its closed form**
   (`split-gate-sphere-zone-folds-into-face-box-rule`, the torus half).
   `FaceBoxRule::TorusWindow` samples the chart rectangle and pays a
   chord-dip charge; the split gate reads `boxes::torus_rect_extent`,
   the exact extent, beside it. One arm, exact, read by both lanes and
   the gate. That retires the sampled construction and with it
   `boxes::subdivision_charge`, the second home of
   `the-chord-dip-charge-has-two-homes` (riding with it, `cost: C`).
   Mechanical in shape: DESIGN names the face box rule's home
   (`FaceBoxRule`), and a closed form beats a sampled bound.
2. **One home for the extent** — `D280` (the two box lanes),
   `an-edges-extent-has-four-dispatchers`, and what
   `sphere-operand-box-is-the-whole-ball` leaves. A design signal: a
   survey maps every door that answers the question, then designers
   weigh where its one home is and what makes a new caller find it;
   an `[ev]` PR if the answer adds a `[one-way.places]` row.
3. The P3/P4 tail rows are arms of the same rule; most will be closed
   or restated by step 2's answer, so they wait on it.

## Review posture

Step 1: single FULL review (the box arms are certified enclosures; an
unsound extent is a silent wrong answer).
