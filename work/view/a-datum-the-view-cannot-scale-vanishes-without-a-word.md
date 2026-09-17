---
id: a-datum-the-view-cannot-scale-vanishes-without-a-word
kind: issue
title: A datum the view cannot scale draws nothing and says so nowhere
status: closed
opened: 2026-09-15
closed: 2026-09-16
---


## Finding

`crates/viewer/src/datums.rs` refuses per mark: `screen_metres_at`,
`half_patch_at` and `grid_pitch` each answer `None` when this view
lends that point no length, and the mark is simply not appended. A
datum whose every mark refuses contributes a `DatumDraw` with an
EMPTY segment list, which `pane::viewport` pushes zero positions from.

So the viewport shows nothing, and nothing anywhere says a datum was
dropped rather than absent: no fault, no badge, no line in the tree.
A reader cannot tell "this document has no datums" from "this view
has no scale for the ones it has".

**The sweep on `datums.rs` widened this rather than creating it, and
by less than a first draft of this row claimed.** Refusing was already
the answer for a scale that overflowed. The sweep added one input
reachable through document data — **the eye exactly on a datum**,
reachable by flying the camera into a plane, which used to draw a mark
about `1e-307 m` across off a `f64::MIN_POSITIVE` floor and now draws
nothing — plus **a ruling whose extent is lost to the datum's own
magnitude**, which used to emit zero-length segments and now emits
none. The viewport cases (`viewport_px`, and `datum_view`'s two
sides) are NOT among them: `ViewerBehavior::viewport_ui` returns
before `datum_view` when `ViewportSize::aspect` refuses a pane with no
area, so no viewport that is not a positive number of pixels reaches
this module from the app. All of these refusals are right and none is
announced.

**The project is fail-loud** (`CLAUDE.md`, `docs/DESIGN.md`), and the
crate has the machinery: `pane::viewport` already carries a
`projection_fault` latch for exactly the neighbouring case — a camera
that will not project — with `work/view/projection-fault-has-no-sweeper.md`
tracking its sweep. A datum drawing that refused is the same shape of
fact.

## What it would take

A per-frame count or latch — "n datums this view has no scale for" —
raised out of `datums::draws` and shown the way the projection fault
is. That is a change to `draws`'s return shape in `datums.rs` and to
its one caller in `crates/viewer/src/pane/viewport.rs`, which is
VIEW's ground.

## Fence

`crates/viewer/src/datums.rs` and `crates/viewer/src/pane/viewport.rs`
— CHROME's and VIEW's.

## Territory

**VIEW acts on this, not CHROME.** The call site is
`ViewerBehavior::viewport_ui` in `crates/viewer/src/pane/viewport.rs`
— the `for drawn in datums::draws(...)` loop inside the `show_datums`
block, and the `projection_fault` latch a few lines above it, which is
the existing machinery for exactly this shape of fact and whose own
sweep is `work/view/projection-fault-has-no-sweeper.md`. The
`datums.rs` half — raising a count or a reason out of `draws` — is
CHROME's.

Filed on CHROME's slate because the refusals are `datums.rs`'s; it
cannot be discharged without VIEW.

## Re-homed to VIEW, 2026-09-15

Moved out of `work/chrome/` by the CHROME orchestrator, for the reason
its Territory section already gives: the act is in
`ViewerBehavior::viewport_ui` (`crates/viewer/src/pane/viewport.rs`),
ceded to VIEW by the 2026-09-15 carve-out, and the existing machinery
for this exact shape of fact — the `projection_fault` latch — is a few
lines above the `datums::draws` loop, with its own open row
`work/view/projection-fault-has-no-sweeper.md`. Those two are plausibly
one piece of work, which is another reason this belongs on VIEW's slate
rather than CHROME's.

**The `datums.rs` half stays CHROME's**: raising a count or a reason out
of `draws`, so there is something for the latch to carry. That is inside
CHROME's fence and needs no negotiation.

One caution carried over from the unit that filed this: the sweep that
prompted it **widened** the silent-drop, and the widening is real — two
more inputs now refuse where they previously drew something wrong. The
row is not "a pre-existing defect someone should get to"; it grew on
2026-09-15 and the growth is recorded on the row above.

Signed: (CHROME orchestrator)

## Closed, 2026-09-16

