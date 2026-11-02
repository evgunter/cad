---
id: step-import-reads-area-signs-raw-outside-decide
kind: issue
title: step-import reads two area quantities' signs raw, outside Decide: infer_outer's shoelace orientation check and the NURBS-net plane recogniser's Newell alignment
status: open
opened: 2026-10-25
priority: P3
cost: E
---

Found by PRED's S18 sweep (the step-export shell-role pickup, branch
`pred/s18-step-export-volume`), which looked in step-export and
step-import for a volume's or an area's sign read as a raw `f64`
comparison rather than through `Decide`.

## The two sites

- `crates/step-import/src/chart.rs`, `infer_outer`: each ring's
  chart polygon carries a shoelace area (`signed_area`, a local
  helper), and the "independent orientation signal" reads it raw —
  `candidate.area == 0.0 || other.area == 0.0 ||
  candidate.area.is_sign_positive() == other.area.is_sign_positive()`
  refuses `OuternessRefusal::OrientationContradicts`. The containment
  probe before it is band-guarded (`boundary_distance <= tol` refuses
  `UndecidableAtEps`); the area sign is not. A thin ring whose rounded
  shoelace sum lands on the wrong side of zero refuses a valid face
  with the wrong reason. It only ever refuses, so no wrong body is
  built.
- `crates/step-import/src/recognize.rs`, the plane recogniser over a
  NURBS net: `normal_sum` is the Newell area vector of the net's
  boundary ring, and its alignment with the chart normal is read raw
  (`align != 0.0`, then `align < 0.0` flips the normal). On a planar
  net the two are parallel, so `|align|` is near 1 and the read has
  headroom; nothing states that headroom, and a nearly degenerate net
  could orient its plane by rounding.

## Posture

Neither site builds a wrong body today that I know of: the first only
refuses, the second has headroom on every planar net. Hence P3. The
fix is either a band-decided read (`Decide`, with the refusal naming
the margin) or a stated headroom argument at each site.
