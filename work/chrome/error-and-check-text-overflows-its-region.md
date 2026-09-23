---
id: error-and-check-text-overflows-its-region
kind: issue
title: viewer: the CONCISION half — error messages should be shorter, and most of the text is the kernel's typed refusals (Ev's request; the layout half landed in 3058)
status: open
opened: 2026-09-17
pr: 3108
priority: P0
cost: E
refs: [3058, 3088]
---

**Ev reported this** (in chat, 2026-09-17). There are two halves.

1. **Layout.** Error messages and check results either run off the
   page or wrap strangely. They should wrap inside the region they are
   drawn in, and not return all the way to the window's left edge.
   Sibling of `the-toolbar-row-does-not-wrap.md`. It may be the same
   egui cause: text laid out against the full available width rather
   than the width of its own container.
2. **Concision.** Error messages should be edited to be shorter. Much
   of the text the viewer shows comes from the kernel's typed refusals
   (`Display` impls across `profile`, `editor-core` and others), not
   from `viewer`. So this half reaches past VIEW's territory, and
   whoever takes it should split off per-crate rows as needed rather
   than editing kernel prose from a viewer lane.

Not investigated when filed.

## A worked example (Ev, 2026-09-17)

The Boolean refusal Ev hit unioning two dumbbell halves is about 280
words. It covers the MAY-vs-DOES box semantics, which pairs are live,
the dispatch table's wiring, and why the refusal is structural.
Nearly all of that is for kernel developers, not the person holding
the mouse. It also opens by naming undeclared coincidence as "the
common case" before saying that this case is a torus×plane pair,
which points the reader at the wrong recourse. The full text is in
this session's chat; a fixture that fails the same way (a torus face
against a plane face in a union) reproduces it.

## The layout half, measured and answered (2026-09-22)

**Both symptoms are one cause with two faces**, and neither is a
per-label flag. `egui::Ui::wrap_mode` answers in three steps — the
`Ui`'s own `egui::Style::wrap_mode` if something set one, else `Extend`
inside a grid, else the layout's — and nothing in this chrome sets a
style wrap mode, so what decides a label's wrap today is the layout it
happens to be in. A sentence is not what either of the layout's answers
is for.

- *Runs off the page.* In a non-wrapping `ui.horizontal` the mode is
  `TextWrapMode::Extend`, which lays the galley out at infinite width.
  Measured: the 89-character fixture in a 220-point region paints one
  row 491.7 points wide, its right edge at x = 538.2 — 318 points past
  the region's own right-hand edge.
- *Returns to the window's left edge.* In `ui.horizontal_wrapped` the
  mode is `Wrap`, and `egui::Label::layout_in_ui` takes its branch that
  places the whole galley at `ui.max_rect().left()` and indents only
  the first row to the cursor. Measured in the REAL toolbar, at a
  400-point window: the first line starts at x = 279 and the second at
  x = 8, the window's own left edge — 271 points of drift. The toolbar
  is this crate's only wrapping row, and the status line — where every
  refusal goes (`frame::apply`) — is drawn in it.

The repair is `crates/viewer/src/widgets.rs`'s `message`,
`message_link` and `message_toned`: lay the sentence out into a galley
at `egui::Ui::available_width` and hand it over already laid out, which
is the one path `layout_in_ui` neither extends nor re-places. The wrap
is asked for explicitly, so a future context-wide `Style::wrap_mode`
would move every other label in the chrome and leave a message where it
is. `message_toned` routes the voice through `app::toned`, so what
`Advisory` looks like stays decided in one place rather than
hand-spelled at each call site.

Converted: the checks window's findings and its skipped-checks line and
the toolbar's status line (`app.rs`), the feature tree's failure line
and standing note (`pane/features.rs`), the profile editor's preview
verdicts (`pane/profile.rs`), the view pane's status line
(`pane/view.rs`). The roster is re-derived from the crate's own source
by `widgets::roster_tests::the_message_roster_is_what_the_crate_actually_calls`.

`widgets::message_tests` holds the measurements. The load-bearing one
is `a_message_fills_the_region_it_is_given_rather_than_a_fixed_width`:
the other rows are all satisfied by a sentence laid out NARROWER than
its region, so they cannot tell the fix from a hardcoded constant — this
one measures the widest line across three region widths and requires it
to grow. `app`'s
`the_toolbars_status_line_wraps_under_itself_rather_than_at_the_windows_edge`
measures the second symptom in the real toolbar, which is where it was
reported. Proven red both ways: with `wrapped_in_region` hardcoded to
150.0 (the fill row goes red at 40.4 points past a 150-point region;
the toolbar row goes red because an unconstrained window then also
wraps) and with `message` degraded to `ui.label` (all four widget rows
and the toolbar row go red).

