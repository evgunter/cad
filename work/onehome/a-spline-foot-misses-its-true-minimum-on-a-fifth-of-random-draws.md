---
id: a-spline-foot-misses-its-true-minimum-on-a-fifth-of-random-draws
kind: issue
title: Curve3::foot on a spline misses the point's nearest point by more than 1e-9 on about a fifth of random draws: six Gauss–Newton steps from four samples a knot span
status: closed
opened: 2026-11-14
closed: 2026-11-17
priority: P2
cost: M
---


Found by ONEHOME's chord-join-cost lane (2026-11-14), measuring a spline foot held to an interval against the whole-domain one (branch `onehome/foot-within-probe`, `crates/geom/src/curves/on_carrier/rows.rs` `a_spline_foot_held_to_an_interval_is_the_whole_foot_where_that_lies_inside`).

**What the foot is.** On a spline, `Curve3::foot` (`crates/geom/src/curves/on_carrier.rs`) samples `SPLINE_SAMPLES` (four) on every knot span and the domain's ends, then polishes by Gauss–Newton (`POLISH_STEPS`, six) from the seed, the samples' local minima and the `LEAST_SAMPLES` least gaps, each step kept only where it decides the gap smaller. Its module docs call the result a polished stationary point, not the nearest point, and a foot certifies nothing (`Foot::seed`).

**The measurement.** Over 160 drawn splines (degree 2 to 5, 6 to 40 control points, uneven knots, a third rational) and eight points drawn within 0.3 of each, against a dense oracle (the least gap over 1500 even steps, refined by a ternary search on its bracket): of 1227 points whose nearest point lies at least a hundredth of the domain clear of the ends, the whole-domain foot reaches the oracle's gap within `1e-9` on 980 and misses it on 247 (20%). A miss is a start on a steep stretch whose Gauss–Newton step overshoots, is not kept, and stops the polish short; two searches from different starts were seen to stop as far as about `1e-2` metres of gap apart. With 400 splines and a 4000-step oracle the rate is the same (2444 of 3071 reached).

**Who reads it as a nearest point.** No reader found here certifies the foot as the nearest point: the curve-pair walk's stop, ends and witnesses read it as a seed under a window (`crates/geom-brep/src/curve_pair/walk.rs`), `Span::place`/`Span::read` read the span's certificate at it (`crates/geom-brep/src/curve_pair/span.rs`), and `Foot::nearest` is `None` on a spline. But the stop's verdict depends on which foot it polishes (`work/onehome/the-curve-pair-walks-stop-verdict-rests-on-which-heuristic-foot-it-polishes.md`), so a foot that reached the nearest point more often may move verdicts too. P2 unless that row finds a verdict that needs the true minimum.

**Owed.** Decide whether the spline foot should reach the nearest point (a damped or bracketed polish, more steps, a certified interval minimum), and measure its cost and the verdicts it moves first.


## Closed (ONEHOME, fork 275 step 3b, 2026-11-17)

Decided the other way: the spline foot does not seek the nearest point. Fork 275 (PR 7025) retired the sampled search, and its step 3b leaves a foot as Gauss–Newton from the caller's seed over the interval it is read over (`Curve3::foot_within`). No reader rests on the foot being nearest. `On` rests on the gap and `Off` on the whole-carrier walk, and where the seed's gap does not decide zero the polish restarts once from the walk's first kept piece. The one reader that did rest on nearness, `pair_section::CarriedComponent::reads`, ordering sites by the foot's parameter, now reads its foot over the tube pieces the site lies in. The stop's verdict no longer reads a foot (PR 7048).
