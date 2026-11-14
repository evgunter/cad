---
id: collapsed-locus-refusal-has-no-reachable-fixture
kind: issue
title: The collapsed-line refusal's wording pin has no reachable fixture once the glue door reads the cover
status: open
opened: 2026-11-14
---

`fb4087a837` (PR 6890, "the collapsed locus named") added a glue-door part to `crates/topo/src/boolean/recl.rs:2805` `an_edge_edge_membership_tie_offers_a_declaration_only_where_one_settles_it`. Its fixture pairs a skeletal cylinder face with a skeletal plane face, each with a lone-vertex outer loop. Before PR 6596 (BOXES `cover-levers`), that pair reached the tangency reading and refused on `LeverArm::TangentLocus`, which pinned the collapsed-line refusal's wording.

Since PR 6596, the glue door reads the face's cover. A face whose loop is a lone vertex has no cover, so the door reads no consumed extent (`PairUnread::Extent`) and leaves the pair to the stage that meets it. The merge re-measured the row instead of steering it back: it now asserts that the door returns Ok with no glued pair.

The refusal site is still counted in `offer_rows`, but nothing now reaches its wording. To pin it again the row needs covered faces whose witness line has zero length, for example two coaxial cylinder faces with real loops that touch along a degenerate line. Alternatively, show that the cover reading makes the site unreachable and retire it.