Not converted, and scheduled by
`messages-in-the-creation-and-properties-panes-still-draw-past-their-row`:
fifty sites in `pane/create.rs` and `pane/properties.rs`, which were
live in other lanes' open PRs that wave. That row's census was re-taken
here — its first count of eleven was both internally inconsistent and a
large undercount.

**The concision half is untouched** and the worked example above
stands.

## Ev's ruling on the concision half (in chat, 2026-09-22)

Asked whether the viewer should summarise (a one-line summary with the
full text behind a toggle), the kernel prose should be rewritten, or
both, Ev answered:

> probably rewrite kernel prose. if it is IMPOSSIBLE to include all
> IMPORTANT information within a reasonable amout of space in the gui,
> then you could consider adding a summary, but i'd really like to
> avoid that added complexity

So the concision half is fixed **at the source**: the refusals the
viewer shows are rewritten in the crates that raise them, keeping what
the person holding the mouse needs and dropping what is written for
kernel developers. A viewer-side summary is the fallback for a refusal
whose important content genuinely cannot fit, and needs that case
shown, not asserted.

## The standard a refusal is rewritten to

This is the one statement of it; every
`*-refusal-prose-outgrows-the-viewer` row points here.

A refusal the viewer shows says, in the user's terms: what could not be
done, the short reason, and what they can do about it. **The recourse
is the part never to drop**, and where there is no way through, the
sentence says so plainly rather than labelling a dead end "Recourse".
Operands are "first"/"second"; arena keys, predicate routing, dispatch
tables, doc paths, issue numbers and stage prefixes (`boolean_reduce:`)
are developer detail and live in the variant's rustdoc or in the
payload `Debug` carries. Types, variants and payloads do not change;
prose only. A test that asserted the old text is re-baselined, never
weakened into something that cannot go red.

**The budget is 75 words, measured on the RENDERED text**: the
sentence exactly as the feature tree's fault line and the status line
draw it (`NodeError`'s `Display`, the "node N failed: the Boolean op
refused:" wrapper included), on a representative payload.
`editor-core/tests/refusal_concision.rs`
`every_rewritten_boolean_refusal_renders_within_the_budget` enforces it
for every `topo::BooleanError` arm the concision pass wrote and every
`topo::PointInSolidError` arm as it arrives through
`BooleanError::Containment`.

## The concision half, done (2026-09-22)

**The worked example, measured first.** A ring torus (R = 2, r = 0.5)
unioned with a block straddling its tube reproduces Ev's refusal:
`editor-core/tests/refusal_concision.rs` builds it through the public
document doors. The raising site is `topo::boolean::reduce`
`gate_operand_pairs` (the operand gate, `op: None`), the payload
`CurvedPairUnsupported { operand: A, kind: Torus, other_kind: Plane }`,
and the text the viewer drew was **277 words**, opening with
`editor-core`'s wrapper guessing at undeclared coincidence. It is now
**65 words**:

> node 5 failed: the Boolean op refused: the first operand's torus face
> may meet the second operand's plane face, and the Boolean cannot yet
> work out where such a face meets another solid. Recourse: reshape the
> parts so they meet only where a plane face meets a plane, cylinder or
> sphere face, or move them so the torus face stays clear of the other
> solid

**Rewritten at the source (prose only).**

- `editor_core::NodeErrorKind::Boolean`: the wrapper is "the Boolean op
  refused: {e}" and guesses no cause.
- `topo::BooleanError`, the whole `Display`: every arm lost its stage
  prefix, and every arm a person can act on was rewritten to the
  standard. That covers the worked example's family, the two "no way
  through yet" rim arms, the coincidence and escalation arms, the
  containment wrapper and the operand-gate arms. The kernel-bug arms
  (invariants, desyncs) keep their keys for the bug report.
- `topo::PointInSolidError`, every arm.

**Measured on rendered text.** Under the old prose, 21 of the 30 arms
the budget test renders were over 75 words (the worked example at 279,
`Containment(PartialTorusFace)` at 228). Under the new prose the
longest is `Containment(PartialConeFace)` at 74, and the worked
example's arm renders 68 on the test's NURBS payload.

**A measurement that changed a sentence.** `CurvedBooleanUnsupported`,
`ArcLoopContainmentUnsupported` and `FallbackExtentUnsupported` name no
operand: their `operand` field is the face's operand at some raise
sites and the edge's or the scanned body's at others (measured: a NURBS
wall on B reports `operand: A`). Filed as
`work/reach/boolean-refusal-operand-field-means-two-things.md`.

**Census and rows.** A static census over every `impl Display for`
counted literal words per arm (before: 50 arms at 60+; the PR body has
the table). Every arm outside `BooleanError`/`PointInSolidError` at 50+
literal words is filed on its owner's slate as
`<program>-refusal-prose-outgrows-the-viewer` (`paths`, `encl`,
`atrest`, `tess`, `chart`, `carve`, `exch`, `props`, `contact`, `ssi`,
`pcert`, `wire`, `tquery`, `offset`, `reach`), plus
`work/issues/unowned-refusal-prose-outgrows-the-viewer.md`.

No refusal needed a viewer-side summary.

## What remains open

The row stays open on one gap. The filed rows use a **literal**
threshold of 50 words, and the budget is 75 **rendered**. An arm under
50 literal words that forwards a nested refusal (`{e}`, `{source}`) can
render past 75 once the nested sentence is inside it, and the census
cannot see that: it counts literals only. Closing the row wants what
the budget test does for `BooleanError`, done for the other forwarding
chains the viewer shows: `NodeErrorKind`'s other kernel arms (`Extrude`,
`Revolve`, `Blend`, `Profile*`, `Split`, `Skin`, `Loft`, `Tube`,
`Transform`), `EditError`, and the checks window's findings. Each chain
is rendered on a representative payload and held to the budget, and
what is over is filed on its owner.

