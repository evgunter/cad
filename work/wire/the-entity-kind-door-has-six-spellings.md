---
id: the-entity-kind-door-has-six-spellings
kind: issue
title: Read a name, test its EntityKey kind, refuse: three copies in eval/wire.rs and six spellings of the refusal across the crate, with no shared door
status: closed
pr: 2517
opened: 2026-09-12
closed: 2026-09-14
---



## Finding

Found by the full review of PR 2480 (MINOR 1), which read the operand
door's sweep and found its blind spot: that sweep grepped for
`NodeErrorKind::WrongOperand`, so a kind refusal raised under a
DIFFERENT error kind was invisible to it. PR 2480's hit list disposed of
`Selected::faces` as *"one enum, two arms, no duplication yet"*, which is
**false of the class as it stands** and is retracted here.

## The three copies, in one file

`crates/editor-core/src/eval/wire.rs` carries the same five lines three
times — one `ladder::resolve_in`, one `let EntityKey::X(k) = ent.key
else`, one refusal carrying `name` and `found: ent.key.kind()` —
differing only in the wanted kind and the error arm:

- `resolve_open_faces` → `NodeErrorKind::ShellOpenKind { name, found }`
  (wants `EntityKey::Face`)
- `resolve_selection` → `NodeErrorKind::BlendSelectionKind { verb, name,
  found }` (wants `EntityKey::Edge`)
- `wire_datum`'s `Datum::FaceFrame` arm, inline →
  `NodeErrorKind::FaceFrameKind { name, found }` (wants
  `EntityKey::Face`)

All three already share their RESOLUTION door (`ladder::resolve_in`) and
its N5 refusal trio. What none of them shares is the kind test and the
refusal after it — which is exactly the shape `eval::wire`'s `operand` /
`node_operand` now has for VALUE kinds.

## Six spellings of one question, crate-wide

The class is "read a thing, test its kind, refuse", and the crate says
it six ways. The field names are the census:

| spelling | where |
| --- | --- |
| `expected` / `found` | `NodeErrorKind::WrongOperand` — the value door, given one home by PR 2480 |
| `name` / `found` | `ShellOpenKind`, `BlendSelectionKind`, `FaceFrameKind` |
| `verb` / `found` | `MeasureSelectionKind` (`Selected::faces`, in the same file) |
| `wanted` / `found` | `crates/editor-core/src/names/interrogate.rs` `kind_mismatch` |
| `expected` / `found` | `crates/editor-core/src/stackup.rs` |
| the recipe road | `crates/editor-core/src/mate/member.rs` — filed separately as `work/docm/the-third-datum-axis-phrase-lives-in-mate-member.md` |

## What a taker owes

A decision about the ENTITY-kind door, which is a different vocabulary
from the value-kind one: entity kinds are `EntityKey`'s variants, not
`ValuePayload` families, and the refusal carries the authored NAME
rather than an input id. The three `wire.rs` copies are the instance
worth fixing first; whether the other three spellings collapse onto it,
or stay distinct because their subjects genuinely differ, is the
question this row asks and does not answer.

Read beside `eval::mod`'s `operand_vocabulary_census`, whose own doc
records this class as what it cannot see.

## Closed 2026-09-13 (PR 2517)

**One home, and the word it answers with is unspellable by a road.**

- `crate::eval::entity_door` (in `eval/mod.rs`) holds
  `Found(EntityKind)` — **field private to that module** — and
  `entity(key, read, refuse)`, the only thing that can mint one. `read`
  is the projection (`EntityKey::face`, `EntityKey::edge`, or a wider
  one) and is a **`fn` pointer**, so it cannot capture a second key:
  the value the door returns and the kind it reports come off the same
  key. `refuse` is the road's own refusal constructor, handed the
  token.
- `named_entity(name, doc, table, unresolved, read, refuse)` stays in
  `eval/wire.rs` — the designation road: the `ladder::resolve_in`
  walk, then `entity`, plus the `Box::new(name.clone())` the three
  name-carrying refusals need, which was written three times.

The door is in **two files**, and that is forced rather than chosen:
`Found`'s field must be private to a module that is not an ancestor of
the roads, and the roads are in `eval/wire.rs`. Putting `Found` beside
`EntityKind` in `names/` would need a crate-visible constructor every
road could call, which is no guarantee at all. Both sites say so.

There are no `ENTITY-DOOR` sentinels: the earlier shape had them, for a
census that policed the rule textually, and both are gone.

