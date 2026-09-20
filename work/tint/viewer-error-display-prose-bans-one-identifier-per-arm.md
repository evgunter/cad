---
id: viewer-error-display-prose-bans-one-identifier-per-arm
kind: issue
title: viewer's error_display prose helper bans one identifier per arm, so a sibling leak passes at 29 call sites
status: open
opened: 2026-09-19
---

Found by S-DUP's F6 fold of `crates/viewer/tests/panel_edits.rs`, whose
census re-derivation reached this file. **It sits on S-TINT's slate
rather than S-DUP's because S-DUP claims no paths** (`work/dup/plan.md`,
"Territory — none") and this file is S-TINT's by territory; S-TINT also
owns every other row in this class
(`assert-f6-dump-lists-are-hand-written-mirrors-of-error-enums`,
`sibling-display-contract-suites-hand-mirror-their-enums-too`,
`dump-ban-lists-spelled-guts-are-a-fourth-copy-and-two-are-dead`).

## The finding

`crates/viewer/tests/error_display.rs` holds its own Display-vs-Debug
predicate:

    fn debug_shaped(message: &str, variant: &str) -> bool {
        message.contains(" { ") || message.contains(variant)
    }

`prose(message, variant)` wraps it and is called at **29 sites** over
roughly a dozen error types (`CameraError`, `CameraOpError`,
`SceneError`, `SceneDocError`, `PickError`, `PickIndexError`,
`IdMapError`, `EdgeNameFault`, `ReplayError`, `MateToolError`,
`TessellateError`, `NodePickError`, `InterrogateError`, `EditError`).

**Each site bans exactly one identifier — its own arm's.** That is the
same approximation `panel_edits.rs` carried until S-DUP folded it, and
it is the half that actually differs in what the assertion catches: a
rendering that leaks a **sibling** arm's identifier passes here and
fails under `test_utils::f6::assert_f6`, whose `dumps` is the enum's
whole roster.

S-DUP proved the delta rather than asserting it. With a sibling
identifier planted in one `Refusal` arm's `Display`
(`"… — GestureInFlight, declare it first"`), the per-arm form passed
and the roster form failed at `test_utils/src/f6.rs`'s dump clause. The
same plant would pass at all 29 sites here.

Two further clauses `assert_f6` carries and `prose` does not: the
**field-punctuation** ban (`"node:"`-shaped tokens) and
`assert_ne!(shown, format!("{err:?}"))`.

## Why this is not the row the CENSUS program already has

`work/census/the-field-brace-fingerprint-is-spelled-at-eight-sites-in-six-crates`
lists this file as one of seven sites re-spelling the `" { "` needle,
and that is a true and different fact: its class is
`pncad-py`'s `reads_as_prose` and its complaint is the missing
bare-CamelCase fingerprint. The defect here is the **identifier half**,
which that row does not measure.

`sibling-display-contract-suites-hand-mirror-their-enums-too` names
this file once, for a **different** site — its lone
`assert!(!duplicate.contains("PatchId"))`, called out as "that shape at
N=1 and not an enum mirror". `debug_shaped`/`prose` is the N=29 shape
and is unnamed there.

## What the conversion would cost, so the row is not read as a rename

Unlike `panel_edits.rs` — one enum, one `f6_variants!` block, one
loop — this is roughly a dozen rosters over types declared in six
crates, plus a `fields` roster each. Two things are unmeasured and the
conversion owes them first: whether any of those enums is
`#[non_exhaustive]` to `viewer` (`f6_variants!` cannot serve one, and
`VariantCensus::hand_written` is the documented door, which the site
then owes a reason for), and whether `prose`'s containment-forwarding
rows — the ones asserting an inner layer's sentence appears verbatim —
can take a roster at all without banning the inner enum's identifiers
too. S-DUP measured neither; it measured only that the approximation is
the same one and that the delta is real.

## Instrument that found it, and its blind spot

`git grep -nE '(contains|find|matches|starts_with)\s*\(\s*["'"'"']\s*\{'`
over **every tracked file, no path argument** — the brace literal in any
spelling, which is the one clause every full copy of F6 must contain.
Its blind spot: a copy that bans only identifiers and never the brace.
