---
id: param-names-are-not-unicode-normalised
kind: issue
title: Parameter names are not Unicode-normalised: two look-alike spellings are two admissible keys
status: open
opened: 2026-09-29
priority: P4
cost: M
design: true
---

Found by the review of `edit/param-name-door` (PR #3164).

## The finding

A parameter name is admissible when the expression parser reads it
back as a reference to itself (`crates/editor-core/src/doc.rs`,
`ParamName::new`; `crates/editor-core/src/parse.rs`,
`param_name_fault`). The lexer opens an identifier on
`c.is_alphabetic() || c == '_'` and continues on `is_alphanumeric`
(`parse.rs`, `lex`), with no normalisation, and a name is keyed by its
raw text (`doc.rs`, `impl Borrow<str> for ParamName`, and the derived
`Eq`/`Ord`/`Hash`).

So U+212B ANGSTROM SIGN (`Å`) and U+00C5 LATIN CAPITAL LETTER A WITH
RING ABOVE (`Å`) are both admissible, render identically, and are two
different parameters. Measured through the Python binding on the PR's
head: `ParamName("Å") == ParamName("Å")` is `False`, and
both construct. The same holds for any decomposed/precomposed pair
(`é` against `é`).

What it costs: a document can declare two parameters a reader cannot
tell apart, and an expression typed with one spelling resolves against
one of them silently (`ParseError::UnknownParam` if the author's
keyboard produced the other).

## What has to be decided

One of:

1. **Normalise at the door**: `ParamName::new` stores the NFC form.
   The key a document holds is then not the bytes offered, which the
   `Padded` arm's reasoning ("a different key from the one offered")
   currently refuses rather than repairs.
2. **Refuse non-NFC**: a new `ParamNameReason` arm; the offered text
   must already be NFC. Keeps "the key is the bytes offered" and makes
   the expression text door (`parse_expr`) the place an author meets
   the same rule, since a lexed identifier is looked up by its bytes.
3. **Accept as is**, and say so in `ParamName`'s docs: names are
   byte-keyed, and look-alike spellings are distinct.

Either of the first two needs a normalisation table (a dependency, or
the lexer's own), and has to decide whether the parser normalises
identifiers it lexes, so that a declared name and a typed reference
meet.