## The remaining chains (2026-09-23)

All three chains the row left open are now rendered the way the viewer
draws them and held to the 75-word budget, with each rewrite made at
the source (prose only; no type, variant or payload changed).

**What is rendered.** Every arm, on a representative payload, as the
viewer draws it:

- **The feature tree** —
  `editor-core/tests/refusal_concision_chains.rs`
  `every_node_refusal_renders_within_the_budget`: every
  `NodeErrorKind` arm, and every arm of each refusal a forwarding arm
  carries (`ExtrudeError`, `RevolveError`, `TubeError`, the three
  split stages, `BlendError`, `TransformError`, `SkinError`,
  `LoftError`, `ProfileError`, every `PathError` arm through
  `ProfileReplay` with each routed escalation, `StructureRefusal`,
  `EvalError`, the seed, box, placement, naming and name-ladder
  payloads, `ShellError`, `MateFault`, `PartFault`,
  `InterrogateError`, `ReadbackError`, `UnitVec3Error`) — 357 rows.
  The four arms whose `Found` only the entity door can mint are raised
  through a real document. The `Boolean` arm stays with
  `every_rewritten_boolean_refusal_renders_within_the_budget`.
- **The status line's edit refusals** —
  `viewer/tests/refusal_concision_edits.rs`
  `every_edit_refusal_renders_within_the_budget`: every `EditError`
  arm through the viewer's own `Refusal::Edit` wrapper, plus
  `MaintenanceRefused` and `MateRefused` over every `MateFault` arm and
  `ProfileProgramRefused` over the longest path refusals — 99 rows.
- **The checks window** — `every_check_finding_renders_within_the_budget`
  in the chain file: every `CheckEvidence` arm, with the separation
  arm over every containment refusal it can forward — 23 rows.

The rows are also held to the rest of the standard's shape (see the
fix pass below for how).

