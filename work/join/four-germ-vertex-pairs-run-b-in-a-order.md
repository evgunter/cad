---
id: four-germ-vertex-pairs-run-b-in-a-order
kind: issue
title: A vertex pair with four crossing germs runs its B null edges in A's germ order: 41 reflex-probe poses refuse and one union ships a wrong body
status: parked
opened: 2026-10-02
priority: P0
cost: H
refs: [a-flush-declared-reflex-union-ships-the-wrong-volume, reflex-corner-vertex-vertex-sites-refuse-under-a-tilted-cap]
blocked_on: [3990]
---


(Found by the reflex-corner lane, PR on `join/reflex-corner`, on
JOIN-1's head 0b6e39ca with the strut-order fix applied. Release build;
`crates/sweep/tests/join1_r1_probes.rs` `join1_r1_reflex_battery`, one
pose at a time by `join_rc_probes::rc_detail`.)

## What

`insert_null_pairs` (`boolean/insert.rs`) pairs a vertex pair's
surviving germs consecutively in A-major order, guards that each pair
is also cyclically adjacent in B's order (F12 guard 1), and then mints
each pair's null edge in BOTH solids with the run taken forward
`r0 → r1` in A's order, swapped only when that run would swallow the
whole orbit (`mint_directed`, `run_degenerates`). With two germs that
is enough. With four, B's forward run from `r0` can be the long way
round B's vertex: it then holds the other pair's germs, and the two B
runs overlap.

Where: `a`'s 315° reflex corner `(0, 0, 1)` with `b`'s wall flush on
the corner's notch wall (the profiles `sqQ1`, `dRight`, `eBot`,
`eLeft`) and the cap tilted down over the notch (`sx < 0`). The corner
then keeps four germs: the cap across the 45° wall, the cap or a `b`
wall across `a`'s top, and the two bounds of the flush patch on the
notch wall (or, for `eLeft`, the cap across the 0° wall).

## Measured

Every pose of the probe that does not build sound on the strut-fixed
head (∩, ∪, `a ∖ b`) is in this class, and no sound pose is: an
instrumented run that reports whenever B's own cyclic order and the
mint disagree on a pair's direction fires on all 42 and on none of the
822 sound or empty ones.

| profile | ops | (sx, sy) | outcome |
|---|---|---|---|
| `sqQ1` | ∪ | sx ∈ {−0.5, −0.25}, sy ∈ {−0.5, −0.25, 0} | `Euler(FanStartMismatch)` (6) |
| `sqQ1` | ∪ | (−0.5, 0.25) | **volume 16 against 15.979** (`work/zip/a-flush-declared-reflex-union-ships-the-wrong-volume`) |
| `dRight` | ∪ | 6 poses at sx < 0 | `Euler(FanStartMismatch)` |
| `dRight` | ∪ | (−0.25, −0.5) | `JoinDesync` "B senses agree at a matched pair" |
| `eBot` | ∪ | sx < 0, sy < 0 | `Euler(FanStartMismatch)` (4) |
| `eBot` | ∪ | (−0.5, 0), (−0.5, 0.25), (−0.25, 0) | `JoinDesync` "B senses agree …" |
| `eLeft` | ∩ ∪ ∖ | sx < 0, sy ≠ 0 (5 shears) | `JoinDesync` "B senses agree …" (15) |
| `eLeft` | ∩ ∪ ∖ | sy = 0 (2 shears) | `JoinDesync` "every chord arc separates a loose scaffolding pair" (6) |

Traced by hand at two poses, `sqQ1 (−0.5, 0) ∪` and
`eLeft (−0.25, −0.25) ∩`: A's runs are right; in B one run (sqQ1) or
both (eLeft) are the complement of the wedge. At sqQ1 the second B
run's fan, `[+z edge, cap edge]`, already lost the cap edge to the
first run's copy, and `mev_fan_plan` refuses `FanStartMismatch`.

The wrong body: the join refuses `JoinDesync` "B senses agree at a
matched pair", and `rest::try_rest_union` answers the declared union
from the same reduction (`ops::through_the_join`), with the overlap
counted twice. The insertion is the first step that goes wrong there
too; the REST zip admitting a pose that is not a pure REST contact is
the second, ZIP's.

## The fix, and why it does not land alone

Take each solid's run direction from that solid's own cyclic order of
the survivors: the run starts at the germ the other one follows, so it
holds no third germ; with two survivors each follows the other and the
orbit-swallowing test stays the tie-break.

```rust
fn run_order(n: usize, p0: usize, p1: usize) -> Option<bool> {
    match ((p0 + 1) % n == p1, (p1 + 1) % n == p0) {
        (true, false) => Some(false),
        (false, true) => Some(true),
        _ => None,
    }
}
// mint_directed(.., order: Option<bool>, ..):
//   swapped = order.unwrap_or(run_degenerates(..)?)
// A: run_order(n, i0, i1); B: run_order(n, b_pos(i0), b_pos(i1))
```

With it applied, the two hand-traced poses mint the right wedges, and
the 42 poses go further and stop later:

| outcome with the fix | poses |
|---|---|
| `Euler(SelfLoopEdge)` in `zip::fuse_by_joint` (every ∪ but `eBot`'s) | 19 |
| `JoinDesync` "a section vertex's null-edge copies have not exactly one kept end" (`finish::discarded`, `eLeft` ∩) | 6 |
| `JoinDesync` "conflicting seam vertex correspondence" (`finish`, `eLeft` ∖) | 6 |
| `JoinDesync` "B senses agree at a matched pair" (`eBot` ∪ at sy ≥ 0, `eLeft` at (−0.5, 0.25)) | 6 |
| `RestZipUnsupported(ChordBetweenIsolatedPierces)` (`eBot` ∪) | 2 |
| a wrong volume through the REST zip (`sqQ1 (−0.5, 0.25)`, `eBot (−0.5, −0.25)`, `eBot (−0.25, −0.5)`) | 3 |

Two poses move from a refusal to a wrong body, so the fix waits on the
REST zip refusing a union that is not a pure REST contact (ZIP's row
above). The finish rows are the corner vertex holding two null edges
at once: `discarded`'s `kept_end` maps a vertex to every copy across
its null edges and needs exactly one survivor.

Also unmeasured: B's order sorts survivors by `(b, a)`, so two germs
in one B sector are ordered by their A sector rather than by angle.
It was right at both traced poses; nothing checks it.

## What the taker owes

The run-order fix with the four downstream stops above, each traced
to where it starts, and the probe's 42 poses building sound at tiers
2, 3′ and the certificate with the closed-form volume, no pose of
the JOIN-1 batteries moving from sound or to a wrong body.
