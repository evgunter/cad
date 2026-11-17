---
id: main-red-swept-elbows-step-fixture-drifted-in-five-cap-directions
kind: issue
title: main red (slow set): swept_elbow.step drifted in the last digits of five cap DIRECTIONs
status: closed
opened: 2026-11-16
priority: P0
cost: E
closed: 2026-11-17
---


`step-export::all export::committed_fixtures_are_byte_golden`
(`crates/step-export/tests/export.rs:442`) is red on main at `253d3bbd58`.
It was measured there on a checkout of main, 2026-11-16, while ONEHOME ran
its workspace suite. This row records the red; it does not fix it.

    cargo nextest run -p step-export -E 'test(committed_fixtures_are_byte_golden)'

It fails at `swept_elbow.step drifted — regenerate deliberately`. Both
texts are 260 lines, and five lines differ, every one a cap
`DIRECTION` and only in the last digits of a component of size 6.1e-17:

| entity | written now | committed |
| --- | --- | --- |
| `#3`, `#19` | `(1.0, 6.123233995736766E-17, -6.123233995736761E-17)` | `… -6.123233995736765E-17)` |
| `#33` | `(-1.0, -6.123233995736766E-17, 6.123233995736761E-17)` | `… 6.123233995736765E-17)` |
| `#11` | `(6.123233995736762E-17, -1.5700924586837754E-16, 1.0)` | `(6.123233995736766E-17, …)` |
| `#27` | `(-6.123233995736762E-17, 1.5700924586837754E-16, -1.0)` | `(-6.123233995736766E-17, …)` |

No point and no edge moved. The fixture was last written by `10f862ba3c`
(2026-11-15, the slow set's head-only re-cut, "STEP fixtures at the caps'
placements"), so whatever moved it landed after that and was not
bisected. The test stops at the first fixture that drifts, so a later
fixture may drift behind this one: regenerating every fixture
(`cargo run -p step-export --example export_fixtures`) shows all of them.

## Done when

The test is green on main. The move has been traced to the change that
caused it and judged rounding (or not), and the fixture has been
regenerated deliberately.

## Closed (2026-11-17)

Measured again at `origin/main` `362cd72fd2`: the same five lines, and
regenerating every fixture moves `swept_elbow.step` alone. Of the five,
`#3` is the end cap's plane reference direction and `#11`, `#19`, `#27`,
`#33` are the directions of that cap's four edge lines; each moved by
3 ulps in one component of size 6.1e-17 (3.7e-32 absolute).

**Cause: the station step's least turn took its W1 spelling.** PR 6988
(`onehome/least-turn-mint`, merged as `034c11e242`) moved the sweep's
station step (`PathCarry` in `crates/sweep/src/skin.rs`) from
`RigidMotion::turned_between`, the quaternion matrix over `n²`, onto
`RigidMotion::least_turn_selected`, which spells a pair whose `a·b`
reads non-negative by Rodrigues (`least_turn_rodrigues`, `ad0da66ac6`)
and keeps the quaternion matrix near the pole. Every station step of the
elbow is far from the pole, so it now takes Rodrigues, and the end
station's reference carries different rounding. Proven by A/B on
`362cd72fd2`: forcing `least_turn_spelled` to the quaternion arm writes
all seventeen fixtures back byte for byte.

**Judgment: rounding.** Both spellings are the one turn over the reals
(geom-core README W1: a few ulps at f64 decides nothing); the moved
component is zero over the reals, left by the path's turn by
`FRAC_PI_2`; the reference's dot with the cap normal is −9.6e-33 before
and after. Fixture regenerated, and the move recorded on
`committed_fixtures_are_byte_golden`, in `a25315379b`.
