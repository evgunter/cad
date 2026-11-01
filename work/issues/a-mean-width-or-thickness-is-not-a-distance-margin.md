---
id: a-mean-width-or-thickness-is-not-a-distance-margin
kind: issue
title: Margins metered as a mean width (2A/P) or mean thickness (V/A) are not distances within which a point is right to ε
status: open
opened: 2026-10-16
---


DESIGN `[intrinsic.epsilon]` and `[tolerance.margins]` (the principle-first
revision of `docs/DESIGN.md`): ε bounds the deviation at a single point,
and a margin is a distance at which a point is or is not within ε. A mean
width `2A/P` or a mean thickness `V/A` is not such a distance (Ev,
2026-10-16: V/A "is not checking 'things are right to within distance
eps'"), and the revision drops both from the blessed `Margin` doors. The
code still meters with them:

- `crates/geom-core/src/predicate.rs:743-762`: `Margin::over_lever`'s
  docs name `2A/P` and `V/A` as its cases. Most of its ~68 call sites
  divide by a lever at which the quotient is a real point displacement
  (a cross-product norm over a length), and those stay.
- `crates/geom-brep/src/props/curved.rs:140,1441,1485,1501,1514,4404`:
  region and face area signs metered as a mean width `2·A/P`.
- `crates/geom-brep/src/props/quad/`:
  mean width and "check 7's V/A metering".
- `crates/editor-core/tests/dsc_checks.rs:288-403`: tests posed on V/A
  in band.

Related, already filed: FLUSH's shell-role reader (PR 5119,
`work/flush/check-10-reads-a-cavitys-role-from-its-volume-sign.md`)
retires V/A as the reader of a shell's role and orientation, and its
follow-up row `the-volume-backstops-plus-v-posture-reads-a-sign-the-side-reader-cannot`
takes the boolean volume backstop. This row is the rest: every sign
decided on a mean width or thickness, which no row questions today. For
each site, either meter the sign at a lever where it is a point's
displacement, or decide it by its certified sign alone (as PR 5119 does
for the volume fallback).

## The volume backstop left the operation path (2026-10-27)

PR 5842, under `[ev]` PR 5780 (the boolean's volume relations are a test oracle, `topo::test_support::volume_relations`), deleted the backstop: `BooleanError::{ResultVolumeImplausible, VolumeUnmeasured, VolumeCorrupt, VolumeUndecided}`, `BooleanDecision::VolumeBackstop`, `AtRestPolicy::gate_volume_backstop` and the `volume_backstop*` predicates are gone. The backstop's V/A readers are gone with it (FLUSHTAIL's `the-volume-backstops-plus-v-posture-reads-a-sign-the-side-reader-cannot`, closed); the other sites stand.
