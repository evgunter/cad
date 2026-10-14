---
id: a-declared-tangent-seam-flips-invalid-when-eps-tightens
kind: issue
title: A declared conventional seam valid at one ε refuses TangentNotIntrinsic at a tighter one (main): D2's 'never flip' and 'exempt by its own declaration' are both false
status: open
opened: 2026-10-14
priority: P2
cost: E
---


## Finding

Filed by TANG's `OnPair` build (PR 4998) from its second full review. The flip is on `main`; it predates that PR and is for Ev.

D2 (`docs/DESIGN.md:785-789`, Ev's b846df1f10) says two things the code does not do:

1. "The check reads the edge's authority record, so a declared conventional description is exempt by its own declaration." On main both not-intrinsic arms of check 4 fire **only** on declared edges (`curve.authority().is_declared()`; on PR 4998's branch, `crates/topo/src/validate.rs:7016` and `:7169`). The declaration does not exempt an edge; it enrolls it.
2. "So ε-tightening can escalate but never flip a valid body to invalid." A declared conventional seam that is valid at one ε refuses `TangentNotIntrinsic` at a tighter one.

**Witness (the reviewer's, on main).** Take a slot: `(0,0)→(2,0)`, a `+1` bulge to `(2,2)`, then `→(0,2)`, extruded `h = c·√(2ε)` at ε = 1e-9, for `c ∈ {0.25, 0.5}`. Its two tangent wall seams are declared `Chart`s. Tier 3 at 1e-9 gives `[]`. Tier 3 at 1e-12, read in-process at an injected band (`topo::test_support::tier3_at`), gives `[TangentNotIntrinsic × 2]`.

**Why it flips.** The seam is under-determined at the build's ε. That is a zero-side second-order margin: its sagitta `c²ε` is below ε. A zero-side margin turns definite as ε tightens, so the must-carry demands the tangency at the tighter ε and refuses the declaration.

**The transverse arm has the same shape.** `TransverseNotIntrinsic` (`validate.rs:7016`) refuses a declared conventional edge, and an `OnPair`, whose every station reads transverse. A first-order smooth reading can also turn transverse as ε tightens. PR 4998's orchestrator ruled that this arm stays a refusal ("definitely-transverse stays invalid, as on main"). So this is a flip D2's sentence does not admit either.

**What PR 4998 did for `OnPair`.** A derived pair edge is the population that PR newly minted. Where tier 3 reads such an edge jet-determinate, it now escalates (`ValidationError::PairTangencyDetermined`) and does not refuse. The flip is closed for that arm. Declared `Chart`s are left as main has them.

## What it needs

An `[ev]` ruling on D2's two sentences:

- whether a declared conventional seam escalates as a pair does (code change);
- or whether the sentences are amended to what the code does (wording change), scoped to declared edges and the transverse arm.
