---
id: escalation-recourse-dispatch-has-three-homes
kind: unit
title: blend: two Display impls dispatch a recourse by predicate name, with two different answers on an unknown name
status: review
opened: 2026-09-08
branch: blend/15-recourse-roster
pr: 2514
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

## Two homes now (BLEND-12, 2026-09-13)

The third home is gone. `EscalationSite::Fillet` had no producer, so its
`match source.predicate` under `ProfileError::Escalated` rendered for
nobody; unit 12 retired the variant and the arm together and moved the
nine `fillet_*` names to `validate::fillet_recourse_for`, a single
`&str -> Option<&'static str>` function that the second home
(`PathError::Escalated`'s `Display`) calls.

What that changes for this item:

- the count is two implementations, not three, and the retired one's
  silent `_ => {}` failure mode is gone with it;
- the surviving `profile` home now has a census:
  `crates/profile/tests/fillet_recourse_followability.rs`'s
  `every_fillet_predicate_has_its_own_sentence_and_never_the_shared_one`
  reads the `fillet_*` names out of `sugar.rs`'s own source and fails if
  one has no sentence. That is the enforcement this item says nothing has
  — for the fillet family only. The `path_*` keys and `sweep`'s blend
  table still have none, and `path.rs`'s `_ =>` arm still ASSERTS "path
  junction classification" for any unknown name.

## Landed (BLEND-15)

Two homes still, because the sentences stay with the crates that own
them — disposition (2), with the fall-through made one decision instead
of two.

- **The gap sentence has one home**: `geom_core::MissingRecourse`, beside
  `COINCIDENCE_RECOURSE` in the predicate vocabulary. `geom-core` is the
  only crate both routers can see (`sweep` depends on `profile`,
  `profile` on `geom-core` alone). Both fall-throughs compose it, so
  "no recourse is recorded for predicate …; this is a gap in the error
  table, not advice to act on" is one sentence in one place.
- **`PathError::Escalated` no longer asserts a category on an unknown
  name.** The two junction keys are a named arm — the label is a claim
  about them and about nothing else — and every other unknown name
  renders the gap sentence.
- **A roster row per crate** (`crates/profile/tests/recourse_roster.rs`,
  `crates/sweep/tests/recourse_roster.rs`): every name the crate's `src`
  decides is routed to a sentence or listed with the reason it carries
  none, measured by rendering the door's own refusal. A `decide*` call
  whose name the reader cannot read at the site is declared with what
  carries it, so a name reaching the funnel through a parameter, a const
  or a struct field is on the roster too.
- **The dispatch order is pinned**: no name sits in both
  `fillet_recourse_for` and a `path.rs` match pattern, where the map wins
  silently and the pattern arm is dead.
