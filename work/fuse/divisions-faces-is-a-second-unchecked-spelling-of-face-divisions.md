---
id: divisions-faces-is-a-second-unchecked-spelling-of-face-divisions
kind: issue
title: lineage::Divisions.faces is a second, unchecked spelling of face division rows beside FaceDivisions
status: closed
opened: 2026-10-12
priority: P3
cost: E
pr: 4809
closed: 2026-10-12
refs: [face-lineage-rows-are-checked-by-their-readers-not-where-they-are-added, 4868]
---


Found by lane A's review of PR 4868 (`fuse/face-lineage-at-append`),
which made the boolean's and the split's face division rows a
`crate::lineage::FaceDivisions`: a list that refuses, where a row is
added, a self-row or a new face an earlier row names.

`lineage::Divisions` (`crates/topo/src/lineage.rs`) is the form a
surgery verb hands its record carry
(`ContactRecords::carried_through_pieces`), and its `faces` field is a
plain `Vec<(FaceKey, FaceKey)>` of the same rows. Three builders:

- `splitting/finish.rs` (`SplitResult::carry_contacts`): filtered from
  `SplitNaming::face_fragments`, already a `FaceDivisions`, so its rows
  were checked where the join wrote them;
- `shell.rs` (`ShellResult::carry_contacts`): each outer rim's holes,
  `(hole face, rim face)`;
- `sweep/src/blend/build.rs`: always empty.

Not converted in PR 4868, because it is not local:

- `Divisions`' own doc admits a row whose two cells are one ("no
  division, read as none"), so a `FaceDivisions` would refuse rows the
  type accepts today;
- a refusal would surface from `carry_contacts`, whose error is the
  public `CarryRefusal`, so it would owe a new arm with its pncad tag;
- and there is no reader refusal to retire: the one reader,
  `Lineage::of`, is a visited-set walk, total over any rows.

What is owed: decide whether `Divisions.faces` becomes a
`FaceDivisions`, measuring first whether `shell.rs` ever writes a
self-row or names a hole face twice, and give `CarryRefusal` its arm if
it does.

## Closed

`lineage::Divisions` is gone: CONTACTHOLD's PR 4809 retires it with
the carry it fed (`carried_through_pieces`). A surgery verb now states
each cell's fate in a `topo::Lineage`, and the split reads its face
divisions straight off `SplitNaming::face_fragments`, the checked
`FaceDivisions` (`SplitResult::lineage`, through `FaceDivisions::rows`).
The shell states each opened face's division as a `Fate::Divided`, not
as `(hole, rim)` rows, so no unchecked spelling of face division rows
is left for a `FaceDivisions` to replace. The visited-set walk
(`Lineage::of` on main, `lineage::Descent` here) is the one reader
and stays total.
