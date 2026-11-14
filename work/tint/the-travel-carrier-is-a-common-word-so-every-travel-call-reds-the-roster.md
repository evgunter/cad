---
id: the-travel-carrier-is-a-common-word-so-every-travel-call-reds-the-roster
kind: issue
title: topo's recourse roster declares a carrier under the common word 'travel', so every new .travel() call in a new fn reds the nightly
status: open
opened: 2026-11-14
priority: P4
cost: E
---


Filed by GREEN (`green/censuses`, PR 6921), from the review of that PR.

## What

`crates/topo/tests/recourse_roster.rs` declares `NameCarrier::Call("travel")` for `boolean/join.rs`'s forwarder `travel(name, …)`. Carriers are crate-wide, so every `.travel()` call in topo is read as that carrier's site. `ring_path/spline_wall.rs`'s `IsoPiece::travel()` accessor (PR 6891) made three such sites, and the nightly went red. GREEN listed them in `INDIRECT` under a second constant, `SAME_NAMED_TRAVEL`, beside `SAME_NAMED`. That covers one situation with two constants and three hand-kept entries, and the next `.travel()` in a new fn reds the nightly again (the roster is in the slow set).

The roster's own header has the rule this list stands in for: a forwarder that hands on its caller's name is spelled `*_as`, so that no other call shares its token.

## Owed

Rename `join.rs`'s `travel` to a `*_as` spelling (`travel_as`), declare that as the carrier, and strike the three `SAME_NAMED_TRAVEL` entries and the constant. `join.rs` sits on SECTARMS' and others' ground (`work.py territory`), so announce the rename there.
