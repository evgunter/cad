---
id: the-circle-split-cap-offers-counts-the-document-refuses
kind: issue
title: The circle_split count cap offers counts the document escalates on, at every radius below K*eps/(1-cos(pi/n)) - 2.12 mm at the cap, and falling as n squared
status: open
opened: 2026-09-22
priority: P3
cost: D
---


## Finding

`crates/viewer/src/forms.rs`, `MAX_CIRCLE_SPLIT`. The cap's doc
comment says the count is "a product cap and not the kernel's: the
kernel takes any count". **That sentence is precisely true**: the
cap's rationale really is preview cost, it scopes itself to authoring
explicitly, and `circle_split_kernel` really does reject only
`n < 2`. A performance cap and a validity limit share a shape here;
they have not drifted apart. What is missing is a **third** bound
nobody wrote — the form has no radius-aware limit at all, and it is
the (r, n) pairs below that limit that the document escalates on.

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
than **2.12 mm** gets a document refusal at the shipping tolerance.
The form has no radius-aware bound and nothing warns.

The sample above only brackets that number between 2 and 3 mm. The
closed form pins it: the lower wall is `r > K·ε/(1 − cos(π/n))`, which
at `n = 1024` and K = 10 is `2.1249e6·ε` — **2.1249e-3 m** at the
default ε. Rounding it to "about 3 mm" overstates the safe radius by
40% in the permissive direction, which is the wrong way to be wrong
for the number a person will quote. It is a wall in `r/ε`, so it also
moves with the run's tolerance: at ε = 1e-6 the cap needs a radius
above 2.12 m.

## The boundary is a curve in (r, n), not a radius — and that is the finding

`1 − cos(π/n) ≈ π²/2n²`, so `r_min ∝ n²`: **each halving of the count
buys a factor of four in radius.** Read the other way round,
`n_max(r) ≈ π·√(r/2Kε)` ∝ √r — **quartering a circle's radius halves
the count at which the document starts refusing it**, and no circle is
exempt, however large.

| n | `r_min = K·ε/(1 − cos(π/n))` at the default ε |
|---|---|
| 1024 (the cap) | 2.12 mm |
| 512 | 0.53 mm |
| 256 | 0.133 mm |
| 64 | 0.0083 mm |
| 16 | 0.00052 mm |

The sample above already shows `n=256 r=1e-3 OK` and `n=64 r=1e-3 OK`,
so the data was in the row before the law was, and a title that leads
with a radius invites a reader to bound the problem to small circles.
It is not bounded: a 1 m circle is refused from n ≈ 22,000, a 100 m
one from n ≈ 222,000. The cap is what keeps the product's exposure to
sub-3-mm circles, not the geometry. Measured identically at ε = 1e-9,
1e-6 and 1e-12 by bisecting the radius through `apply`: the wall is
the closed form to five digits at every one.

There is a second wall above — the `carrier_circles_identity` residue
reaches ε at `r ≈ 1.5e12·ε` at n = 1025, falling as ~1/n — but at the
default ε that is 1.5 km against a lower wall of 2.12 mm, six decades
away and outside anything the form offers. It matters only to the
fixture row below.

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
`work/fit/a-flat-rung-row-uses-an-absolute-epsilon-on-a-scaled-value.md`,
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
