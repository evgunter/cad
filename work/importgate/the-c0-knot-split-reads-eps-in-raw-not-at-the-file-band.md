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

`crates/step-import/src/knot_split.rs` takes the file band
(`split_c0_knots`, from `normalize::normalize_shell`) but spends ε_in raw
as a distance (as of main after PR 7084):

- `crossing_knots`: the reach `eps_in / speed` each foot is held to on
  either side of a crease knot;
- `clear_of`: the setback `eps_in / speed` from the knot line, and the
  box-near tests `worst(corners…) + bent <= eps_in` and
  `gap.norm().hi() <= eps_in`;
- `on_the_line`: `worst(hull…) + bent <= eps_in`;
- `split_face_once`: the corner `near` closure,
  `(a - b).norm() <= eps_in`.

None is read through the kernel's funnel at the file's band
(`interpret::file_band`: `(e, K·e)`, `e` the larger of ε_in and the run's
ε), so a file declaring ε_in below the run's ε is read finer here than
everywhere else in the import, and no reading has an in-band arm. The iso
line's recognition (`recognize_curve::try_line`) already takes the file
band whole.

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
