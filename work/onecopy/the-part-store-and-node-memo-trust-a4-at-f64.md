---
id: the-part-store-and-node-memo-trust-a4-at-f64
kind: issue
title: "The part store and node memo trust A4: at f64, a resolver answering a pin with other content is reused under that pin, carried costs included"
status: open
opened: 2026-11-16
priority: P3
refs: [6790]
---

This is the residue PR 6790 leaves, from its review (R1).

PR 6790 pairs a **dual** run's part rows and memoized instances against the record by content. At **f64** there is nothing to pair against, because an f64 run is its own record. So `PartStore::enter` reuses a prior's part row by key (doc ref and pin), the instance memo then hits on it, and the carried costs are keyed by pin too. All of this rests on A4: a pin always names one content.

A resolver that breaks A4 gets reused under that pin. One example is a mutable store answering an old pin with new content.

**Route 2** is the only route that closes this: the kernel checks A4 when it resolves a reference (`Entered::enter`, beside `ReferenceCycle`), by canonicalizing the resolved document and comparing it with its pin. The cost is one canonicalization on every f64 part build. 6790 declined to pay that, so this row now holds that decision. The two rows that pointed here are closed: `a-dual-part-bound-under-…-reads-record-refused` and `a-dual-instance-memo-hits-a-prior-built-from-content-the-record-never-read`.

**Open question (R2):** does a prior survive a change of `opts.resolver` between two f64 runs, or does `RunLane` or the DI3 prior check refuse it? If it survives, the residue is reachable just by swapping resolvers, not only through a mutable store. Settle this first.

**Done when:** either the kernel checks A4 at resolve time, with its cost measured and pinned by a row in which a pin answered with other content is refused; or DESIGN states A4 as a premise the kernel does not check, naming who owns it.
