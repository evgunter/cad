---
id: union-face-names-follow-fold-order
kind: issue
title: A union's face names follow fold order: merged-then-cut is Merged(set)#SideOf, cut-then-merged is the merge plus a bare constituent, and two-step cuts stack SideOf
status: open
opened: 2026-09-25
priority: P1
cost: H
---


## What

This is what is left of `declared-flush-union-edge-and-vertex-names-follow-member-order`
after PR 3198. Vertex and member-edge names are now order-free. On the
rebind probe, 816 (order, name) absences remain: 486 faces and 330 seam
edges. Every seam edge among them cites one of those faces. They come
in two shapes:
- **Merged then cut, against cut then merged.** Merging first publishes
  `Merged(set)#SideOf(..)`. Cutting first, or cutting in the merging
  step, publishes the merge plus a bare constituent.
- **Cut in two steps, against one.** Two steps stack two `SideOf`
  qualifiers in fold order. One step gives a single `SideOf` with four
  partners. The undeclared `r1two` has this shape.

## The fork (needs Ev)

Fixing either shape means choosing a canonical fragment form for a
union's faces, which decides how N2 and N3 compose. There are two
options:
- **Re-derive** every face group's qualifiers over the finished body,
  the way edge ranks were re-derived in #3168 and #3198. Measured, it
  renames 90 of the 702 distinct published face names on the rebind
  probe, not every fragmented face (log, 2026-09-25).
- **Refuse** these shapes. A refusal that does not itself follow order
  has to refuse the document in every order: `r1two` and 25 other
  documents of the probe's 45.

The recommendation is to re-derive, over each face's merge closure;
N2 and N3 in `crates/editor-core/src/names/README.md` state the rule.

## Also here

- **The collision freeze in `retire_into_merges` is silent.** When
  rewriting two rows would make their names collide, both keep the
  fold's own spelling, which embeds a retired constituent. N3 would
  have that reference fail with the merge offered. Main does the same
  more often (18 sides against 8 on the ZIP document), so this is not
  a regression, but a face rule should settle it.
- **`side_is` in `a_union_cites_only_what_the_finished_body_holds` is
  loose.** It accepts any constituent or sub-merge of the face's merge
  without a geometric test. A geometric check found nothing it masks
  today.
- **The rebind row's `cases()` omits the ZIP document.** On it, the
  row's own neighbour signature shows 32 mismatches on #3198's head
  against 12 on main. All of them trace to the leftover vertex, which
  is the zip row's defect.

## Ruled (2026-09-25)

Ev chose (a) on #3222 ("(a) sounds great!"). A union names its faces
from the finished body, as N2 and N3 now state in `names/README.md`:
- a face's parent is its merge closure, named `Merged(closure)`, or the
  bare member face when nothing merges it;
- a parent held as one face takes the parent's name;
- a parent held as several faces gives each one `SideOf`, whose
  partners are the parents across the group's seams;
- a seam cites the parents on its two sides.

This row is now the build. It is expected to close
`a-face-cut-and-merged-in-one-step-publishes-a-piece-under-the-name-its-merge-retires`
along with it, since under the rule a constituent is never published
beside its merge.
