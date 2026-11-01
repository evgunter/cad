---
id: the-side-reader-refuses-a-body-thin-in-two-directions-whose-pairs-are-definite
kind: issue
title: The side reader refuses a body thin in two directions whose pairs are all definite (check 7 as a thinness gate on needles)
status: closed
closed: 2026-10-19
opened: 2026-10-14
priority: P1
cost: M
branch: flush/shell-role-reader
pr: 5119
refs: [4990]
---

Found by both reviews of PR 5119 (the shell-role reader, D10
"Booleans" as ratified on PR 4990), at the frozen head `e672e97ac`.
D10's premise "a ray's reading lies in band only where a pair does" is
false for one class of body, so check 7 refuses solids whose every face
pair is definite (> Kε): a thinness gate, which D1 says it is not.

## Measured

- At rest, `brick (0,1)×(0,w)×(0,w)` refuses `OrientationUndecided {
  Volume }` for w = 10.5, 12, 15, 19, 21 and 30ε at ε 1e-9 and 1e-6, and
  passes at 25 and 40ε. For w ≲ 2Kε no planar face offers an aim
  (`planar_interior_point` wants a point more than Kε inside its
  boundary); at 21ε every ray is set aside on a crossing within Kε of a
  loop boundary (`point_in_loop`); the volume fallback then holds V/A
  ≈ w/4 against the band.
- Through a public boolean, `brick ∩ brick` leaving a needle 2–3Kε
  across refuses at the door in 8 of 18 poses that main builds, typed
  `ResultInvalid`.
- The sweep needles in `fillet_h6_cap_rim` and
  `review_blend_k_rk_probes` refuse `OrientationUndecided { Volume(None)
  }`; they used to require "must validate at rest".
- At ε 1e-12 a revolve-door torus (R 1e-10, r 2e-11, tube 4Kε across)
  refuses: every 60° aim is set aside on a point-in-face margin within
  Kε of a face boundary, and V/A is Kε less one ulp.
- Main passes all of these at rest through the sliver hand-off PR 5119
  retires, which reads no orientation, so main passes their inside-out
  copies silently too.

Probes: `flush/shell-role-review-r1` at `0731fe3f7`,
`flush/shell-role-review-r2` at `c5d6fa21b`
(`crates/topo/tests/r2_shell_role_probes.rs`), and the designers'
`flush/needle-design-x` at `feb5b660e` and `flush/needle-design-y` at
`bfec39bf9`.

## The fork

Two designers converged in round 2 (fork-log row 176): a ray that
decides each edge it passes once for both faces it bounds and reads the
role as the sign of its count of crossings reads every planar needle
with definite pairs (measured: all 20 needle cases, both orientations,
ε 1e-9 and 1e-6); the volume's sign certified by its interval enclosure
alone, never held against the band, reads the needles and the torus
(measured: width/|V| ≈ 5e-16 on axis needles, 1.7e-14 on the torus).
The enclosure of an oblique needle carries an absolute floor of about
1e-16 from the re-derivation's fan about one corner of the body (width
1.28·V at 10.5ε, ε 1e-9; straddles zero on two boolean needle results
at ε 1e-12). The question is on PR 5119.

## Measured on the fix pass (`flush/shell-role-reader`, after `e672e97ac`)

The fix pass changes no reading of this class; it changes how the
refusals are typed and what they say.
- **At the door** a needle no aimed ray reads, its volume in band, is now
  `Escalated { SolidSide { solid, aimed, corners, others } }`, not
  `ResultInvalid` (D10: in band at the door). `brick ∩ brick` leaving a
  needle 2Kε across escalates `SolidSide` at every pose of review R1's
  three, at ε 1e-9 and 1e-6; at 3Kε the axis pose escalates at ε 1e-9 and
  the oblique pose at ε 1e-6, the others build.
- **At rest** the refusal names what did not read: how many faces offered
  an aim and each ray set aside, and why (`SetAside`). Every ray of the
  21ε needle is a crossing in band of a face's boundary; at ε 1e-6 the
  inside-out 30ε needle's ray at face 4v1 is set aside at the corner of
  1v1 and 5v1 (once refused as their pair).
- **The torus** (`shell7_r2_probes::p3b`, ε 1e-12): both its faces aim
  at the same point, `(−8e-11, 0, 0)`, the inner equator, which is their
  shared boundary (the default minor-window middle, `v = π`); each
  crossing there is set aside in band (`bool_ray_torus_split`, 8.0e-12).
- **Pins** that move when the fork's answer lands:
  `crates/topo/tests/shell_role_reader.rs`
  (`a_needle_refuses_its_orientation_at_rest`,
  `an_inside_out_needles_corner_is_set_aside_not_a_pair`,
  `a_boolean_leaving_a_needle_escalates_its_side`),
  `crates/sweep/tests/fillet_h6_cap_rim.rs`,
  `crates/sweep/tests/review_blend_k_rk_probes.rs` and
  `crates/sweep/tests/shell7_r2_probes.rs`.

## Ruling

Ev, PR 5119, 2026-10-15 ("this sounds good!", after asking why the
near-edge question should not escalate and rewording the tie-break as
a principle). Fork-log row 176. In DESIGN:
- D10: a ray within the band of an edge meets it, half a crossing for
  each face the edge bounds, and reads the role as the sign of its
  count of crossings; it refuses only on a pair; the volume's sign is
  certified by its enclosure alone, never held against the band, read
  where no ray reads.
- Q1: an instrument does not represent a tie-break the band forbids it
  to make.
- D1 check 7: definitely inside-out by any reading is invalid.

Sequencing: the volume half (V/A off `read_role`, the enclosure's sign
alone, the planar re-derivation as plane distance × face area) lands in
PR 5119. The tie-reading ray and the volume backstop's V/A (Ev: "filing
removing V/A on flush works") are the next FLUSH unit.

## Resolution

Closed by PR 5119 (2026-10-19) with the volume half of Ev's ruling.
- **The volume's sign:** read from its certified enclosure alone, with
  V/A gone from every role and orientation read:
  - the planar arm is exact for line-bounded plane shells;
  - other plane terms are widened by their own tilt bound;
  - the lane-free path is read against the walk's own rounding bound.
- **Measured** (both delta reviews, executed):
  - the bar `brick (0,1)×(0,w)×(0,w)`, w 10.5–30ε, every pose, at ε
    1e-9 and 1e-6: plain validates, inside-out is `NegativeVolume`;
  - every `brick ∩ brick` needle at 2–5Kε that builds reads;
  - the 1e-12 torus validates at rest;
  - the sweep needles validate at rest again;
  - width/|V| on turned needles went from 0.15 to 4.5e-16.
- **What remains:**
  - the edge-meeting ray, `a-ray-meets-an-edge-in-band-as-half-a-crossing-for-each-face`;
  - needle cavities at 10.5–15ε refusing at the join's section loop
    before any read, `a-needle-cavity-refuses-at-the-joins-section-loop-before-any-read`.
