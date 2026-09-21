---
id: creation-form-fields-lost-two-spellings-egui-accepted
kind: issue
title: The creation forms stopped parsing interior spaces and U+2212, which egui's own parser accepted
status: open
opened: 2026-09-21
priority: P2
cost: E
refs: [3007]
---

Filed by the review fix pass on #3007, which disclosed this cost in
three places and gave it no file. `work/README.md`: *"Disclosing a
residue is therefore not scheduling it — give it its own file at the
moment you disclose it"*. The row it was disclosed inside
(`a-fields-text-commits-within-the-renders-own-tolerance`) is open for
a different bullet, so it would have gone when that one closed.

## Finding

`crates/viewer/src/widgets.rs`, `number_field`. The echo veto lives in
a `custom_parser`, and installing one REPLACES `egui`'s
`default_parser` for every field the constructor builds. The two do
not read the same texts:

```
// egui-0.36.1/src/widgets/drag_value.rs, default_parser
    .filter(|c| !c.is_whitespace())            // thousands separators
    .map(|c| if c == '−' { '-' } else { c })   // U+2212
```

`crate::props::field_edit` trims and then asks `f64::from_str`, so
`1 234 567` and `−5` (U+2212, what a keyboard layout or a paste from a
document produces) now parse to nothing in every creation form; the
field keeps the value it held and says nothing. The properties panel's
two value fields have read text through `field_edit` since AUTH-2 and
have never accepted either, so the chrome is now consistent rather
than half-and-half — which is why this is a P2 and not a P0.

## The fork

1. **Leave it.** One parser, one rule, and the panel's behaviour is
   the one that generalised. Costs the two spellings everywhere.
2. **Normalise in `props::field_edit`** — strip interior whitespace
   and map U+2212 before `from_str`. One home, both doors, and the
   panel GAINS the two spellings. The question it owes: whether
   `field_edit`'s *"what Rust reads as a float is a number and
   everything else is source"* rule should admit a text Rust does not
   read, since the same function decides Number-versus-Expression and
   `1 234` would stop being an expression.
3. **Normalise at the widget**, above `field_edit`, so only the
   fields change and the expression door is untouched.

Ev was told in chat per #3007's orchestrator disposition; if the
answer is that both spellings are kept, this row is where it lands.

## Fence

`crates/viewer/src/widgets.rs` and `crates/viewer/src/props.rs` —
VGEOM's, double-claimed with CHROME and VIEW.
