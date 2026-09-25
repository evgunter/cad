---
id: tier3-prime-still-couples-plus-v-to-the-reporting-target
kind: issue
title: tier 3' still couples its +V check to the reporting target, so it refuses bodies tier 3 admits
status: dispatched
opened: 2026-09-11
priority: P0
cost: D
parent: ATREST-3
---


## The finding

PERF-6 made tier 3's check 7 certify a SIGN: `validate_geometric`
refines its certified quadrature only until the body's volume
enclosure excludes zero (`crates/topo/src/validate.rs`,
`validate_geometric_certified` -> `crate::props::sign_certified` with
`plus_v_decide`). Tier 3' did not change. Its check 7 arrives through
the `PlusVCheck` hook `lane_certificate`
(`crates/topo/src/validate.rs:2733`), which is
`crate::props::mass_properties_with` — the scalar's own lane, run to
the REPORTING target `1024*eps` — and `tier3_local_checks_marked`
applies `plus_v_invariant` to whatever that returns.

So the two doors disagree on a valid body whose quadrature cannot
reach the reporting target while its sign is definite. Pinned, on
purpose, at `crates/sweep/tests/tcost_k3_certificate.rs`'s planted row
(the `exhausted schedule` block): `validate_geometric` passes,
`validate_pseudomanifold` and `validate_pseudomanifold_certificate`
refuse `VolumeUncomputable`. That is the exact false refusal
`tier3-plus-v-needs-a-sign-and-pays-for-a-precision` recorded, still
live one door over.

**Which door the STEP import path pays, precisely.** The per-solid
gate is `topo::validate_geometric` (`crates/step-import/src/lib.rs`,
the `gate` helper) and is ALREADY the sign door — a single-solid
import is unaffected by this item, and dm1's three tier-gate cells
moving is that door working. What still couples is the AGGREGATE gate
(`gate3`, the tier-3' door over the assembled body), which every
import pays over what it assembled, so a rational-walled assembly at a
tight eps still meets it there.

The cost half is live too: tier 3' still pays a full certified
quadrature where a sign would do.

## Why PERF-6 did not take it

Out of that unit's fence, which named `validate_geometric` and the
doors that exist only to run check 7. Tier 3' is the shared local
battery: `tier3_local_checks_marked` has four callers (the structural
door, `contact_marks`, `validate_pseudomanifold` and its certificate
twin), its hook is generic over `PropsQuadLane` — a `Dual` included,
which may not certify — and `validate_pseudomanifold_certificate`'s
public return type would move with it. A previous lane declined the
same restructure for the same reason.

## What a fix is

The machinery PERF-6 built is scalar-generic already: `face_flux`
takes a `RoundWindow` and a `QuadHook`, and `sign_certified` differs
from the reporting walk only in the hook it is handed and in the
`settled` predicate it asks between rounds. What is missing is a
`SignCertificate` whose hook is the LANE's rather than the certified
one — a plain fn pointer would do, since both candidates
(`certified_hook` and a `lane_hook`) are free functions — and a
`PlusVCheck` that yields it. `plus_v_decide` and `PlusVSubject`
already take either subject.

Measure before it moves: what tier 3' costs today on the corpus's
NURBS-walled bodies, and what fraction of that is check 7.

## The fix sketch above names a symbol the tree no longer has

Checked at the ATREST cut (2026-09-20). `PlusVSubject` was introduced
by `da8f803fd` ("tier 3's +V check certifies a sign") and **removed by
`ee74cdbc6`** ("check 7 stops on a VERDICT, not a stopping condition"),
which landed after this row was filed. There is no `PlusVSubject` in
the tree, so *"`plus_v_decide` and `PlusVSubject` already take either
subject"* is half a stale claim: `plus_v_decide` exists
(`crates/topo/src/validate.rs`) and takes an enclosure and a band; the
type that carried the subject does not.

What the sketch was reaching for survives in a different shape —
`validate_geometric_certified` hands `sign_certified` a `settle`
closure and a `last_word` closure, and it is the HOOK inside
`sign_certified` (`certified_hook`) that is certified rather than the
lane's. So the missing piece is still a lane-hooked sign walk; the
spelling has moved. Re-derive it from the tree, not from the paragraph
above.
