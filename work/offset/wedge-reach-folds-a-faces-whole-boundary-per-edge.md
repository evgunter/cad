---
id: wedge-reach-folds-a-faces-whole-boundary-per-edge
kind: issue
title: The attach setter's wedge reach folds each listed edge's faces' whole boundaries, so a face with n edges costs n² across its rim
status: open
opened: 2026-10-24
priority: P3
cost: E
---

Found by `a-staging-doors-end-derives-tier1-once-and-its-orphan-sweep-is-batched`
(OFFSET, 2026-10-24) while profiling an n-gon prism offset.

`Body::set_face_surfaces_certifying` (`crates/topo/src/attach.rs`)
asks a wedge reach of every listed edge: the largest distance from the
edge's midpoint to any boundary point of its two faces
(`wedge_reach::WedgeReach::reach`, `crates/topo/src/wedge_reach.rs`).
The setter now holds one `WedgeReach` for the call, so a face's boundary
points are collected once, but each edge still folds over all of them.
A rim edge of an n-gon prism lies on a cap with n boundary points, and
there are 2n rim edges, so the call costs O(n²) for that fold alone:
callgrind, release, `ngon_prism_offset_scaling` at n = 64 / 256 / 1024,
about 0.9 G of the door's 1.8 G instructions, and the reason the door
is 1.15 / 6.0 / 49 ms rather than linear.

The fold is exact and order-free (a max), so a bound that answers in
less than O(boundary) per edge has to give the same number, or the
certification that reads the reach has to be shown to accept an upper
bound. Which one is a design question about what `reach` is for: read
its callers (`policy_lane::certify`) before choosing.
