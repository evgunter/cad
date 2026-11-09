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

2. **A second forwarding hop in one file collapses into the first.** A site whose name is a forwarded parameter is keyed by its expression (`<file>: <expression>`, e.g. `link.rs: name`), and the census's `indirect` is a set. One `INDIRECT` entry therefore stands for every forwarding site in that file, present and future, that spells the same expression. A new forwarder under a common word in a file that already has such an entry does not red the roster, and its callers' names go unread. `read_identity` sites are already keyed by their ordinal in the file for this reason. Doing the same for forwarded `decide*` and carrier sites (`<file>: name#3`) would make each site its own entry, at the cost of renumbering when one is added above. Naming a site by its enclosing function (`<file>::<fn>: name`) would avoid the renumbering, but the reader does not parse functions today.

Item 1's `TangentRow::predicate()` arrived with main's tangent-row change: `boolean/contact_verify.rs` decides its off-surface rows through `TangentRow::Off(side).predicate()`, which spells `contact_tangent_off_1` and `contact_tangent_off_2` in `contact.rs`'s match. The roster lists the site as `boolean/contact_verify.rs: row.predicate(`.
