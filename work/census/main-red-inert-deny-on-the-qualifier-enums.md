---
id: main-red-inert-deny-on-the-qualifier-enums
kind: issue
title: Main red: the deny_unknown_fields census finds the two Qualifier enums inert since 'nothing is ranked'
status: open
opened: 2026-10-26
---


Found by PR 5742's slow-set run (2026-10-26), on a head carrying main `d67ac3cbde`; neither file is in that PR's diff.

`test-utils` `deny_unknown_fields_census::every_deny_unknown_fields_attribute_has_a_named_field_to_deny` (`crates/test-utils/tests/deny_unknown_fields_census.rs:369`) is red. It names two sites:

- `crates/editor-core/src/names/nest_reference.rs:114`, `enum Qualifier`;
- `crates/editor-core/src/names/role.rs:1112`, `enum Qualifier`.

Each still carries `#[serde(deny_unknown_fields)]`, but neither declaration has a named field any more. 353f5163b6 ("names: an orientation is read, never chosen; nothing is ranked") removed their one named variant, `OrderAlong { rank, of }`. So the attribute denies nothing.

**Done when** both attributes are removed, any doc that reasons from them says what actually refuses, and the census row is green on main.

## Two more census rows red on main since D′3 (2026-10-26, main `be2b7a139c`)

PR 5742's slow-set run found these on the same day. Neither file is in that PR's diff.

- `deny_unknown_fields_census::the_walk_still_sees_every_attribute_site`: `crates/editor-core/src/var.rs` holds four `deny_unknown_fields` sites on main, and `ATTRIBUTE_SITES_TODAY` still says three.
- `hand_written_impl_census::every_hand_written_walk_is_held_to_its_declaration` and `::the_walk_still_sees_every_file_holding_one`: `crates/editor-core/src/product.rs:1040`, `impl Debug for Census<T>`, reads `self.node` with no exhaustive pattern, and `product.rs` is not in `IMPL_FILES_TODAY`.

## 2026-10-27 — the hand-written-impl half is fixed by the nightly triage (`triage/nightly-2026-10-27`)

Nightly run 40455870959 (main `fb476030a9`) found a second offender
beside `product.rs`: `crates/topo/src/props.rs:2439`,
`impl Debug for CertifiedBody`, read `self.held` with no exhaustive
pattern (from `c6dc4643`, merged with PR 5741). The triage branch
destructures `Self` in both impls and adds `product.rs` to
`IMPL_FILES_TODAY`, so both `hand_written_impl_census` rows are green
there. What stays open here is the two `deny_unknown_fields_census`
rows.

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. Both `deny_unknown_fields_census` rows were red at every ε.

- `every_deny_unknown_fields_attribute_has_a_named_field_to_deny` is
  pinned: `PINNED_INERT` names `names/nest_reference.rs` and
  `names/role.rs`, and the row requires exactly one inert site in each.
  It goes red the day either attribute leaves; this row's fix removes
  both and the pin with them.
- `the_walk_still_sees_every_attribute_site` is re-blessed:
  `editor-core/src/var.rs` holds four sites since D′3 (`a752273555`, 2026-10-23: `Var`, a
  named-field struct the census reads as live), and the table says four.
- `hand_written_impl_census::the_walk_still_sees_every_file_holding_one`
  was red too: `editor-core/src/checks/at_rest/carried.rs` arrived with
  PR 5871 (`PinnedDoc`'s one equality and elided `Debug`, both reading the
  newtype's `self.0`). `every_hand_written_walk_is_held_to_its_declaration`
  passes over it, so the line is added (re-blessed).

Reproduce: `cargo nextest run -p test-utils --profile ci
--ignore-default-filter -E 'test(/deny_unknown_fields_census|hand_written_impl_census/)'`.
