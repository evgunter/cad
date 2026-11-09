---
id: topo-has-no-predicate-name-roster
kind: issue
title: topo has no predicate-name roster, so no census reader sees the names its decide and read_identity sites pass
status: open
opened: 2026-11-09
priority: P3
cost: M
---


Found by QML T1's review (F7 of `question-mark-ladders-the-t1-review-left-unported`).

`test_utils::source::predicate_census` is the tree's one reader of the predicate names a crate decides under. Sweep (`crates/sweep/tests/recourse_roster.rs`) and profile (`crates/profile/tests/recourse_roster.rs`) run it, so a name decided outside their doors' closed types reds there until it is listed. Topo runs no such roster. The names its `decide*` calls and its `read_identity` data pass are read by nothing at the source, so:

- a name decided at two doors under one spelling, or one door's name drifting from the rows that render it, is caught only at runtime by the K-telemetry probe census, where a site is reached at all;
- the identity data the ladder ports moved off `decide("…")` (`rim_wedge::rides`' `seam_line_collinear`, `surface_group::WrapRims::holds`' `bool_wrap_rim`, `splitting::containment::certify_plane`'s `point_in_loop_*`, and the older `rim_circle_*`, `carrier_eq` data) left the literal at the datum, which the census cannot read (it reads names at `decide*` calls, or from a declared `NameCarrier`).

`geom_brep::ladder::read_identity`'s doc states the spelling rule: a literal at the datum, and a `const` declared as a `NameCarrier::Const` where a roster reads the crate (sweep's `LOFT_RIM_PINNED`). A topo roster would declare each identity datum's name that way and list the boolean, splitting and replace-face doors' names against their error types, as sweep's lists its doors.

**Closing this row updates `geom_brep::ladder::read_identity`'s doc** (`crates/geom-brep/src/ladder.rs`, the paragraph on where a datum's name is spelled), which names topo as a crate with no roster.
