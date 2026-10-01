---
id: witness-midpoint-and-the-schedules-middle-station-are-two-spellings-of-one-point
kind: issue
title: WitnessMidpoint and the certification schedule's middle station evaluate the same carrier point under two spellings, so the sym walk builds both chains
status: review
opened: 2026-10-01
priority: P1
cost: E
pr: 3697
branch: cleave/schedule-mid
---


(cleave-sym-ledger lane, 2026-10-01, measured on main 8e19eb07-era with PR 3645 merged.)

`geom-brep/src/certify.rs` `schedule_param` spelled its interior stations
`t₀ + (t₁ − t₀)·f`, the middle one included, while PR 3645 moved the
WitnessMidpoint check (`spec.carrier.mid_point(t0, t1)`, `certify.rs`
~2673) and the witness `sweep/src/extrude.rs` `upgrade_rim` emits onto
`geom::mid_param`, `(t₀ + t₁)·½`. Before 3645 both read
`sample_param(t0, t1, (CERT_SAMPLES - 1) / 2)`, one node. Since, a
certificate evaluates the carrier at its middle twice, under two
spellings, and the symbolic walk builds both chains.

Measurement (`editor-core` `m10_sym_profile_interval::the_forms_the_walks_build_are_pinned_per_eps_row`,
`#[track_caller]` logging on `mid_param`: only those two sites reach it):

- main after 3645: slab Plain/Decision forms 9852 at all three ε rows;
  plate Plain/Decision 15086 frozen 720, Plain/Assertion 2941 frozen 420,
  Early/Decision 7973, Early/Assertion 3753 frozen 112, Door/Decision
  12012 frozen 112.
- the same tree with the schedule's middle station read from
  `mid_param`: slab 9426; plate 14609 frozen 672, 2594 frozen 372, 7741,
  3406 frozen 104, 11550 frozen 104 — the pre-3645 counts (Assertion
  +1, Door +2), digests moved.
- `m10_9_pins` / `m10_10_pins` decision pins pass both ways: no decision
  moved, only what the walk builds and freezes.

Fix: `schedule_param` assigns the station at fraction ½ from
`geom::mid_param`, as it assigns the ends.
