---
id: the-circle-split-cap-offers-counts-the-document-refuses
kind: issue
title: The circle_split count cap offers, at any radius under a few millimetres, counts whose figure the document escalates on
status: open
opened: 2026-09-22
priority: P3
cost: D
---


## Finding

`crates/viewer/src/forms.rs`, `MAX_CIRCLE_SPLIT`. The cap's doc
comment says the count is "a product cap and not the kernel's: the
kernel takes any count", and that is true of the COUNT. It is not
true of the figure the count builds, and the form offers the pairs
where it is false.

A circle of radius `r` split `n` ways is judged by two margins that
are both proportional to `r` and both read against the run's band
(ε, K·ε):

- `segment_straightness` — the sagitta `s = r·(1 − cos(π/n))`, which
  decides each piece is an arc rather than a chord;
- `carrier_circles_identity` — nought in the reals, in floating point
  the residue of rebuilding each arc's centre from a chord of length
  `2r·sin(π/n)`, which decides two pieces are one circle.

So the radii the document admits at a given `n` are an interval in ε,
and at the cap that interval starts well above zero. **Measured**, on
this branch, `n = MAX_CIRCLE_SPLIT = 1024` at the DEFAULT ε (1e-9),
through `apply(InsertNode { Profile })` exactly as the create form
commits one:

```
n=1024 r=1e-3  Escalated  segment_straightness margin 4.706190423828488e-9
                          band Band { zero: 1e-9, escalate: 1e-8 }
n=1024 r=2e-3  Escalated  segment_straightness margin 9.412380847656976e-9
n=1024 r=3e-3  OK
n=1024 r=1e-2  OK
n=256  r=1e-3  OK
n=64   r=1e-3  OK
```

A person who drags the split count to the cap on a circle smaller
than about 3 mm gets a document refusal at the shipping tolerance.
The form has no radius-aware bound and nothing warns.

The lower wall is `r > K·ε/(1 − cos(π/n))`, which at `n = 1024` and
K = 10 is `2.12e6·ε`. It is a wall in `r/ε`, so it moves with the
run's tolerance: at ε = 1e-6 the cap needs a radius above 2.1 m.

## What a fix would have to decide

Three shapes, none adjudicated here:

- bound the count by the radius in the form (the cap becomes
  `min(MAX_CIRCLE_SPLIT, n_max(r, ε))`), which makes the field's
  range depend on a sibling field;
- leave the range and say the refusal in the preview's verdict, where
  `preview_verdict` already speaks for a figure the commit door would
  refuse — the cheapest, and it tells the truth rather than
  preventing it;
- leave it entirely, on the argument that a 1 mm circle split 1024
  ways is not a figure anybody means, and the escalation is the
  honest answer for it.

The second is the one that costs least and hides least.

## Not the fixture row

`work/chrome/a-split-circle-fixture-sits-inside-the-1e-6-escalation-band.md`
is about a TEST fixture that sat in a band; its fix ties that
fixture's radius to ε. This row is about the PRODUCT surface that
offers the same figure to a person, and no change to a test fixture
touches it. It is also not
`work/chrome/a-flat-rung-row-uses-an-absolute-epsilon-on-a-scaled-value.md`,
which is about `f64::EPSILON` standing in for a tolerance in a
`scene.rs` row and says nothing about the run's ε.

## Home

CHROME. `crates/viewer/src/forms.rs` (the cap and its doc comment),
`crates/viewer/src/widgets.rs` (the field that draws the range),
`crates/viewer/src/pane/profile.rs` (`preview_verdict`, if the
verdict is where it is said). Pre-existing; not introduced by the
unit that found it.

## Found by

The CHROME split-circle-eps lane, sweeping for viewer `app`-feature
fixtures whose scale sits near a gated eps band
(implementer-discipline §5/§6). The sweep found no second red row;
it found this, one level out from the fixture.
