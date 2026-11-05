---
id: cone-reads-given-reference-loop-waits-on-its-unsettled-classes
kind: issue
title: link::Cone::reads_given's reference loop hand-rolls the ladder, and waits on its Unsettled variants being classed
status: open
opened: 2026-11-05
priority: P2
cost: M
---


`topo::link::Cone::reads_given`'s reference loop
(`crates/topo/src/link.rs`, the `for arc in [ArcReach::Near, ArcReach::Out]`
loop) is the ladder's `Any` (every reference reads one material), hand
rolled. It keeps the first refusal at the `Near` reach, in read order,
and drops one at the `Out` reach. Each rung is a conjunction: a
reference's clearance, then its crossing of every other sector.

A port onto `geom_brep::ladder::climb`:
- folds each reference's sector reads into one rung;
- turns an `Out`-reach refusal into `Rung::Passes`, or chains two climbs
  with `Evidence::then`;
- ranks the `Near` refusals by `Evidence`, not by read order.

That last needs `link::Unsettled` to state each variant's standing
beside its definition (an `Unsettled::refusal` returning
`ladder::Refusal`).

**The classes are to be ruled from what each variant means, before the
port, and `Unplaced` most of all.** It may be a limit (a reading not
built: the parity arc runs along a plane, or meets it on a bound) rather
than in band. The starting position, from `onecopy/ladder-climb-3`:

| variant | proposed class |
|---|---|
| `InBand(Indeterminate)` | `Refusal::Margin` |
| `Zero` | in band: a tighter band may separate it |
| `Slack` | in band |
| `Unplaced` | in band: **contested**, possibly a limit |
| `Contradicted` | `Refusal::Abort` (`reads_given`'s own; no reference read meets it) |

Reach, measured with probes at `onecopy/ladder-climb-3` (merge base
`f4f6a24a`):

| suite | answered past a kept `Near` refusal | a second `Near` refusal kept |
|---|---|---|
| topo + geom-brep, `ci` set (5,709 tests) | 497 | 2,363 |
| sweep + step-import, `ci` set (3,164 tests) | 15 | 20 |

So a port under `Any` supersedes about 500 kept runs per suite (the
ladder's intended record change), and the ranking change is reachable
wherever two `Near` refusals of different classes are kept. The hits are
led by `link::cone_fuzz`, `census::tests::a_slit_reads_degenerate` and
`link::oracle_rows`.
