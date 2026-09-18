---
id: viewport-adapter-drops-part-of-two-toolkit-values
kind: issue
title: The viewport adapter reads part of the scroll delta and part of the modifiers, and states neither
status: closed
opened: 2026-09-12
closed: 2026-09-15
branch: view/viewport-adapter
---



Filed by the DOOR unit that closed
`work/door/viewport-pointer-buttons-mirror-a-toolkit-enum-by-hand`
(PR 2450). Same shape as that row's defect — **an adapter silently
incomplete against its upstream, with no statement either way** — at
two more sites in the function that unit rewrote,
`ViewerBehavior::viewport_ui` in `crates/viewer/src/pane/viewport.rs`,
which is VIEW's ground.

The button half of the class is fixed and stated. These two are not,
and they are one line apart.

## 1. Two of the modifiers' five fields

`crates/viewer/src/pane/viewport.rs:195`

    let (shift, alt) = ui.input(|i| (i.modifiers.shift, i.modifiers.alt));

`egui::Modifiers` has five fields — `alt`, `ctrl`, `shift`, `mac_cmd`,
`command` (`egui-0.36.1/src/data/input/modifiers.rs:19-37`). Two are
read. `ctrl`, `command` and `mac_cmd` reach nothing, and the
vocabulary has no room for them either: `ViewportEvent::Drag` carries
a bare `shift: bool` and a bare `alt: bool`
(`crates/viewer/src/input.rs:84,88`) and nothing else, so a
ctrl-drag is indistinguishable from a plain one at every reader
downstream.

That is very likely the right product answer — `InputMap`'s only
modifier binding is `alt_orbit_button`'s `alt`, and `shift` is the
constrain modifier — but nothing at the site says so, and the two
bare bools read as "the modifiers" rather than as "the two modifiers
this viewer binds".

**Note the second-order cost**: `ctrl` unread also means a
ctrl+scroll and a plain scroll are the same event, which is the
gesture most CAD and browser users expect to be a zoom.

## 2. One of the scroll delta's two axes

`crates/viewer/src/pane/viewport.rs:209`

    let scroll = ui.input(|i| i.smooth_scroll_delta.y);

`smooth_scroll_delta` is an `egui::Vec2`. `x` is the axis a trackpad's
two-finger sideways swipe and a tilt wheel produce, and it reaches
nothing; `ViewportEvent::Scroll` carries one scalar
(`crates/viewer/src/input.rs:93-95`), so no binding could read it even
if the adapter passed it on.

Again plausibly right — the viewer's only scroll binding is zoom
(`input::InputMap::fold`, `crates/viewer/src/input.rs:337-342`), and
horizontal scroll has no CAD convention behind it the way middle-drag
orbit does — and again unstated.

## Why these are one row and not two

They are the same decision at two adjacent reads: *which parts of the
toolkit's pointer state this viewer binds*. Whoever answers one is
holding the other in their head, and the answer wants to be one
sentence about the adapter rather than two footnotes.

## The shape an answer has

What the DOOR unit did for the button half, at PR 2450: state the
decision where the narrowing happens. Either the dropped part gets a
binding — and then a vocabulary that can carry it — or the site says
which parts the viewer reads and why the rest are not gestures here.
`viewer_button`'s doc in that PR is the worked example; the `None` arm
names what would have to change to reverse it.

Cheap either way; the point is the statement, not the code.

## Closed (2026-09-15, branch `view/viewport-adapter`)

**Both narrowings are stated, and both are compiler-held.** The two
reads go through named conversions in `pane/viewport.rs` —
`viewer_modifiers` over `egui::Modifiers` and `scroll_event` over the
scroll vector — whose docs are one statement in two halves, each
pointing at the other. Each destructures the toolkit's struct with
every field named, so a sixth modifier field (or a third axis) fails to
compile at the site rather than passing silently through a field
access. `egui::Modifiers` is a plain struct and not
`#[non_exhaustive]`, which is what makes that lever available; it is
the struct's form of the exhaustive `match` PR 2450 used on
`egui::PointerButton`, and it fires at the same moment — a version
bump.

**What the compiler still cannot hold is the PAIRING**, exactly as at
the button half: `(shift, alt)` is a pair of bools and swapping the two
type-checks. `tests::a_drag_carries_the_two_modifiers_this_viewport_
binds` is the second statement of that table, over six modifier
combinations driven through a real toolkit frame.

**The `ctrl` half of this row's premise was wrong, and the
re-derivation is the unit's main finding.** The row said an unread
`ctrl` makes a ctrl+scroll and a plain scroll the same event. They are
not: `InputState::begin_pass` routes a wheel whose modifiers match
`InputOptions::zoom_modifier` into `zoom_factor_delta` and leaves
`smooth_scroll_delta` at zero, so **a ctrl+wheel produces no
`ViewportEvent` at all** and reading `ctrl` in the adapter would
recover nothing. That is a live product gap in the gesture CAD and
browsers zoom with, and it is filed rather than disclosed here:
`work/view/ctrl-wheel-reaches-no-zoom.md`.

**The same mechanism answers shift and alt**, which the row did not
reach. `horizontal_scroll_modifier` is SHIFT, so a shift+wheel is
folded onto `x` and dropped; `vertical_scroll_modifier` is ALT, so an
alt+wheel is folded onto `y` and zooms exactly as a plain wheel does.
Three rows hold those three facts, each with the positive control that
stops it passing over a wheel the probe never delivered.

**`x` stays dropped and the reason is now specific.** It is not only a
trackpad's sideways swipe: it is where the toolkit puts shift+wheel on
any mouse. Zoom is the only binding a scroll has, so an `x` passed on
would have to zoom, and a sideways swipe that zooms is worse than one
that does nothing.

**No README amendment, and none is owed.** Behaviour is unchanged, so
`crates/viewer/README.md`'s mouse-bindings table stays true; and
neither new item is a membership list in either shape
`viewer-vocab-declared-once.sh` scans for — a destructuring pattern in
a function body is no `const ALL` and no `const` array literal — so
its roster neither sees them nor wants a row.
