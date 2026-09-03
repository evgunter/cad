---
id: stored-spans-read-raw-past-winding-bound
kind: issue
title: "props: two more stored spans read raw past the winding bound (torus single-edge meridian; the rim Δu sum for all four kinds)"
status: open
opened: 2026-09-03
github: 1618
refs: [1617, saturated-sphere-span-folds-short, MESH-12, MESH-R]
---

## From GitHub issue 1618

opened 2026-09-03, 0 comments.

**Found by:** MESH-12's class sweep (PR #1617, the sweep table), filed by the S-MESH orchestrator. Same class as issue 1601 (closed by MESH-12): a stored edge span `e.t1 − e.t0` read into a closed form without re-deciding certification's per-edge winding bound. MESH-12 closed the sphere meridian arm (`props_meridian_span_winding` at the parse and at the branch door); MESH-10 closed the torus CHAIN arm (`props_meridian_pieces_winding`, chains of ≥ 2 pieces). Two homes remain:

1. **Torus, single-edge meridian** (`crates/geom-brep/src/props/curved.rs` ~:1937 / ~:1985 — `m0.dt.sin_cos()`, `p.anchor.dt` into `require_extent`). `fold_chain`'s single-edge arm says "a single edge is its own certified interval and re-decides nothing", so a hand-built single torus meridian of span 3π folds through `sin`/`cos` silently. Fix shape: the same fn as MESH-12's `require_meridian_span_within_period`, applied in `fold_chain`'s single-edge arm (the torus arm is MESH-10's home).
2. **The rim Δu sum, all four kinds** (`curved.rs` ~:1315 / ~:1467 / ~:1852 / ~:2090 — `dt: e.t1 − e.t0` for rims → `du_of_rims`). A rim span past τ inflates Δu. Different premise from the meridian decide (the sum, one home for four kinds), so the decide belongs in `du_of_rims` for all kinds at once, not in one kind's parse. MESH-12's control row `a_full_period_rim_is_not_a_meridian_span` pins that the meridian decide must NOT touch rims; the rim decide is a second named key.

**Reach:** hand-built `LoopEdge::hand_built` only. Every certified door (`Body::mev`, `set_edge_curve`, `split_edge`, import's `endpoint_params` minting `t1 ∈ (t0, t0 + τ]`) bounds the span at τ + zero/R (MESH-12's measurement 1). So no certified body is wrong today; the gap is that a consumer building a loop without a body gets a silent fold instead of a typed refusal, and the never-infer ladder wants the premise decided, not assumed.

**Owes:** two named decides (rows in `docs/predicate-dimension-audit.md`), red-first rows per consumer on hand-built loops (the MESH-12 suite's shape: one row per consumer on the same pair), a control that a full-period rim is admitted by the rim decide (period, not less), D9 identical.

Difficulty S. Band: S-MESH (1200–1299). Not scheduled; a candidate for the MESH-R track lane after MESH-12.

## Home

`work/mesh/` — filed by the S-MESH orchestrator out of MESH-12's class sweep, named for the S-MESH band and the MESH-R track lane, continuing the meridian-winding class MESH-12 and MESH-10 closed.
