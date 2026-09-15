---
id: py-discriminant-getters-under-src-py-are-outside-every-inventory
kind: unit
title: 23 Python-visible discriminant words are minted by getters under src/py/, outside TAG_INVENTORY
status: spec
opened: 2026-09-15
branch: census/py-getters
---


Found by the style review of CENSUS-TAG-REACH (2026-09-15) and
re-measured against the tree by that unit's fix pass. It is the same
class as `py-reason-and-variant-literals-outside-any-enum` and a
different SHAPE, which is why that row's sweep — a literal beside a
`"reason"` or `"variant"` key — found none of it.

## The population, measured

`crates/pncad-py/src/py/` holds **30** functions returning
`&'static str`. **Seven of them mint their words as string literals**
rather than delegating to `crate::tags`. Six of the seven reach a
Python caller, and between them they spell **23 distinct lowercase
discriminant words**:

| site | words |
| --- | --- |
| `py/mate.rs`, `Subgroup::variant` | 7 — `se3`, `planar`, `cylindrical`, `prismatic`, `revolute`, `trivial`, `empty` |
| `py/mate.rs`, `primitive_tag` | 4 — `coaxial`, `planar_rest`, `clocking`, `frame_coincidence` |
| `py/mate.rs`, the gauge-edit `variant` | 4 — `join`, `split`, `drop`, `gauge_rewrite` |
| `py/mate.rs`, `ClassAdmission::variant` | 3 — `mints`, `no_at_rest_record`, `not_admitted` |
| `py/assembly.rs`, `entity_kind_tag` | 4 — `face`, `edge`, `vertex`, `body` |
| `py/refactor.rs`, the refactor `variant` | 1 — `mate` |

The seventh, `py/doc.rs`'s `_binds_every_kernel_window`, is **not** in
that count and its two words (`arc`, `full`) are not Python-visible:
the function is a never-called compile-time tripwire over
`d::TubeWindow`, as its own doc says. The style review that opened
this row counted it, which is a two-word overstatement corrected here
rather than repeated.

Every one of the 23 is named in `crates/pncad-py/pncad.pyi`, so every
one is public Python vocabulary a caller branches on. **Two of the six
functions are literally named `*_tag` and live outside
`src/tags.rs`** — `entity_kind_tag` and `primitive_tag` — which is the
whole of the naming convention that is supposed to say "this word is
inventoried".

**Sixteen of the 23 appear nowhere in `src/tags.rs`.** The other
seven are the sharper half: `face`, `edge`, `vertex`, `empty`, `join`,
`split` and `no_at_rest_record` are ALSO minted in `src/tags.rs`, by a
different map, for a neighbouring concept — so they are a second
spelling of one word rather than a word outside every list.
`py/assembly.rs`'s `entity_kind_tag` and `crate::tags::entity_id_tag`
agree on `face`/`edge`/`vertex` by hand today, over two different
kernel enums; `ClassAdmission::NoAtRestRecord` and
`MintRefusal::NoAtRestRecord` do the same for
`no_at_rest_record`. Nothing holds either pair equal.

## Two shapes beside it, one of them a clean negative

* **The `Option<&'static str>` getters are clean.** The same sweep
  over the 17 `fn … -> Option<&'static str>` under `src/py/` (a
  population the `-> &'static str` pattern does not match) finds
  **zero** direct literals: every one delegates to `crate::tags`.
  Executed, not reasoned.
* **A word minted in tuple position is invisible to both sweeps.**
  `py/mesh.rs`'s `stl_err` builds `(variant, message, …)` per arm and
  takes six of its seven variants from `crate::tags`; the seventh,
  `NotUtf8`, spells `"not_utf8"` in the tuple. That one is a raise-site
  literal and belongs to the sibling row, which now carries it.

## Why no census sees any of this

`TAG_INVENTORY` (`crates/pncad-py/src/tests.rs`) lexes `src/tags.rs`
as text: a word spelled anywhere else is not in its population.
`NODE_KIND_ROSTER` beside it covers `crate::node_kind` alone.
`src/surface_census.rs` matches kernel tags against `pncad.pyi` — its
device (*"the witness is a MATCH on the kernel tag, not a list"*) is
the closest fit of the four instruments and reaches none of these
because they are not kernel tags. So a rename of `se3` to `se_3` is
green everywhere in Rust and breaks every Python caller.

## Shape of the fix, if it is taken

Not obviously "move them to `src/tags.rs`": several are getters on a
`#[pyclass]` whose `match` is already exhaustive over a kernel enum,
which is the existence half; what is missing is a guard over the
VALUES. Either they become `crate::tags` maps (and the inventory
covers them for free), or the inventory's population stops being "one
file" and becomes "every exhaustive `&'static str` map in the crate" —
which is a reader over `src/py/`, and a reader needs a guard of its
own, this program's standing trap. The choice is the row.

Territory: `crates/pncad-py/*` is LIB's fence and this program's
`keep_out` announces its pncad-py rows there.

## Verified at spec time, and sharpened (orchestrator, 2026-09-15)

The counts above were re-measured independently and **hold** — 30
functions, six Python-visible minting six maps, 23 words, the `doc.rs`
tripwire correctly excluded. First row in this program whose numbers
survived that check.

**The sharpening the row does not make:** all six functions are
exhaustive matches over a kernel enum with **zero wildcard arms**, so
`E0004` already holds each map to its kernel type — a new variant stops
the build today. What they are outside is the INVENTORY, which pins a
word's TEXT. So the live defect is a **rename**, not an addition:
change `revolute` to `hinge` and every test passes while public Python
vocabulary moves.

That narrows the mechanical half to a **relocation** — a map in
`tags.rs` is lexed, pinned in `TAG_INVENTORY` and reds on an addition or
a rename — which CENSUS-TAG-REACH proved one door over. The design half
is the seven second spellings, and that call is the lane's, per pair,
because the pairs are not alike.

Also for the sweep: `mate.rs`'s subgroup getter hand-lists its four
words in prose four lines above the delegation to the map that holds
them, with nothing tying the two.

`docs/CENSUS-PY-GETTERS-SPEC.md` binds the unit.
