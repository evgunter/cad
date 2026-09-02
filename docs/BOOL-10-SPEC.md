# BOOL-10 — the arc_continue retirement and the declared-subdivision arc form

**Binding at dispatch** (S-BOOL program, `docs/S-BOOL-PLAN.md`;
difficulty logged pre-draw: **L**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
The primary specification is the Q1 ruling's second-round extension
(Evan, in-chat, 2026-09-01; `docs/S-BOOL-PLAN.md` §Rulings), quoted
here because it bounds the unit:

> `arc_continue` is REMOVED rather than kept as the axiom's
> exception: it consults incoming-carrier data and branches on what
> the previous leg was, both of which §2c calls unrepresentable. Its
> recorded need (authored arc subdivision — the half-disc equator)
> re-spells as declared subdivision on the arc leg itself, the
> open-carrier analog of `circle_split`, with vertices minted at the
> chain's emission layer where the axiom's bookkeeping legitimately
> lives. The enforcement Evan believed already existed becomes real:
> the sealed verb module's signatures narrow to the bare state values
> only, so a verb needing chain state is unwritable.

PATHS §2c (the directed-point axiom, the sealed verbs kernel — read
`crates/profile/src/path/verbs.rs`'s header in full), the §3 verb
table's `arc_continue` row and §4's arc_continue paragraphs (~:956,
~:987) are the text this unit re-records. `circle_split` (`path.rs`'s
`circle_split_kernel`, `program.rs`'s doc "the declared-subdivision
closed carrier") is the precedent. **Precondition: BOOL-12 has
merged** (this unit and BOOL-12 both rewrite `path.rs`,
`path/program.rs`, the wire vocabulary and PATHS §2c/§3/§4; sequencing
avoids a second merge of the same files). BOOL-9's raw-door survey
runs beside this unit; the two do not share files beyond `lift.rs`,
which BOOL-9 owns — report, do not edit, any `lift.rs` need.

## Situation

`arc_continue(p)` continues the incoming ARC carrier to `p`, minting a
structural subdivision vertex, and runs no junction check because it
is a same-carrier identity. To do that it reads what the previous leg
was — the one thing §2c says a verb cannot see — and it is the axiom's
recorded exception. The ruling removes it and puts the need where the
axiom's bookkeeping lives: a declared subdivision ON the arc leg
(`.arc_to(p).split(n)`-shaped, or a subdivision argument on the arc
data — the spelling is this unit's design surface), with the vertices
minted at emission. And it makes the seal real: the sealed verb
module's signatures narrow to bare state values, so a verb needing
chain state cannot be written at all.

`arc_continue` reaches: `path.rs` (the verb, its kernel, two refusals
`ArcContinueNeedsArcCarrier` / `ArcContinueOffCarrier`),
`path/program.rs` (the table row, `Step`/`Verb`), `editor-core`'s
`program.rs` (`ProgramStep::ArcContinue`), `eval/mod.rs` (the arm, the
content-key tag), `persist/wire.rs` (`WireStep`), `lift.rs` (four
sites), `pncad-py` (tags, `py/path.rs`, the surface census, the
`.pyi`), and the **viewer** (`app.rs`'s tool palette and `sketch.rs`'s
`PathStep::ArcContinue`) — the GUI is a thin client over the API, so
its arm goes with the verb.

## FIRST, before the build — two things reported

1. **The need, measured.** Enumerate every in-tree use of
   `arc_continue` that authors geometry (the half-disc equator vertex
   revolve naming's pole elimination anchors on; any tour scene; any
   fixture that is not a refusal row), and for each say what the
   declared-subdivision form must produce for it to author the SAME
   vertex table bit-for-bit. If a use needs something a subdivision
   count and phase cannot express (a vertex at an authored non-uniform
   parameter, say), report it — that shapes the design.
2. **The seal, measured.** Which signatures in `verbs.rs` still admit
   chain state (a `Core`, a previous leg, a carrier) once
   `arc_continue` is gone, and what narrowing makes a chain-consuming
   verb a compile error rather than a convention. Report the list.

## Deliverables

1. **The declared-subdivision arc form — design surface, argued for
   Evan (PR HELD).** The open-carrier analog of `circle_split`: an arc
   leg that declares its own subdivision (count, and the phase or the
   authored parameters the measurement in (1) requires), minting the
   vertices at the chain's emission layer. Spelling is yours to
   argue, bounded by §2c (no verb reads chain state; admissibility is
   a trait matrix; one `transition_table!` row per verb — prefer an
   argument on the existing arc data over a new verb), by
   never-infer (the subdivision is DECLARED; no vertex is placed by
   reading the carrier for anything but the arc's own parametrisation),
   and by D2 (a count below 2 refuses typed as `circle_split` does).
   Write the §2c/§3/§4 text for Evan's eyes.
2. **`arc_continue` removed**: the verb, its kernel, its table row,
   `Step`/`Verb`/`ProgramStep`/`WireStep` arms, the eval arm and its
   content-key tag (retired numbers stay dead — D365's append-only
   rule; do not renumber), the two refusals and their `PathErrorKind`
   arms, `pncad-py`'s tags / bindings / census / `.pyi` rows, the
   viewer's tool and `PathStep` arm. Every exhaustive match found by
   compiling — the red-first record is the receipt. Wire documents
   carrying the step: post-BOOL-13 there is no version; regenerate the
   checked-in corpus through the release tour build and state whether
   any document carried the step (if one did, it re-authors through
   the new form and the vertex table is compared).
3. **The half-disc equator (and every use from (1)) re-authored
   through the new form, bit-identical** — revolve naming's pole
   elimination anchors on that vertex, so the render and the naming
   census are the gate; the tour byte-stable (BOOL-12's `diff -rq`
   instrument).
4. **The seal made real**: `verbs.rs`'s signatures narrowed to the
   bare state values per the measurement in (2); a row that proves
   a chain-consuming verb is a compile error (a `compile_fail`
   doc-test or a trybuild-class row — the precedent in tree, if one
   exists, is the shape; otherwise say how you prove it); the module
   header's "signature purity" paragraph re-recorded as enforced.
5. **PATHS re-recorded**: the §3 verb-table row retired and the new
   form's row added; §2c's exception paragraph becomes history; §4's
   `arc_continue` paragraphs (~:956, ~:987) re-record; the axiom's
   parenthetical about emission-layer bookkeeping cites the new form.
6. **Rows**: the new form both directions (declared subdivision
   authors N vertices ON the carrier at the declared parameters;
   count < 2 refuses typed; a subdivision on a straight leg is
   inadmissible at compile time or refuses typed — decide with the
   reason); the half-disc equator; `coverage_corpus` chains replaying
   the new arm at `Dual64` and `Interval`; the lift layer's arm for the
   new step (report, do not edit `lift.rs` — BOOL-9's; if the new form
   cannot lift without a `lift.rs` change, STOP and report).
7. **ε posture** (issue 1356): the subdivision places vertices by
   parameter — no new comparand unless the form admits authored
   parameters compared against the arc's span (then a named key,
   banded, stated per band). Three-ε battery; the trailer decision.
8. **Class sweep** (discipline §5): every other verb that reads chain
   state after the narrowing (should be none — the seal proves it);
   every other site that names `arc_continue` in prose (docs, memories,
   the guide page — the guide is LIB's: report); the `ArcData` matrix
   docs.

## Acceptance

- `arc_continue` gone everywhere with the red-first receipt; the new
  form authoring the equator bit-identically and the tour byte-stable;
  the seal proven by a compile failure; PATHS text for Evan; hosted CI
  green; gate record per head.

## Hard rules

- NO `Co-Authored-By`, no model names. No closing keywords; no issue
  closes here (the ruling has no issue of its own).
- **The PR does not merge on green** — the new form's spelling and the
  §2c/§3/§4 text are design surface; the orchestrator holds the merge
  for Evan's sign-off.
- Scope fence: `crates/profile/src/path.rs`, `path/program.rs`,
  `path/verbs.rs`, `lib.rs` exports, the profile suites;
  `crates/editor-core`'s `program.rs` / `eval/mod.rs` / `persist/wire.rs`
  arms (Track V C6 anchors — disclose exactly which arms move; `node.rs`
  only if `StepArg` carries a subdivision argument, disclosed);
  `crates/pncad-py` and `crates/pncad` rosters/tags/`.pyi` (minimal
  arms); `crates/viewer`'s `app.rs` / `sketch.rs` arms for the verb
  (removal + the new form's arm if the palette carries it — the GUI is
  a thin client; disclose as GUI ground touched); the regenerated
  corpus; `docs/PATHS-DESIGN.md`. NOT: `lift.rs` (BOOL-9's — report),
  `RawLoop` (BOOL-9), `validate` semantics, the seam family
  (BOOL-12's, landed), `circle_split` itself beyond citing it.
- Re-merge main before opening the PR.
