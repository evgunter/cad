---
id: the-fillets-registered-tangency-says-it-holds-where-the-doors-decisions-hold
kind: issue
title: The line×line fillet's registered tangency and endpoint facts say they hold "at every value of the corner's data the door's decisions hold for", which a registration may not rest on
status: open
opened: 2026-11-15
priority: P3
cost: E
---


Filed by BOXFINISH (2026-11-15) from the design fork on
`work/boxfinish/a-verb-stores-a-radius-as-a-magnitude-and-the-symbolic-tier-cannot-use-the-sign-its-gate-decided.md`
(`[ev]` PR on E12's wording).

`register_incoming_tangency` (through `Arc2::register_tangent_at`) and
the "Registered" arm of `fillet_facts` (`crates/profile/src/path.rs`)
register facts whose doc says they hold "at every value of the corner's
data the door's decisions hold for". Under E12's registration sentence
a registration is equal at every parameter value, not only where a
decision of the registrant came out as it did.

The fillet already spells its turn side as the literal `sgn` ("so no
sign atom enters"), so each fact may already be an identity of the
expressions the path built, with only the doc over-cautious. Nobody has
done the algebra.

**The fix is one of two, per fact:** show it is an identity under the
literals and re-word the doc; or take `Facts::Decided` (the profile's
existing home for an endpoint that holds only to a decision) and
register nothing.

The other registrants (`crates/geom-core/src/arc.rs`,
`crates/sweep/src/swept.rs`, the only two files on
`scripts/gates/register-equal-allowlist.sh`) were read by both
designers: apart from the radius facts that retire with BOXFINISH's
signed-radius unit, each is an identity of what it built.
(BOXFINISH orchestrator)

Moved from PROFILETAIL to PROFILE on 2026-11-20, by `git mv`, when PROFILETAIL merged into PROFILE: PROFILETAIL held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one PROFILE carries.
