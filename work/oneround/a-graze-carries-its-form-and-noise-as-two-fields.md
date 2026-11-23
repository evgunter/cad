---
id: a-graze-carries-its-form-and-noise-as-two-fields
kind: issue
title: curve_face::pose's Graze carries its form and noise as two fields, its magnitude spelled by hand
status: open
opened: 2026-11-22
priority: P2
cost: E
---

Found by the merge read of PR 7221 (`oneround/rounded-pairs`). It is the
class that PR closed (`value-error-pairs-are-carried-beside-rounded-in-four-modules`),
one step from the code it converted.

`geom_brep::curve_face::pose`'s `struct Graze` carries the arm's form at the
touch as `value: T` and `noise: T`. Its one reader, `arc`, spells the
magnitude by hand: `width + g.value.abs() + g.noise`. The first-harmonic
arm (`first_harmonic`'s `graze`) fills the two fields by unpacking a
`Rounded` extreme.

**The fix:** `form: Rounded<T>`, with `arc` reading `width + g.form.magnitude()`.
The arm then passes its extreme whole. A mutant that drops the bound in
`arc` should turn a row red. No row pins it today, so the fix needs one:
a graze whose noise alone carries its arc past the span.

**Why it was not done in PR 7221.** Three of `Graze`'s four constructions
read an exact form with `noise: T::zero()`:
- the conic over a plane (`r − |d0|`);
- the line arms' vertex over a wall or sphere;
- the vertex over a cone.

As a `Rounded`, each would be a `Rounded::exact(…)`, a construction that
`scripts/gates/bare-rounding-allowlist.sh` counts as a site. None of the
gate's six reasons is true of an exact read: it charges nothing, so it is
not `re-pose`, and it is not an oracle. So the change reaches the gate's
reason set, either a reason for "read exact under the admission rule" or
an uncounted door for it, and that is beyond the one file.
