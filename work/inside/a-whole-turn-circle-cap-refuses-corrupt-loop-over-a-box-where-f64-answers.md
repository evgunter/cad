---
id: a-whole-turn-circle-cap-refuses-corrupt-loop-over-a-box-where-f64-answers
kind: issue
title: Point containment refuses a face bounded by one whole-turn circle as CorruptLoop over a box, where the same body answers at f64
status: open
opened: 2026-11-15
priority: P2
cost: M
---


Filed by the BOXFINISH orchestrator from the review of PR 6979 (the
curved edge's straddle). Reported by the reviewer as out of its fence
and `unsure` whether the behaviour is intended; not re-measured by the
orchestrator.

**What the review measured.** Bodies built through the public Euler
doors over a parameter box (a cylinder with a boxed radius or height, a
cone frustum with its apex or top boxed, a sphere cap with its radius
or plane boxed; boxes 1e-7 to 0.1; ε 1e-9) close each cap on ONE
whole-turn circle. For a ray or a point query that reaches such a cap,
`splitting::containment::loop_points` refuses
`Limit(Loop(CorruptLoop))` ("a whole-turn scaffold circle"), so the
cap's own trim (`point_in_face`) never runs over a box. Over 24,000 of
the review's seam-adjacent rays end there. At `f64` the same bodies
answer through `point_in_solid`.

**Why it is filed.**
- The name says the body is corrupt, and it is not: the body is valid
  and reads at `f64`
  (`work/onecopy/a-lone-loop-refuses-under-a-corruption-name.md` was
  the same shape for a lone loop).
- It hides what stands behind it. PR 6979's rule for a curved edge's
  straddle over a box is never exercised on a buildable body, because
  this refusal comes first: with that rule's gates opened by mutation,
  no row and no probe reads a wrong side. The first unit that lets a
  whole-turn cap be read over a box removes the refusal that today
  holds those rays.

**Not established:** whether a whole-turn circle is refused on purpose
over a box (no seam vertex to start the walk from at an enclosure), and
which shipped documents reach it. A two-half-arc cap, the shape the
sweep doors build, does not build by hand over a box (`mef` refuses its
`EndpointEnd` certification on the boxed radius), so the review could
not compare.

**Where to look:** `crates/topo/src/splitting/containment.rs`
`loop_points`; the review's probes, `crates/topo/tests/review_curved_straddle_probes.rs`
on branch `boxfinish/review-curved-straddle` (commit `ec9035f4`).
