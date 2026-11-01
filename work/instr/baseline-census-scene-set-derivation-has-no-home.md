---
id: baseline-census-scene-set-derivation-has-no-home
kind: issue
title: Four spellings of the scene set in the one file that gives every other derivation one home
status: open
opened: 2026-09-16
priority: P3
cost: E
---



## Finding

`tools/tess-lint/tests/baseline_census.rs` deliberately gives each of
its derivations one home — `census`, `groups`, `key`,
`indistinguishable_pairs` and `sweep` are written once and called from
every test that needs them, so two readings of one thing cannot
disagree. Its module docs argue the same rule for prose: *"A census
has one executable home and every other site points at it"*.

**The scene set got no home.** "Scenes carrying a sized row" is
derived three times, in two idioms:

- `the_committed_baseline_carries_this_many_indistinguishable_pairs`'s
  `sized_scenes` — `Vec<&str>` + `sort_unstable` + `dedup`;
- `the_committed_baseline_gates_a_re_key_in_exactly_these_scenes`'s
  `gating` — `.collect::<BTreeSet<&str>>()`, then back to a `Vec`;
- `no_scene_carrying_a_sized_row_carries_a_name`'s `sized_scenes` —
  `BTreeSet` again, under the same binding name as the first.

Two more scene sets over other row collections are spelled in the same
two idioms beside them: `census`'s own `scenes` (`Vec` + `sort` +
`dedup`, over the grouped rows) and the same test's `by_totals` (`Vec`
+ `sort_unstable`, over `totals`).

The three agree today — `14` is asserted twice over the same predicate
— and the second test's doc already notices the READING is duplicated
(*"already asserted 130 lines up over an identical predicate over the
identical corpus"*). What it does not notice is that the DERIVATION is
duplicated too, which is the thing the file's own rule is about: the
agreement is a coincidence of three correct spellings, not something
the file checks.

The repair is one helper — `sized_scenes(&[Row]) -> BTreeSet<&str>` —
and three call sites. Small, in a file whose whole argument is that a
second copy of a derivation is the defect.

## Was

Disclosed by INSTR unit 4's style review (`instr/u4-census-partition-assert`).
