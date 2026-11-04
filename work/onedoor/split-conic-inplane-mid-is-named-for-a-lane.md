---
id: split-conic-inplane-mid-is-named-for-a-lane
kind: issue
title: chord_join decides split_conic_inplane_mid, a K name with a lane's prefix, not its deciding module's
status: open
opened: 2026-11-03
---


Found by `crossing-doors-leave-topo-for-geom-brep`'s rename sweep (2026-11-03).

`crates/topo/src/chord_join.rs` (`between_edge_is_section`) decides `split_conic_inplane_mid`, a crate-root module deciding under the splitting lane's prefix. `[topo.homes]`: a K name's prefix is the module that decides the margin. One site, one population, so a rename (`chord_join_conic_inplane_mid`, say), recorded in the K-REPORT census and in `docs/predicate-dimension-audit.md`'s `chord_join.rs` row.
