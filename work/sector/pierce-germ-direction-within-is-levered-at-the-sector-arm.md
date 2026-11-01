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

## Evidence: the pierce reader no longer reads `within` (branch `cleave/near-tangent-invariants`, 2026-10-10)

`vtxfac.rs` `pierce_germ_dir` now signs the germ line by the
transition's own codes (`OutBound`: which of the sector's bounds the Out
run holds) and reads no membership at all, so its `bool_sector_within`
escalation is gone, along with the `ClassificationInvariant` both-or-
neither arm CLEAVE's census reached 534 times over three ε rows. On
JOIN's near-tangent census that took 735 runs at 1e-9 (393 at 1e-12,
309 at 1e-6) from a `Coincidence(Sectors)` escalation to sound bodies,
and 483 more at 1e-6 to bodies within 2.2e-7 m³ of the oracle's volume
(the census's line calls them BAD at 1e-7). Of those 483, 231 fail
tier 3′ on in-band census escalations only, and 252 pass it. The review's 5·10⁴·ε
wedge witness above is unmeasured on that branch. `sectors::within`
and its other callers (`pair_search`, `insert::germ_dir`, the
polygon-cone reader) are unchanged, so the per-bound lever owed here
still stands for them.

## Re-read (2026-10-26, tracker sweep)

The pierce half is on main. `vtxfac::pierce_germ_dir`
(`crates/topo/src/boolean/vtxfac.rs:2774-2834`) signs the germ by
`OutBound` and reads `sectors::within` only in a debug assertion. What
this row still owes is the per-bound lever in `sectors::within` itself:
`within_signs` levers both bound readings at `s.arm`
(`crates/topo/src/boolean/sectors.rs:1266-1290`), against
`sectors::least_lever` (`:2341`). That serves its remaining callers:
`pair_search`, `insert::germ_dir` (`crates/topo/src/boolean/insert.rs:2181`,
`:2233`) and the polygon-cone reader.

