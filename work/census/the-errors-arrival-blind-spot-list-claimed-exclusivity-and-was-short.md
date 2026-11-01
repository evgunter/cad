---
id: the-errors-arrival-blind-spot-list-claimed-exclusivity-and-was-short
kind: unit
title: a fourth consecutive blind-spot list claimed exclusivity and was short by two shapes, and the survivors are what matter
status: closed
opened: 2026-09-15
branch: census/arrival-residue
closed: 2026-09-16
---


Found by the style review of CENSUS-ERRORS-ARRIVAL (2026-09-15) and
filed by that unit's fix pass. This program's standing finding 2 says
a blind-spot list that claims exclusivity has been wrong every time it
has been written here; this is the **fourth consecutive** one, and it
was wrong in two places the review executed.

## What the list said, and the two it missed

`ERRORS_MINTING_ITEMS`'s doc disclosed one blind spot — *"a word that
reaches Python from this file without being a literal here"* — and the
file header claimed *"a further one, in any form and at any depth,
reds that roster by name."* The review executed two counterexamples
against the real tree:

* **A character literal was read and DROPPED**, and so was the item
  that spelled nothing else. Splicing
  `impl ValidationRefusal { pub const fn sep(self) -> char { '/' } }`
  onto `src/errors.rs` gave no complaint at all.
* **A misattribution to the row ABOVE was not loud, and it cancelled.**
  An outer attribute sits above the item it decorates, and attribution
  was by the nearest declaration above, so `#[deprecated(note = "…")]`
  inflated the previous row's count — which a deletion in the same
  item cancels to silence.

Both are closed, both with a test that executes the case. The list is
not the point; the point is that it claimed exclusivity and was short
by two, on a unit whose spec named that hazard in writing.

## The residue as it stands after the fix, each executed

1. **An item that spells NO literal is invisible.** A map forwarding
   `crate::tags`' word adds nothing for the reader to see. Executed by
   `the_errors_mint_census_cannot_see_a_word_that_is_not_a_literal`,
   disclosed in the roster doc and in the file header, which now names
   it as the one exception to its claim. **Open by design, not by
   oversight** — but it is the half that matters, because a forwarded
   word is still a Python-visible word.
2. **A word swapped for another INSIDE one item is silent.** Executed
   2026-09-15 by this fix pass: replacing `"reason"` with `"renamed"`
   in `ValidationRefusal::ATTRIBUTES` leaves the count at 2 and the
   census green. That is the roster's stated design — it is an arrival
   alarm, and values are the `held_by` column's business — so the real
   question this leaves is whether every `held_by` entry is true.
   **One of the TEN says it is not**: `EvalReason::ATTRIBUTE` records
   *"no Rust check names this word"*, held only by `pncad.pyi` and the
   Python suite.
3. **An `impl` at indentation loses its qualifier.** `impl_spans`
   (in `crates/pncad-py/src/tests.rs`, not in the shared module) keys on
   column 0 — `line.starts_with("impl")`. Executed 2026-09-15: an
   `impl ValidationRefusal` inside a `mod` reports `NEW item
   \`seventh\`` — loud, but under a bare name rather than the qualified
   one, so the complaint names an item that does not exist by that
   name. The site now says so.

   **The repair has a worked precedent one file over, and it is this
   program's own.** `crates/test-utils/tests/hand_written_impl_census.rs`
   scans freely (`code[from..].find("impl")`) and guards the hits with
   `test_utils::source::{boundary_before, boundary_after}`, taking the
   `impl Trait` return position out through `item_body`'s `Declaration`
   arm and broken text through `Unterminated`. All three helpers are
   already `pub` in `test_utils::source` and this file already depends
   on that module. **So this is the SECOND thing unit 2's census had
   solved that unit 6 re-invented more narrowly** — the first was the
   `(path, trait, self type)` key — and that pattern, rather than the
   nesting itself, is what the repair should be read against.
4. **`held_by` is prose and nothing re-derives it.** The reviewer
   checked the nine rows that existed then; the orchestrator re-checked
   at close-out, over TEN rows naming nine distinct tests, and every
   one exists and says what the column claims. So this is a risk and
   not a defect today; a renamed test leaves the column pointing at
   nothing, and the census stays green.

   **This row said "nine" in two places and the roster holds ten.** The
   tenth, `is_bare_camel_token`, appeared in the same fix pass that
   filed this row, when closing the char-literal hole made the
   population every literal. Corrected 2026-09-15 by the orchestrator;
   it is standing finding 12 (*a correction can be born stale in the
   sentence that corrects a stale count*) in its fourth instance inside
   one unit, and the reason the row's counts now say where they were
   taken.

## Why this is a row and not a paragraph in a PR body

`work/README.md`: *"disclosing a residue is not scheduling it."* Three
of the four above are disclosed at the site, which is where a reader
of the code needs them; none of them is scheduled anywhere, and the
unit that wrote them closes with its spec deleted.

Territory: `crates/pncad-py/*` is LIB's fence and this program's
`keep_out` announces its pncad-py rows there.

