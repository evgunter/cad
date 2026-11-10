---
id: topo-name-forwarders-hide-their-callers-names
kind: issue
title: names spelled in a closed type's predicate() match, and a second forwarder in a file that already forwards, are invisible to topo's roster
status: closed
opened: 2026-11-09
priority: P3
cost: M
closed: 2026-11-10
pr: 0
---

Found closing `topo-has-no-predicate-name-roster`; narrowed by the forwarder renames (PR on branch `onecopy/topo-forwarders`).

That PR renamed topo's generic name forwarders to `*_as` spellings and declared them as `Call` carriers in `crates/topo/tests/recourse_roster.rs`, so their callers' names are read. Two things the census still cannot see:

1. **A closed type's `predicate()` match.** `clearance.rs`'s `bound.predicate()` and `self.as_bound()`, and `boolean/contact_verify.rs`'s `TangentRow::predicate()`, decide under names spelled as the arms of a `match` in the type's own `predicate()` method. The roster lists them as `CLOSED_TYPE`. The census reads names at calls and in consts, not as a function's return values. The fixes are a carrier kind that reads the literals of a declared function's body (`NameCarrier::Fn("predicate")` in `test_utils::source`, shared with sweep's and profile's rosters), or making each arm a const the roster declares. Sweep holds its closed types (`BlendDecision::ALL`) to the census another way: its roster iterates the type. A topo closed type with an `ALL` could be read the same way.

2. **A second forwarding hop in one file collapses into the first.** A site whose name is a forwarded parameter is keyed by its expression (`<file>: <expression>`, e.g. `link.rs: name`), and the census's `indirect` is a set. One `INDIRECT` entry therefore stands for every forwarding site in that file, present and future, that spells the same expression. A new forwarder under a common word in a file that already has such an entry does not red the roster, and its callers' names go unread. `read_identity` sites are already keyed by their ordinal in the file for this reason. Doing the same for forwarded `decide*` and carrier sites (`<file>: name#3`) would make each site its own entry, at the cost of renumbering when one is added above. Naming a site by its enclosing function (`<file>::<fn>: name`) would avoid the renumbering, but the reader does not parse functions today.

Item 1's `TangentRow::predicate()` arrived with main's tangent-row change: `boolean/contact_verify.rs` decides its off-surface rows through `TangentRow::Off(side).predicate()`, which spells `contact_tangent_off_1` and `contact_tangent_off_2` in `contact.rs`'s match. The roster lists the site as `boolean/contact_verify.rs: row.predicate(`.

A carrier call with no literal and no const is keyed by its first argument even where the name is a later one: `relation_step(e, RelOp::Ge, bound, name, band)` and `contains_step(e, lo, hi, name, band)` are both `props.rs: e` (`NOT_FIRST`), so distinct sites merge on an argument that is not the name at all. Keying by the name argument's position needs the reader to know which argument a carrier's name is, which a declared `NameCarrier::Call` does not say today. A `NameCarrier::CallAt(token, n)` would say it.

**Closed** (2026-11-10, ONECOPY forwarders-by-fn).

- **Item 2, widened to every roster.** `test_utils::source::predicate_census` now keys every indirect site by its innermost enclosing `fn` item (`<file>: <fn>: <head>`, through the shared `source::fn_items` over `item_body`; `-` outside any). That covers a forwarded `decide*` argument, and a declared carrier's call that holds no literal.
  - A second forwarder in a file is therefore its own entry, and a new one reds until it is declared: as a `FORWARDS` entry naming it, or as a carrier.
  - topo's, geom's, geom-core's, geom-brep's, editor-core's and sweep's `INDIRECT` lists are re-keyed. Each keeps the reason its file-level entry gave; topo's list went from 107 to 159 entries and geom-brep's from 107 to 149. The `FORWARDS` reason no longer says one entry stands for a file.
  - The discriminating checks:
    - the unit test `a_forwarder_is_keyed_by_its_enclosing_fn`, with two same-named forwarders, a `fresh_forward` in an `impl`, a nested `fn`, and a site in a `static`;
    - on the tree, the review's plant: a `fn fresh_forward(x, predicate, bnd)` added to `editor-core/src/names/emit_topo.rs` and called with a literal now reds editor-core's roster (`names/emit_topo.rs: fresh_forward: predicate`). It was green under the file-level key.
  - `Segment::ahead` is renamed `ahead_as` and declared a carrier, and the stale `Track::cover` doc in `names/discriminate.rs` reads `Track::cover_as`.
- **Item 1**, the closed type's `predicate()` match, is the same question as `the-census-reads-no-closed-type-name-table`, which carries it.
- **The `NOT_FIRST` note** (a carrier call keyed by an argument that is not its name) narrows to one function: two such calls in different `fn`s are now distinct entries. Within one `fn` they still share a key. A `NameCarrier::CallAt(token, n)` that names the name argument's position would part them, and nothing needs that today.

