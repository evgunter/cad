---
id: arc-rim-gate-reports-a-degenerate-carrier-as-an-infinite-residual
kind: issue
title: step-import RimOffWallBoundary's residual is a measurement type carrying a non-measurement sentinel
status: open
opened: 2026-09-12
refs: [step-adopt-let-ok-iso-discards, 2406]
---

## What

`StepImportError::RimOffWallBoundary`'s `residual` is documented as a
measurement — *"the worst sampled deviation (meters)"* — and its
sentence tells the reader the wall's boundary column *"deviates from
the rim's circle by up to {residual:e} m (ambient tolerance
exceeded)"*. **Two arms of `arc_rim_on_wall_boundary` reach that
sentence with `f64::INFINITY`, having sampled nothing or having failed
to evaluate**, so the number is a sentinel wearing a measurement's
words.

Citations are on `door/step-adopt-iso-discards` at `c35d1ff` (the PR
that filed this, #2406):

1. **`crates/step-import/src/adopt.rs:900`** — the carrier screen. A
   `CIRCLE` whose radius is not positive-and-finite, or whose axis or
   reference direction has a zero/non-finite norm, returns
   `Err(ArcRimRefusal::Residual(f64::INFINITY))` before any sampling
   runs. The fact is a **degenerate parsed carrier**, a statement about
   the file's `CIRCLE` entity.
2. **`crates/step-import/src/adopt.rs:941`** — the sample loop. A
   non-finite `ring` or `theta` sets `worst = f64::INFINITY` and
   breaks, and that flows out through the same `Residual` arm. The fact
   is a **sample that would not evaluate**, not a measured distance.

The renderer is `crates/step-import/src/error.rs:436`.

This is the class the row it came from
(`step-adopt-let-ok-iso-discards`, PR #2406) closed one instance of: a
fault dressed as the gate's own verdict. There the structural refusal
of a wall's boundary column was charged to the rim as a deviation;
here two more non-measurements are. The PR converted only the
`boundary_iso_*` discards, deliberately, because
`crates/step-import/*` is EXCH's ground and one PR is one row.

## Measured, so the scope is known

**A diagnostics defect, not a soundness hole**, and the measurement is
taken **on the fixture class the gate actually runs for** — a
rational-walled arc loft, where `nurbs_plane_pair` is true. (The first
version of this item measured on `cylinder.step`, whose walls are
`CYLINDRICAL_SURFACE`, so `arc_rim_on_wall_boundary` is never called
there at any radius; that measurement said nothing about this screen
and the reviewer of #2406 caught it.)

Executed: the native arc loft of
`review_probes_m7_3::native_arc_loft_for_probe` exported through
`step_export::step_string`, its first rim record
`#30 = CIRCLE('', #29, 1.414213562373095)` rewritten to radius `0.` and
to `-1.4142135623730951`, re-imported through `import_step` at
`Tol::witness()`. Both produce, verbatim:

```
step import: edge #31: an ARC cap rim does not lie on its adjacent
NURBS wall's boundary — the wall's own boundary column, sampled at the
certification schedule, deviates from the rim's circle by up to inf m
(ambient tolerance exceeded). On a rational wall this residual gate is
the rim's only certification, so the file is refused rather than
adopted wrong
```

Nothing was sampled and nothing deviated. The file IS refused — an
`inf` residual can never pass the gate — so no bad body rides out; what
is wrong is that the sentence names the wrong subject and the wrong
recourse, and the id it names is the `EDGE_CURVE` rather than the
`CIRCLE` that is actually malformed.

Worth knowing while fixing: the `CIRCLE` parse arm screens nothing
(`crates/step-import/src/entities.rs:918`; `as_length` at `:369`
scales without metering), unlike `CONICAL_SURFACE`, which refuses
`!(radius.is_finite() && radius >= 0.0)` in its own arm at `:640`.

## Shape of the fix

The narrow version is one refusal arm per fact. The version that
closes the class is to stop letting a measurement type carry a
sentinel:

- screen the degenerate carrier at the `CIRCLE` parse arm, the way
  `CONICAL_SURFACE` already does, so the refusal names the offending
  entity instead of an edge three phases later;
- give the non-evaluating-sample arm its own refusal, so `residual`
  means what its doc says at every site that can produce it;
- after which `RimOffWallBoundary.residual` can be documented — and
  ideally typed — as a finite measured deviation, which is the only
  thing its sentence can honestly render.
