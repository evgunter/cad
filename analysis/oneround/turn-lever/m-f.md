# M-f: what a door decided mid-construction against what tier 3 decides at rest

## What was computed, and against what

Each `edge_wedge_over_as` and `must_carry_over_edge` verdict was keyed by its
inputs — the two surfaces (order-free for must-carry), the faces' senses, the
carrier and the span — and recorded once per test as decided by a door and
once as decided inside tier 3 (`tier3_local_checks_marked`). A key read by both
compares the two verdicts' actions. No truth: a consistency count, target zero.
Coverage as M-c (ε = 1e-9).

## Result

| Composite | both, same action | both, different | door only | tier 3 only |
|---|---|---|---|---|
| `edge_wedge_over_as` | 174,499 | **0** | 32,768 | 650,097 |
| `must_carry_over_edge` | 0 | 0 | 68,114 | 0 |

The wedge fold: no edge a door read identically to tier 3 was decided
otherwise.

The must-carry rule is **not measured**: tier 3's must-carry re-ask runs
outside the scope this instrument marks as tier 3, so its calls were recorded
as door calls and no key reached both sides. Measuring it needs the mark moved
to wherever validate calls the re-ask.

"Door only" and "tier 3 only" are reads whose inputs differ between the door
and the finished body (a span or carrier the door read before the edge took
its final form, or edges tier 3 reads that no door touched); they are not
compared, and an edge whose door read and final read differ in input is
exactly the case this count cannot see.
