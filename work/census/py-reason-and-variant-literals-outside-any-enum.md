---
id: py-reason-and-variant-literals-outside-any-enum
kind: unit
title: Python-visible discriminant words are minted as literals at raise sites under src/py/, one of them a second spelling of a word an inventoried map already mints
status: closed
opened: 2026-09-15
branch: census/py-raise-literals
closed: 2026-09-15
---



Found by CENSUS-TAG-REACH's sweep (2026-09-15), which closed the
evaluation door's half of the same class and left this half measured
rather than repaired.

## The class, and what stays open

A Python-visible discriminant reaches a caller as an exception's
`reason` or `variant` attribute. `TAG_INVENTORY`
(`crates/pncad-py/src/tests.rs`,
`the_whole_tag_table_matches_its_committed_inventory`) lexes
`crates/pncad-py/src/tags.rs` alone, so a word minted as a string
literal at a construction site under `src/py/` is public Python
vocabulary no inventory reads.

CENSUS-TAG-REACH closed that for the EVALUATION door: the reason is
now `crate::errors::EvalReason`, carried by `ErrorClass::Evaluation`
itself, `crate::tags::eval_reason_tag` is the exhaustive map, and no
raise of that class can be written without naming a variant of the
enum. **Nine raise sites in four files are unchanged, spelling nine
words, and they are not one shape:**

| word | site | door | what it is |
| --- | --- | --- | --- |
| `unclassified` | `py/flush.rs`, the unknown-`ContactClass` refusal | `SelectRefusal` (`ErrorClass::Select`) | **A SECOND MINT of a word that door's own map already mints.** `crate::tags::select_refusal_tag` returns `"unclassified"` at its wildcard arm, and `py/select.rs` writes `reason` from that map; this site writes the same attribute of the same class with a hand-spelled copy |
| `wireframe` | `py/value.rs`, the STEP-import success arm this door does not adopt | `StepImportError` | **A second mint on the same attribute too**: every other `variant` on this class comes from the inventoried `step_import_error_tag`, and the site's own comment says the word shares that namespace ("which contains no `wireframe`") |
| `not_utf8` | `py/mesh.rs`, `stl_err`'s `NotUtf8` arm | `StlError` | The same again, in TUPLE position: six of the seven arms take their `variant` from `crate::tags`, the seventh spells it |
| `name_serialize` | `py/doc.rs`, the `StableName` serialization refusal | `EditError` | A word with **no enum anywhere** — the site says so, and `boundary_edit_err`'s doc states the rule it is the exception to. Its two sibling callers pass `crate::tags` words |
| `mass_properties_failed` | `py/value.rs`, `measurement_err` | `ValidationError` | A word with no enum anywhere: two measurement doors share one construction site and the word is that site's literal |
| `validate`, `validate_closed`, `validate_geometric`, `validate_pseudomanifold` | `py/value.rs`, the four `run_validator` calls | `ValidationError.door` | A whole four-word vocabulary on a DIFFERENT attribute, minted as four arguments at four call sites. `pncad.pyi` documents all four |

**So "outside any enum" is true of six of the nine words and false of
the three that matter most** — `unclassified`, `wireframe` and
`not_utf8` each ride an attribute whose other words come from an
inventoried map. The common shape is a word minted at a raise site,
and the sharper sub-shape is a word on an attribute a map already
speaks.

### Re-measured, 2026-09-15 (CENSUS-PY-RAISE-LITERALS)

