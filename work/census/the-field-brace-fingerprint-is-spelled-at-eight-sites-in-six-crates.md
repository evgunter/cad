---
id: the-field-brace-fingerprint-is-spelled-at-eight-sites-in-six-crates
kind: issue
title: the Display-vs-Debug field-brace fingerprint is spelled at eight sites in six crates and the roster names one of them as what holds the rule
status: open
opened: 2026-09-15
---


Found by the style review of CENSUS-ERRORS-ARRIVAL (2026-09-15) and
filed by that unit's fix pass. **It is a class, not an instance**: the
roster that provoked it names one site as what holds the rule, and
there are eight.

## The rule, and where it is said to live

`crates/pncad-py/src/errors.rs`'s `reads_as_prose` is the crate's
statement of the Display-vs-Debug rule, and its doc names the two
fingerprints: `" { "` — the field brace `std` puts in every struct and
struct-variant rendering at any depth — and a message that is one bare
CamelCase token. `ERRORS_MINTING_ITEMS` in
`crates/pncad-py/src/tests.rs` rosters it, with
`the_prose_rule_separates_a_display_from_a_debug_dump` named as what
holds its words.

## The eight spellings of the needle

    crates/topo/src/validate.rs            (x2, assertions)
    crates/viewer/tests/error_display.rs   (x1, a predicate)
    crates/sweep/src/blend/mod.rs          (x3, one doc + two assertions)
    crates/editor-core/src/names/emit.rs   (x2, assertions)
    crates/pncad-py/src/tests.rs           (x1, an assertion)
    crates/pncad-py/src/errors.rs          (the rule itself)

Measured 2026-09-15 by `grep -rn '" { "' crates/ --include=*.rs`.
Six crates outside the one that states the rule each re-spell the
needle rather than call `reads_as_prose`, and each carries its own
half of the rule: none of the seven outside `errors.rs` carries the
bare-CamelCase fingerprint at all, so each is a weaker test than the
rule it is quoting, silently.

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
