---
id: select-all-edges-names-one-reason-for-three-absences
kind: issue
title: Select all edges' disabled reason is one sentence for three absences, and reads the index's absence without the index seam
status: open
opened: 2026-09-28
priority: P4
cost: E
---


Found by the sweep on `vnews/progress-names-its-own-states`, whose
unit made the index seam a value (`pickcache::IndexSeam`, minted by
`PickCache::index_seam`) because two consumers derived "is an index
coming" in two different ways.

`crates/viewer/src/pane/create.rs`'s `all_edges_row` (about line 1568)
zips three absences into one disabled state:
`target.zip(self.session.evaluation()).zip(self.index)`. The state
gets one hover sentence: *"click an edge or a face of the body first,
and let it evaluate — a feature picked in the tree does not say which
body"*. While the index is being built (or while the fit it waits on
is running), the target and the evaluation are both present, and the
sentence tells the user to pick something they have already picked.
It never says that the index is still coming, which the toolbar's
`indexing…` is saying at the same moment. The same pane has no
`IndexSeam` to read. The fix is small: split the `None` arm by which
absence it is, and hand the pane the seam value that the viewport
already gets (`ViewerBehavior::indexing`).

The gated-button shape itself is
`a-gated-button-with-a-reason-is-spelled-five-ways`. This row is about
what the sentence claims, not how it is spelled.
