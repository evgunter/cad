---
id: creation-form-fields-lost-two-spellings-egui-accepted
kind: issue
title: The creation forms stopped parsing interior spaces and U+2212, which egui's own parser accepted
status: closed
opened: 2026-09-21
priority: P2
cost: E
refs: [3007]
closed: 2026-09-22
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

## Ruled: option 1, leave it (Ev, in chat, 2026-09-22)

Put to Ev by this orchestrator at the VGEOM hand-over, as the fork
above states it. Ev's answer: *"less expressive, simpler parser is
fine if that means we have simpler code / more consistent behavior /
less duplication."*

That is option 1 on all three of its own terms, and the fork's own
text says so: one parser and one rule is the simpler code; the panel
and the creation forms reading the same texts is the more consistent
behaviour; and options 2 and 3 both add a normalisation layer that
option 1 does not have — option 3 explicitly a second one, above
`field_edit`, so the widget and the expression door would read
different texts.

**So `crate::props::field_edit` stays as it is**: trim, then
`f64::from_str`. `1 234 567` and a U+2212 minus do not parse, in the
creation forms or in the properties panel, and the rule *what Rust
reads as a float is a number and everything else is source* keeps
`1 234` an expression rather than a quantity.

**What this closes and what it does not.** It closes the question the
row was opened to ask. It is not a finding that the two spellings are
undesirable: if a person hits the missing U+2212 in practice — a
paste from a document is the realistic route — that is new evidence
and a new row, against the shared parser, not a reopening of this one.
