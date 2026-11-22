---
id: a-margin-whose-form-is-a-nonzero-constant-escalates-over-a-box
kind: issue
title: A margin whose form is a nonzero constant escalates over a box: a premise false at every member is refused as undecided, and a drive bisects it to its budget
status: open
opened: 2026-11-22
priority: P3
cost: M
---


Found by the review of PR 7344 (BOXFINISH, the volume reading's
premises), probe P4 on `boxfinish/review-volume-premises`
(`crates/sweep/tests/review_volume_premises_probes.rs`, commit
`0fc892fd`). Filed by BOXFINISH; not measured further.

A seamless cylinder whose wall radius is written 64 ε, 1e-6 or 1e-3
off its rims' radius is false at every member of a ±0.05 box on the
radius: the margin `r_wall − r_rim` has the constant form `δ`. At
`Interval` and at `Sym<Interval>`, in an eager and in a deferring
session, the premise `props_rim_fit` (`crates/topo/src/props.rs`, the
closed form's walk) is refused `Escalated`, the kind that says the
decision is undecided over the box. It reads `OffSurface`, the
definite kind, only at an exact parameter or over a box inside the
band.

- At plain `Interval` nothing else is possible: the enclosure of
  `(r + δ) − r` has the box's width by dependency.
- At `Sym<Interval>` the tier holds the form. It answers `Zero` from a
  zero form; a form that is a nonzero constant, or whose enclosure
  over the box excludes 0 by more than the band, is not read as a
  definite sign here, so the numeric channel's width decides.

The refusal is sound. What it costs: a subdivision driver treats
`Escalated` as the cue to bisect (`docs/ERROR-DESIGN.md` E6), and no
split narrows a dependency width, so the drive walks this leaf to its
depth budget and reports `Budget` where the honest verdict is a
definite refusal at the root. Whether a real document reaches it (a
false premise is a kernel defect or hand-restated geometry) is not
known; the general shape, a symbolic margin whose form decides a sign
the enclosure cannot, may be wider than premises.

Not established: which of `decide_sign`'s arms sees this margin and
why the form's constant is not read (the ladder may never be asked for
a sign, only for zero); how often a definite-by-form margin escalates
in the corpus.