## What CENSUS-ARRIVAL-RESIDUE did with each, and what is left

Landed on `census/arrival-residue`.

1. **Measured first, and the measurement moved the disposition.** Two
   numbers, both re-derived on this branch: **twelve** words reach
   Python from `src/errors.rs` without being a literal in it, all
   through `QuantityOpMismatch::op` — a `&'static str` FIELD, minted at
   call sites under `src/py/` and read by no instrument, which is
   `dimension-error-op-carries-twelve-words-minted-at-call-sites`'s row
   and not this one; and **three of the file's thirteen nameable items
   spell no literal at all** (`EvalReason::ATTRIBUTES`,
   `QuantityOpMismatch::new`, `ValidationRefusal::ALL`), the first of
   them Python-visible vocabulary. The disclosure named neither shape:
   it offered *a map forwarding `crate::tags`'* and *a word built from
   a kernel `Display`*, and the file holds no instance of either. So
   the ARRIVAL half is closed — the census's population is now the
   file's declarations, with the literals as what each contributes, and
   a forwarding map reds by name. What stays open is a word channel
   that is not a declaration, disclosed at the site with its count and
   its date, and executed.
2. **Closed at its live instance.** The one `held_by` that said *no
   Rust check names this word* has one:
   `the_discriminant_attribute_names_are_declared_in_the_stub` reads
   `pncad.pyi` and holds each discriminant-carrying class's attribute
   set against what the stub declares. It also turned up that
   `ValidationError` raises `reason` and the stub declares only `door`,
   filed as
   `validation-error-reason-is-raised-and-the-stub-declares-only-door`
   and carried by that test's gap column, so closing it is loud.
3. **Repaired with the sibling census's mechanism.** `impl_spans` is
   `scope_spans`: a free scan guarded by `boundary_before` /
   `boundary_after` and `item_body`, over `impl` **and** `mod`, so a
   module is part of the key and `nested::ValidationRefusal::seventh`
   is what a nested arrival is called. It needed one thing the
   precedent did not: a type-position `impl` (`-> impl Display`) whose
   `item_body` is the enclosing FUNCTION's body, which the sibling
   census is immune to only because its subject filter drops it.
4. **Re-derived.** `held_by` is a list of holders rather than prose;
   every `Holder::Test` is looked up in this file's own source by
   `every_test_this_census_names_is_one_this_file_declares`. What it does not
   close, and says so: a `Holder::Outside` names no Rust check by
   design, and nothing re-derives that a named test holds what the row
   says it holds.

## What the fix pass did with the style review's findings

The review executed sixteen findings against the tree and the fix pass
re-derived each. **The fifth exclusivity claim was short by two more
shapes**, which is this program's fourth consecutive one and the fourth
found by a reader who did not write the fix.

1. **A `trait` body was no scope, and the declaration walk was still
   keyed on line starts.** The unit had written its own counterexample
   into its fixture: a `pub trait Declared` whose method was pinned as
   the bare name `declared`, added for the `impl Trait` argument
   position with nobody reading the row under it. And
   `read_minting_items("impl Subject { pub const ALL: &[u8] = &[]; }")`
   answered `{}` — silent — while the same shape under a rostered row
   above it charged the const's literal to that row, which is the
   inflate-then-cancel defect the unit was dispatched to close, in the
   population key rather than in the attribution rule. Both closed: the
   scope walk reads `trait` and `fn` as well as `impl` and `mod`, and
   the declaration walk finds `fn`/`const`/`static` at the keyword
   under an allow-list of what may precede an item.
2. **The hard-stop's premise was false and this unit had widened its
   reach.** *"Rust rejects two such items in one crate"* is not true of
   `#[cfg(test)] mod pick` beside `#[cfg(not(test))] mod pick`, of two
   functions each holding a `const` of one name, of a free `fn` beside
   a trait method of that name, or of two blocks in one function.
   Making `fn` and `trait` scopes turned three of those into two rows
   each — and took a method-local `const` off the enclosing `impl`,
   where it had been a phantom `A::W` carrying `A::m`'s word. What is
   left is the `cfg` pair and the two blocks, which no qualification
   can separate: the refusal says so now instead of asking its author
   for something Rust cannot spell.
3. **The scope walk's two guards were pinned by nothing**, and so were
   both of its panic arms. All four are executed now, each with an
   input that fails differently with that guard alone removed.
4. **The stub reader's convention had already drifted from the Python
   one it cites** — a bare `Final` was a class attribute there and an
   instance attribute here. Closed in Rust and pinned by a fixture row;
   the pair of readers is `one-stub-convention-has-two-readers-in-two-languages`.
5. **The known-gap column is a filed row or it is a suppression.** Each
   entry names the tracker row that schedules it and the test goes
   looking for that row under `work/`.

Filed rather than taken: `item-body-takes-a-const-generic-brace-for-an-item-body`
(TINT's — the repair is in `test_utils::source` and the sibling census
shares the exposure) and `the-mint-reader-hosts-three-lexer-operations-of-its-own`.
