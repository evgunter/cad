---
id: contact-class-has-two-content-tag-functions
kind: issue
title: ContactClass is tagged into content keys by two functions: contact_class_tag (u8, wildcard arm) and ContactClass::content_tag (u64, exhaustive)
status: closed
opened: 2026-09-06
closed: 2026-09-11
---


Found by MSOLVE-4's style review (PR 1960), outside the unit's fence;
filed by the MSOLVE orchestrator. The content key is `eval/mod.rs`'s,
DOCM's ground; the enum is `topo`'s.

`crates/editor-core/src/eval/mod.rs`: the `Node::Mate` arm of
`content_key` writes `contact_class_tag(*class)` — a `u8` with a
wildcard `_ => 0` arm whose own doc admits the collision hazard — while
the `Node::Declare` arm fifteen lines away writes
`class.content_tag()` (`crates/topo/src/contact.rs`, `u64`,
exhaustive), and the interface-crossing feed is a third site. Same
numbers, two homes, one carrying the `_` arm the other exists to
forbid. One tag function, exhaustive, and a sweep for any other
`ContactClass` tagging.

## Re-homed to WIRE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

WIRE is the evaluation seat's successor. `docs/DOC-LEDGER.md` sweep 9
parked three of these rows in `work/issues/` as "the successor's opening
slate" when EVAL closed on 2026-09-08, naming two more already there;
this program is that successor, and it takes the rest of the seat's
ground with them.

Its class at the cut was **E** — appears already discharged: only
`ContactClass::content_tag` survives in `eval/mod.rs`; verify and close.
The class is a dispatch estimate made by reading the row against the
tree on 2026-09-11, not a verdict on the finding, and a lane that finds
it wrong says so in its PR. The id, the `track:` letter where the row
carries one, and the body above are unchanged by the move.

## Closed (2026-09-11) — verified discharged, no unit

Verified against `8851abb` by the WIRE orchestrator at the program's
opening, which is what the cut's class estimate (**E**, "appears already
discharged: verify and close") asked for. Three greps, and the finding's
own three sites:

- **`contact_class_tag` does not exist.** `rg 'contact_class_tag'` over
  `crates/` returns nothing but one rustdoc sentence
  (`crates/topo/src/contact.rs:64`) naming `ContactClass::content_tag`
  as the tag. The `u8` with the `_ => 0` wildcard arm is gone from the
  tree, and with it the collision hazard its own doc admitted.
- **The content key has one tagging function.** `ContactClass::content_tag`
  (`crates/topo/src/contact.rs:94`, `u64`, exhaustive) is what all three
  `eval/mod.rs` key sites feed — `:3687`, `:3712` and `:3728`, the last
  two being the `Node::Mate` and `Node::Declare` arms the finding named
  fifteen lines apart. They now write the same call.
- **The "interface-crossing feed" is not a second numbering.**
  `crates/editor-core/src/persist/kernel_wire/contact_class.rs` is a
  **string** vocabulary (`"rest"`, `"tangent"`) for the persisted form,
  anchored on `ContactClass::ALL`, with an explicit refusal arm for a
  class a newer kernel adds. Persisted spellings and content-key numbers
  are different commitments with different change rates; one enum, two
  exhaustive projections, both over `ALL`, is the shape this repo
  chose deliberately and not the twin the finding reported.

The sweep the finding asked for ("a sweep for any other `ContactClass`
tagging") is the third bullet: every `ContactClass` use outside
`contact.rs` was read, and the only conversions to a scalar are
`content_tag` and the persist module's spelling. **What that sweep could
not match**: a tagging written without the type's name in it — a call
site that takes the class as a generic or through a trait object would
not appear. None was found by reading the three key sites, but the grep
alone is not evidence about that shape.

No residue, so nothing is filed onward.
