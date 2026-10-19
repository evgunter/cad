---
id: kernel-computed-specs-at-callers-sites-return-a-kernel-bug-typed
kind: issue
title: A spec the kernel computed reaches a Callers certification site, so its span refusal returns typed instead of panicking
status: closed
pr: 5413
opened: 2026-10-19
branch: encl/kernel-specs-origin
closed: 2026-10-19
---


Found by the review of PR 5401 (`encl/kernels-own-type`).

Every `topo` edge certification now names an `Origin`
(`policy_lane::Origin`, `crates/topo/src/policy_lane.rs`). The Euler
and attach doors pass `Origin::Callers`, because the spec they certify is
the one their caller handed them. A span refusal there stays typed.

That is right when the caller is a user. But booleans and other kernel
operations call the same doors with specs **the kernel computed**, and
then a span refusal is a kernel bug that D2's must-panic half says should
panic. Today it comes back typed:

- `Body::mev_fan` / `mev_lone` (`crates/topo/src/euler.rs` ~2739, ~2990),
  `mef_chords` / `mef_lone` (~3219, ~3391), and
  `Body::set_edge_curve` (`crates/topo/src/attach.rs` ~1260), each
  passing `Origin::Callers`;
- driven with kernel-computed specs by the boolean's edge join:
  `kev_describing` (`crates/topo/src/boolean/edge_join.rs` ~1072, whose
  gate is `kev_describing_gate`, `crates/topo/src/euler_kill.rs` ~1470),
  `mef_chord` (~1107) and `mev_line` (~1117).

**Repair shape.** The driving kernel operation passes `Origin::Kernels`
(with a new `policy_lane::KernelSite`). That needs an origin parameter one
level up, on the `pub(crate)` form of each door the kernel calls, while
the public door keeps `Origin::Callers`.

**Sweep owed.** Every production caller of `mev*`, `mef*`, `kev_describing`
and `set_edge_curve` outside `euler*.rs` and `attach.rs`, sorted by
whether its spec is a caller's or the kernel's.

**Resolution (2026-10-19).** The sweep found the cited `edge_join`
drivers to be input-reachable or test code: `join_one`'s `kev_describing`
states a span joined from two edges of a body any caller can hand
`join_edges`, and the `mef_chord`/`mev_line` calls near it are in the
`conventional` test module. The kernel-computed specs at `Callers` sites
are the ones that restate a stored edge's carrier and interval: the seam
description pass (`describe_edges`), a split's section boundary
description and section re-chart, a shell's rim re-chart,
`describe_at_rest` and the parted-wrap restatement. Each now names a
`KernelSite` through a `pub(crate)` door that takes only the new
description and restates the stored carrier and interval itself
(`Body::restate_edge`, `Body::rechart_restating`). The PR carries the
full site table.
Sweep's own restatements go through the public door and are filed as
`work/encl/cross-crate-restatements-of-stored-edges-certify-as-callers.md`.
