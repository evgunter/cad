---
id: flush-invents-a-predicate-name-for-a-nameless-escalation
kind: issue
title: eval: the flush selector invents a predicate name when an escalation carries none
status: open
opened: 2026-09-17
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
