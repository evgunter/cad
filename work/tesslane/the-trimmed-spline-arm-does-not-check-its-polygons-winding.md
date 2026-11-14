---
id: the-trimmed-spline-arm-does-not-check-its-polygons-winding
kind: issue
title: The trimmed lane's spline arm does not check its polygon's winding against the face's sense, though the rule holds there
status: open
opened: 2026-11-14
---


## Finding

Found by TESSLANE's winding-rule unit (`tesslane/winding-rule`), 2026-11-14.

The rule "the polygon a face's boundary draws in its chart winds
counter-clockwise exactly when `Face::sense` is set" holds on a spline chart as
on an analytic one: DESIGN states interior-left against the sense-signed chart
normal, and a spline chart's normal is `S_u × S_v`
(`geom_brep::chart_winding`, module docs; its row
`the_rule_holds_on_every_chart` reads it off a spline chart too).

`mesh::trimmed::tessellate_trimmed` compares the trim polygon's winding with
`ChartWinding::of_face(face.sense)` on its cylinder and sphere arms only, and
refuses `TrimLoopOrientation` there. On the spline arm (`Lane::Nurbs`, which an
`Approx` face takes too) the comparison is skipped: the emit pass winds its
triangles by the polygon, so a spline face whose bit contradicts its loop is
meshed wound by its loop and nothing says the bit disagrees.

## Evidence

With the comparison made on every arm (the `!nurbs_chart &&` guard removed,
the unit's head), `cargo nextest run -p mesh` is 430 passed, 0 failed: no spline
face the mesh suite meshes has a polygon wound against its bit. Not run with the
guard removed: `sweep`, `step-import`, `demos/tour`, `demos/wild`.

## What closing it takes

Decide whether the spline arm refuses as the analytic arms do. If it does, drop
the guard, widen `TessellateError::TrimLoopOrientation`'s docs and the Python
binding's text past "analytic chart", and run the suites named above with it,
since an imported or approximated spline face is where a contradicting bit would
come from. If it does not, say at the site what a spline face is allowed that an
analytic one is not.
