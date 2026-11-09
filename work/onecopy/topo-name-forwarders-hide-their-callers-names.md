---
id: topo-name-forwarders-hide-their-callers-names
kind: issue
title: names spelled in a closed type's predicate() match, and a second forwarder in a file that already forwards, are invisible to topo's roster
status: open
opened: 2026-11-09
priority: P3
cost: M
---

Found closing `topo-has-no-predicate-name-roster`; narrowed by the forwarder renames (PR on branch `onecopy/topo-forwarders`).

That PR renamed topo's generic name forwarders to `*_as` spellings and declared them as `Call` carriers in `crates/topo/tests/recourse_roster.rs`, so their callers' names are read. Two things the census still cannot see:

1. **A closed type's `predicate()` match.** `clearance.rs`'s `bound.predicate()` and `self.as_bound()`, and `boolean/contact_verify.rs`'s `TangentRow::predicate()`, decide under names spelled as the arms of a `match` in the type's own `predicate()` method. The roster lists them as `CLOSED_TYPE`. The census reads names at calls and in consts, not as a function's return values. The fixes are a carrier kind that reads the literals of a declared function's body (`NameCarrier::Fn("predicate")` in `test_utils::source`, shared with sweep's and profile's rosters), or making each arm a const the roster declares. Sweep holds its closed types (`BlendDecision::ALL`) to the census another way: its roster iterates the type. A topo closed type with an `ALL` could be read the same way.

2. **A second forwarding hop in one file collapses into the first.** A site whose name is a forwarded parameter is named by that expression (`link.rs: name`), so one `INDIRECT` entry stands for every hop in the file that spells it. A new forwarder under a common word in a file that already has such an entry does not red the roster, and its callers' names are not read. This is the census's own convention for `decide*` sites, which the PR extended to carrier calls. Closing it means naming a site by its enclosing function (`<file>::<fn>: name`), which the reader does not parse today.
