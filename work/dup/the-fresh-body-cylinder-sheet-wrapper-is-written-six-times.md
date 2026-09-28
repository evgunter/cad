---
id: the-fresh-body-cylinder-sheet-wrapper-is-written-six-times
kind: issue
title: The fresh-body cylinder-sheet wrapper is written six times across two probe suites
status: closed
opened: 2026-09-20
priority: P4
cost: E
closed: 2026-09-26
pr: 3284
---


## Finding

- **Where**: `crates/topo/tests/mate5_cyl_eps_rung.rs` — `sheet_a`,
  `sheet_b`, `sheet_b_r` and the two closures `tilted` / `small` inside
  `one_axis_tilt_two_levers_two_answers` — and
  `crates/topo/tests/r1_mate5_probe.rs`'s `sheet`.
- **Importance**: low — six copies of a four-line wrapper, no verdict
  rides on it
- **Confidence**: sure; all six read source-to-source at merge base
  `cd9fdfd6b`
- **Raised by**: the `dup/src-cyl-sheet` lane, 2026-09-20, from the
  reviewer's instruction to census the direct `cyl_wall_sheet(` sites
  once the `&mut Body` adapter had a home

The shape is one thing, six times:

```
fn X(frame, src, u0, u1, v0, v1) -> (Body<f64>, FaceKey) {
    let mut body = Body::<f64>::new();
    let f = cyl_wall_sheet(&mut body, frame, Some(src), (u0, u1), (v0, v1), Tol::witness());
    (body, f)
}
```

It is **not** the adapter PR #2925 homed. That one takes `&mut Body`
and returns a `FaceKey`; `crates/topo/tests/probe_support/mod.rs`'s
`wall_sheet` is its one spelling and `r1_mate5_probe.rs`'s `sheet` is
now three lines over it. This class is the *fresh-body* wrapper, which
returns the body it made, and the five in `mate5_cyl_eps_rung.rs` still
spell the door's whole argument list themselves.

## What a unit here owes

Decide whether the fresh-body shape gets a home beside `wall_sheet` in
`probe_support`, or whether returning the body is enough of a per-suite
judgement to leave. Two things a taker should have in hand:

- **`mate5_cyl_eps_rung.rs` was out of the finding lane's fence**, so
  the five there were read and not touched. Nothing is known to be
  wrong with them.
- **The `interval_lane` pair in that file is NOT in this class.**
  `a_half_period_tie_declines_period_fold` builds `Body<Interval>`, and
  both `wall_sheet` and this shape are `f64`-only; a shared home that
  wanted them would have to be generic over the scalar, which
  `cyl_wall_sheet` already is and these wrappers are not.

## Why this row is not on the territory owner's slate

`scripts/work.py territory` reads both suites as `tcost`'s and
`tint`'s. The finding is a wrapper written six times, which is this
program's charter; it is filed here so it sits with the rest of the
cylinder-sheet class, and either owner may claim it.

## Closed (2026-09-26, PR #3284)

**The fresh-body shape got the home, and it is the only shape.** At
the merge base every caller of `probe_support`'s `wall_sheet` and
`try_wall_sheet` handed it a `Body::new()` made on the line above, so
the `&mut Body` adapter and the fresh-body wrapper were one thing
spelled at two layers. Both doors now build the sheet in a body of its
own and return `(Body<f64>, FaceKey)` (`try_wall_sheet`:
`Option<(Body, FaceKey)>`, still the one `catch_unwind`, still wrapping
`wall_sheet`). Folded onto them:

- `mate5_cyl_eps_rung`: `sheet_a`, `sheet_b`, `sheet_b_at_radius` and
  the `tilted` / `small` closures — each keeps its name and its chart
  transfer, and its body is now one `wall_sheet` call;
- `r1_mate5_probe`: its `sheet` wrapper (deleted; its 18 calls name
  `wall_sheet`) and the two inline fallible pairs;
- `r2_probes`: seven inline `Body::new()` + `wall_sheet(&mut ..)`
  pairs and two inline fallible pairs — members by construction that
  the row's name census did not list.

Every folded site passes the door the arguments it passed
`cyl_wall_sheet` before (`Some(src)`, the two windows,
`Tol::witness()`), so the bodies are the same by construction.

**Not folded, each for a stated reason**: the `interval_lane` pair in
`mate5_cyl_eps_rung` (`Body<Interval>`; the door is `f64`, as the row
said); `split_edge_pcurve_rows::wall`, a fresh-body wrapper that
passes `None` as the source — the door always records one, so folding
it would change the body those rows read.

**Census** (merge base `0c1932667`, `git grep 'cyl_wall_sheet\|wall_sheet'`
over every tracked file): outside `topo/src`, the callers are exactly
the three MATE-5 suites, `probe_support` and `split_edge_pcurve_rows`.
Blind spot: a sheet built from `cyl_wall_sheet_keyed` or
`unit_cyl_sheet` directly — both are called only inside
`topo/src`'s fixtures module.

**Plants** (filter: the three suites, 30 rows):

| plant | reds | per site |
| --- | --- | --- |
| `wall_sheet` / `try_wall_sheet` panic at the caller, outside the catch | 21 / 30 | the first call of each row; `r2_diag_mintable_tilts` (~:244) printed the plant and passed — it maps panics by design and asserts nothing |
| the same, sparing every site already attributed | 21 / 30 | every second-side call: `mate5` ~:50, ~:74, ~:252; `r1_mate5` ~:62, ~:172, ~:210, ~:260, ~:293, ~:335, ~:385, ~:448; `r2` ~:68, ~:137, ~:172, ~:202, ~:273 (~:245 printed, as above) |
| the same, at `r1_mate5` ~:230 / ~:304, then at ~:231 / ~:305 | 2 / 9, then 2 / 9 | `probe4` and `probe6` each time — the last four masked sites; every one of the 37 folded calls reached |
| both windows shifted `+0.05` in `u` | 0 / 30 | — |
| every radius ×1.5 | 1 / 30 | — |
| even-`src` (B-side) radius ×1.5 only | 8 / 30 | — |

The two symmetric plants are the trap method item 16 names: a pair
built from two calls of one door keeps its agreement under a change
applied to both sides. The asymmetric plant is the one that measures
whether the answers are read, and seven `mate5_cyl_eps_rung` rows and
`r2_flush_cylinder_seat_declines_touching_boundary` read them.

**Fix pass (2026-09-26, PR #3284): the two kept members stay, and why.** Folding the
`Body<Interval>` pair and `split_edge_pcurve_rows::wall` would need
`wall_sheet` generic in the scalar and its source an `Option<u64>`.
The first puts the scalar only in the return type, so each of the 37
`f64` calls, whose consumers (`declared_pair_overlap`) are themselves
generic, would need a turbofish or an annotation; the second puts
`Some(..)` on those same 37 calls to serve one sourceless caller. Two
members do not pay for either; a generic door with an `f64` wrapper
is the `x` / `x_at` pair `work/helper/sweep-test-support-two-wrapper-conventions.md`
counts as avoidable.
