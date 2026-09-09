---
id: node-slot-literals-erase-the-authored-notation
kind: issue
title: node-slot literals record the canonical row, whatever unit the caller wrote
status: open
opened: 2026-09-08
needs_ev: true
---


Found by LIB-B-NOTATION's sweep, banked rather than taken: the family
it closes is parameter-scoped, and this is the same erasure one
vocabulary over.

## What happens

Every Python door that takes a `Length` or an `Angle` into a NODE SLOT
builds its stored expression through
`crates/pncad-py/src/py/doc.rs:80`'s helper, which calls
`Expr::literal(value, dim)` — the constructor that stamps
`UnitSym::canonical_for(dim)` (`crates/editor-core/src/expr.rs:616`).
So `Node.extrude(profile, 25 * mm)` stores a literal whose display unit
is `m`, and the recipe reads back `0.025 m` rather than `25 mm`. The
sibling constructor `Expr::literal_with_unit` (`:632`) — and
`Expr::written_length` / `written_angle` (`:672`/`:687`) above it — is
what records the notation, and no binding calls it.

The kernel's own words for the cost are on `Expr::written_length`: it
is "the door library and GUI authoring should reach for", and
`crates/pncad/src/prelude.rs:149` says it is "how a library recipe
records `300 mm` rather than `0.3` for a reader to interpret".
`Doc.parse_expr("25 mm")` DOES record it, so a Python caller who
authors through text already gets the notation and one who authors
through the typed doors does not — the two spellings of one authoring
disagree.

## The hit list

`grep -n "literal(py" crates/pncad-py/src/py/*.rs` — 52 call sites in
18 doors:

- `crates/pncad-py/src/py/doc.rs` (48 sites, 15 doors): `Node.polygon`
  `:1022`, `extrude` `:1097`, `revolve` `:1113`, `tube` `:1166`,
  `hollow_tube` `:1213`, `sketch_frame` `:1279`, `datum_axis` `:1318`,
  `datum_axis_in_plane` `:1355`, `datum_face_frame` `:1408`,
  `datum_plane` `:1432`, `fillet` `:1484`, `chamfer` `:1523`, `shell`
  `:1565`, `transform` `:1611`, and `TubeWindow.arc` `:2553`.
- `crates/pncad-py/src/py/place.rs` (3 sites): `PatternKind.linear`
  `:282`, `PatternKind.circular` `:297`.
- `crates/pncad-py/src/py/select.rs` (1 site): `GeomPred.datum_distance`
  `:667`.

**What that pattern cannot match.** It finds the sites that build a
literal through the shared helper and nothing else. A door that
constructed an `Expr` some other way, or one that takes a bare `float`
into a dimensioned slot without going through `literal`, would not
appear — and 20 of the 52 hits are `Dimension::Scalar` and 3 are
`Dimension::Count`, which have no notation to lose, so the affected
count is smaller than the site count.

## Why it was not fixed at LIB-B-NOTATION

Fixing it needs a `WrittenLength`/`WrittenAngle` seat at each door,
because a Python `Length` cannot carry the unit (see that unit's
derived scope: `quantity::written` gives authored quantities no
arithmetic, and `Length` is the arithmetic type). That is a signature
change on fifteen node constructors and two selector vocabularies —
a surface decision about the whole authoring lattice, not a mechanical
census row. The census could not report it either: every one of these
doors is a member behind `Node`, `PatternKind`, `TubeWindow` or
`GeomPred`, names rule 1 accounts whole.

## What closing it would have to decide

Whether the written seat is a second argument, an overload of the
existing one, or a `Node.written_*` sibling family; and whether the
kernel's canonical fallback stays reachable at each door for a caller
who genuinely has no notation to record.

## Question for Ev (2026-09-08, LIB orchestrator; `[ev]` PR)

Fifteen node constructors and two selector vocabularies take a
`Length`/`Angle` into a node slot through `Expr::literal`, which stamps
the canonical unit — so `Node.extrude(profile, 25 * mm)` reads back
`0.025 m` while `Doc.parse_expr("25 mm")` remembers. LIB-B-NOTATION
bound `WrittenLength`/`WrittenAngle` for document parameters; this is
the same erasure one vocabulary over, and the fix is a seat at each
door. Which seat?

