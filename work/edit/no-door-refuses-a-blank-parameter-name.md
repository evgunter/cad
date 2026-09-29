---
id: no-door-refuses-a-blank-parameter-name
kind: issue
title: No door refuses a blank parameter name — ParamName::new validates nothing and write_doc_param does not ask
status: spec
branch: edit/param-name-door
opened: 2026-09-19
priority: P1
cost: E
---

Filed by a VNEWS census lane (`work/vnews/a-disabled-control-says-why-
in-four-shapes`) at merge base
`2654cc111417da806d9786c40136106469096fec`. An announced crossing:
`crates/editor-core` is EDIT's and MSOLVE's. **Nothing under `crates/`
was touched.**

## The hole

`crates/editor-core/src/doc.rs`'s `ParamName` is
`pub struct ParamName(pub String)` and its `ParamName::new` is a
`impl Into<String>` passthrough — no predicate, and the field is `pub`,
so the constructor is not even the only route.

`crates/editor-core/src/edit.rs`'s `write_doc_param` — which the header
names as the one place every declaration door routes through, *"the
check order is the LOAD door's"* — runs four checks and none is about
the name: the non-finite walk (`DocParam::first_non_finite`), the
distribution's `check`, the structural/continuous divide
(`is_continuous_count`), and the unit/dimension pairing (`measures()`).
Then `new.params.insert(name.clone(), value)`.

`crates/viewer/src/session.rs`'s `Session::create_param` — the door
above it — refuses exactly one thing, `Refusal::ParamExists`.

So `DocEdit::SetDocParam { name: ParamName::new(""), … }` **succeeds**
and the document holds a parameter declared under the empty name.

## What that costs

- An expression referring to it cannot be written.
  `crates/editor-core/src/parse.rs`'s lexer opens an identifier only on
  `c.is_alphabetic() || c == '_'` and then takes more, so `Tok::Ident`
  is non-empty by construction and the declaration is unreachable from
  the one surface parameters exist for.
- `Refusal::NoSuchParam`, `EditError::ContinuousParamCannotBeCount` and
  every other refusal that frames a name render it through
  `ParamName`'s bare-name `Display` (the *"one home for the spelling
  every refusal that FRAMES a parameter name in a sentence of its own
  uses"*), so each of those sentences gets a hole where the name goes:
  *"no document parameter named  — declare it first"*.
- The save/load round trip carries it: nothing in the persistence
  validator asks either, and the header's own claim is that the edit
  door and the load door *"refuse the same declarations"*.

## How it was found, and what it is not

The VNEWS census classified `crates/viewer/src/pane/properties.rs`'s
add-parameter Create button. Its gate is `ready = !name.is_empty() &&
dimension.is_some()`, and the name conjunct is **silent** — no
`on_disabled_hover_text`, no sentence beside it. By the census's rule
that is correct *given the doors as they are*: there is no refusal for
a blank name, so there is no sentence for the control to be a second
copy of.

That is the wrong reason for a correct outcome. The chrome's
`!name.is_empty()` is the tree's **only** enforcement of a rule every
door is silently relying on, and it lives in a GUI panel. The repair is
a door-side refusal, at which point the chrome's silent conjunct
becomes an ordinary census hit and gets the door's own sentence.

## What a fix has to decide

Whether "blank" is the whole rule. A name is a parse-time identifier
elsewhere (`ParseError::UnknownParam` echoes the bytes an author
typed), so a door that refuses `""` and admits `"1 2"` has moved the
inconsistency rather than closed it. Siblings on this slate that share
the question: `quoted-parameter-name-in-error-prose-has-no-decision`,
`param-ref-refusals-spell-two-facts-four-ways`.

## Home

EDIT's: `crates/editor-core/src/doc.rs` and
`crates/editor-core/src/edit.rs`. `crates/viewer/src/session.rs` and
`crates/viewer/src/pane/properties.rs` are VSEAM's and VNEWS's, and the
chrome side of this is a hand-off that lands after the door exists.

## Ruled and spec'd (2026-09-24, EDIT orchestrator) — middle tier, one opus style review, branch `edit/param-name-door`

