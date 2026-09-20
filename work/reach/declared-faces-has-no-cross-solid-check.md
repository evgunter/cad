---
id: declared-faces-has-no-cross-solid-check
kind: issue
title: Declared.faces has no cross-solid check, so a contact record naming two faces of the SAME solid would back events within it
status: open
opened: 2026-09-16
refs: [750]
priority: P3
cost: E
---

The adjacent observation from issue 750, filed at BOOL-4's spec time
as `work/bool/plan.md` directs. In `crates/topo/src/census.rs` the
`Declared` index built from `ContactRecords` is consulted by the
sweeps and the backstop as "the certifier's pair" without checking
that the two faces a record names belong to DIFFERENT solids; a
record naming two faces of one solid would defer intra-solid events
to a confirm pass that reads them as an interface. Not claimed as a
defect on any document in the corpus — noted because it is adjacent
to the containment examination BOOL-4 rewrites, and a record that
turns an intra-solid check off is the same shape PR 737 removed. The
fix is a structural check at `Declared::index` (a record's faces
resolve to two solids, refused typed otherwise). Measured, not acted
on; difficulty S.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to CURVED (its charter names S-BOOL's ceded ground and inherits at S-BOOL's exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.
