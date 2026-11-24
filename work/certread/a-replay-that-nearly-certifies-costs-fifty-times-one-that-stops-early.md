---
id: a-replay-that-nearly-certifies-costs-fifty-times-one-that-stops-early
kind: issue
title: An interval replay that gets past its first indeterminacy costs 10x to 50x one that stops early, certified or not
status: open
opened: 2026-11-22
priority: P3
cost: M
---


Found by OFFER's kernel certify unit (`work/offer/the-certified-range-refines-away-from-the-current-value`) while re-taking the per-replay cost for `editor_core::range`'s module docs.

## Finding

One replay of a corpus document under the E6 leaf protocol (`drive::classify`, default `SymbolicDials`) costs seconds while the replayed interval is wide, and ten to fifty times that once the interval is narrow enough for the replay to get past its first indeterminacy — whether or not it then certifies.

Measured per replay, dev profile, 4-vCPU x86_64 cloud container with a second build sharing the cores (`docm9_range::the_per_replay_cost_on_the_corpus`, ignored, prints the rows; the plate's long column is from the same search run to its first proof):

| document | field | interval refuses early | undecided, wide | nearly certifies, or certifies |
| --- | --- | --- | --- | --- |
| `corpus::plate_param` | `hole_r` | 0.1 s | 1.4–4.5 s | 21–24 s |
| `corpus::die` | `pip_depth` | 0.2 s | 2.6 s | 115–132 s (none certified in 8 replays) |
| `corpus::die` | the cube's `Distance` slot | 0.4 s | 1.8–2.1 s | 26–42 s (none certified in 8 replays) |

On the plate the step from 1.7 s to 22 s happens two halvings BEFORE the width that certifies (about `ε`, where `ε/4` certifies), so eight replays at 22 s each decide nothing.

## Why it matters

The certified range search (`range::RangeSearch`) spends its replays near the current value, which is exactly where the expensive ones are: on the die's `pip_depth` every replay after the third costs two minutes. The viewer's `range?` (`work/offer/certify-affordance-on-the-bounds-panel`) runs that search on a user's behalf.

## Not established

Where the time goes. The candidates are the symbolic tier's retry ladder (`sym::with_session_memo_retry`) re-running a replay that froze, and the at-rest finish a replay reaches only once its early predicates are definite; neither was profiled. The numbers are from one box under load and are a ratio, not a benchmark.
