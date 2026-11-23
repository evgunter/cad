---
id: select-refusal-inband-copies-its-sources-predicate-name
kind: issue
title: SelectRefusal::InBand carries a predicate name beside its source, which already records it
status: open
opened: 2026-11-06
priority: P4
cost: E
---


## Finding

`editor_core::SelectRefusal::InBand`
(`crates/editor-core/src/names/geompred.rs:204`) carries
`predicate: &'static str` beside `source: geom_core::Indeterminate`.
Its producers fill the field with the name the funnel already recorded
on the source: `geompred.rs:589` (`SEL_PLANE_DISTANCE`, beside an
escalation from `pose::pose_distance_sign`, which decides under it),
and the sample rows `geompred.rs:661` and `decision.rs`
(`a_selections_undecided_distance_ends_sized`). So the field copies
`source.predicate`, and a producer that passed a different name would
invent one: the class
`work/refs/flush-invents-a-predicate-name-for-a-nameless-escalation.md`
closed.

PR #6543 removed the same copy from `NodeErrorKind::{Escalated,
MeasureEscalated, PlacementAxisEscalated, PoseDirectionEscalated}`,
`NamingError::Escalated` and `eval::measure::PrimitiveRefusal::Escalated`,
which now read `source.predicate`. `InBand` was left because its field
is Python-visible: `crates/pncad-py/src/py/select.rs:779`–`:786`
publishes it as `SelectRefusal.predicate`, a `str`, which reading
`source.predicate` would make optional (stub, docs and
`work/bind/select-refusal-predicate-names-are-unpinned.md`'s pin
follow).

## Shape

Drop the field and read `source.predicate`; the Python attribute
becomes `str | None` or renders a `None` as
`geom_core::UNNAMED_PREDICATE`. `decision::ending` already takes an
`Option<&str>`.
