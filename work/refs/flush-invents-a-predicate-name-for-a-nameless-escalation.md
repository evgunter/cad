---
id: flush-invents-a-predicate-name-for-a-nameless-escalation
kind: issue
title: eval: the flush selector invents a predicate name when an escalation carries none
status: closed
opened: 2026-09-17
closed: 2026-11-06
priority: P0
cost: E
---


## Finding

`crates/editor-core/src/names/flush.rs:271`, inside the pair walk that
builds a `SelectRefusal::PairInBand`:

```
    predicate: source.predicate.unwrap_or("carrier_pair_relation"),
```

An escalation that carries NO predicate name is published to the caller
under the name `carrier_pair_relation`, which nothing in the tree
decides: `grep -rn '"carrier_pair_relation"' crates/` finds this line
and no `decide` site. So a refusal whose name field is that string
means one of two different things — the pair-relation predicate really
escalated, or some predicate escalated anonymously and this line named
it — and a reader cannot tell which. `pncad-py` republishes the field
(`SelectRefusal.predicate`), so the invented name reaches Python.

The honest shapes are either to carry `Option<&str>` through to the
caller (the payload's own shape — `geom_core::MissingRecourse` renders
a nameless decision as one), or to attach the name at the leaf that
decides, which is the D4 ¶3 convention `Indeterminate::with_predicate`
already states.

Found by BLEND unit 15's v6 review (R2 NOTE-4) while sweeping for
predicate-name-keyed renderers. EVAL's ground; filed to `work/issues/`
because no program directory obviously owns `names/flush.rs`.

Moved from `work/issues/` to `work/refs/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

## Closed

The cited site was already gone: `names/flush.rs` and
`SelectRefusal::PairInBand` no longer exist, and `"carrier_pair_relation"`
survives only as a census row name in `topo::review_d18`. The class
survived elsewhere, and a shape sweep (`predicate.unwrap_or`,
`predicate.or(`, `map_or`, `match ….predicate`) found it at four sites:

- `editor_core::eval::measure::link_reads` published a nameless link
  escalation as `"gap_link_reading"`, which nothing decides.
  `PrimitiveRefusal::Escalated` and `NodeErrorKind::MeasureEscalated`
  now carry `predicate: Option<&'static str>`; the sentence renders a
  `None` as `geom_core::UNNAMED_DECISION`. (Its one producer,
  `topo::Link`'s refusal, decides under `link_plane`/`link_wedge` and
  names itself, so no run changes.)
- `topo::boolean::section_cert` credited a nameless escalation to
  `section_axes_offset` (a name geom-brep decides) or `pair_section`
  (decided nowhere); `topo::clearance` credited one to
  `CLEARANCE_MARGIN`. These are `&'static str` slots, so each now marks
  a nameless escalation `"<unnamed>"` — the marker `clearance::sliver`
  already used, now one helper there.

Why an escalation can arrive nameless at all is geom-core's:
`Indeterminate::predicate` is optional, and dozens of producers leave it
`None`. Filed as
`work/scalar/an-escalation-may-carry-no-predicate-name-so-every-publisher-must-mark-one.md`.
