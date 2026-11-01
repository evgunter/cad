---
id: a-single-source-product-re-gates-a-body-its-verb-already-gated
kind: issue
title: a one-source product re-gates at tier 3 a body its verb's last act already gated (shell_open)
status: open
opened: 2026-09-28
priority: P4
cost: M
---


Found by the sweep of `gather/assemble-single-local-battery`, which
made `editor_core::assemble` pay tier 3's local battery once per
aggregate. The sweep looked for callers that gate a body and then gate
the same body again, and this is the one instance where the second gate
is on a copy of the first gate's body rather than on the body itself.

## The finding

`topo::shell_open` (`crates/topo/src/shell.rs`, its last act) runs
`validate_geometric` over the body it built. A document whose one
body-denoting root is a shell node therefore reaches
`product_recorded` (`crates/editor-core/src/product.rs`) with a body
tier 3 has already passed. The gather grafts it into a fresh aggregate
(`graft_disjoint_all_keyed`) and gates that aggregate
(`T::gate_at_rest_kept`) with the same battery. The aggregate of one
source is a key-renamed copy of the source, so the second battery
re-derives a verdict the verb already had.

## Why it is not simply fixed

The verdict is about the verb's body, and the gate's subject is the
graft's output. Carrying it across the graft takes an argument that a
keyed disjoint graft of one validated body preserves every tier-3
verdict, and nothing states or tests that argument today. It also
helps only one-source products, since an aggregate of several sources
is a body no source was gated as. It is a cost row, not a correctness
one: no measurement has been taken, and a shell is not the common root.

## Re-homed

Moved from `work/gather/` with GATHER's close (2026-09-29), id kept. PERF's charter is the cost owed; the double gate is a cost row, and the product gate it touches (`crates/editor-core/src/product.rs`) has no other live owner besides WIRE.