**Census, before and after** (the same rows run over main's text and
over this branch's):

| chain | rows | over 75, before | longest, before | over 75, after | longest, after |
|---|---|---|---|---|---|
| feature tree | 357 | 41 | 173 (`Blend/UnsupportedChain`) | 0 | 75 |
| edit refusals | 99 | 5 | 140 (`ProfileProgramRefused` over `SeamArrivalOffDirection`) | 0 | 75 |
| checks window | 23 | 8 | 99 (`ChartCoherence`) | 0 | 71 |

**The shared tail that moved most rows.** `geom_core::Indeterminate`'s
own `Display` ends most escalations in the tree; it lost "(D4)" and its
explanatory clause ("coincident at any precision you could care about,
too close to build sound geometry from" is now "a near-coincidence"),
which took about eight words off every escalation that forwards it.
`MissingRecourse` lost its explanatory tail the same way.

**Where the prose moved.** `editor-core` (`NodeErrorKind`,
`CheckFinding`, `InterrogateError`, `ProgramRefusal`'s wrapper in
`EditError`), `sweep` (extrude, revolve, tube, skin, loft, blend and its
long raise-site details), `profile` (`PathError`, the fillet recourses,
`StructureRefusal`), `topo` (the split stages, `TransformError`,
`ShellError`, `PcurveMintError`, the Boolean containment wrapper,
read-back, point-in-loop, void insertion), `geom-brep` (`NewellError`)
and `geom-core` (`Indeterminate`, `MissingRecourse`). The routed blend
escalations now render the payload view with their own recourse,
which is the shape `work/band/every-escalation-carries-the-coincidence-recourse-first.md`
asks for (noted there).

**No refusal needed a viewer-side summary.** The tightest case was
`BlendError::UnsupportedChain`: a raise-site detail plus the assembly
recourse, which names both terminations, the closed-rim clause and the
rings' clearance. It fits at 75 once the sixteen raise-site details
over 19 words are shortened where they are raised, which this pass
did.

**Rows closed and filed.** Five of the literal-census rows are closed
by this pass: `carve-`, `offset-`, `paths-`, `reach-` and
`wire-refusal-prose-outgrows-the-viewer`. The unowned row keeps the
certify arm and gains the one residue this pass left:
`topo::ShellClassifyError` and `topo::MassPropsError` still name a
shell or face by arena key, in `topo/src/props.rs`, which two open PRs
were reworking; the chain test lists the two rows it reaches them
through in `KERNEL_KEYED`, pointing there.

**What the rows do not see.** A refusal forwarded two levels below
`NodeErrorKind` (an `EulerOpError` inside `ExtrudeError::Op`, a
`CertifyError`, an `OffsetFitError`) is rendered on one representative
arm; those enums' long arms are held by the literal-census rows still
open on their owners (`unowned-`, `encl-`, `atrest-`, `chart-` and the
others this row's first pass filed).

**The one case a rewrite at one site cannot hold.** A part's
product-root failure forwards the part's own node refusal inside an
eleven-word wrapper, so its line is the inner refusal plus eleven
words: up to 83 on today's longest. That is the case Ev's ruling keeps
the summary fallback for, and it is filed with its measurement on the
edit program's slate as
`work/edit/part-root-failure-nests-a-whole-refusal-past-the-budget.md`.

## The fix pass (PR #3108 review, 2026-09-23)

**The shape is checked structurally.** `test_utils::refusal`
(`crates/test-utils/src/refusal.rs`) is the one statement of what a
refusal on screen must look like, checked on the rendered sentence:
the 75-word budget; no stage prefix, read by its SHAPE (a clause of one
or two lowercase words ending in a colon) rather than from a list of
the prefixes a rewrite removed; no `Debug` struct (`Ident { field:`);
no arena key outside the kernel-bug rows named by exact id
(`KERNEL_KEYED`); and one recourse marker (`Recourse:` or "There is no
way through"). All three chain tests call it. The checks window's rows
are namespaced `Check/…` so an id cannot collide with a node row's.

**What it found that the list could not see**, on rows already
rendered: `invalid band:` (every `BandError`, 17 rows), `internal:`
(three `NodeErrorKind` arms), `guided validation:` /
`guided elaboration:`, `fit:` and `knot algebra:` (under `Skin`),
`parameter width:` (three `EditError` arms), a `Debug` struct in
`EditError::PathOffTree`, and `section:` (under the split join). Each
is rewritten at its source. It also found, in files open PRs are
reworking, `replace_face_offset:` (under `Shell/Face`, `Shell/Lift`),
`shell classification:` and `mass properties:` (`topo/src/props.rs`),
`certification:` (`geom-brep/src/certify.rs`) and the clearance
engine's `Debug` payload (`editor-core/src/measure.rs`). Those are
filed, each admitted by exact row id and exact label in the chain
test's `FILED` / `FILED_DEBUG` lists with the row that owns it:
`work/shell/replace-face-refusals-open-with-a-stage-prefix-and-name-keys.md`,
`work/props/props-refusal-prose-outgrows-the-viewer.md` and
`work/issues/unowned-refusal-prose-outgrows-the-viewer.md`.

**Every blend raise-site detail is rendered.**
`every_blend_detail_renders_within_the_budget` reads the detail of
every `unbuilt_chain`, `unbuilt_run_out`, `unbuilt_geometry` and
`not_intact` call in `sweep/src/blend` from source (a literal, or a
`const` resolved in the same tree) and renders each through the
feature tree's chain under both verbs: 181 sites, 362 rows, longest
74. It went red on one detail the representative row did not reach
(76 words), shortened at its site.

**A pair's corner list states one recourse.**
`PathError::NoCornerOfPair` renders every refusing corner; each corner
used to carry its own "Recourse:", so two swallowed carriers rendered
two recourses and 81 words. The corners now state facts and the pair
states one recourse after them (`CornerReason::recourse`); the worst
two-corner case renders 73.

**Census after the fix pass:** feature tree 360 rows, longest 74;
edit refusals 99, longest 72; checks window 23, longest 71; blend
details 362, longest 74.

## What remains open

The row stays open on one residue without an owner: the stage
prefixes and arena keys in `topo/src/props.rs` (`ShellClassifyError`,
`MassPropsError`) and the stage prefix in `geom-brep/src/certify.rs`
(`CertifyError`), which `work.py territory` assigns to no program and
which open PRs #2861 and #3049 are reworking. They are filed on the
unowned row. The row closes when that residue lands or a program
takes it; the chain test's `FILED` and `KERNEL_KEYED` entries naming
`Shell/Roles`, `Check/Unsupported` and `Transform/Certify` are what
say it has.
