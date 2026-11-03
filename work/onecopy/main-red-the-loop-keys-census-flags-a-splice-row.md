---
id: main-red-the-loop-keys-census-flags-a-splice-row
kind: issue
title: main red: live::tests::loop_keys_is_read_only_where_a_miss_is_reported flags boolean/spline_region_rows.rs, a splice row that reads Face::loop_keys
status: open
opened: 2026-11-03
priority: P1
cost: E
---



Found by `onecopy/foot-certificate`, which ran topo's full suite on a branch that touches neither file.

`topo live::tests::loop_keys_is_read_only_where_a_miss_is_reported` panics at `crates/topo/src/live.rs` (~1536):

> a face's loops resolved outside the home: ["…/crates/topo/src/boolean/spline_region_rows.rs"]

Two merged PRs crossed:
- PR 6460 (`onecopy/face-loops`, `7c1c89b76`) made the census, which allows a face's loops to resolve only in their one home.
- PR 6435 (`splice/held-pierce`) landed `925fa061a`, "the seam row reads a face's loops through Face::loop_keys", in `boolean/spline_region_rows.rs`.

The test is in the slow set (`.config/nextest.toml`), so neither PR's gate ran it.

Fix: the splice row reads the face's loops through the home PR 6460 made (or the census lists it with its reason), whichever its own row text supports.

Reproduce: `cargo nextest run -p topo -E 'test(=live::tests::loop_keys_is_read_only_where_a_miss_is_reported)'`.