**Only ONE of the three is a second spelling of a word its map
mints**, and this row said three. `crate::tags::select_refusal_tag`
does return `"unclassified"` at its wildcard arm, so `unclassified` is
genuinely one word spelled twice. `step_import_error_tag` does **not**
mint `wireframe` — the site's own comment says so, and `grep
'"wireframe"' src/tags.rs` returns nothing — and `stl_error_tag` does
not mint `not_utf8`. Those two are NEW words on an attribute an
inventoried map otherwise fills, which is a weaker claim than this
row's and is the one that holds.

**The `pub const` route offered below for `unclassified` does not
exist.** This row says the inventory "already lexes that form", which
is true of a top-level `pub const` and false of a `match` ARM that
reads one: `tests.rs`'s `ArmShape` admits a literal, a nested `match`,
a block, `None`, `Some(..)` and a delegation, and nothing else, so
`select_refusal_tag`'s wildcard arm cannot read a const without
teaching the reader a seventh shape. The one-arm map is what was
taken — `crate::tags::unmirrored_select_tag` over
`crate::errors::UnmirroredSelect`, whose two arms are the query
door's wildcard and the contact-class crossing, both answering one
word. The wildcard arm DELEGATES to it, which the reader does admit.

**The second defect at that site is confirmed and repaired.**
`py/select.rs`'s eight-attribute list is now
`py::select::refusal_fields`, and both doors build from it; a door
with a payload writes it by NAME through `py::select::fill`, so a
reordering of that list cannot silently drop the entry it overwrites.
**The repaired path is unreachable today** — the kernel's
`ContactClass` has exactly the two arms the crossing matches, so
nothing can execute the raise, and the repair cannot go red either.
That is `both-unclassified-crossings-are-unreachable-and-so-is-the-repair-on-one`
on this slate, filed rather than fixed because it is a question about
what deserves a constructed test.

**Each is covered only by accident.** `mass_properties_failed` is
named in `crates/pncad-py/pncad.pyi` and asserted once in
`crates/pncad-py/tests/test_validate.py`; `wireframe` is named in the
stub and two Python test files; ONE of the four `door` words
(`validate_pseudomanifold`) is asserted on `.door`, in
`tests/test_validate.py` and `tests/test_checks.py`, while the other
three appear in the suite only as METHOD names and in the stub;
`not_utf8` is named in the stub and in no Python test; `unclassified`
and `name_serialize` are in neither. Nothing reds if one is renamed,
and nothing reds if a tenth is added — which is the standing lesson,
not the words.

`crates/pncad-py/src/tests.rs`'s inventory doc carries three of these
(`mass_properties_failed`, `unclassified`, `wireframe`) as "what is
still outside", so the fact is stated where
the guard is; a PR body is not a slate, which is why it is also here.

The GETTER half of the same class — 23 Python-visible words minted by
six `-> &'static str` functions under `src/py/`, which no sweep for a
literal beside a `reason`/`variant` key can find — is
`work/census/py-discriminant-getters-under-src-py-are-outside-every-inventory.md`.

## Shape of the fix, if it is taken

The evaluation door's shape transfers for the words with no enum:
give the door's reason vocabulary an enum in `crate::errors`, an
exhaustive map in `src/tags.rs`, and carry it on the class — which is
what makes the word unspellable at the raise site rather than merely
spelled elsewhere.

**The `unclassified` case is smaller but is NOT "call
`select_refusal_tag`".** That function takes a `&SelectRefusal`, and
`py/flush.rs` has an unknown `ContactClass` in hand — a different
kernel type, at a crossing that has no refusal value to pass. What
fits is either a `pub const` in `src/tags.rs` that both the wildcard
arm and this site read (the inventory already lexes that form), or a
one-arm map for the crossing's own word. Either way the two spellings
become one.

A second defect at that site, found while measuring this one and worth
repairing in the same change: `py/flush.rs`'s raise attaches `reason`
ALONE, where `py/select.rs` attaches all eight of `SelectRefusal`'s
attributes with `None` where inapplicable. So a caller that reads
`err.name` after catching a `SelectRefusal` gets an `AttributeError`
on exactly one of the class's paths — the house
every-attribute-always-present rule, broken at the one site that does
not go through `select_refusal`.

Territory: `crates/pncad-py/*` is LIB's fence and this program's
`keep_out` announces its pncad-py rows there.
