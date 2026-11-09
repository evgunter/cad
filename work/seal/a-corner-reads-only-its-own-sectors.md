---
id: a-corner-reads-only-its-own-sectors
kind: unit
title: a corner meeting an unglued pair reads only its own sectors: no carrier re-read, unglued_coincidence and unglued_tilt retire, an off far bound ends Coincide::Sectors
status: closed
opened: 2026-11-09
priority: P1
cost: M
closed: 2026-11-09
parent: glue-disagreements-and-held-contradictions-after-stage-4
---


Step 4 of the parent's design (fork `glue-disagreements-carry`, 2026-11-09). Step 1 gave `bounds_off` its `Decided` (`Result<Decided, Indeterminate>`), and this row was opened when step 1 closed.

## What

Today a corner that meets a pair the glue door did not glue re-reads the carrier pair over its own arm: `carrier_eq_reading` in recl.rs and vtxfac.rs. `unglued_tilt` then decides `bool_plane_parallel` a third time. All of it serves only to build a refusal, and C22 (`unglued_coincidence`) is that refusal.

- **`recl` and `vtxfac` stop calling the carrier ladder.** These go with it: `unglued_coincidence`, `unglued_tilt` and both "ON sector with definitely-distinct carriers" invariants.
- **The glue door keeps no verdict table.** No corner branches on the door's Apart; it would feed only wording.
- **Precedence, as today (`unglued_corner`):**
  1. the bounds' reading, where it offers a tolerance (`Coincide::Sectors`);
  2. else the door's in-band reading (`door_refusal`);
  3. else the sector's own funnel reading.
- **The open case: `ReadOn::Faces`.** The near arm meets within band (`fan::meeting`'s `bool_faces_parallel` decided Zero at the shorter arm), while `bounds_off` decides the far bound off.
  - These are two readings of the sector over two extents. The case is reachable on valid input: a 1 mm and a 10 m bound tilted by θ with 1 mm·θ < ε and 10 m·θ > Kε.
  - It ends `Coincide::Sectors` with `Off(Decided)` (the far bound's departure), told by the Sectors lever.
  - `gate_rejection` is not available here: `Sectors` passes `AnySign`, so the door's `unreachable!` would fire.
- **`ReadOn::FirstBounds`** reads one quantity twice (`BoundSupport::departure`). It ends the same way, retiring its `ClassificationInvariant`, unless a probe shows the two reads cannot disagree.
- topo README C4's sentence "An unglued pair is read at a corner by the ladder over the corner's own arm, where every verdict but `Distinct` refuses" retires with the code. It is agent text (`c7571919b0`, PR 6079) with no ratification found.

## To probe before filing anything

- On the `Faces` path, a tolerance that `bounds_off` offers above `bool_faces_parallel`'s margin at the short arm may still read Coincident and refuse again.
- Whether a fold step's corner reaches this arm.

## Done when

No corner reads a carrier pair. Both `ReadOn` arms end on the sector's own reading, pinned by a scene of the 1 mm / 10 m shape.

## Closed (2026-11-09)

No corner reads a carrier pair. `recl::require_same` and `vtxfac::unglued_flat_sector` no longer call `carrier_eq_reading`. An unglued pair at a corner refuses through `boolean::unglued_corner` on its first sector's own bounds (`BoolSector::bounds_off` against the other sector's tangent plane), with the precedence unchanged:
1. the bounds' reading where it offers a tolerance;
2. else the glue door's in-band reading;
3. else the bounds' reading as it stands.

Every arm is `Coincidence(Sectors)`. A far bound decided off is the corner's coplanar question's gate rejection: `k_stats::gate_rejection("bool_sector_coplanar", off, band, SizedPass::Zero)`, logged, `rejected_sign() = Positive`, read `Moot`, ending in the Sectors lever alone. The PR body has the per-site table, each ending before and after.

- **Gone:**
  - `unglued_coincidence` (C22, `carrier_unglued_coincidence`);
  - `unglued_tilt`;
  - `ReadOn` (both arms now end the same way);
  - the three "definitely-distinct" invariants (two in `recl`, one in `vtxfac`);
  - both "a sector bound read On departs definitely from the (other's) plane" invariants (`recl`'s FirstBounds one, and `vtxfac`'s decided-tilt one, which ends the same way);
  - the corner's plane-ladder `Escalated`, `Undecided` and `OffRing` arms;
  - the offer rows `planes_facing_at_a_short_arm` and `facing_planes_lengthened`, whose `PlaneOrientation` question no corner asks any more.
- **Premise correction:** "`gate_rejection` is not available here" was premised on Sectors' `AnySign`. The corner's own question passes only a Zero (`SizedPass::Zero`), so a far bound decided off is a rejection of it, and the door's `unreachable!` cannot fire. The tag's pass set (`Zero`) is the corner's, and the table's (`AnySign`) is the question's. Under `AnySign` a margin past the band ends in the lever with no tolerance (ruling (b) on the fork's residue).
- **Probes:**
  - **The 1 mm / 10 m scene** (`tests/a_corner_reads_only_its_own_sectors.rs`, θ = 100ε) reaches the `Faces` arm through the public union: arm 1e-3, span 10, `bounds_off = Ok(Positive, 10θ)`. On main it ended C22 with margin `0.0`. It is pinned at ε 1e-9 and 1e-12.
  - **Probe 1** (`Faces` with the far bound in band): on chord bounds the corner is not reached. The vertex-on-face sweep refuses the far vertex first, on the same 10θ. Followed to that offer's tolerance, the op refuses at the glue door or the containment walk, not at the corner. Curved and bisector far bounds were not probed.
  - **Probe 2** (fold step): an n-ary union of A, B and a far brick, in three member orders, refuses at the pairwise judgement of (A, B) with C22 (main) before any step runs. No step's corner reaches the arm on this shape.
  - **Probe 3** (FirstBounds): the edge-sector flankers read their bounds on the run band (`flank_read`, `pair_codes`), the same departure `bounds_off` takes, so two tied bounds cannot decide off as their max. The edge-edge path reads a representative on the line cone's reading, which can carry a slack. `recl`'s `a_sheets_side_decided_within_the_cones_tie_is_the_corners` shows it: dipped 1.5·zero inside a 1.8·zero slack, the flank reads On, and the corner's own bound decides off. That row now pins the gate rejection.
- **No tolerance offered:** the ending offers none, though a smaller tolerance (below the near arm's margin over K) clears the corner. That is recorded as evidence on `work/flushtail/a-door-refusal-with-no-offer-hides-the-looser-tolerance-that-glues`.
- **Closed into this row:** `decided-coincidence-carries-a-synthetic-invalid-margin` (claimed from TWICE) and `the-corners-senses-decide-the-plane-orientation-a-second-time`.
- **Filed (HONEONE):** `a-crossing-germ-re-decides-the-faces-parallelism-the-fan-decided` and `the-covered-arc-re-reads-a-carrier-pair-the-glue-door-decided`.
