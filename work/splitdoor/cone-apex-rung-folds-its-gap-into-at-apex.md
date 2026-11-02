---
id: cone-apex-rung-folds-its-gap-into-at-apex
kind: issue
title: conic_quadric: off_the_apex folds an apex distance in the band's gap into Unsettled::AtApex, a limit
status: open
opened: 2026-10-29
priority: P3
cost: E
---

Found by the review of PR 5993 (`splitdoor/roots-unsettled-split`).

`crates/topo/src/boolean/conic_quadric/mod.rs:476` (`off_the_apex`):
a certified root stands only if its distance from the apex decides
positive (`CONE_APEX`). A root decided at the apex is `Unsettled::AtApex`
(`BooleanError::CrossingAtConeApex`, whose doc tells the definite
half). The arm is `Ok(Sign::Zero | Sign::Negative) | Err(_) =>
Err(Unsettled::AtApex)`: an apex distance in the band's gap, the
geometry's ambiguity, leaves through the same limit with its reading
dropped, where the band owes an escalation carrying the diag
(`RootRefusal::Touch`).

The door's slack meter refuses most such roots first (the function's
doc), so the reach may be narrow; the repair splits `Err(diag)` off to
the band's ending, with a row that hands the rung a root in the gap
directly, as its existing row does for one at the apex.
