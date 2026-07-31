# M5 S9 — chord_spec arc-side repair: azimuth-window containment (binding spec)

Repairs the pre-existing PR 5 defect found by PR 6's mint pass
and independently confirmed at the merge-base by PR 6's review
(see #144's writeup): `splitting::join::chord_spec`'s arc-side
rule selects the COMPLEMENT arc on the tilted belly cut — its
premise ("the run sample's azimuth lies inside the chord's
interval") fails whenever the divided face spans more azimuth
than the chord. Interim posture on main: the configuration
refuses typed (`LoopNotClosed`, azimuth advance −2τ), pinned by
`tilted_belly_cut_refuses_on_the_long_way_arc_defect`. Branch
`ev/m5-s9-arc-side` from main.

## 1. The correct criterion

The chord's stored arc must be contained in the divided face's
own azimuth window — the same statement PR 6 certifies for
pcurves (and the reason its mint pass caught the defect). Shape:

- Compute the divided face's azimuth window from the run
  structure the join already walks (the sub-face's boundary
  azimuth extent on the chart), NOT from a sample point's
  membership in the chord interval.
- Select the arc (of the two complementary candidates) whose
  sweep lies within that window; the selection is exact
  structure (interval containment of closed forms), not a
  sampled predicate. If NEITHER candidate is contained (a
  genuinely degenerate window) or BOTH are (window ≥ τ with an
  ambiguous chord), refuse typed with a named sub-case — do not
  guess. In-band window boundaries escalate F6 with a named
  lever arm (azimuth × chart radius, meters — the PR 6
  convention).
- D9: fixed evaluation order; no data-dependent iteration.

## 2. What flips

- The belly-cut refusal row FLIPS to a passing construction:
  both parts closed and valid, all section arcs contained in
  their faces' windows, spans summing per part, bit-identical
  replay, both lanes. Rewrite the pinned evidence row into the
  certified-pass row (keep the history note).
- The upgraded seam-coincident cut and every other PR 5
  configuration must stay bit-identical (their windows exceed
  their chords' spans only in the belly class — verify and state
  where the new criterion changes selection: it must be exactly
  the configurations that previously refused or mis-selected).
- PR 6 pcurve minting on the repaired bodies: the belly-cut
  bodies now mint pcurve caches — add the acceptance row
  (certification green on the repaired arcs; this closes the
  loop with the machinery that found the defect).

## 3. Acceptance

- The flipped belly row (both lanes + 1e-12); a rotated-frame
  belly variant (the window computation must not depend on seam
  placement); the m5_pr5_tilted_cut suite green throughout; the
  neither/both refusal sub-cases pinned (construct the window ≥ τ
  case via a face that wraps almost fully); escalation row for
  an in-band window boundary; M2/M3 plane-lane suites
  bit-identical.
- Local: -p topo -p sweep both lanes, fmt, clippy touched. CI
  gates the matrix.

## 4. Out of scope

Pcurve window tightening (named separate unit in #144); any
change to the pcurve certification; census/tessellation.

## 5. Process

Standard rules (foreground; one row per call; push per unit;
OUTPUT DISCIPLINE ≤30 lines, numbered deviations). Review: one
adversarial pass — the reviewer must construct at least one
configuration where the OLD rule and NEW rule disagree outside
the belly class if any exists (the spec claims none), and attack
the window computation's seam-placement independence.