`EntityKey::face`/`edge` are new projections beside `EntityKey::kind`
in `names/table.rs`, so no call site hand-writes a `match` over
`EntityKey` to say "a face".

**This shape was reached by being wrong twice, and the reasoning was
corrected a third time.** The first version asked roads not to write
`found:` and guarded it with a census that passed partially vacuously.
The second tightened the census to require the refusal be a closure
body binding `found`; a review defeated that three ways in one sitting,
each under a green suite. The third — this one — made the word
unforgeable, and a further review showed the KEY the word is read off
is still the caller's. That last gap is real, is narrowed but not
closed (the `fn` pointer above), and has its own row:
`work/wire/the-entity-doors-key-comes-from-its-caller.md`. The lesson
the program paid for: **stop policing the spelling, make the wrong
thing unspellable — and then check which level the spelling moved
to.**

### The three error kinds kept their identities

No half-fix. `ShellOpenKind { name, found }`,
`BlendSelectionKind { verb, name, found }` and
`FaceFrameKind { name, found }` keep their variants and their `Display`
arms; each road hands the door its own constructor, so the verb still
reaches a chamfer's refusal and the three sentences are still three
sentences. `crates/editor-core/tests/wire_entity_door.rs` pins all of
them byte-exact, and its blend row runs fillet AND chamfer so a
flattened verb reds.

What DID change is the field's type: `found` is
`entity_door::Found` rather than `EntityKind`. `NodeErrorKind` derives
only `Debug`, so nothing persisted or compared moved, and **no rendered
string changed** — `Display` already asked the value for its article
and noun. Four assertions now read through `Found::kind()` instead of
matching the enum, and one hand-minted refusal in `pncad-py`'s tag test
was deleted: it can no longer be minted, which is the point, and the
same tag is already proved through a real document.

### The six spellings, disposed

| spelling | disposition |
| --- | --- |
| `expected`/`found` — `WrongOperand` | PR 2480's; untouched |
| `name`/`found` — `ShellOpenKind`, `BlendSelectionKind`, `FaceFrameKind` | **converted**; the three copies are gone |
| `verb`/`found` — `MeasureSelectionKind` (`Selected::faces`) | **converted**, through `entity` directly (it holds the key and carries no name). Its `found` moved from `&'static str` to `EntityKind`, which deleted the two hand-written words `"an edge"`/`"a vertex"` — the article is the value's to decide. **The rendered message is byte-identical**, proved by compiling the suite's document rows unchanged on `origin/main` |
| `wanted`/`found` — `names::interrogate`'s `kind_mismatch` | **not converted, and not a defect of this class**: it is already one home for four call sites and already computes `found.kind()` itself. What differs is the error TYPE (`InterrogateError`, a read-back refusal) and one extra fact it must keep — a whole body has no frame at all, which is `WholeBody` rather than a kind mismatch. Folding it into a door that builds `NodeErrorKind` would put an evaluation refusal and a read-back refusal in one function. The residue is the word `wanted`, recorded below |
| `expected`/`found` — `stackup.rs` | **not this class at all.** `PairingViolation::ResultArm`'s two words are RESULT ARMS (`Ok`/`Failed`/`Poisoned`) of two evaluations being paired, not entity kinds, and both are already produced by one `arm()` function. Same field names, different subject. PROPS's ground; read, not edited |
| the recipe road — `mate/member.rs` | filed separately as `work/door/the-third-datum-axis-phrase-lives-in-mate-member.md`; not read for this unit beyond confirming the row exists |

### Three more sites the row's census did not list

Found by triaging the sweep to its end (the review of PR 2517). The
pattern was right — `rg 'EntityKey::' crates/editor-core/src/` returns
all three — and the first pass stopped early.

- **`eval/mod.rs`'s `DeclareUnsupportedPair`**, built in
  `resolve_declarations` in `eval/wire.rs` itself (this row said
  `route_declarations`, one function above; corrected), from
  `(n1.kind, n2.kind)`. **Not converted**: it tests a PAIR, and
  `entity`'s single-key shape does not fit it. **Settled since**, by
  PR 2681: the refusal is now raised BEFORE either name is resolved,
  because a pair the vocabulary has no step for is unsupported however
  many entities answer to either name — so no key exists at the moment
  the word is needed and the authored kind is the decided answer, not
  a residue. The row that carried it,
  `work/wire/the-declared-pair-refusal-reads-the-authored-kind.md`,
  closes with that PR. What it leaves for THIS row is a new instance
  of its own subject: `names::interrogate`'s `kind_mismatch` now takes
  an `EntityKind` a caller can write, which
  `work/wire/the-designation-road-resolves-before-it-asks-the-kind.md`
  explains and proposes to unify.
