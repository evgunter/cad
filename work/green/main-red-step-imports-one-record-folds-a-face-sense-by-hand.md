---
id: main-red-step-imports-one-record-folds-a-face-sense-by-hand
kind: issue
title: main is red: step-import's one_record folds a face's sense bit by hand, outside the sanctioned scalars
status: open
opened: 2026-11-22
---


`topo`'s `face_normal::tests::no_source_file_folds_the_bit_by_hand` (`crates/topo/src/face_normal.rs:1517`) is red on `origin/main` at `d5505e6bfa`. It lists the files that fold a face's sense bit by hand. `crates/step-import/src/one_record.rs:184` folds `from_n` under `face.sense` (`if face.sense { from_n } else { -from_n }`), and that site is in neither the sanctioned list nor a `face_normal` door. The assertion's left side carries `("crates/step-import/src/one_record.rs", "from_n")`; its right side does not.

The fold restates a joined record's faces against the kept record's normal, so `from_n` is a normal. By the test's own message it folds through `OutwardNormal::from_chart` or a `face_normal` door. Alternatively, if it is a genuinely signed scalar, the list takes it with what it negates.

Found by `oneround/lever-sites-topo` (PR 7228), which does not touch the file. The test fails identically on that branch's merge with main and on main.