**What a parameter name is.** A document parameter exists to be
referenced from an expression, so a name is admissible exactly when
the expression parser reads it back as a reference to that same
parameter: parsing the name alone yields `Param(name)` and nothing
else. That answers the row's question without a second grammar. Blank
fails it, `"1 2"` fails it, and a name the lexer would take as a
function or constant word fails it in whatever way the parser already
decides. The rule is the parser's, stated once, and the door asks it.

1. **One predicate, beside `ParamName`.** The admissibility check is
   one function in `doc.rs` (or `parse.rs`, whichever owns the lexer's
   identifier rule without a cycle), returning a typed fault that
   names what failed. Prefer making it structural: `ParamName`'s field
   private and its constructor fallible, if the call-site ripple is
   mechanical (test literals through one helper). If the ripple is not
   mechanical, the door-level refusal below is the floor and the PR
   says what stopped the type.
2. **Both doors refuse the same declarations.** `write_doc_param`
   refuses an inadmissible name with a typed `EditError` arm whose
   sentence quotes the name (`quoted-parameter-name-in-error-prose-has-no-decision`
   settled how a refusal frames a name). The load door refuses the
   same name typed, so the header's "the edit door and the load door
   refuse the same declarations" stays true. Measure the corpus and
   the test tree for any stored name the rule would now refuse; any
   hit is reported, not silently re-authored.
3. **Rows.** Blank, whitespace, a leading digit, an embedded operator,
   and a function or constant word each refused at the edit door and at
   the load door with the same fault; a legal name accepted; the
   document `bit_eq` after each refusal; the F6 census gains the arm.
4. **The chrome side is a hand-off.** `properties.rs`'s silent
   `!name.is_empty()` conjunct and `Session::create_param` are not this
   unit's. File one row on the owning program's slate (whoever claims
   `crates/viewer/src/pane/properties.rs` today) naming the door's new
   sentence as the one the control should show.

Pre-draw not applicable (middle tier, outside the protocol).

## Three more consumers relying on the rule (PORT, `port/wrap-a`)

Added as evidence, not as a second row. Each renders a name into text
whose structure assumes the name carries none of its marks, and an
identifier-shaped name is what makes each assumption true:

- `crates/editor-core/src/stackup.rs`, `render_rss`: a refused RSS
  column now lists its blockers one per line, so a name carrying a line
  break would split one blocker in two (the same assumption every
  `∂m/∂name` row of that report already makes).
- `crates/editor-core/src/stackup.rs`, `Stackup::serialize`'s `rss`
  line: the blocking names joined on a bare `,`, the content key's input.
- `crates/editor-core/src/report.rs`, `MassBasis::Forced`'s `Display`:
  the forcing names joined on `", "`.

## More evidence, and a live asymmetry (2026-09-20, from #2960)

VNEWS's `vnews/app-controls-read-their-refusals` gave the viewer's
OTHER Create button the rule this one still lacks, so the two are now
visibly out of step in one crate:

- **The New-document Create button** (`crates/viewer/src/app.rs`,
  `toolbar_ui`) gates on `Refusal::new_document_name`, which trims and
  refuses a blank with `Refusal::EmptyName` — the same door
  `DocSession::new_document` refuses at — and shows that refusal's
  words while it is disabled.
- **The add-parameter Create button** (`crates/viewer/src/pane/properties.rs`,
  the `ready` gate) still reads
  `!name.is_empty() && self.drafts.new_param_dimension.is_some()`.
  **Untrimmed**: a name of spaces passes the gate, and `create_param`
  refuses only `ParamExists`, so the click reaches a door that refuses
  nothing and a parameter named `"   "` is what comes out.

The trim is the cheap half and could be fixed in the panel today, but
doing it there alone would mint a chrome-side rule with no door behind
it — which is the defect this row exists to prevent. **The order is:
this row first** (a door that refuses a blank parameter name, with its
own sentence), and the panel's gate reads it afterwards the way the
toolbar's now reads its own.

One naming consequence worth knowing when this lands. The viewer's door
is deliberately named `Refusal::new_document_name` rather than
`empty_name`, so it does not read as the general blank-name rule and
get reached for from the parameter side; its sentence is
document-specific (*"a new document needs a name; its identity is
derived from it"*) and would be wrong here.
