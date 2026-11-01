---
id: three-readings-of-a-points-rounding-charge-it-three-ways
kind: issue
title: A conic point's rounding is read three ways: carrier_resolution (relative to the window), implicit::sample_rounding and ring_path::resolution (absolute)
status: open
opened: 2026-10-14
priority: P1
cost: M
design: true
---


Found by the review of PR 4810 (Q1).

## What

Three functions bound the rounding of a point on a carrier, each its
own way:

- `geom_brep::carrier_resolution::resolution` (PR 4810): a section
  carrier's own resolution over a parameter window, charged relative to
  the window (`u·speed_hi·(32·reach + τ)`), leaving the coordinates'
  rounding at the model's extent to the band's `K` headroom (Q1). The
  join's pin reading admits on it.
- `geom_brep::implicit::sample_rounding`: one sampled residual's
  rounding over a whole conic, charged at `|C₀| + speed_hi`, the point's
  distance from the world origin.
- `topo::ring_path::resolution` (same name): `4ε` times the farthest
  stored point's distance from the origin, for a reading between points
  that should coincide.

They answer neighbouring questions: what a carrier amplifies, and what
coordinates resolve at the model's extent. But nothing says which a
reader wants, and two of them charge from the world origin.

## Owed

One home that names the two terms (the coordinates' floor, the carrier's
amplification) and lets each reader take the ones it needs. Rename
`ring_path::resolution` so one name means one quantity.


## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Design signal.** **Cluster C, the scale a margin is charged at.** Ev (2026-10-26): every point of the representation must be within ε of the real shape, so a decision's charge should be derived from that one contract, not chosen per site. The three readings disagree: `carrier_resolution` leaves the coordinate rounding to K, while `implicit::sample_rounding` and `ring_path::resolution` charge it explicitly. Unclear: whether that floor is always K's job, and where the floor and the amplification terms are defined once. `[verdicts]` presumes K covers it, and no canonical-places row says so.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P1: three functions charge one rounding three ways: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
