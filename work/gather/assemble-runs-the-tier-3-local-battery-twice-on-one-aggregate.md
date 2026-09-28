---
id: assemble-runs-the-tier-3-local-battery-twice-on-one-aggregate
kind: issue
title: assemble runs the tier-3 local battery twice on one aggregate: once in the product gather's gate, again inside tier 3′
status: open
opened: 2026-09-28
priority: P3
cost: M
refs: [product-gate-refuses-a-declared-cusp-sweep-the-verb-now-declares, product-per-part-gate-counts-solids-but-gates-sources]
---

Found by the sweep of `gather/per-part-aggregate-gate`, which removed
the product's own per-source-then-aggregate double (PERF-SCAN-2026-08
item 16). The sweep looked for the same shape one level up: a caller
that gates a body and then gates the same body again.

## The finding

`editor_core::assemble` (`crates/editor-core/src/assembly.rs`) is
`product_recorded` followed by `assemble_gathered`:

- `product_recorded` (`crates/editor-core/src/product.rs`) gates the
  aggregate with `T::gate_at_rest`, which is `topo::validate_geometric`,
  the tier-3 local battery;
- `assemble_gathered` then gates the SAME aggregate with
  `T::gate_at_rest_declared`, which is `topo::validate_pseudomanifold`,
  and its body (`pseudomanifold_certificate_via` in
  `crates/topo/src/validate.rs`) runs `tier3_local_checks` verbatim
  before the census.

So every successful `assemble` reads every face through the local
battery twice. Not measured here; the product module doc's figures
(the heat sink at 160 fins: ~250 ms for the gather, ~11.4 s for tier 3′)
suggest the second local pass is small beside the census, so this is
a cost row rather than a hot one.

## Why it is not a plain duplicate

The two gates are not the same predicate. The product gate reads NO
declarations, so it refuses what tier 3′ would accept given the
records the product carries (a declared cusp: `UndeclaredCusp`, the
dispatched row
`product-gate-refuses-a-declared-cusp-sweep-the-verb-now-declares`).
Whatever that row decides about which gate the product runs decides
this one's shape too, so this waits on it: once the product gate reads
the records it carries, the assembly path could hand the product's
verdict on the local battery to tier 3′ rather than re-derive it.

## Update (branch `gather/derive-cusp-legality`)

The difference above is gone. The row it waited on resolved by deriving
wedge-end legality at rest (D1 tier 3, PR 3317): check 4 reads no
declaration, `UndeclaredCusp` is retired, and tier 3′'s local battery
no longer maps the aggregate's curve records to `Tangent` declarations
(`pseudomanifold_certificate_via` calls `tier3_local_checks` with the
body alone). So the product gate's `gate_at_rest` and the first half of
`gate_at_rest_declared` now run the same checks on the same aggregate,
differing only in the gating the door roster states (the composed door
makes check 7 behind the whole structural phase, the battery behind
checks 1–6), and the second run is a duplicate in substance: the
assembly path could hand the product's local verdict to the census.
