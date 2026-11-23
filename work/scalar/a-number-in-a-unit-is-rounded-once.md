---
id: a-number-in-a-unit-is-rounded-once
kind: unit
title: A number in a unit is rounded once over the unit's exact factor; the fallback, the float unit arithmetic and Display on a told number retire
status: closed
opened: 2026-11-20
priority: P1
cost: H
branch: scalar/a-number-in-a-unit-is-rounded-once
refs: [7291]
closed: 2026-11-23
---


## What

PR 7229 built the told rule's notation paragraph, and its review (about two million renders against exact arithmetic) found six places where the rule as written gave an answer nobody chose: an offer that reads `300.0399999 mm`; a fallback to metres or radians where a unit "cannot say" a number (several percent of ordinary values); two units in one sentence; an editable field seeded `5e-5`; `Display` on a told number silently meaning metres; an equality under which equal numbers read apart in degrees. Two designers traced the first three to one cause below the rule (the text door reads the decimal as a float and then multiplies by a float factor, so it rounds twice) and converged over three rounds on four changes. The rule's text is in `crates/geom-core/README.md`, "A number told to a person"; this unit builds it.

## The four changes

1. **`N unit` is the float nearest N times the unit's exact factor, rounded once, at every door.** `quantity` holds exact factors (1/1000, 1/100, 254/10000, π/180 and π with true π) and one function that returns the correctly rounded product; the text parser (`editor-core` `parse.rs`), `WrittenLength::in_unit`, the Rust and Python doors (`300.04 * MM`, reading the float by its shortest decimal) and the told rule's "reads back as" all call it. A told spelling is held as a decimal, not a float of the unit. The fallback to the canonical unit goes, in `told.rs` and in `quantity::fmt`, with `written`'s overflow check and the typed-back search loops.
2. **A sentence is written through a teller that carries the notation.** `Told`, a compared pair's members and a position have no `Display`; a `Tell` body receives the teller, not a `fmt::Formatter`, and writes words and tellable values through one macro. A refusal's `Display` is its telling in metres and radians, derived. The script sweep and the gate for `{}` inside `tell` go. The viewer's `valid from … up to …` becomes one sentence.
3. **Two told quantities are equal when role, dimension and held reading are equal**, and `Debug` shows the held reading. `Sentence` needs a dyn-equality bound on what it keeps, or no equality: say which.
4. **An editable field over a written value shows its exact positional spelling in the unit it was written in**; the field holds the value and its unit, not a float already divided by the factor. On the viewer program's ground (`work/vgeom/the-field-door-tells-a-written-number-with-no-unit-in-hand.md` is the same defect and predates PR 7229).

## To measure before building

- Every finite float has a spelling of at most 17 significant figures in every unit that types back to it under the once-rounded door: argued and sampled, not proved. Pin it with an exact oracle over magnitudes, binade edges, ties and subnormals, and say whether `ROUND_TRIP_DIGITS` stays 17.
- How the π refinement is made total: one that can extend π, or a typed refusal that is never reached.
- Whether any production path compares refusals, `Told` or `Sentence` (a redraw, a dedupe, a result diff): a grep found none.
- How the Python door hands a unit-bearing number to the kernel; whether `turn/12` is the float nearest π/6; whether a written literal's stored leaf is a float.
- Which typed literals move by one float (`30 deg` does), and what that re-baselines.

## Also found, each to fix here or file

A compared pair of two dimensions should not construct (it falls back today); `readout::computed_number` and `bare` strip the unit word off rendered text; `quantity::fmt::render_shortest` duplicates `geom_core::Readable` by hand and both should spell from the one decimal type.

## Sequencing

PR 7229's `Tell`, `Sentence` and the notation's home survive all four changes, so it merges as built. Change 2 rewrites every `Tell` body, so it lands before the per-crate sweeps convert more refusals.

## Ratified

Ev, PR 7261, 2026-11-21: all four changes. On equality Ev asked why a display-rounding equality existed at all; the answer is that it should not, and equality is a plain `#[derive(PartialEq)]` with no hand-written logic (not `cfg(test)`-only). On the teller Ev confirmed the rule stays "the reader's working unit", not the unit a number was written in.

## Dispatch (2026-11-21)

This unit builds changes 1 and 3 (the once-rounded door at every door, the fallback and float unit arithmetic retired, equality a plain derive with `Debug` the held reading) and the "also found" items that fall in the same code. Change 2 (the teller) and change 4 (the editable field) are their own units, stacked on this one, because both build on a told spelling that every unit can say.

## Review

Letter H, tier dual: the meaning of a number in a unit at every door (text, Rust, Python), with typed literals moving by a float; an error here is in every model built from typed numbers.

## Closed (2026-11-23)

PR 7291 built changes 1 and 3. Change 2 (the teller; no `Display` on a told number) is `display-on-a-told-sentence-silently-means-canonical` (P2: formatting, held by a gate today). Change 4 (the editable field) is the viewer's.
