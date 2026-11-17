---
id: the-c0-knot-split-reads-eps-in-raw-not-at-the-file-band
kind: issue
title: step-import: the C0-knot split reads ε_in as a raw distance, not through the file band
status: open
opened: 2026-11-17
priority: P2
cost: M
parent: adoption-rebuilds-caches-at-eps-from-eps-in-interpretation
---

Found by `importgate/interp-layer`'s sweep of every ε_in read in
`crates/step-import` (the unit that routed the rest through
`interpret::FileBand`).

## What

`crates/step-import/src/knot_split.rs` takes ε_in as a bare `f64`
(`split_c0_knots`, from `normalize::normalize_shell`) and spends it raw:
`interior_param_at` (`(carrier.eval(t) − q).norm() <= eps_in`),
`crossing_knots` (`reach = eps_in / speed`), `clear_of` (`setback`, the
box-near tests `… + bent <= eps_in` and `gap.norm().hi() <= eps_in`),
`on_the_line`, and the corner `near` closure in the face split. None is
read through the kernel's funnel at the file's band
(`interpret::file_band`: `(e, K·e)`, `e` the larger of ε_in and the run's
ε), so a file declaring ε_in below the run's ε is read finer here than
everywhere else in the import, and no reading has an in-band arm.

## Why not fixed with the rest

`importgate/knot-place` (`the-c0-knot-split-refuses-what-it-cannot-place`)
was rewriting most of `knot_split.rs` while the interpretation layer
landed; converting the same reads there would have collided with it.

## Fix

Take `interpret::FileBand` at `split_c0_knots`, read each "within ε_in"
certificate as a Zero reading at the file band (an enclosure's upper end
against the band's zero is the same certificate), and give the structural
ones (a vertex the knot line lands on) their in-band refusal
(`interpret::Refusal::Ambiguous`).
