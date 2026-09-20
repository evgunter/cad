---
id: descendant-chase-spends-its-budget-into-a-dropped-contact-record
kind: issue
title: KeyView's live_vertex/live_face answer None on a spent budget, so a cycling fusion row drops a declared contact instead of refusing
status: open
opened: 2026-09-13
---


## Finding

Found by WIRE's sweep for PR #2474's invariant — *a bounded lineage
walk whose budget is spent has revisited a key, and a revisit is a
corrupt record, so it refuses* — carried to its third `emit_topo`
instance. The sweep grepped the SHAPE (a cursor walked over a map
under a `0..=map.len()` budget) across `crates/editor-core/src/` and
`crates/topo/src/`; these two are the only hits outside `names/` and
`Body::split_root`, and they are BOOL's.

`KeyView`'s descendant chase in `crates/topo/src/boolean/ops.rs`:

```rust
fn live_vertex<T: Real>(&self, body: &Body<T>, v: VertexKey) -> Option<VertexKey> {
    let mut k = v;
    for _ in 0..=self.vertices.len() {
        if body.get_vertex(k).is_some() {
            return Some(k);
        }
        k = *self.vertices.get(&k)?;
    }
    None
}
```

`live_face` is the same walk over `self.faces`.

**It does not return a wrong key** — which is the half `emit_topo`'s
`chase` got wrong and this one gets right. What it does is answer
`None`, and `None` is *already* this walk's answer for the benign
case: `self.vertices.get(&k)?` returns `None` for a key with no
further row. So the two are indistinguishable at the call site, and
both callers — the `vert`/`face` closures in `remap_contacts` and in
`remap_carried` — read `None` as *this entity was genuinely consumed*
and drop the contact record. A cycling fusion row therefore silently
loses a DECLARED contact rather than refusing, and the declaration
layer's whole argument is that a declared contact is not dropped
without a word.

## The repro, measured

Not a reading. WIRE's review lane built it and ran it, in
`boolean::ops`' own test module, against two mutually-referring dead
face keys and one declared v-on-f contact:

```
PROBE_B a_on_b after a CYCLE:    0 record(s)
PROBE_B a_on_b after a DEAD END: 0 record(s)
```

The two are **indistinguishable at the call site**, which is the
finding. The probe is committed beside this row at
`work/bool/probe-descendant-cycle.patch` — apply it to
`crates/topo/src/boolean/ops.rs` to re-take the measurement. Authorship
is the review lane's (2026-09-13); it is a probe, not a merge
candidate, and it asserts the two counts EQUAL so that separating them
is what turns it red.

## The comment that says otherwise

The row's own comment asserts the cycle cannot happen — *"rows never
cycle: a dead key maps to its survivor"* — which is exactly the
argument `emit_topo`'s three chases carried before one of them was
measured. If it is true, the budget and the fallthrough are both
dead code and the walk should say so loudly; if it is not, the
fallthrough is a dropped declaration.

## What a taker owes

A decision, not a patch: either the fallthrough is unreachable and
becomes a refusal that says so, or the two `None`s are separated so
a spent budget is distinguishable from an exhausted chain. Either
way the cost lands in `BooleanError`'s vocabulary and therefore in
`crates/pncad-py`'s `boolean_error_tag`, which is what makes this a
BOOL decision rather than a sweep WIRE could have finished.

`crates/topo/src/boolean/ops.rs` is BOOL's and CURVED's
(`work.py territory`); filed on BOOL's slate as the program whose
declaration semantics the drop lands on.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to CURVED (its charter names S-BOOL's ceded ground and inherits at S-BOOL's exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.
