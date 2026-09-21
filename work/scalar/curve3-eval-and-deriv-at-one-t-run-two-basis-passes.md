---
id: curve3-eval-and-deriv-at-one-t-run-two-basis-passes
kind: issue
title: Curve3::eval and Curve3::deriv at one t run two span locations and two basis passes, with no order-1 jet door to collapse them into
status: open
opened: 2026-09-11
branch: scalar/curve3-jet
pr: 2708
priority: P1
cost: E
---

## What

D306 collapsed the SPAN-level pair — `Window::eval_in_span(t)` then
`Window::deriv_in_span(t)` at one `(span, t)` — into the order-1 jet
door `NurbsCurve3::ders1_in_span`. Sweeping for that shape turned up
the same defect one level up, where **no door exists to collapse it
into**.

`Curve3::eval(t)` and `Curve3::deriv(t)` at one `t` each run
`t.locate_spans(&self.knots)` and then a per-span basis pass on the
`Nurbs` arm (`crates/geom/src/curves/nurbs.rs:1409-1414` and
`:1435-1440`): the locate is repeated and the basis pass is doubled,
for a point and a tangent one order-1 pass answers.

There is no `Curve3::ders1` and no `NurbsCurve3::ders1`. At the
whole-curve level the enum publishes `eval`, `deriv` and `deriv2`
(`crates/geom/src/curves.rs:336`, `:377`, `:413`) and nothing that
returns a jet — the span level has `ders1_in_span`
(`crates/geom/src/curves/nurbs.rs:500`) and the whole-curve level has
no counterpart.

**Cost is arm-dependent, and only the `Nurbs` arm pays it.** `Line`,
`Circle` and `Ellipse` answer both from closed forms, so on those arms
the duplication is a frame construction at worst.

## Why this is `scalar`'s and not DOOR's

Closing it means minting a whole-curve order-1 jet: a new public door on
`NurbsCurve3` (the located-span walk, once) and a new exhaustive arm set
on `Curve3`, plus a decision on what the non-NURBS arms return and
whether the enum-level door earns its surface. That is the same shape as
**`S393`** on this program's slate — class `M`, "new door in
`crates/geom/src/curves/nurbs.rs`", "small diff once the door's home,
name and roll question are decided". Same file, same open question,
same class. DOOR's charter takes only rows whose fix is written in the
row, so it was re-homed here at the DOOR orchestrator's direction rather
than carried at the wrong class.

## Production sites

Seven, all reaching a carrier that can be `Curve3::Nurbs`:

| site | shape |
| --- | --- |
| `crates/topo/src/validate.rs:3778-3779` | `carrier().eval(t)` then `carrier().deriv(t)` into `geom_brep::tangent_jet` |
| `crates/topo/src/boolean/ops.rs:1054-1055` | the same pairing |
| `crates/topo/src/boolean/contact_verify.rs:316-317` | `carrier.eval(t)` then `carrier.deriv(t)` |
| `crates/sweep/src/revolve/upgrade.rs:193-195` | `.eval(t)` then `carrier.deriv(t)` into `geom_brep::tangent_second_order` |
| `crates/sweep/src/blend/battery.rs:863-864` | `.eval(mid)` then `carrier.deriv(mid)` |
| `crates/geom-brep/src/certify.rs:1743` and `:1805` | the same parameter EXPRESSION, `sample_param(t0, t1, i)`, sixty-two lines apart in one sample loop |
| `crates/sweep/src/skin.rs:1120` and `:1160` | routed through a helper: `unit_tangent`'s `path.deriv(t)` and the caller's `path.eval(t)` at the same `t = t_of(i)` |

Four of the seven hand the pair straight into a `tangent_jet` /
`tangent_second_order` call, which is the shape the span-level door was
minted for.

**One conditional near-instance, deliberately not counted.**
`crates/sweep/src/blend/battery.rs:1429-1441`'s `pick` closure evaluates
at BOTH `t0` and `t1` and then takes the derivative at exactly one of
them, on the branch the comparison chooses. A jet door there would
compute a derivative thrown away half the time, so the collapse is not
free and the site needs its own decision.

**`crates/geom/src/curves.rs:562-563` is NOT a site**, though the
obvious pattern matches it. Those two calls are inside `param_near`'s
**`Circle`** arm, and `:566` sends `Ellipse` and `Nurbs` to `None` — so
the pair can never reach the NURBS path, and on a circle both doors are
closed forms. It was counted once, in D306's PR body, and is corrected
here.

## The sweep, and what it could not match

Two passes over `crates/*/src` with the `#[cfg(test)]` tail cut off:

1. `rg --pcre2 -U '\.eval\((\w+)\)[\s\S]{0,300}?\.deriv\(\1\)'` and its
   mirror — a same-IDENTIFIER backreference in a 300-character window.
   This is the pass D306's PR reported, and it found five of the seven.
2. A same-EXPRESSION pass (any argument text, not just an identifier)
   with a 120-line window, per file, both orders. This is what found
   `certify.rs` and `skin.rs`, and what removed `curves.rs:562-563`.

What neither pass can match:

- **A pair whose two calls name the parameter differently** — `t` and
  `*t`, or `t` and a rebound copy.
- **A pair routed through two different helpers**, where neither call
  site mentions `eval` or `deriv` at all. `skin.rs` was caught only
  because ONE half was still written literally at the call site.
- **A different receiver reached by the same expression** — the pass
  ignores the receiver, so `certify.rs:1922`'s `chart.pcurve.eval(...)`
  matched against `spec.carrier.deriv(...)` and had to be hand-rejected.
- **Anything outside `crates/*/src`** — `demos/`, `tools/` and
  `benches/` were not swept, and `demos/tour` and `demos/wild` are
  ordinary API consumers where the pair would be just as real.

## Unit CURVE3-JET (2026-09-15)

Dispatched as block SCALAR-B4 slot 0 under `docs/CURVE3-JET-SPEC.md`
(deleted at merge per the ledger). The survey of 2026-09-15 corrected the
finding on two points the spec carries: the whole-curve order-2 jet
`NurbsCurve3::ders` already exists (the order-1 door is its sibling,
`ders1`), and the `revolve/upgrade.rs` pair now lives in
`geom-brep/src/dihedral.rs`; it added six `NurbsCurve3<f64>` pair sites in
the tour and the `pncad` example that feed S393's `path_start_frame`.

## Digest receipt: the D9 pin for "nothing's bits move"

The recipe is `work/scalar/rate-pair-in-geom-core.md` §Digest receipt,
zero-parameter (the release binary run directly, outdir the literal
`tour-out`, both streams digested whole). Taken at the merge base
`88201b83f` (the branch was `origin/main` plus the spec commit, which
touches no code) before the first code change:

- listing (1766 emitted files, sorted per-file digests):
  `c678b14381e6c0d9e835e91883a294f8f588507aedf60e8e820a5ec1834b0863`
- narration (729 lines):
  `65da9dd7349507058bc4d8dc2b302764009e1c44879b6480dd67731a9e591d50`

The head receipt is on the PR, taken the same way at the head SHA.
