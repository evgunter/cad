---
id: escalation-recourse-dispatch-has-three-homes
kind: issue
title: blend: three Display impls dispatch a recourse by predicate name, with three different answers on an unknown name
status: open
opened: 2026-09-08
---


## Finding

Three `Display` impls in two crates route a recourse sentence by
matching an escalation's predicate NAME, each with its own table and
its own answer when the name is not in it.

- `crates/sweep/src/blend/mod.rs:1299` — `BlendError::Escalated`'s
  `match source.predicate`. Unknown name: the F6 gap sentence, "no
  recourse is recorded for predicate `Some(...)`; this is a gap in the
  error table, not advice to act on"
  (`crates/sweep/src/blend/mod.rs:1333-1340`). It NAMES the hole.
- `crates/profile/src/path.rs:1629` — `PathError::Escalated`'s
  `match source.predicate`. Unknown name: `_ => write!(f, "path
  junction classification: {source}")`
  (`crates/profile/src/path.rs:1660`). It ASSERTS a category. Three of
  the four keys the arm handles explicitly are not junction
  classifications, which is exactly why they were pulled out; a new
  non-junction key gets the false label back by default.
- `crates/profile/src/validate.rs:691` — the fillet constructor's
  riders, `match source.predicate` under
  `EscalationSite::Fillet`. Unknown name: `_ => {}`
  (`crates/profile/src/validate.rs:725`). It SAYS NOTHING — the
  escalation renders with the shared coincidence tail and no recourse,
  and no reader can tell that from a predicate whose situation
  genuinely has no recourse.

So the same question — "which recourse does this escalation carry?" —
has three implementations and three failure modes, and adding a
predicate anywhere obliges an author to remember all three. Nothing
enforces the obligation: no test enumerates predicate names against
the tables, and two of the three fall-throughs are silent about being
one.

Found in BLEND unit 1's fix pass, which added one arm to the first of
the three and corrected another; the sweep for siblings is what turned
up the other two homes.

## Fix shape

Not obvious, and the choice is a design statement, not a cleanup:

1. One table. A predicate name is a `geom-core` fact already
   (`decide(name, ...)`), so the routing could live beside the band —
   one map from predicate name to recourse, with each crate's Display
   composing site context around the sentence it returns. That makes
   the fall-through one decision instead of three.
2. Or keep three tables and make the three fall-throughs agree, with
   the honest one (`mod.rs`'s named gap) as the shape to follow, plus
   a roster test per crate that fails when a `decide` site names a
   predicate no table knows.

(2) is cheap and closes the silent arms; (1) closes the duplication
and is the larger change. Either needs a ruling on whether
`geom-core` should carry user-facing recourse prose at all.
