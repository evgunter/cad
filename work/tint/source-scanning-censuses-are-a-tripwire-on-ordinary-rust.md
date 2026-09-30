---
id: source-scanning-censuses-are-a-tripwire-on-ordinary-rust
kind: issue
title: the source-scanning censuses hand-parse Rust and fail loud, so an ordinary-but-unusual signature reds another program's test with a byte offset for a message
status: open
opened: 2026-09-04
priority: P3
cost: D
---


Found by FIX's `no-parametric-loop-constructor` lane (PR 1765) by
tripping it. Filed by the FIX orchestrator; S-TCOST is the natural
claimant (`crates/*/tests/*` is its territory).

## What happened

`crates/geom-core/tests/bounds_census.rs`'s `every_sole_bracket_bound_door_is_in_the_roster`
walks the tree's **source text** and hand-parses signatures. Its
`angle_end` helper closed a generic parameter list at the first `{` or
`;` with no bracket-nesting check, so an ordinary signature —

```rust
pub fn polygon_expr(points: impl IntoIterator<Item = [Expr; 2]>) -> Self
```

— read the `;` inside `[Expr; 2]` as the item's body and panicked:

```
a generic parameter list at byte 56361 does not close before its item's
body: <Item = [Expr; 2]>) -> Self {
```

The lane's repair was right: keep `[Expr; 2]` (it is exactly what
`ProgramStep::At` holds and what `pt_lit` returns — distorting a door's
type to suit a census parser is the wrong direction), and fix the
scanner to read the terminator at square/round-bracket depth zero only,
which is the nesting its own sibling `top_level_params` already
respects for commas.

## Why this is a class and not that one bug

Two censuses hand-parse Rust and both stop the walk **fail-loud by
design**: `bounds_census` and `flagged_census`. The lane checked and
`flagged_census::skip_turbofish` does **not** carry this particular bug
(it counts angle depth alone, with no `{`/`;` break), so that negative
result stands. The structural point survives it:

**A lane writing an ordinary-but-unusual signature can red a census in
another program's territory, with a panic that names a byte offset
rather than the rule it was enforcing.** The lane that trips it has
done nothing wrong, learns nothing from the message, and must repair a
test it does not own — under CI-red pressure, in a file it has never
read. That is the worst combination of circumstances in which to edit a
hand-written parser, and it is where a second bug hides.

Note the shape is not "the censuses are wrong". They enforce real
invariants and failing loud is correct. The defect is that their
failure mode is indistinguishable from a defect in the *code being
scanned*, and their coverage of Rust's grammar is whatever the
signatures in the tree happened to need so far.

## Dispositions worth weighing

1. **Make the panic name the rule.** The cheapest real improvement: a
   message that says "this census hand-parses signatures and could not
   read yours; the census is likely wrong, not your code" would have
   saved the whole diagnosis. It does not fix the parser and does not
   need to.
2. **Fence the scanners' grammar explicitly** — state at each scanner
   what it does and does not parse, so the next lane can tell in one
   read whether it is in scope.
3. **Stop hand-parsing.** The heavy option; only worth it if the
   censuses grow.

(1) is almost free and closes most of the cost. Not decided here.

## Home

`work/issues/` — the sites are `crates/geom-core/tests/` and
`crates/*/tests/`, S-TCOST's territory. Re-home by header edit.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/tcost/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Moved to S-TINT (2026-09-11)

Moved by `git mv` from `work/tcost/` at S-TINT's opening. Id, title and
body are unchanged; the directory is the claim (`work/README.md`).

**Why it moved.** S-TCOST's board was re-sorted on 2026-09-11 against the
repository going public on 2026-09-03 (`work/tcost/log.md`, the
2026-09-11 seam). That sort found this row is not a cost lever in either
currency — it neither shortens the gate's critical path nor saves a
billed minute, and it was never argued on one. It reached S-TCOST by the
tracker-wide re-home of 2026-09-04, which routed rows by PATH GLOB
(`crates/*/tests/*`, `crates/test-utils/*`) rather than by question.
This program is the question it was always about: whether the suite
asserts what it claims to assert.

## A live instance, still armed (2026-09-11, from FIX)

Reported by FIX's `checks-product-refusal-degrades-to-string` lane
(PR 2344) and placed here by the FIX orchestrator — an instance, not a
claim on the row.

`crates/editor-core/tests/docm5_subject.rs`'s `gathers_in` counts lines
of `crates/editor-core/src/checks.rs` **source text** containing
`" product("` and asserts the count is 1. The lane added an ordinary
method named `ChecksError::product`, and the row went red claiming
"one gather call in the registry's source" — **a false accusation
against correct code**, from a declaration that is not a call at all.

The lane renamed its constructor to `product_unavailable` rather than
patch a scanner in another program's file (it is a better name beside
`separation_unavailable` anyway, and it matches the published tag). So
nothing is fixed: **the tripwire is still armed** for the next person
who names anything `product` in `checks.rs`.

Worth recording for this row's argument specifically: the row's
existing instances are censuses that scan a *declaration* grammar. This
one scans for a **call** by matching `" name("` in source text, which
cannot tell a call from a declaration, a definition, a doc comment or a
string. That is a second grammar with the same failure mode, and it
suggests the row's subject is source-text counting as such rather than
any particular parser's gaps.

