---
id: cone-reads-given-reference-loop-waits-on-its-unsettled-classes
kind: issue
title: link::Cone::reads_given's reference loop hand-rolls the ladder, and waits on its Unsettled variants being classed
status: closed
opened: 2026-11-05
priority: P2
cost: M
closed: 2026-11-09
pr: 6637
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
`Result<ladder::Refusal, ladder::Aborts>`, since `Contradicted` aborts).

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
| `Contradicted` | `Err(Aborts)` (`reads_given`'s own; no reference read meets it) |

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

## A second site on the same classes: `Cone::place` (2026-11-06)

`link::Cone::place` (`crates/topo/src/link.rs`, the wedge loop keeping
`in_band.get_or_insert(why)`) is the same type's second hand-rolled
ladder, found by `kept-refusal-loops-the-ladder-row-never-listed`'s sweep
(`onecopy/refusal-standing`). `Place::Inside` answers; `Bound` and
`Outside` pass, `Bound` remembered beside the climb; every
`Unsettled<T>` is kept first-of-any-class and refuses ahead of `Bound`.
Its port is the same port: once `Unsettled::refusal` is ruled, `place`
climbs under `Any` (one wedge holding the point inside answers for the
cone), and the first-of-any-class order becomes `Evidence`'s. It moves
outcomes exactly where `reads_given`'s does: two kept classes that rank
differently.

## Resolved (2026-11-09, `onecopy/refusal-standing`)

**The classes** (orchestrator ruling (A), 2026-11-09), stated once on the type, `link::Unsettled::refusal() -> Result<Refusal, Aborts>`, exhaustive, each variant's doc citing its `[failure.taxonomy]` row:

| variant | standing |
|---|---|
| `InBand(Indeterminate)` | `Refusal::Margin` |
| `Zero` | `Refusal::InBand` |
| `Slack` | `Refusal::InBand` |
| `Unplaced` | `Refusal::InBand` |
| `Contradicted` | `Err(Aborts)` |

`Unplaced` is in band because each of its three arms in `Cone::crossing` is a band decision: an end read zero on the sector's plane, a stretch the band places the crossing on that does not decidedly miss the sector, or a crossing `Sector::place` reads on a bound. A tighter band may part each, as it may `Zero`. An arc running exactly along the plane is read the same way, and is not split out. `Unread::refusal` delegates to its reading's standing. Where no reference read, it is split by standing (PR 6637's reviews, F3 and the delta's item 1): `NoReference` is a limit (the cone has no wedge, or every reference's arc decidedly meets the apex), and `ReferenceInBand` is in band (a plane `q` reads in band of, a side the cone's own reading did not decide, or a reference refused out at its reach). `ArcReach::clears` returns a clearance the band reads zero as `Unsettled::Zero`, in band, so the limit is exactly the decided case. `recl`'s line reads the limit as `LineUnread::NoReference`, a limit, and `ReferenceInBand` as `LineUnread::Unplaced`, through `recl::line_unread_of`; `cone_fuzz` tallies the two apart.

**The ports:**
- `Sector::place` (the row's `Cone::place`) climbs its wedges under `Any`: inside answers, `Bound` passes and is remembered beside the climb, and a reading in band is kept by `Unsettled::refusal`. It refuses ahead of `Bound`, as before.
- `Cone::reads_given` climbs every reference under **`First`**: the `Near` reach first, then the `Out` reach, one lazy chain. A `Near` refusal is kept by `Unread::refusal`; an `Out` refusal passes; an arc decided to meet the apex passes. Not `Any` (PR 6637's review, F1): "every reference reads one material" is a premise about the input, shown failing on `work/oneread/a-cones-parity-reference-is-off-its-sectors-plane-by-the-tie.md`, and `editor_core::drive`'s `classify_replay` acts on the escalation log a superseded splice would drop. Its flip to `Alike` is on `side-at-infinity-aimed-rays-disagree`.

**Reach**, by a temporary probe in `climb_evidence` (not committed) over topo, geom-brep, sweep and step-import `ci` (8,919 tests): 432 answers given past a kept `Near` refusal, all at `reads_given`, led by `link::cone_fuzz` 144, `link::oracle_rows` 99, `census::tests` 43 and `shell_role_reader` 40. `Sector::place` answered past nothing kept. The old loop answered past the same refusals too, and under `First` the kept runs stay live as they did, so **neither outcome nor record moves**. **No ranking moved:** no refusal whose class differs from the first kept one was named in any test, at either site.

**Pinned:** `link::standing_rows` (each class as its doc states, `Zero` and `InBand(diag)` included). The ranking move has no fixture: no suite reaches two kept classes that rank differently.

**Which answering each is:**
- `Sector::place`'s `Any` is a theorem of the rule: the wedges are the sector's parts, so one holding `x` inside holds it for the sector.
- `reads_given`'s premise is about the input, so it climbs under `First` until [ev] PR 6592's `Alike` lands.