`datums::draws` answers a `DatumDraws` — the wireframes, plus
`DatumDraws::vanished()`, how many of them came out with nothing drawn
at all. The type change is what carries the fact: the count travels
with the drawings and no caller can push the segments without having
been handed it.

**The property is "drew nothing", not "has no scale"**, and the
reason is THREE independent refusals rather than the two this row
supposed. Instrumented on a plane `z = 0` whose origin sits at
`x = M`, with the camera at `(0, -0.15, 0.1)` looking at the world
origin — an ordinary view, so the whole extremity is the datum's own
coordinate:

| `M` | scale at the patch centre | scale at the datum origin | ruled direction u | ruled direction v | vanished |
|---|---|---|---|---|---|
| `1e15` | `1.87e-4` | `1.04e12` | 25 lines | 27 lines | 0 |
| `1e20` | `1.87e-4` | `1.04e17` | 1 line | extent lost | 0 |
| `1e100` | `1.87e-4` | `1.04e97` | 1 line | extent lost | 0 |
| `1e200` | `1.87e-4` | none | 1 line | extent lost | 0 |
| `1e300` | `1.87e-4` | none | 1 line | extent lost | 0 |
| `f64::MAX` | `1.87e-4` | none | bounds overflowed | extent lost | 1 |

So: the patch centre has a scale at EVERY magnitude, because the
centre is the looked-at point and the eye is a decimetre from it. The
three mechanisms are a mark's point lending it no length, a ruling's
`coordinate / pitch` overflowing past `rule_patch`'s finiteness guard,
and a ruling keeping its scale and losing its EXTENT — and they switch
on in different bands. A count named for the scale would have been the
first mechanism wearing the name of the set.

**The `f64::MAX` fixture is emptied by all three at once**, which is
what makes it the wrong witness for any one of them, and the first
draft of this section named it as the lost-extent case.
`a_plane_can_lose_its_extent_while_every_point_of_it_still_has_a_scale`
is the row that splits them: at `1e100` the origin still scales, the
tick draws, one direction still loses its extent, and the plane has
NOT vanished — which is the two predicates coming apart in one
drawing. The eye-on-datum fixture is the clean member of the first
mechanism: every point of every datum is at a depth of exactly zero.

**The lost-extent arm is reachable through `datum_view`** and needs no
pathological window: the table above was taken through the same
formula the door uses, at a 1280x800 window with a 45° field. The
ratio `half / cv` is NOT a magnitude-independent constant — that would
hold only if the eye were about as far from the patch centre as the
datum's origin is, and it is not, because the centre is where the
camera is aimed. Measured, `half` is `0.26 m` against a `cv` that runs
to `1e308`.

**The shape pinned is the DIFFERENCE**, because "the segment list is
empty" is true of a document with no datums as well.
`how_many_datums_this_view_drew_nothing_of_is_a_fact_the_caller_is_handed`
measures four cases under one view: four datums at `f64::MAX` (four
vanished), the same four with the eye exactly on them (four vanished),
the same four from an ordinary place (**none** — without which the
first two are satisfied by a module that never draws), and a document
holding no datums at all (none, and an empty drawing list). The last
pair is the whole of the row.
`a_datum_that_drew_some_of_itself_has_not_vanished` holds the other
boundary: a plane whose ruling went and whose normal tick stayed is
something a reader can see, and is not counted.

**It is shown as a badge and it holds nothing.** `frame::datums_badge`
reads a count and says *"datums: 4 datums this view draws nothing of"*
— `Subject::Camera`, `Tone::Actionable`, silent at zero.

**No second latch was minted, and this is the part the sibling row
gates.** `work/view/projection-fault-has-no-sweeper.md` is open
because `projection_fault` is written only where the viewport draws,
so a pane tabbed away leaves the last value standing forever. The
count is not written that way: the frame entry point
(`<ViewerApp as eframe::App>::ui`) zeroes a local
before the panes draw and assigns it back **unconditionally** after,
whether or not the viewport was among them — `profile_form_drawn`'s
discipline, which that row names as the pattern the fault still lacks.
A frame the viewport does not draw therefore reports none, and there
is no sweeper to be missing. What is NOT covered is a headless row
over that discipline, for the reason §2 of the sibling row gives: it
is two assignments in an `app`-gated draw path and there is nowhere
headless to put one.

Landed with `datum-view-propagates-rather-than-refusing-by-name`.
