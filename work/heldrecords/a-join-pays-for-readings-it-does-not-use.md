---
id: a-join-pays-for-readings-it-does-not-use
kind: issue
title: A join confirms what it carries twice, reads meetings inside one arrival that the gather discards, and runs the full census for one arrival
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Found in review of PR 7124 (`embed/records`, 2026-11-23). Three
readings a `⊔`'s gate pays for and does not use
(`AtRestBody::validate_assembled`, `crates/topo/src/validate.rs`):

1. **What it carries is confirmed twice**: by `census::confirm_only`,
   which is what refuses, and again inside `census::record_at_rest`,
   whose census confirms the records it is handed before it searches.
2. **Meetings inside one arrival are computed and discarded.** The
   census has no group scope, so it reads every pair; the gather drops
   what it found inside an arrival a door already decided
   (`crates/editor-core/src/product.rs`, "A meeting inside one arrival
   a door already decided is that door's").
3. **A join of one arrival runs the full census**, with nothing between
   arrivals to record.

Measured on the fast set (PR 7124's body, 23 November, one box): a
`⊔`'s gate makes 6,061 full-census calls in 9.2 s of 486 s of test
time; the deciding doors' confirm-only gates cost 0.08 s. How much of
the 9.2 s is each of the three was not separated.

None is a few lines: (1) needs the census to hand back which carried
records it confirmed, so the gate can refuse from that one pass; (2)
needs a group scope in the census's pair search; (3) changes what
`Censused::findings` holds for an import of one solid, which the
import's gate reads (`gate3_findings`).
