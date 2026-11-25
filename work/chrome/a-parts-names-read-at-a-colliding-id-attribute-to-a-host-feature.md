---
id: a-parts-names-read-at-a-colliding-id-attribute-to-a-host-feature
kind: issue
title: A part copy's faces, at a host-colliding id, are attributed to the host's feature by tree::feature_of and lit by marks::focus
status: open
opened: 2026-11-25
priority: P2
cost: M
---


A mint chain starts empty in every document (`crates/editor-core/src/mint.rs`,
`Mint::empty`), so a part and a host whose first edits are byte-identical
(both open with the same profile and extrude) mint the same node and
step ids. An instance's copy carries its part's names verbatim, and a
host union of that copy names its faces `From { read: <the instance>,
of: <the part's name> }`. `tree::feature_of` (`crates/viewer/src/tree.rs`)
reads the feature off `role_leaf(name).node`, which for such a face is
the part's extrude id, equal to the host's extrude id. So:

- a click on the part copy's wall reports, selects and offers to delete
  the host's extrude (`FaceSelection::feature`, the property header, the
  tree highlight);
- `marks::focus` (`crates/viewer/src/marks.rs`) for the host extrude
  lights the part copy's faces with its own.

Measured on the fixture of
`crates/viewer/tests/focus_highlight.rs`'s
`a_parts_wall_is_no_wall_of_the_hosts_step_whatever_its_ids`: the union
draws 10 faces, 4 of them read through the instance's copy.
`marks::focus(Selection::Node(host extrude))` lit 8, and `feature_of`
attributed the same 8 to the host extrude: the host block's 4 and all 4
of the part copy's. Only the 2 faces the union minted itself were told
apart.

`marks::step_focus` does not have this fault: it reads each carried level
at its site (`swept_by` / `spelled_at` in `marks.rs`), and a level reached
through an instance is the part's. `feature_of` takes a name alone, so the
same cure needs the drawn site at every caller: a signature change across
the viewer's pick surface, which is why it is filed rather than made
there. The editor-core doors' side of the same fact (`Doc::part_spelled`
and `resolve::part_name` ask the mint log, which a colliding id passes) is
`work/refs/a-part-name-in-the-host-is-spelled-in-the-parts-ids.md`.

**The everyday case is a copied document.** Duplicating a document into
a variant part (Save As, then edit) keeps every id minted before the
fork, so a host that instances the variant collides on every one of its
own early nodes. The worst symptom is the property header's Delete: a
click on the part copy's wall offers to delete the host's extrude
(`feature_of` is the node that Delete deletes, `tree.rs`'s doc on it). Read
from the code; not driven through the chrome. This row is P2 as filed; it is P1 if that workflow is
common.
