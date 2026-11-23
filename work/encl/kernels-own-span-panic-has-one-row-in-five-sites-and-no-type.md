---
id: kernels-own-span-panic-has-one-row-in-five-sites-and-no-type
kind: issue
title: kernels_own's span panic is pinned at one of its five sites, and a re-mint site that forgets it is silently typed
status: closed
opened: 2026-10-19
branch: encl/kernels-own-type
pr: 5401
closed: 2026-10-19
---


Found by ENCL's param-span-doors final review (PR 5365).

`topo::policy_lane::kernels_own` (`crates/topo/src/policy_lane.rs:112`)
turns a `CertifyError::StatedInterval` at a kernel re-mint of stored
data into a kernel-bug panic. It wraps five sites:

- `transform_rigid` (`crates/topo/src/transform.rs:627`);
- a split child, `split_edge_minting` (`crates/topo/src/split.rs:497`);
- a graft's `Bridge::Recertify`, `graft_solids_impl`
  (`crates/topo/src/boolean/combine.rs:913`);
- `vouch_described_move` (`crates/topo/src/attach.rs:925`);
- `certify_rebased_run` (`crates/topo/src/euler.rs:4237`).

**A. Only the first is pinned.** The tier-3 row
(`crates/topo/src/tier3_tests.rs:1429`) drives `transform_rigid` into
the panic and reads its site name. No row drives the other four:
removing the wrapper at any of them leaves every test green, the
refusal then passing on typed as the caller's input.

**B. The wrapper is placed by hand.** A new kernel re-mint site that
forgets `kernels_own` is silently typed: nothing fails to compile and
no gate counts the sites. The removed `Interval::Kernels` argument made
every mint choose between the caller's interval and the kernel's at
the call; a type with that shape (a door argument, or a distinct
re-mint entry point whose error has no `StatedInterval` arm) would
force the choice again.

**Sweep:** `kernels_own` over `crates/topo/src` (five call sites,
above). Blind spot: a kernel re-mint that calls
`EdgeCurve::recertify*` or `certify*` without the wrapper matches no
spelling of it; `rg 'recertify(_via)?(_stated)?\(|::certify(_via)?(_stated)?\(' crates/topo/src`
outside tests is the census for B.

## Evidence, 2026-10-19 (PR 5393 review)

After PR 5393, `kernels_own` holds the only kernel-bug panic for an `Unminted` reading, so this row now carries all of D2's must-panic half for span checks. Also, topo arms that match `CertifyError::IntervalNotForward`/`WindingExceeded` look dead, since no door returns those variants: `replace_face.rs` ~2083 and `validate.rs` ~2859, 2913, 2999. Census them with the five sites. (ENCL orchestrator)
