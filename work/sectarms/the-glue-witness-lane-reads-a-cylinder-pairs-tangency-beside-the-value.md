---
id: the-glue-witness-lane-reads-a-cylinder-pairs-tangency-beside-the-value
kind: issue
title: The glue door's witness lane verifies two cylinders' tangency along a ruling on its own reading, beside the section value's cc_parallel_gap
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Filed by the cylinder × cylinder stage 3 lane's §5 sweep (branch
`sectarms/cylinder-join-reads-value`, 2026-11-02).

## What

`boolean::glue::tangency` admits two cylinders to the witness lane
(`glue::ruled`). It verifies a tangency along their touching ruling
through `glued_tangency` → `contact_verify::tangent_locus_relation`,
read on the two surfaces.

The section value already decides the same fact on the pose:
`cc_parallel_gap` or `cc_parallel_nest` Zero is a tangent ruling. So the
glue door reads a cylinder pair's tangency a second time
(`[one-way.readings]`), on its own margins.

## Why it is not stage 3's

The witness lane verifies a declared or glued contact (C8) along a
locus, with its material sides. That is not the section's reading the
join takes. Whether it should take its touch from the value, or is a
legitimately separate verification, needs a look at C8's contact
contract.
