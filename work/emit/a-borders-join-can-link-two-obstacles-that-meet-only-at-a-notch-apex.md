---
id: a-borders-join-can-link-two-obstacles-that-meet-only-at-a-notch-apex
kind: issue
title: The Borders chase links every discard one piece edge reaches through a join, which joins two obstacles that meet only at a reflex apex of a non-convex parent
status: open
opened: 2026-10-13
priority: P3
cost: M
---


## What

PR 4949 made the `Borders` chase (`names/borders.rs`, `Obstacles::walls`) follow the output stage's `EdgeJoin` from `kept` to every `gone`. Discards that one piece edge reaches are now one obstacle. The argument for this sits in the module doc and at the join chase: "both border it on its far side".

N2 defines an obstacle as "a connected part of the parent's region". The far-side argument gives connectivity only when the parent's region is locally convex at the join vertex.

**The case that breaks it** (from the PR 4949 review; not built):
- a non-convex parent, e.g. an extruded profile with a V-notch;
- the notch's apex v lies on the far-side wall line;
- the discards over [x, v] and [v, y] touch only at v, so N2 makes them two obstacles;
- v ends at degree 2 and gets joined, so the chase links them into one.

The same premise covers a later split of a joined `kept` edge. That split climbs to `kept`, then follows `joined_from` into discards bordering a stretch it does not lie along.

**What guards this today:**
- A gross over-join (join every discard of the parent) turns four `emit_union_borders` rows and `name_words_corpus` red.
- No row covers a non-convex parent at a join.
- No row pins the join rule directly. It is held only by order-consistency over `cross`, and an over-join that every order makes alike passes that.

**Owed:**
- a fixture of a non-convex parent at a join;
- a join rule that reads connectivity (N2) rather than reachability.