- **`clearance.rs`'s `SelectionRefusal::NotAFace { name }`** — same
  road as the converted `Selected::faces`, names no found kind, and
  conflates a wrong kind with a wrong body. SHELL's ground; filed as
  `work/shell/named-face-scope-refuses-not-a-face-without-naming-what-it-found.md`.
- **`assembly.rs`'s `RefusedRef::NotAFace { kind: other.kind() }`** —
  already computes its own word and renders through
  `EntityKind::article`/`noun`, so no correctness defect; the residue is
  a third field name for one answer. DOCM's ground; filed as
  `work/edit/assembly-mint-spells-the-entity-kind-refusal-a-seventh-way.md`.

### This row closes on a PARTIAL fix, and says so

**Four sites of at least nine.** The four are the `NodeErrorKind`
carriers; the other five sit on four other error types
(`InterrogateError`, `MintRefusal`, `SelectionRefusal`, and the pair
refusal's own variant), each now on a slate rather than only in a PR
body. And the four that ARE closed are closed against a road writing
the word, not against a road handing the door the wrong subject —
`work/wire/the-entity-doors-key-comes-from-its-caller.md`.

**The residue**: three words for one answer (`found`, `wanted`, `kind`)
across those error types, and one site that answers off the authored
name rather than the resolved key. The census cannot see past
`NodeErrorKind` — and the general form of that limit is the sharpest
thing this unit learned: **a census that finds its sites by the
spelling it is normalising can only ever find the ones that already
comply.** It is in the suite's own doc, with the known members named.


## Closed 2026-09-14 (PR 2517) — a partial fix, labelled, after three review rounds

**Four sites of at least nine.** The three identical copies plus
`MeasureSelectionKind` go through one door; the other five are on four
programs' slates, each with its own row rather than a sentence in a PR
body.

### The door, after the guard it started with was defeated three ways

It began as a textual guard over the roads' shape. R2 defeated that three
ways — an inner closure, a one-line IIFE, and a `let`-bound closure
inside the argument list, the last being the shape the guard's own doc
declared impossible — all with the suite green, because the road's answer
happened to be *correct*.

The answer was to stop policing the spelling: `entity_door::Found` is a
newtype over `EntityKind` whose field is private to the door's module, so
a road passes a kind through and **cannot write one**. All four defeats
then failed `cargo build`, and R3 confirmed the out-of-crate case is
total — **eight attack shapes, eight compile errors**, including a
struct-update `E0451` checked in isolation so other errors could not mask
it.

### And R3 found the last mile, which is the durable lesson here

The token makes the **word** unforgeable; the **key the word is read off**
was still the caller's. A road could hand the door a synthetic
`EntityKey` of any kind — variants are `pub`, payloads are slotmap keys
with `Default` — and the door would mint the lie for it. Compiled, with
a byte-identical success path, and **the census passed on it.**

Closed as far as this unit reaches, in two moves rather than one
overstatement:

- **the claim is narrowed at the site**, in the door's own words, and
  measured rather than hedged: `EntityKey` is constructed in about **150
  places** crate-wide and `EntityRef` in 30, nearly all legitimate
  naming-layer mints, so full closure is a naming-layer redesign and not
  this unit. The narrower `Resolved`-token option fails today on module
  privacy — `ladder` lives inside `eval::wire` and `entity` does not —
  and the row records that so a taker does not re-derive it.
- **`read` became `fn(EntityKey) -> Option<R>`**, not a closure. A `fn`
  cannot capture, so the value the door returns and the kind it reports
  come off **the same key**: a road that substitutes a key substitutes it
  for its own success path too and **stops working**, instead of
  succeeding on one entity while refusing about another. That does not
  make the key unforgeable; it makes a forgery **self-defeating rather
  than invisible**, which is what the compiled attack depended on.

What remains is `work/wire/the-entity-doors-key-comes-from-its-caller.md`,
carrying the compiled attack, what the `fn` change closes, the realistic
residue (a road passing the wrong key **by mistake**), what covers it
today (the byte-exact rows), and the measured reason it was not closed
structurally.

### What the token did and did not buy, kept in the narrowing

`Found` is **not** a new spelling in this row's sense. A spelling is a
second thing that can *answer differently*; `Found` computes nothing and
forwards `kind`/`article`/`noun` unchanged. It made three of the six
spellings' failure modes **unspellable rather than merely unwritten**,
and out-of-crate the result is total. The narrowing takes back the last
mile, not the move.
