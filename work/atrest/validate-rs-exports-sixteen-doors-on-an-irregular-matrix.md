---
id: validate-rs-exports-sixteen-doors-on-an-irregular-matrix
kind: issue
title: validate.rs exports sixteen at-rest doors on an irregular door-by-form matrix
status: closed
opened: 2026-09-21
priority: P4
cost: D
parent: ATREST-10
closed: 2026-09-25
pr: 3227
---

## Finding

`crates/topo/src/validate.rs` (about 9,500 lines) exports sixteen
public at-rest doors on an irregular matrix — {`validate_geometric`,
`validate_pseudomanifold`, `contact_marks`} × {plain, `_certificate`,
`_declared`, `_structural`, `_structural_declared`,
`_certificate_structural`, `_declared_structural`, `_measured`} with
holes: there is no `contact_marks_certificate` and no
`validate_pseudomanifold_declared`, `validate_geometric` alone has
`_measured` and `_certificate`, the marks pass alone has
`_declared_structural`. LANE-1 (PR 3010) kept the count (four
`_certified` names folded into the plain ones, four `_structural`
twins added) and named the shape in its PR body; `props.rs` grew
`QuadLane`, `mass_properties_structural` and
`classify_shells_structural` in the same change. Raised by the LANE-1
R2 review as NOTE 8 (Q8, module size and roster), not fixed there
because the matrix is ATREST's and no single unit's rename should
redraw it.

## What to do

Draw the roster deliberately: which forms every door has (a
`_structural` twin for each certified door is ruling 3's rule and is
now uniform), which forms are one door's alone and why, and whether
the file should split along the {geometric, pseudomanifold, marks}
axis. A door table in `validate.rs`'s module doc that a census reads
would keep the next rename from growing a hole.
