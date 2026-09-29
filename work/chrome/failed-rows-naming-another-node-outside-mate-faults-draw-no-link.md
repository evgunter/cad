---
id: failed-rows-naming-another-node-outside-mate-faults-draw-no-link
kind: issue
title: Failed rows whose NodeErrorKind names another node (outside MateFault) draw no link to it
status: closed
opened: 2026-09-23
priority: P4
cost: E
closed: 2026-09-29
branch: chrome/create-residue
pr: 3450
---


## Finding

`chrome/placer-link` gave a `Failed` mate row a link to the node its
`MateFault::PlacerRefused` names (`crates/viewer/src/tree.rs`,
`repaired_at` and `TreeRow::repair_at`; drawn by
`crates/viewer/src/pane/features.rs`, `failure_lines`). `repaired_at`
is exhaustive over `MateFault` only. A `Failed` row whose error is any
OTHER `NodeErrorKind` arm is not asked the question at all: its words
may name another node and the row draws no link to it.

The sweep (the shape: a `NodeErrorKind` arm carrying a
`RecipeNodeId` besides the failing node,
`crates/editor-core/src/eval/mod.rs`, `enum NodeErrorKind`) hits:

- `MissingInput { input }`, `WrongOperand { input }`,
  `EmptyOperand { input }`, `EmptyHalf { input }`,
  `InstanceOutOfRange { input }` — an operand of the failing node.
- `SeedPinnedSection { section }`.
- `AxisInDifferentPlane { axis, axis_plane, profile_plane }`.
- `DeclareSiteNotAnOperand { at }`.
- `DerivedFrameSection { profile, frame }`,
  `FrameDirection { profile, frame }`.

None is decided either way. Ev's ruling on
`blamed-mates-sends-the-eye-past-the-node-the-fault-says-to-fix`
(option c) covered `PlacerRefused` alone, whose kernel doc calls the
named node *"the node an author goes and fixes"*; the test for each arm
above is the same one — does the kernel's own doc for the arm call the
named node the thing to repair — and nobody has applied it.

**Blind spot of the sweep:** it read `NodeErrorKind`'s own fields.
Node ids nested inside a boxed payload (`BooleanError`,
`TransformError`, `ShellError`, …) were not searched beyond
`PlacementRuleFault`, `NamingError` and `EvalError`, which carry none.

## Where to look

`crates/viewer/src/tree.rs` — `repair_of` is where a second fault
family would be read; `crates/editor-core/src/eval/mod.rs` for each
arm's doc (MSOLVE's and the evaluator's ground; read, do not edit).

Signed: (CHROME implementer lane, `chrome/placer-link`)

## Closed 2026-09-29 (`chrome/create-residue`, PR 3450)

`tree::repair_of` now asks `repair_named`, an exhaustive `match` over
every `NodeErrorKind`, with `Mate` delegating to `repaired_at`. The
test is PR 3100's: does the kernel's doc for the arm call the named
node the thing to repair?

- `FrameDirection { frame }` — **links.** The frame's own direction
  slot refused, and the profile only read it. The frame's row can read
  `Ok`, so without the link nothing gets a reader there. Held by
  `tree_badges::a_profile_refused_for_its_frames_direction_links_to_the_frame`,
  which fails on the old code.
- `WrongOperand`, `EmptyOperand`, `EmptyHalf`, `InstanceOutOfRange` —
  no link. The operand is the row the failing node hangs from, and the
  refusal is the failing node's use of it.
- `MissingInput` — no link. The id names no live node, so there is no
  row to link to.
- `AxisInDifferentPlane` — no link. The kernel says *"the fix depends
  on which one is wrong"*, and one link would pick for the reader.
- `SeedPinnedSection`, `DerivedFrameSection` — no link. These are lane
  limits: neither node is wrong, and the f64 lane builds them.
- `DeclareSiteNotAnOperand { at }` — no link. The site is what the
  `Declare` chose, and the error does not name the `Declare`.
- `CrossingUnverified { instance }` — no link. It names the failing
  instance itself (`wire.rs` raises it with `instance: id`).

**The blind spot, checked.** A second pass read the payloads the first
sweep skipped. `NamingError` is NOT id-free, contrary to the finding
above: `MissingUpstream { node }` and its `StableName`s carry ids.
`PartFault::PartRootFailed { node }` carries an id in ANOTHER
document's id space. `ResolveError`, `StableName`, `SitedRef`,
`FaceName` and `InterrogateError` carry minting or site nodes. Every
one of these is evidence rather than the repair, and none links. The
kernel-crate payloads (`BooleanError`, `ShellError`, …) cannot name a
`RecipeNodeId`, because those crates sit below `editor-core`.

The arms whose answer is a judgement call rather than read off the
kernel's doc are filed as
`failed-row-repair-links-for-arms-with-two-candidate-repairs`.
