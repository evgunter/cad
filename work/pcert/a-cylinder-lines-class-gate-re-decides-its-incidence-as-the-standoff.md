---
id: a-cylinder-lines-class-gate-re-decides-its-incidence-as-the-standoff
kind: issue
title: On a cylinder, a line's class gate reads incidence and then decides the same wall gap again as the norm_gap standoff
status: closed
priority: P1
cost: M
opened: 2026-11-10
closed: 2026-11-10
---


Filed by the PCERT main-reds lane (PR 6528, review fix EE10).

## What

`derive_class`'s cylinder arm (`crates/geom-brep/src/pcurve_cache.rs`)
reads the carrier's incidence first
(`incidence::incident(carrier, surface, place, band)`), which decides
how far the line stands off the wall. For a line it then decides the
class gate's margin as `ruling_miss(...) + standoff`, where
`standoff = norm_gap(radius, radius², w_r)` is the origin's radial
distance from the wall: the same wall gap, decided a second time. PR
6504 added that second read (its fix AB1), so that a Zero bounds the
meridian form's whole miss.

It is two readings of one fact (P1, the definition). On a rigidly
copied body the fact is a value identity the Sym rules do not reduce,
so it costs the D-tab 4 extra numeric decisions
(`pcurve_chart_radial_moving [22, 20] → [18, 24]`,
`work/sym/a-rigidly-copied-cylinder-walls-incidence-identity-has-no-sym-rule.md`).

## Fix

The class gate's margin reads the standoff from the incidence read's
own margin (`shown.margin()`), not from a re-decided `norm_gap`. A Zero
then still bounds the whole miss, because the incidence read already
decided the standoff. Keep the mutant `meridian-standoff-unread` red.
Re-pin the D-tab row (expected back at `[22, 20]`).

## Resolution: two facts, not one

Closed with no fold. The standoff and the incidence read are different
facts, and the class gate keeps reading the standoff.

- **Where.** The standoff is the gap at the meridian form's anchor, the
  line's origin. It is check 4's `Radius` term for the form
  (`pcurve_cache::incidence`, `EnvelopeTerm::Radius`), and the class
  gate reads check 4's two terms so that a Zero certifies. The incidence
  read is the gap's supremum over the span. When the span holds the
  origin, the span's supremum bounds the gap at the origin. When it does
  not, it does not: a line leaning radially at `δ = 0.45·zero` that
  crosses the wall at `t = 1.5`, read over `[1.3, 1.7]`, stands `0.2·δ`
  off over the span but `1.5·δ` off at its origin. Folded, the gate
  reads `1.9·δ = 0.855·zero`, a Zero, takes the meridian form, and that
  form's certificate escalates at `pcurve_envelope` on `1.44·zero`
  (executed). Pinned as
  `ruling_reading_rows::a_line_whose_span_holds_no_origin_reads_its_origins_standoff`.
  The AB1 row stays green under the fold, because its lines lean
  tangentially and stand off by the same amount across the span.
- **Not even a geometric bound.** "Span supremum plus lean from the
  origin" is not an exact bound on the form's true miss either. At
  `R = 1`, with a span about `t = 2.848` and a lean of `8.67e-3`
  tangential and `2.17e-4` radial, the miss exceeds it by `7.6e-5` of
  the lean. That excess is second order (`~ lean²/R`), so no f64 decision
  sees it, but an `Interval` decision would stand on an unsound bound.
- **Direction.** The incidence read's margin is a `MarginDiag`. That
  type is reporting-only by contract (its compile-fail doctests), so no
  class gate can decide on it. Its value is also the wrong direction: at
  f64 it is charged downward by the read's rounding (`incidence_at`), so
  it is a lower bound ("not shown off"). A class Zero needs an upper
  bound on the miss.
- **The count does not return.** On the D-tab's rigid copy the
  wall gap is zero only through `cos² + sin² = 1`. Every reading of it
  there decides numerically: the incidence rows (`[16, 4]`, `[20, 4]`),
  check 4's `pcurve_envelope` (`[16, 4]`), and the class gate. Any gate
  that bounds the form's miss carries a value of that gap. So
  `pcurve_chart_radial_moving [18, 24]` returns to `[22, 20]` only
  through the Sym rule
  (`work/sym/a-rigidly-copied-cylinder-walls-incidence-identity-has-no-sym-rule.md`),
  or by dropping the gap, which is the `meridian-standoff-unread` mutant.

One read could serve both only by re-anchoring, which is not done
here. The meridian form's azimuth and check 4's lemma would move to a
point of the span, and the incidence read would export an upper-bound
value of the gap: the cylinder's foot in closed form, uncharged. That
moves every line's image and check 4's lemma, and it buys no decision on
the D-tab.
