---
id: a-documents-registrations-do-not-reach-its-products-gather
kind: issue
title: A document's registrations do not reach its product's gather: the gather decides in a scope opened over no record, though the instances it gathers feed it
status: open
opened: 2026-11-24
priority: P1
cost: M
---


Found in the delta review of the session-state unit (PR 7492).

**The fact.** `product::gather_sources`
(`crates/editor-core/src/product.rs`) runs the gather in a scope of its
own opened over no record (`geom_core::sym::apart`). On main before the
session-state unit there was one registry per replay, so the gather's
decisions at a symbolic scalar read every registration the replay had
made. Now they read none. ERROR-DESIGN E12 says a registration is read
by the node that made it and the nodes it feeds; the gather is fed by
the instances and bodies it gathers, so by that rule it reads their
records.

**Why it is not fixed in PR 7492.** Two of the gather's three doors
hold pieces and no node value: a door's census over its pieces
(`census_of`) and a pick (`gather_pick`) build their `Source`s from
`eval::Pieces`, which carries bodies, tables and carries and no
symbolic record. Only the plain read (`product_recorded`'s last arm)
has the `NodeValue`. Handing the gather its sources' records means a
record on each piece (`eval::Pieces`, `Carry` or beside them), which is
the ground the gather-carries unit is working in
(`the-gathers-finish-re-derives-certificates-its-moved-members-carry`).

**What done looks like.** Each `Source` carries the record of the value
it came from; `gather_sources` hands them to its scope in source order
(`geom_core::sym::reads`); a row over a product whose sources register
goes red when the gather reads none (`Product::sym`'s
`NodeRecord::read` is zero today on every document).

**Not measured**: whether any decision of a gather in the suites or the
tour needed a registration. No pin moved when the gather stopped
reading them (PR 7492's differential), so none that is pinned did.

**Also**: where the gather refuses, its record is dropped
(`gather_sources` writes the record on `Ok` alone; `ProductError` has
no place for one). A replay's receipt is then short by what the
refused gather decided.
