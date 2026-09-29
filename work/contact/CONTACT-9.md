---
id: CONTACT-9
kind: unit
title: the boolean and splitting side codes: trace whether a levered chord-direction Zero is ever a wrong verdict, and fix it in metres where it is
status: closed
opened: 2026-09-28
priority: P2
cost: M
branch: contact/9-side-codes
closed: 2026-09-29
---


Carries `boolean-side-codes-lever-a-chord-direction-where-a-zero-is-read-as-on`.
Spec: `docs/CONTACT-9-SPEC.md`.

Review tier: set at the trace's hand-back. If the trace finds no
reachable wrong verdict, the orchestrator reads it. A fix that changes
what the boolean classifies goes to a single full review.

## Closed

A line bound's side of a face plane is read at its far vertex, in
metres (`Reach::Chord`, `sector_shape::plane_offset`, one reader shared
with the splitting lane). A conic or fitted bound is levered at its own
extent, and a bisector at its sector's arm. The needle-on-a-face,
needle-on-a-corner and tilted-wedge poses, which refused "(kernel bug)"
at base, now answer against ground truth at every ε run. A pair whose
normals are parallel at the shorter arm stays a coincidence for the
carrier ladder; `vtxfac`'s lump requires its bounds to read On; the
pierce germ gate is levered at the sector's span. Both bisector sites
refuse typed where only the band's Zero could decide (K ≤ 2). The
`UnpairedLooseEnds` message no longer says "(kernel bug)".

Review tier: the trace found refusals, not wrong verdicts, and the fix
changes what the boolean classifies, so it went to a single full
review. It asked for changes, with one MAJOR: the first gate turned a
corner near-coincidence's typed refusal into an invariant refusal. The
fix pass root-caused it and restored the typed refusal; it pinned every
change with a row (mutant table in the PR) and recalibrated the rows'
volume floor. The orchestrator read the diff and the fix pass.

Filed (all in `work/contact/`):
- `a-vertex-pair-near-coincidence-refuses-where-its-long-edges-decide`;
- `a-wedge-edge-on-a-block-top-from-its-corner-refuses-at-the-join`;
- `boolean-bound-parallelism-verdicts-are-levered-at-a-short-or-unit-arm`;
- `boolean-conic-side-code-zero-is-first-order`;
- `coplanar-lump-carrier-verdict-is-levered-at-the-sector-arm`;
- `pierce-germ-direction-within-is-levered-at-the-sector-arm`;
- `seam-description-reads-a-dihedral-at-the-seams-own-length`;
- `volume-door-reads-a-tiny-valid-boolean-result-wrong`.
