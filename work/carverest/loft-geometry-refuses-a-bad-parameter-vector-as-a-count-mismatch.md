---
id: loft-geometry-refuses-a-bad-parameter-vector-as-a-count-mismatch
kind: issue
title: loft_geometry refuses a NaN, descending or unclamped parameter vector as a count mismatch
status: open
opened: 2026-10-06
priority: P3
cost: E
---


## Finding

`sweep::skin::loft_geometry` takes the v-parameters as an argument and
leaves their check to `skin_on`, which hands them to
`geom::curves::fit::interpolate_columns`. Every malformed vector of the
right LENGTH comes back as a count mismatch, naming two equal counts:

```text
loft_geometry(.., &[0.0, f64::NAN, 1.0], ..) → Err(Fit(ParamCountMismatch { params: 3, points: 3 }))
loft_geometry(.., &[0.0, 0.7, 0.4], ..)      → Err(Fit(ParamCountMismatch { params: 3, points: 3 }))
loft_geometry(.., &[0.1, 0.5, 1.0], ..)      → Err(Fit(ParamCountMismatch { params: 3, points: 3 }))
loft_geometry(.., &[0.0, 0.5, 2.0], ..)      → Err(Fit(ParamCountMismatch { params: 3, points: 3 }))
```

Measured 2026-10-06 on `carve/loft-v-is-the-whole-sets`: three unit
squares at z = 0, 1, 2, v-degree 2. The refusal's text is false (the
counts agree) and names no recourse.

## The fix it points at

A refusal arm for a parameter vector that is not clamped `0 → 1` and
strictly ascending, naming the first offending index. It belongs at the
fit's door, so `skin_on` and `loft_geometry` both say it.
