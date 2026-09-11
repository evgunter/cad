---
id: curve3-eval-and-deriv-at-one-t-run-two-basis-passes
kind: issue
title: Curve3::eval and Curve3::deriv at one t run two span locations and two basis passes, with no order-1 jet door to collapse them into
status: open
opened: 2026-09-11
refs: [D306]
---


## What

D306 collapsed the SPAN-level pair — `Window::eval_in_span(t)` then
`Window::deriv_in_span(t)` at one `(span, t)` — into the order-1 jet
door `NurbsCurve3::ders1_in_span`, which answers both from one basis
pass and is bit-identical to both
(`crates/geom/src/curves.rs:1145`, executed). Sweeping for that shape
turned up the same defect one level up, where **no door exists to
collapse it into**.

`Curve3::eval(t)` and `Curve3::deriv(t)` at one `t` each do, on the
`Nurbs` arm, a `t.locate_spans(&self.knots)` and then a per-span basis
pass:

- `crates/geom/src/curves/nurbs.rs:1409-1414` (`eval`) and `:1435-1440`
  (`deriv`) — the locate is repeated and the basis pass is run twice,
  once per covered span, for a point and a tangent one order-1 pass
  answers.
- The enum arms that reach them: `crates/geom/src/curves.rs:355`
  (`eval`) and `:399` (`deriv`).

There is no `Curve3::ders1` and no `NurbsCurve3::ders1`: at the
whole-curve level the enum publishes `eval`, `deriv` and `deriv2` and
nothing that returns a jet (`crates/geom/src/curves.rs:336`, `:377`,
`:413`).

## Production sites, at the merge base of D306's PR

Swept with `rg --pcre2 -U '\.eval\((\w+)\)[\s\S]{0,300}?\.deriv\(\1\)'`
and its mirror, over `crates/`, then hand-filtered to production code
(everything else is `tests/` or a `#[cfg(test)]` module):

- `crates/geom/src/curves.rs:562-563` — the ellipse/near-parameter arm,
  `self.eval(near)` and `self.deriv(near)`.
- `crates/topo/src/validate.rs:3778-3779` — `curve.carrier().eval(t)`
  then `curve.carrier().deriv(t)` into `geom_brep::tangent_jet`.
- `crates/topo/src/boolean/ops.rs:1054-1055` — the same pairing.
- `crates/topo/src/boolean/contact_verify.rs:316-317` — `carrier.eval(t)`
  then `carrier.deriv(t)`.
- `crates/sweep/src/revolve/upgrade.rs:193-195` — `.eval(t)` then
  `data.carrier.deriv(t)` into `geom_brep::tangent_second_order`.
- `crates/sweep/src/blend/battery.rs:863-864` — `.eval(mid)` then
  `carrier.deriv(mid)`.

Three of the six hand the pair straight to a `tangent_jet` /
`tangent_second_order` call, which is the shape the span-level door was
minted for.

**Cost is arm-dependent, and only the `Nurbs` arm pays it.** `Line`,
`Circle` and `Ellipse` answer both from closed forms, so on those arms
the duplication is a frame construction at worst.

## Why it is not D306's diff

Closing it means minting a whole-curve order-1 jet — a new public door
on `NurbsCurve3` (the located-span walk, once) and a new exhaustive arm
set on `Curve3` — and then deciding what the non-NURBS arms return and
whether the enum door is worth its surface at all. That is a design
question, so by DOOR's charter (`work/door/plan.md`, *Territory*) this
row is not DOOR's as written: **it may want re-homing**, and the
placement here is only so the finding has a file rather than a sentence
in a merged PR body. The orchestrator has the board.

Four of the six sites are also outside D306's fence (`topo/`, `sweep/`),
which is the second reason the diff is not this unit's.

## What the sweep could not match

The pattern is literal `.eval(x)` / `.deriv(x)` on the same identifier
within 300 characters. It cannot see a pair split by more code than
that, a pair whose two calls name the parameter differently (`t` and
`*t`, say), or one routed through a helper that wraps only one of the
two doors.
