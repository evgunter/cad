# M10-DI — the Dual contract implementation (unit spec)

**Status: BINDING at dispatch** (orchestrator-authored; every
decision here is ratified — `docs/DUAL-DESIGN.md` DL1–DL6, #1146).
Branch `m10/m10-di-dual-contract`. Sizing **M**. Read
`docs/prompts/implementer-discipline.md` in full first, then
DUAL-DESIGN in full, then `crates/editor-core/tests/e4_dual_door.rs`
and `crates/editor-core/src/eval/memo.rs` end to end.

## Scope

1. **DL2 — `impl<T: ContentBits> ContentBits for Dual<T>`**: feed
   the value channel's exact representation, then the tangent
   channel's (position separates them; both through the base
   scalar's own `feed`). `Dual<Interval>` comes free from the
   generic impl under the `interval` feature. Flip the two
   anticipating pins at their self-named update sites: the
   `e4_dual_door` suite rewrites to its successor law (its module
   docs say which day this is), and `memo.rs`'s `compile_fail`
   doctest becomes a passing companion. Update the S44/D1 record
   reference the suite names.
2. **DL3 — the scalar-policy seam**: certified validation (the
   product gather's `validate_geometric` + `recertify_approx`
   consumption, `product.rs:410/:445`; the census door) runs at
   scalars with certification rights and is STRUCTURALLY ABSENT at
   `Dual` — a typed policy on the scalar (marker on the lane-trait
   surface; exact spelling is the implementer's call, but it must
   be per-scalar and compile-time, never a runtime flag or a
   swallowed per-face error). f64 / Interval / Probe behavior is
   unchanged bit-for-bit.
3. **DL4 — gate**: `scripts/gates/bounds-allowlist.sh` greps
   `Enclosure` exactly as it greps `Bounds` (same allowlist; keep
   the `CertifiedBounds` definition-line skips working). Add a
   planted-red row proving the gate fires on a `T: Enclosure`
   bound outside the allowlist (the gate-tests pattern, if one
   exists; else a documented manual probe in the PR).
4. **DL5 — ledger**: record the delegation rule on the `Bounds`
   ledger in `real.rs` as the standing criterion (payload/report
   reads and value-channel-decided selections are lane-exempt;
   certificate-minting reads never are); retire the fillet seam's
   standing-obligation text by pointing it at the rule; update the
   `bounds-allowlist.sh` header pointer sentence accordingly.
5. **Acceptance (the door opens)**: `evaluate::<Dual64>` compiles
   and a full corpus build at `Dual64` SUCCEEDS — including
   documents with trimmed/spline faces (that is DL3 working) —
   with the value channel bit-identical to the f64 evaluation
   (assert per the `scalar_channels` precedent, over
   `corpus::documents()`). Memo-soundness rows: two passes with
   different seeded parameters never share a seed-downstream
   node's memo entry; parameter-independent nodes DO reuse when
   the prior evaluation is threaded; same-seed replay reuses
   everything.

**Out of scope**: any E4 API (seeding surface, sensitivities,
Stackup — M10-4's); any change to what f64/Interval validation
covers; DL6 (contract text, already ratified; its audit belongs to
the class issue).

## Review claims to falsify

1. f64/Interval/Probe evaluation is bit-identical to merge base
   (differential — the reviewer's unique signal).
2. Memo soundness at `Dual`: construct an aliasing attempt (two
   seeds, shared subgraph, threaded prior) and show the keys
   separate exactly where the seed reaches and merge where it
   does not.
3. DL3's policy is per-scalar and total: no site skips certified
   validation at f64/Interval; no site at `Dual` swallows a typed
   error instead of being structurally absent.
4. The extended gate fires (planted red) and stays green on the
   tree.
5. The corpus Dual build's value channel is bit-identical to f64
   at every node (not just final bodies).
6. e2e: drive a document build at `Dual64` through the public
   evaluation door as a consumer would and report the friction.

## Acceptance

Hosted CI green on the unit's own head; the interval lane matters
here (the generic impl is `cfg(interval)`-adjacent) — state the
drawn point and run `ci-local` only if the draw misses both axes
you touched and a red would be expensive (the standing calculus).
