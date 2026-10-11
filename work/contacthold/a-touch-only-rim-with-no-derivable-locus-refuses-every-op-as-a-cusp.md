---
id: a-touch-only-rim-with-no-derivable-locus-refuses-every-op-as-a-cusp
kind: issue
title: verify_tangency_declaration routes a touch-only rim with no derivable tangent locus to RimCuspArmUnbuilt at every op, including the differences and the intersection
status: open
opened: 2026-10-11
priority: P3
---


Reported by one designer on `a-same-side-tangent-touch-refuses-at-every-op`.
Not measured: read from code only. Measure first.

`verify_tangency_declaration`'s rim routing reads the tangent pair's
wedge without the op. So a touch-only kiss whose tangent locus cannot be
derived (for example torus × torus, where `tangent_locus` answers
`Unsupported`) refuses `RimCuspArmUnbuilt` at ∖ and ∩ as well as at ∪.

Under the same-side fork's recommendation, a touch-only pair has no
junction. Its ∖ and ∩ return the operand or Empty, and its ∪ needs only
a witness lane for the record, not a cusp edge. Confirm the reach with a
fixture, then decide what each op should do there.
