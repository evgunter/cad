---
id: two-seam-runs-whose-seams-differ-in-unopened-faces-can-read-alike-within-the-evaluation
kind: issue
title: Two seam-run vertices whose seams differ only in faces the detail leaves unopened read alike within the evaluation
status: open
priority: P3
cost: M
opened: 2026-10-14
---


## What

Found in review of PR 5073 (branch `emit/union-readings`). `words.rs`'s
`EdgeCrossing` arm says a seam of the vertex's own node by its two
faces, "the seam of X and Y runs through", each face a citation of the
run. Within the evaluation (`Speaker::within`, `table_details`), a face
the detail search leaves unopened reads "a face". So two seam-run
vertices whose seams differ only in faces the search left unopened say
"the seam of a face and a face runs through" alike at those seams.

`table_details` opens citations until no two names of one table read
alike, so within ONE table the gate still holds. What is not pinned is
a pair of runs that share every opened face. The glued-rim row
`name_words_corpus::a_run_of_seams_says_each_seam_by_its_faces` passes
only because the two runs' first seams differ (b's wall at loop 0 step
1 against step 3), which the search opens.

## Next

Measure first: build a fixture whose two seam runs differ only in a
later seam's faces, and say both within the evaluation. Then decide
whether the detail search already separates them (and pin that row), or
whether a seam's two faces in a run should open together.
