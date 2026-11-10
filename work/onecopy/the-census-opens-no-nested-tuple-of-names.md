---
id: the-census-opens-no-nested-tuple-of-names
kind: issue
title: The predicate census opens no array of (name, value) tuples handed to a funnel: torus_arms' signs, read_identity test data, on_surface's (name, side)
status: closed
opened: 2026-11-10
priority: P3
cost: E
closed: 2026-11-10
pr: 0
---


Found by ONECOPY's roster-reach unit.

A declared `NameCarrier::Call` reads the plain literals among its call's direct arguments, and a mixed `(…)`/`[…]` group at a `decide*` site reads its direct literals. Neither opens an array of tuples, or a tuple handed as one argument. Three spellings hide names this way:
- `crates/geom-brep/src/pair_section/torus_arms.rs`: `signs([("name", m), …], band)`. These names are read nowhere.
- `crates/geom-core/src/ladder.rs` unit tests: `read(&[("a", 0.0), ("b", 5e-9), …])`, read through `read_identity`. The references are marked `UNDECIDED-NAME` for now.
- `crates/topo/src/boolean/contact_verify.rs`: `on_surface(("probe_on", PairSide::Second), …)`. Also marked.

**Done** when a carrier's group opens a nested tuple's first literal, or each funnel takes its names another way, and the three markers above are gone.

**Closed** (2026-11-10, ONECOPY census-tables). A declared `NameCarrier::Call` now opens its tuples (`test_utils::source::tuple_heads`). It reads the first element of each tuple among its call's top-level arguments, and of each tuple in an array among them, behind an `&` or not:
- a plain literal there is a name;
- a `const` there is a const site, held to a declared carrier;
- anything else leaves the call a site for what it does not spell.

The three spellings:
- `pair_section/torus_arms.rs`'s `signs` is renamed `signs_as` and declared in geom-brep's and topo's rosters. topo's `section_cert.rs` imports it. Its batches are now read: `section_torus_square_wall_tube` and its siblings, and the `SPHERE_NEAR`/`SPHERE_FAR`/`SQUARE_EDGE_OVER`/`SQUARE_FOLD_HALF` and topo `NEAR`/`FAR`/`REACH`/`NEST` consts, now declared carriers, which no census read before.
- `geom-core/src/ladder.rs`'s unit-test reader is `read_as`, declared, and its rows are read. Its two `UNDECIDED-NAME` markers are gone; the marker ratchet reds them now.
- `topo/src/boolean/contact_verify.rs`'s `on_surface` is `on_surface_as`, declared. Its unit test's `probe_on` is read, the marker is gone, and `probe_on` is listed as a unit test's exception.

Plant: a fixture naming `section_torus_square_wall_tube` was red before and is green now. Unit test: `a_declared_table_and_a_carriers_tuples_are_read`.