- **(A) The same argument accepts either type**: `Length |
  WrittenLength` (and `Angle | WrittenAngle`) at every dimensioned
  slot door — a `WrittenLength` records its notation through
  `Expr::written_length`, a `Length` takes the canonical fallback as
  today. No new arguments, no doubled family, the two spellings of one
  authoring agree, and the canonical path stays reachable for a caller
  with no notation. Recommended.
- **(B) A second optional `notation=` argument** on each door — the
  unit rides beside the number, twice.
- **(C) A `Node.written_*` sibling family** — fifteen doors doubled.

Recommendation: **(A)**; it is the shape `DocParam` already has
(`Doc.set_doc_param` takes either), so the lattice stays one lattice.

### (D), added 2026-09-09 after Ev objected to the elision between dimensioned input and bare kernel numbers

Measured first: a literal's `display_unit` IS presentation metadata
under D7 — it round-trips through persistence and feeds the
formatter, and never enters `bit_eq`, `literal_bits` or any
content/naming key (`crates/editor-core/src/expr.rs`,
`literal_with_unit`'s doc). Kernel numbers are bare canonical; user
input is dimensioned; the unit is how it displays. Nothing else on
that front needs fixing.

The elision is at construction: Python's `25 * mm` — a dimensioned
user input — forgets the `mm`, which is why (A) needs a second type
at every slot to carry it back in. **(D): the Python `Length`/`Angle`
carries the unit it was written in as presentation metadata**, as
`Expr` does — ignored by `==`/hash, dropped by arithmetic (a sum has
no written unit and falls back to canonical), and read by every
existing door, which records it through `Expr::literal_with_unit`
when present and canonically otherwise. `Node.extrude(profile,
25 * mm)` then reads back `25 mm` with no new seat, no second type at
slots, no signature change on fifteen doors. Binding-only; the kernel
`Length` newtype is untouched; `WrittenLength` stays as the explicit
parameter form (retirement, if any, is a separate question).

Recommendation revised: **(D)** over (A).

### (D) withdrawn, 2026-09-09 — it breaks the mirror

Ev: "aren't the python types supposed to closely mirror rust ones?"
They are, and (D) would not. Rust's `Length`/`Angle` are bare `f64`
newtypes (`Copy, PartialEq, PartialOrd`; `crates/quantity/src/lib.rs:89`)
and a Rust author who wants the notation kept writes a `WrittenLength`
and hands it to `Expr::written_length` — `demos/tour/src/ring.rs:190`
does exactly that. Python already mirrors both types, so the mirror of
"a Rust door takes an `Expr` built from either" is **(A)**: each slot
door accepts `Length | WrittenLength`. Recommendation: **(A)**.

### (E), added 2026-09-09 after Ev asked whether Rust has the union and whether Python has an `Expr`

Measured: Rust has no union — the slot's type IS `Expr`
(`Node::Extrude { profile, distance: Expr }`), and an author builds
it first through `Expr::literal` (canonical unit) or
`Expr::written_length` (notation kept; `demos/tour/src/ring.rs:190`).
Python has `Expr`, and three doors already take one into a slot
(`DocEdit.set_param`, `Node.assertion`, `Doc.eval`), but it has no
constructors of its own: a Python `Expr` is minted only by
`Doc.parse_expr` or read back from a parameter, and the fifteen node
constructors build theirs internally from a `Length` through
`Expr::literal` — which is the erasure.

**(E): each dimensioned slot door accepts `Length | Expr`** — the
`Length` the canonical-fallback convenience it is today, the `Expr`
what the Rust slot takes — **and `Expr` gains Rust's constructors as
static methods**: `Expr.literal(value)`, `Expr.written_length(
WrittenLength)`, `Expr.written_angle(WrittenAngle)`. Then
`Node.extrude(profile, Expr.written_length(w))` reads back `25 mm`,
and `Node.extrude(profile, doc.parse_expr("h * 2"))` binds a slot
to an expression at authoring without the insert-then-`set_param`
two-step. Cost against (A): one more call at the site, and three
constructors that mirror Rust's public API anyway. Gain: the slot's
Python type reads as its Rust type, and one seat serves literals,
written literals and parsed expressions.

Recommendation: **(E)**; (A) if the shorter spelling is preferred.
Not both — two spellings of one thing.
