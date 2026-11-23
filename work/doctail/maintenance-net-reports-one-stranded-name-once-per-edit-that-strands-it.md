---
id: maintenance-net-reports-one-stranded-name-once-per-edit-that-strands-it
kind: issue
title: MaintenanceNet keeps two rows for one stranded name when a splice's hand-over and its delete each strand it
status: open
opened: 2026-11-22
priority: P3
cost: E
---


## The finding

Filed by OFFER's lane for `cascade-delete-shows-the-strand-count`,
from outside DOCTAIL's fence.

A delete by continuation (`editor_core::continuation::splice`) that
hands a reader to the body the target continues applies a `SetParam`
at the reader's selection and then the `DeleteNode`. Where the reader
holds a name the target MINTED, both edits report it:

- the `SetParam` moves the read past the target, so the minting node
  is no longer upstream of the carrier, and `stranded_by_repoint`
  (`crates/editor-core/src/edit.rs`) reports the name with
  `Took::Reach`;
- the `DeleteNode` removes the minting node and reports the same name
  on the same selection with `Took::Node`.

`MaintenanceNet::finish` keeps both: each row's test is that its
carrier is live at the end and still holds the name, which is true of
the one name twice. So `Recorded::maintenance` for the action holds two
rows about one reference the end document holds once.

Reproduced on the viewer's die fixture
(`crates/viewer/tests/cascade_delete.rs`, `die_shaped`): deleting the
inner fillet hands the outer fillet to the subtraction chain, and the
outer fillet selects an edge named by the inner one. The action's net
is, in order:

```
Fillet <outer> selects the edge of Fillet <inner>; this edit re-pointed a read past Fillet <inner>, which minted the name, so Fillet <outer> refuses until the name is rebound
Fillet <outer> selects the edge of Fillet <inner>; this edit deleted Fillet <inner>, which minted the name, so Fillet <outer> refuses until the name is rebound
```

The same holds for a shell handed past a deleted shell
(`edit_maintenance::the_delete_button_does_not_count_a_name_the_delete_hands_back`,
`crates/viewer/tests/edit_maintenance.rs`, whose click reports the
minted name in two rows).

## What it costs today

- The viewer's status line says the strand twice
  (`frame::maintenance_notices` words one notice per row).
- The delete button's pre-click count cannot be the row count. It
  counts the distinct answers of `Maintenance::stranded_name` (added
  by the PR that filed this row: the carrier and name a row reports
  stranded, as a `NameCarrier`), in `session::delete::stranded_names`
  (`crates/viewer/src/session/delete.rs`).

A second way one action holds two rows with one answer, which is not
this finding: an instance's interface record that names one `outer` on
two crossings holds the name at two sites, and a delete of its minting
node reports a row per site
(`edit_maintenance::the_delete_button_counts_a_name_an_instances_record_holds`).
The button says one name there too, because one `Rebind` repairs every
site. Whatever the net decides for the doubled report, the count of
distinct `(carrier, name)` answers is what the button keeps.

## What is asked

The net's rule for which rows survive is `MaintenanceNet`'s to state:
whether an action's net holds one row per stranded reference, and if
so which `Took` it says when two edits of the action each took
something from it. Not decided here.
