---
id: main-red-sketch-on-face-folds-the-face-normal-by-hand
kind: issue
title: Main red: sketch-on-face's nearest orientation negates the face frame's axis by hand under its sense bit (face_normal's fold census, since PR 5742)
status: open
opened: 2026-10-27
priority: P0
cost: M
---


Filed by the 2026-10-27 nightly triage (run 40455870959, main
`fb476030a9`).

## What

`topo` `face_normal::tests::no_source_file_folds_the_bit_by_hand`
(`crates/topo/src/face_normal.rs:877`) is red at every ε row of the
full suite, and on main `57e3f95a` at the default ε:

    a value is negated by hand under a face's sense bit outside the sanctioned scalar negations. …
      left: [("crates/editor-core/src/edit.rs", "pose.axis"), ("crates/geom-brep/src/dihedral.rs", "kappa_rel"), …]
     right: [("crates/geom-brep/src/dihedral.rs", "kappa_rel"), …]

The new site is `Editor::nearest_to_face` (`crates/editor-core/src/edit.rs:6831`):

    let outward = if pose.sense { pose.axis } else { -pose.axis };

It folds a NORMAL — the face frame's axis turned outward — by hand,
which is exactly what the row exists to stop: a normal folds through
`OutwardNormal::from_chart` or a `face_normal` door. Adding it to the
row's sanctioned list (which holds only signed scalars) would be the
wrong repair.

**Introduced by** `6840af67` ("msolve: R2 fix pass — the façade picks
sketch-on-face's nearest orientation …"), which reached main with the
merge of **PR 5742** (`intent/value-chart-orientation`, `19eff38c58`).

## Owed

Read the outward axis off the face through a door that already folds
the sense (or give `names::interrogate::face_frame`'s reading one), and
leave the census's list as it is.

Reproduce: `cargo nextest run -p topo -E 'test(=face_normal::tests::no_source_file_folds_the_bit_by_hand)'`.
