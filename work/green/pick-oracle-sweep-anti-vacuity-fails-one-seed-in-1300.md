---
id: pick-oracle-sweep-anti-vacuity-fails-one-seed-in-1300
kind: issue
title: review_gui1_r1's pick-oracle sweep asserts a hit it never plants: about one seed in 1300 draws 120 misses and goes red
status: closed
opened: 2026-10-21
priority: P3
cost: E
closed: 2026-10-28
---


## Finding

`crates/editor-core/tests/review_gui1_r1.rs`,
`random_integer_rays_match_the_exact_oracle`, draws `fuzz::scaled(120)`
integer rays: origins in `[-3, 3]³`, directions in `[-2, 2]³`, against the
unit cube `[0, 1]³`. It then asserts `hits_seen > 0` ("no draw hit the
cube — generator shape broke"). The comment above that assert says the
anti-vacuity is structural, but nothing in the generator guarantees a hit.

Counted exactly over all 7³·5³ = 42875 draws (boundary grazes counted as
hits), 2486 hit the cube: p ≈ 0.058 per ray. At `CAD_FUZZ_EFFORT=1` all 120
miss with probability (1 − p)¹²⁰ ≈ 7.7e-4, about one seed in 1300. Each such
seed is red on every tree.

Witness: `CAD_FUZZ_SEED=0xcf585a26ea39e4b1 CAD_FUZZ_EFFORT=1` fails at
`review_gui1_r1.rs:610` on `main` at 8bcf47b2, and on #5531's head. #5531's
CI drew it.

## Fix

Plant the hit instead of hoping for it. Make case 0 aim at the cube: any
drawn origin, direction `1 − 2·o` per axis (toward the centre, still
integer). The oracle comparison then runs on a hit every sweep, and the
assert becomes the structural claim its comment says it is. The rest of the
sweep stays random.

## Closed (2026-10-28)

Fixed as the row prescribes. `random_integer_rays_match_the_exact_oracle`
still draws case 0's origin, then overwrites its direction with `1 − 2·o`
per axis: an odd, never-zero integer in `[-5, 7]` that reaches the cube's
centre `(½, ½, ½)` at `t = ½` from any drawn origin. The rest of the
stream is drawn exactly as before, so a seed replays the same cases 1…n.
`hits_seen > 0` is now structural: it can only fire if the exact oracle
misses a ray through the centre, and its message says so. Distinct
rational `t`s on the planted ray differ by at least 1/49 (denominators are
the axis components, `|d| ≤ 7`), so the ≥ 1/64 separation the face
comparison rests on still holds.

Witnesses, run against the `editor-core` test binary with
`CAD_FUZZ_EFFORT=1`: `0xcf585a26ea39e4b1` (this row), `0x2870e278e5a1ef24`,
`0x1a9e0f26198e881b`, `0xf30b717118986019`, `0x9c7901b099374603` and
`0x47471a5e48e72b4c` (the passengers) each failed "no draw hit the cube"
before the change and pass after. `0x2870e278e5a1ef24` and
`0x47471a5e48e72b4c` also pass at `CAD_TOLERANCE_EPS=1e-6`.

Both passengers close with this row.
