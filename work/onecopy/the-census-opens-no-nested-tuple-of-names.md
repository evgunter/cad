---
id: the-census-opens-no-nested-tuple-of-names
kind: issue
title: The predicate census opens no array of (name, value) tuples handed to a funnel: torus_arms' signs, read_identity test data, on_surface's (name, side)
status: open
opened: 2026-11-10
priority: P3
cost: E
---


Found by ONECOPY's roster-reach unit.

A declared `NameCarrier::Call` reads the plain literals among its call's direct arguments, and a mixed `(…)`/`[…]` group at a `decide*` site reads its direct literals. Neither opens an array of tuples, or a tuple handed as one argument. Three spellings hide names this way:
- `crates/geom-brep/src/pair_section/torus_arms.rs`: `signs([("name", m), …], band)`. These names are read nowhere.
- `crates/geom-core/src/ladder.rs` unit tests: `read(&[("a", 0.0), ("b", 5e-9), …])`, read through `read_identity`. The references are marked `UNDECIDED-NAME` for now.
- `crates/topo/src/boolean/contact_verify.rs`: `on_surface(("probe_on", PairSide::Second), …)`. Also marked.

**Done** when a carrier's group opens a nested tuple's first literal, or each funnel takes its names another way, and the three markers above are gone.
