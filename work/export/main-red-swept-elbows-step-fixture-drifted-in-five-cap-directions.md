---
id: main-red-swept-elbows-step-fixture-drifted-in-five-cap-directions
kind: issue
title: main red (slow set): swept_elbow.step drifted in the last digits of five cap DIRECTIONs
status: open
opened: 2026-11-16
priority: P0
cost: E
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
