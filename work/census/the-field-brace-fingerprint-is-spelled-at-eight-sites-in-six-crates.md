---
id: the-field-brace-fingerprint-is-spelled-at-eight-sites-in-six-crates
kind: issue
title: the Display-vs-Debug field-brace fingerprint is spelled at seven executable sites in five crates outside the file that states the rule (the id says eight/six; read it as a name)
status: open
opened: 2026-09-15
priority: P3
cost: E
---


Found by the style review of CENSUS-ERRORS-ARRIVAL (2026-09-15) and
filed by that unit's fix pass. **It is a class, not an instance**: the
roster that provoked it names one site as what holds the rule, and
seven more spell half of it by hand.

## The rule, and where it is said to live

`crates/pncad-py/src/errors.rs`'s `reads_as_prose` is the crate's
statement of the Display-vs-Debug rule, and its doc names the two
fingerprints: `" { "` — the field brace `std` puts in every struct and
struct-variant rendering at any depth — and a message that is one bare
CamelCase token. `ERRORS_MINTING_ITEMS` in
`crates/pncad-py/src/tests.rs` rosters it, with
`the_prose_rule_separates_a_display_from_a_debug_dump` named as what
holds its words.

## The spellings of the needle

Measured 2026-09-15 by `grep -rn '" { "' crates/ --include=*.rs`,
thirteen lines, separated into checks and prose:

    crates/topo/src/validate.rs            2 assertions
    crates/viewer/tests/error_display.rs   1 predicate
    crates/sweep/src/blend/mod.rs          1 assertion + 2 doc mentions
    crates/editor-core/src/names/emit.rs   2 assertions
    crates/pncad-py/src/tests.rs           1 assertion
    ----------------------------------------------------------------
    crates/pncad-py/src/errors.rs          the rule + its doc
    crates/pncad-py/src/prose_census.rs    2 doc mentions

**Seven executable checks in five crates outside the file that states
the rule**, each re-spelling the needle rather than calling
`reads_as_prose`. **None of the seven carries the OTHER fingerprint** —
the bare-CamelCase token — so each is a silently weaker test than the
rule it is quoting. Two of them (`editor-core/src/names/emit.rs`) even
cite `reads_as_prose` in a comment three lines above while spelling
half of it by hand.

## Where else to look, because the grep above is one spelling

The needle is a two-space-and-brace literal, so this sweep cannot
match:

* `crates/pncad-py/src/prose_census.rs`'s needle set — it states the
  same rule in its own module docs and its `declaration_verdict`
  reasons about which shapes can reach a `" { "`. Whether its needles
  and `reads_as_prose`'s two fingerprints are the same predicate is
  the sharp question, and the existing row
  `prose-census-cannot-see-a-bypassed-prose-renderer` is next to it
  but is not it.
* Any other Display-vs-Debug detector spelled without the literal — a
  `contains('{')`, a `matches!` on a rendered form, a test asserting a
  message "reads as a sentence" by some other means.

## What is owed

One predicate, called from the sites that mean it, and each site that
deliberately wants only half of it saying so. Not a sweep of eight
`assert!`s into one macro: the judgement is which of the eight are
asking `reads_as_prose`'s question and which are asking a narrower one
on purpose.

Territory: the sites cross `topo`, `viewer`, `sweep`, `editor-core`
and `pncad-py`; `crates/sweep/src/blend/mod.rs` is BLEND's and
`crates/pncad-py/*` is LIB's, and this program claims no paths.