## Re-derived (2026-09-15, lane C)

**VERDICT: PARTIAL** — disposition 2 (fence the grammar) has largely
been done and the original parser bug is fixed and hoisted; disposition 1
(make the panic name the rule) is **not** done, and the live instance is
**still armed**.

### The original bug: fixed, and fixed in the right place

`crates/geom-core/tests/bounds_census.rs` no longer carries its own
`angle_end` reading. Its `fn angle_end` is a one-line wrapper whose doc
says so — *"**The reading is [`test_utils::source::angle_end`]'s**, not a
copy of it: … a copy of a lexer's postcondition at a call site is how
this tree grew its readers in the first place."* The shared
`pub fn angle_end` in `crates/test-utils/src/source.rs` reads the
terminator at bracket depth zero and states the repair as its own
invariant: *"**The item's body opens at the first `{` or `;` OUTSIDE
every square and round bracket.** A fixed-size array in a generic
argument — `<Item = [Expr; 2]>` — carries a `;` that ends no item, and
reading it as a terminator closes the list early."* So the exact
signature that tripped this row — `impl IntoIterator<Item = [Expr; 2]>`
— parses today.

`crates/geom-core/tests/flagged_census.rs`'s `fn skip_turbofish` is still
there, still angle-depth-only, and still does not carry that bug — the
lane's negative result stands.

### Disposition 2: mostly done, and worth crediting

Both scanners now fence their grammar in writing, which is what this
disposition asked for:

- `bounds_census.rs`'s `angle_end` doc states the asymmetry AND the
  silent half: *"**That asymmetry is only half of the exposure, and the
  other half is silent.** … A list closed at the WRONG `>` … answers a
  list that is too SHORT, and a short list still parses, so the census
  would undercount with no signal at all."*
- `source.rs`'s `angle_end` states the same residue at the home
  (*"**The residue, stated:** a genuine `>` comparison inside a const
  generic argument still closes the list early"*), and its `item_body`
  states what it cannot check (*"What this cannot check is that `from` IS
  a head"*).
- The second instance's reader, `gathers_in` in
  `crates/editor-core/tests/docm5_subject.rs`, now carries a *"WHAT THIS
  CANNOT SEE, stated because a count that hides its blind spots is not a
  receipt"* paragraph.

### Disposition 1: NOT done — the message is byte-for-byte the one filed

```
"a generic parameter list at byte {open} does not close before its item's body: {:.120}"
```

That is the panic in `bounds_census.rs`'s `angle_end` today. It still
leads with a byte offset and still says nothing of the form *"this census
hand-parses signatures and could not read yours; the census is likely
wrong, not your code"*. The row's own assessment — *"(1) is almost free
and closes most of the cost"* — is unacted on, and it is the half that
addresses the actual harm (a lane under CI-red pressure editing a parser
it does not own).

### The live instance: still armed, with one arm disarmed

`crates/editor-core/tests/docm5_subject.rs`'s `fn gathers_in` still
counts lines matching `" product("` (alongside `"product_recorded("` and
`"product_named("`), and `the_registry_gathers_once_and_the_door_under_it_never_does`'s assertion
is still `assert_eq!(gathers_in(include_str!("../src/checks.rs")), 1)`.

**What changed**: it reads through `test_utils::source::code_only` now
rather than raw text with a hand-rolled `//`-prefix filter, so two of the
four confusions the row lists are gone — a doc comment and a string
literal can no longer answer for a call. `crates/editor-core/src/checks.rs`
proves it: the file has two `product`-shaped hits, one of which
(`/// … [`crate::product()`] …`) is a doc comment that the view blanks,
leaving the count at 1 from `product::product_recorded(` alone.

**What did not change**: the scan still cannot tell a **call** from a
**declaration**. A method or free `fn product(` in `checks.rs` still
matches `" product("` in the code view and still raises the count. So the
exact tripwire that cost FIX's `checks-product-refusal-degrades-to-string`
lane a rename is live, and the row's closing observation — that this is a
second grammar (a call grammar) with the same failure mode — stands.

### The class, re-counted

Three source-text scanners with this failure mode at this base, not two:
`bounds_census.rs` (declaration grammar, fail-loud, bug fixed),
`flagged_census.rs` (declaration grammar, fail-loud, no bug), and
`docm5_subject.rs`'s `gathers_in` (call grammar, fail-**wrong** rather
than fail-loud — it accuses correct code instead of refusing to read it).
The third is the worse failure mode and is the one still armed.

**Blind spot of this re-derivation**: the scanner census was reached from
the three sites the row names plus the `reader_census.rs` ledger; a
source-text counter in a file dispositioned `Shared` that neither the row
nor the ledger's comment column flags would not have surfaced.

**Recommendation (orchestrator's call).** Keep open, narrowed:
disposition 2 is substantially discharged, so what is left is (1) — three
panic/assert messages that should name the rule — plus the `gathers_in`
call-vs-declaration confusion, which is a scanner change rather than a
message change. `crates/editor-core/tests/` and `crates/geom-core/tests/`
are both on this slate's glob, so nothing here needs routing.
