---
id: LIB-SMALL-2
kind: unit
title: two recourse sentences, the stub's missing __eq__ with its guard, and the façade guard's last line-local readers
status: review
opened: 2026-09-09
branch: lib/small-2
refs: [two-refusals-carry-no-recourse-sentence, pncad-py-stub-omits-eq-on-three-mate-classes, facade-guard-file-keeps-two-line-local-readers]
---


Three small closes on LIB's slate, taken as one mechanical unit.

## Delivered

**1. The two refusals that carried no recourse now carry one.**

- `CONTRADICTORY_RECOURSE` and `NO_AT_REST_RECORD_RECOURSE` live in
  `crates/editor-core/src/mate.rs`, beside `UNDER_RECOURSE` and
  `CLASS_DEFERRAL`, and each Display arm ends on its own the way
  `Under` does.
- **One sentence covers both shapes of the contradiction arm**, not
  two: the arm renders a PAIR of mates and a mate contradicting itself,
  and "delete one of the two" is false of the second. The recourse says
  what to do in either — delete the mate that was not meant, or
  re-author the datum that is wrong — and names the rung as the solve
  door: cosets are intersected exactly and two declarations are never
  averaged.
- **The arm has four exits and all four now end on it.** The empty-set,
  levered, finite and non-finite branches used to `return` their own
  measurement; they write it and fall through to one recourse write.
  `display_contract.rs`'s non-finite row exercises every exit and
  asserts the recourse on each.
- **The rung is named in v1 vocabulary, not by a roadmap id.** The
  filed item suggested "R3/M9 work"; `CLASS_DEFERRAL` and `FIT_DEFERRAL`
  name their boundary as "v1" and "not yet built", and a user-facing
  sentence naming an internal id is a dead pointer. The mint recourse
  says `Rest` is the one class v1 mints and verifies at rest, and that
  a curved contact verified at rest is outside v1 and is not built.
- Both are carried through `crates/pncad/src/document.rs` and bound
  top-level in Python on `UNDER_RECOURSE`'s precedent, so a test saying
  "the refusal ends on its recourse" never re-types the prose. The
  census bullet that argued the first two now argues all four.
- The tour's refusal walk asserts all four against the library's
  constants (it asserted only the pin one before) and its output was
  regenerated through the tour's own door; the diff is in the PR.

**2. The stub declares `__eq__` where the compiled class carries one,
and a guard says so.**

- The guard reads `__eq__` off the compiled class's OWN `__dict__` —
  the direction `hasattr` cannot reach — and requires a stub
  declaration. `module_class_names`' underscore filter is argued at its
  site rather than widened: restating PyO3's slots in 130 class bodies
  documents nothing, and `__eq__` is the one comparison dunder whose
  presence in the own dict is evidence (`__lt__` sits there on a class
  whose `<` raises `TypeError`, `__hash__` sits there as `None` on a
  class that is unhashable).
- **The fieldless mirrors are exempt and the exemption is CHECKED**,
  not listed: a mirror's stub body is its `Final[<the class>]` members,
  the stub declares no dunder on any of them, and that is honest only
  because the members are interned — so the row asserts, per member,
  that two accesses answer the same object. The day one stops being a
  singleton the exemption fails instead of covering for it.
- **It found seven, not the three the item named**, and the four extra
  are the item's own declared blind spot ("a class whose `__eq__`
  arrives some way other than a `fn __eq__` in an `impl` block"):
  `Length`, `Angle` and `Count` compare through `__richcmp__`, and
  `FaceCensus` has a `fn __eq__` the item's sweep missed. All seven are
  declared now. `FaceCensus` also gained `__hash__`, which it really
  implements — a stub declaring `__eq__` without it says the class is
  unhashable.

**3. The façade guard file's last three line-local readers are
statement readers.**

- `crates/pncad/tests/all.rs` reads Rust through `test_utils::source`
  now; its hand-rolled `code_without_comments` is deleted and the four
  root scanners take a blanked view under one convention. `test-utils`
  is the crate's one dev-dependency and the U1 guard admits it as a
  `use` root, argued at the allow-list.
- The U1 guard's check 1 reads `use` STATEMENTS off the `code_only`
  view, so a root on a continuation line is seen and the reader
  selftests' own literal snippets are not read as this file's imports.
  Check 2 stays on the literal-keeping view: a contiguous kernel path
  in a literal is indistinguishable from one in code, and reading it as
  a violation is the false-ALARM direction.
- `root_declared_pub_names` reads its keyword and name as tokens across
  line breaks; column 0 is still the whole scope rule, and
  `pub(crate)` still reads no keyword.
- `code_without_cfg_gated` takes the attribute's extent from
  `balanced_end` and the item's from `item_body`, and blanks rather
  than deletes so every line stays where it was.
- **The item's blind spot for reader 3 is not the one it names.** It
  says a wrapped `#[cfg(…)]` list "is not seen as a gate at all";
  replayed, the old line reader handles that shape. Its real blind spot
  is an attribute sharing its line with the item it gates — the line
  unit then swallows the NEXT declaration too, so a root export
  vanishes from the view and the completeness guard passes over it.
  That is a false GREEN, and it is the fixture the selftest uses.
- `crates/test-utils/tests/reader_census.rs`'s entry for this file is
  `Shared`; `UNCONVERTED_TODAY` is 4.

**Not taken.** `north-star-audit-verb-list-names-arc-continue` stays
open: BOOL-10 (#2135) is still `status: review` and `arc_continue` is
still in the tree, so its condition has not fired.

## Closed

- `two-refusals-carry-no-recourse-sentence`
- `pncad-py-stub-omits-eq-on-three-mate-classes`
- `facade-guard-file-keeps-two-line-local-readers`
