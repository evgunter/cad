---
id: the-hide-toggle-is-drawn-over-a-refusal-the-op-will-give
kind: issue
title: the instance panel draws the hide toggle for a fused instance, and SetInstanceHidden refuses it on click
status: closed
opened: 2026-09-20
refs: [a-disabled-control-says-why-in-four-shapes, the-range-button-re-mints-the-ratified-affordance, the-unit-picker-is-offered-on-a-slot-whose-notation-is-not-the-users, the-mirror-class-is-unswept-outside-the-properties-pane, no-test-can-reach-a-pane-function, a-panels-gate-reads-the-previewed-document-while-its-door-reads-the-committed-one, probe-scale-refuses-a-no-readable-value-case-its-only-caller-pre-empts]
priority: P1
cost: E
branch: vnews/properties-controls-read-their-refusals
closed: 2026-09-20
---


refs the disabled-control census (`a-disabled-control-says-why-in-four-shapes`),
whose **mirror image** this is — that census parks the class explicitly:
*"A control drawn as usable that refuses on click. The rule's mirror
image."* Found by the `is-instance-collapses-absent-and-wrong-kind`
lane while reading `instance_ui` for a different reason.

## The gap between the gate and the door

`PropertiesPane::instance_ui` (`crates/viewer/src/pane/properties.rs`)
gates the whole per-instance section on `display::instance_check`,
which tests the node KIND only. Inside it, the hide toggle is drawn
unconditionally and pushes `SessionOp::SetInstanceHidden`.

That op runs `DisplayState::set_hidden`, which runs `display_check` —
`drawn_targets`, the FULL admission test. So it also refuses
[`AdmissionFault::FusedGeometry`], for an instance whose geometry is
fused into a drawn root with other instances' (a cross-instance
boolean). `instance_check` admits that instance; `set_hidden` refuses
it. **The checkbox is enabled, and clicking it produces a refusal.**

## Stated so it is not overstated

The reader is not left with nothing. Ten lines below the checkbox the
free-move arm renders the same fault's sentence — `free_move_check`
answers `FusedGeometry` too, and `ui.weak(fault.to_string())` shows
*"instance N's geometry is fused into node M together with instance(s)
… — a display operation cannot address it separately"*. So the words
are on screen.

What is wrong is their POSITION and what the chrome implies with it:
the sentence stands in the free-move probe's own place — the arm that
would otherwise draw the probe — so it reads as the probe's
ineligibility, while the hide toggle above it is offered as usable. A
reader is told the fused fact in the place that suggests it governs one
control, and it governs both. (**There is no heading over it**: an
earlier draft of this row said the sentence sits *"under the heading of
the free-move probe"*, and the label `free-move probe (mm, display
only):` is drawn in the OTHER arm, the one where the probe is offered.
The position complaint stands; the heading was not there.)

## The two shapes

- **Gate the section on `display_check` instead** — the fused instance
  then gets no section at all. Cheapest, and wrong: hiding is refused
  but the instance IS there and a reader who selected it deserves to
  know why the pane is empty, which is the census's own rule.
- **Keep `instance_check` as the section gate and disable the toggle on
  `display_check`**, carrying the fault's sentence as the disabled
  control's words. That is exactly the shape the census landed on, and
  it puts the fused sentence where it governs.

The second looks right. Whoever takes it should check whether a fused
instance is reachable through the GUI's own authoring path before
sizing it — a cross-instance boolean has to be authored first.

## Home

VNEWS's: `crates/viewer/src/pane/properties.rs`, double-claimed with
VGEOM and written on both sides, so a change there announces.

## Closed

Landed on `vnews/properties-controls-read-their-refusals`, in the
second shape this row proposed.

`instance_ui` still gates the SECTION on `display::instance_check` —
the kind test, and the silence for its two arms is argued where the
fault is defined and is unchanged. The hide toggle inside it is now
`ui.add_enabled(display_check(..).is_ok(), Checkbox::new(..))`, the
full admission test `DisplayState::set_hidden` itself runs, and the
fault's own sentence is drawn under the control it governs.

**All three tests read `committed_doc()`, which the first draft did
not** and the style review caught. `set_hidden` and `begin_free_move`
are handed `history.doc()`, while `DocSession::doc()` answers the
previewed document whenever a gesture is in flight — and
`SetInstanceHidden` is permitted during a value gesture, so both are
live in one frame. Nobody could construct a preview that flips the
answer today, which is the point: that is a fact about the current
edit vocabulary, not a property of the code, and `session/probe.rs`
records the same bug being repaired once already in the same words —
*"makes that agreement structural instead of circumstantial"*. The
residue outside this fence is
`work/vseam/a-panels-gate-reads-the-previewed-document-while-its-door-reads-the-committed-one`. When the
toggle is refused the section ends there: `free_move_check` runs
`display_check` first, so the probe below would answer the SAME fault,
and the sentence that used to sit under the probe's heading — the row's
actual complaint — is not said a second time.

**This is the first real member of the census's own blind spot.**
`a-disabled-control-says-why-in-four-shapes` parked *"a control drawn
as usable that refuses on click"* as a shape no pass in it could see,
with `pane/create.rs`'s unreachable-in-practice Add-profile arm as its
only named instance. A fused instance's hide toggle is reachable, is
drawn enabled, and is refused on the click.

`a_fused_instances_section_is_drawn_and_its_display_controls_are_refused`
(`crates/viewer/tests/assembly_display.rs`) holds the two tests apart —
`instance_check` admits the fused instance, `display_check` refuses it
— and holds the pre-click sentence against the post-click refusal's
rendering. Verified red twice: by re-wording `Refusal::Display`'s arm
(the two sentences stop being one), and by making `drawn_targets` stop
refusing a fused root (the gap closes and the row has no subject). The
fixture it shares with `fused_geometry_refuses_both_display_ops_typed`
is extracted as `fused_pair`, so the two rows cannot come to disagree
about what fused is.

**The panel half is drawn headless.** The toggle and its sentence are
`hide_toggle`, a free function over the `Ui` — the route
`crate::pane::headless`'s own doc prescribes — and two rows in
`pane/properties.rs` click it: refused, it cannot be flipped and the
fused sentence is painted under it, **planted as a literal**;
addressable, the same click flips it and nothing is said, which keeps
the first row from passing because the harness missed the checkbox.
Verified red by re-introducing the defect (the toggle enabled
regardless) and by minting a sentence in place of the fault's. The
assembly row above also plants the fused sentence beside its
coupling assertion; red when `FusedGeometry`'s wording is changed,
with the coupling still green.

**What no test holds**: which DOCUMENT `instance_ui` reads. That choice
is made in the method, before it calls `hide_toggle`, and the method
hangs off `ViewerBehavior`, which nothing in the crate constructs —
`work/vnews/no-test-can-reach-a-pane-function`, which also names the
shape that would close it.

Filed by the same sweep and its fix pass:
`the-unit-picker-is-offered-on-a-slot-whose-notation-is-not-the-users`
(a sibling member in the same file),
`the-mirror-class-is-unswept-outside-the-properties-pane` (the rest of
the ground), `no-test-can-reach-a-pane-function`, and on VSEAM's slate
`a-panels-gate-reads-the-previewed-document-while-its-door-reads-the-committed-one`
and `probe-scale-refuses-a-no-readable-value-case-its-only-caller-pre-empts`.
