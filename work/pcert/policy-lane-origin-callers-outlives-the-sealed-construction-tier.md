---
id: policy-lane-origin-callers-outlives-the-sealed-construction-tier
kind: issue
title: policy_lane::Origin's Callers arm reads a construction door's spec as a caller's, which no caller outside the kernel hands once the construction tier is sealed
status: open
opened: 2026-10-27
priority: P3
cost: M
refs: [who-placed-it-is-read-off-the-description-not-the-door]
---


## Finding

Filed by PCERT E1d (Ev's ruling on PR 5838), out of that unit's scope.

`topo::policy_lane::Origin` names whose data a certification reads:
- `Origin::Callers`: a spec a door was handed, whose span refusal
  stays typed (`Origin::read`);
- `Origin::Kernels(site)`: the kernel's own stored edge, whose span
  refusal panics as a kernel bug.

Topo's construction doors (the Euler operators, `Body::set_edge_curve`,
the description setters) certify under `Callers`, on the premise that
a caller outside the kernel handed them the spec.

Under the ruling, topo's construction tier is the kernel's internal
API. E1d2 seals it behind `topo::construction::Construct`, which pncad
does not re-export. After that, every spec those doors receive comes
from a kernel crate (sweep, step-import, editor-core) or a test, and
the two arms tell apart only:
- a spec a kernel crate built, against
- an edge topo stores.

D4 ¶1 (iv) as ratified makes a stored contradiction the kernel's defect
at a build. So the question is whether `Origin` collapses to the one
`Kernels` reading, with a span refusal at a construction door a kernel
bug that panics. The alternative is a typed kernel defect. That stays
open while `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md`
asks it.

Not decided here. Cut it after E1d2 lands, so the sealed surface is
what is measured.
