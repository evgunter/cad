---
id: the-errors-arrival-blind-spot-list-claimed-exclusivity-and-was-short
kind: issue
title: a fourth consecutive blind-spot list claimed exclusivity and was short by two shapes, and the survivors are what matter
status: open
opened: 2026-09-15
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
   **One of the nine says it is not**: `EvalReason::ATTRIBUTE` records
   *"no Rust check names this word"*, held only by `pncad.pyi` and the
   Python suite.
3. **An `impl` at indentation loses its qualifier.** `impl_spans` keys
   on column 0. Executed 2026-09-15: an `impl ValidationRefusal` inside
   a `mod` reports `NEW item \`seventh\`` — loud, but under a bare name
   rather than the qualified one, so the complaint names an item that
   does not exist by that name. The site now says so. Teaching the walk
   to nest is the repair.
4. **`held_by` is prose and nothing re-derives it.** The reviewer
   checked all nine and every named test exists and says what the
   column claims, so this is a risk and not a defect today; a renamed
   test leaves the column pointing at nothing, and the census stays
   green.

## Why this is a row and not a paragraph in a PR body

`work/README.md`: *"disclosing a residue is not scheduling it."* Three
of the four above are disclosed at the site, which is where a reader
of the code needs them; none of them is scheduled anywhere, and the
unit that wrote them closes with its spec deleted.

Territory: `crates/pncad-py/*` is LIB's fence and this program's
`keep_out` announces its pncad-py rows there.
