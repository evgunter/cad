---
id: pierce-germ-direction-within-is-levered-at-the-sector-arm
kind: issue
title: The pierce germ direction's within test is levered at the sector's shorter arm, so a transition sector the germ-line gate reads at its reach refuses in band
status: open
opened: 2026-09-29
priority: P3
cost: E
---


Filed by CONTACT-9 (review MINOR-3).

In CONTACT-9's fix pass, a pierce's coplanar lump reads the sector's
bounds before an in-band parallelism escalates. A bound read definitely
off now decides, and the sector is not lumped (`vtxfac.rs`, Delta 2).

**Witness.** The review's P1 at 5·10⁴·ε: the wedge on the block's top
at `(5, 5, 0)`, with edges `a = (10, 1, d)`, `b = (0.2e-3, 1e-3, 0)`
and `c = (1e-4, 1e-4, -1e-3)`.

It used to refuse at `bool_sector_coplanar`, in band. It now refuses
one step later at `bool_sector_within`, margin −5.0e-9, in band: the
pierce germ direction's `within` test (`vtxfac.rs` `pierce_germ_dir`,
`sectors::within`), which levers the direction's angle to the sector's
bounds at the sector's shorter arm. The germ-line gate beside it is
levered at the sector's reach (`BoolSector::span`), so the gate and the
direction test read the same geometry at two scales. The same pose
with 1 m edges answers.

**The fix's shape.** Read the germ direction's membership at the
bounds' reaches: the metric twin of the germ-line gate.

**The same lever, seen from the polygon-cone reader** (TANG, PR 4289).
`sectors::within` levers a direction's angle past each bound at the
sector's arm, the shorter of its two chords, whichever bound it is
compared against. By D4 the reading past a bound `u` flips at the least
move of the points it reads: the direction's far point, at its reach,
or `u`'s far point, at `u`'s own reach. So its lever is
`min(direction's reach, L_u)`, per bound.
- PR 4289's first fix pass levered the polygon-cone reader's own
  `in_sector` at the direction's reach alone. That dropped the bound's
  term, and was the defect review 3 found: beside a 1 mm bound it
  decided a class that a 3e-10 m move of that bound's far point flips.
- Its third fix pass levers each bound comparison at
  `min(direction's reach, L_u)` (`sectors::least_lever`).
- The witness there (seed 1, cone 85, probe 64) passes a 0.5 m bound on
  a sector whose other chord is 1 cm, so `within`'s arm lever reads it
  `Zero` where the bound's own reach decides it. `within` itself, and
  its other callers, are unchanged; the shape above is the same
  per-bound lever.
