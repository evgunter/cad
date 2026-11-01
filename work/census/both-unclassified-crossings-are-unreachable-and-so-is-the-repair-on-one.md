---
id: both-unclassified-crossings-are-unreachable-and-so-is-the-repair-on-one
kind: issue
title: Both unclassified crossings are unreachable today, so the word no test can construct and the AttributeError repair beside it cannot go red
status: open
opened: 2026-09-15
priority: P3
cost: E
---



Filed by CENSUS-PY-RAISE-LITERALS' fix pass (2026-09-15), from its own
style review, and verified against the tree before filing.

## The measurement

`crates/pncad-py/src/tags.rs`'s `unmirrored_select_tag` mints
`"unclassified"` for both arms of `crate::errors::UnmirroredSelect`.
**Neither arm can be constructed today.**

- `UnmirroredSelect::Refusal` is reached only from
  `select_refusal_tag`'s wildcard. That wildcard is FORCED —
  `pncad::select::SelectRefusal` is `#[non_exhaustive]` — but the enum
  has exactly eight arms (`InBand`, `TiedDisagrees`, `Unreadable`,
  `NotADatum`, `NotALength`, `PairInBand`, `BadValue`, `Band`, in
  `crates/editor-core/src/names/geompred.rs`) and the map names all
  eight above the wildcard.
- `UnmirroredSelect::ContactClass` is reached only from
  `crate::py::flush`'s `contact_class`. `pncad::select::ContactClass`
  is `#[non_exhaustive]` too and has exactly `Rest` and `Tangent`
  (`crates/topo/src/contact.rs`), both matched above that wildcard.

So `unclassified` is **the only word in `TAG_INVENTORY` that no test,
Rust or Python, can make the binding emit** — it is inventoried, it is
spelled once, and nothing can execute either path that spells it.

## Why that is worth a row

Two things ride the dead path, and the second is the sharper one.

1. CENSUS-PY-RAISE-LITERALS' PR presents the `AttributeError` repair at
   the contact-class crossing as a defect fixed: that raise used to
   attach `reason` alone, where every other `SelectRefusal` carries all
   eight attributes with `None` where inapplicable, so a caller reading
   `err.name` got an `AttributeError` on exactly one of the class's
   paths. The repair (`crate::py::select::refusal_fields`, shared by
   both doors) is correct and is the right shape. **It sits on a path
   no test can reach**, so it cannot go red either: a later change that
   reinstated the one-attribute list at that site would pass every gate
   in the repo. The fix is real; the claim that a guard protects it is
   not one anyone has made, and this row is here so nobody makes it.
2. The same holds for the WORD. `unmirrored_select_tag` exists so the
   query door's wildcard and the contact-class crossing cannot drift
   apart, and the map is what makes that structural — but the
   inventory row for it is the only evidence the word is ever right,
   because no execution reaches it.

## What a lane could do about it

Nothing here is a bug to fix; the question is whether a word and a
payload shape that only a future kernel variant can exercise deserve a
constructed test. Shapes worth weighing:

- a Rust-level test that builds the `refusal_fields` list directly and
  asserts the eight attribute names, which holds the payload shape
  without needing a kernel variant to exist;
- accepting the dead path explicitly, with the reasoning at the site
  rather than in this row.

Neither needs the kernel to grow a variant, which is what makes this
worth someone's half hour rather than a wait.

Territory: `crates/pncad-py/*` is LIB's fence and CENSUS's `keep_out`
announces its pncad-py rows there.
