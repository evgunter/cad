---
id: cursor-projection-landed-in-marks-for-want-of-a-home
kind: issue
title: cursor_projection is a camera transform living in marks because no module claimed it
status: closed
opened: 2026-09-06
refs: [2083]
closed: 2026-09-06
---


Found by the style review of #2083, which names this the unit's one
judgement call.

`cursor_projection` (`crates/viewer/src/marks.rs:448-478`) takes a
view-projection matrix, a cursor in NDC and a viewport size, and
returns a matrix. It names no index, no selection, no hover and no
document. Nothing about it is a mark: its subject is a camera
transform, and its doc says so — *"composed with
[`crate::camera::Camera::project`] it says that the world point the ray
path un-projects to is the point the id pass rasterizes at the centre
of its target"*.

The reason given for its home is testability, not subject: *"it lives
here, out of the render module, because it is the one part of the id
pass a machine with no GPU can check"*. That is an argument for it not
being in `gpu`; it is not an argument for it being in `marks`. The PR's
own table records it as `none — a matrix`, and its production consumer
set is exactly `gpu.rs:594` — the same consumer it was moved away from.

`crate::camera` is the module the function composes with, the one whose
doc link the move had to rewrite to
`[\`crate::camera::Camera::project\`]` (`marks.rs:460`), and the one a
reader hunting a projection matrix opens. That the move had to widen
that link into a full path is itself the signal: the function's subject
lives in another module.

Taking it made `marks.rs`'s opening universal false (see
`marks-header-asserts-universals-its-own-module-breaks`) and turned a
three-item module into a four-item one whose fourth item the section
title has to mis-describe. Either it should have stayed in `pickindex`
until a unit moved it to `camera`, or the unit should have moved it to
`camera` here.

## Confidence

`likely`. The move is behaviour-preserving and nothing is broken; this
is a judgement that `marks` is the wrong home and that the header
damage is the evidence.

## Closed

`cursor_projection` moved to `crates/viewer/src/camera.rs` — the home
the item proposed, verified before the move rather than assumed:

- **`camera` already holds the projection algebra.** The subject is
  the same one — a view-projection, a cursor, a viewport — which is the
  thing `marks` could never say. **The doors do not meet at the type**,
  and the first version of this close said they did:
  `Camera::view_projection` returns `[[f64; 4]; 4]` where
  `cursor_projection` takes `&[[f32; 4]; 4]`, `Camera::project` answers
  `Option<[f64; 3]>`, and `Camera::ray_through` takes PIXELS with `+y`
  down against this function's `+y`-up NDC. The conversion happens two
  modules away in a driver (`pane/viewport.rs:446`, `:423-424`), so
  `camera` is the home of a function it cannot feed from its own doors;
  `cursor-projection-is-f32-in-a-module-whose-matrices-are-f64` holds
  that question. The move rests on the other three checks.
- **It costs `camera` no import.** The signature is `[[f32; 4]; 4]`,
  `[f32; 2]`, `[f32; 2]` and nothing else, so the `use` block is
  untouched and the module-kind gate's answer is unchanged
  (**vocabulary**, both before and after; `viewer-module-kinds.sh` OK).
- **`camera` already holds free functions** — `apply`, `fold`,
  `fold_recorded` — so a `pub fn` outside `impl Camera` is the file's
  existing shape, not a new one.
- **The doc link narrowed rather than widened**, which is the item's
  own tell running in reverse: `[\`crate::camera::Camera::project\`]`
  is `[\`Camera::project\`]` in its new home.

**Behaviour-preserving.** The body and signature moved verbatim; the
only edits are prose. `crates/viewer/tests/*` needed no re-pointing
because every suite already spelled it `viewer::cursor_projection`, the
crate-root re-export, and that spelling is unchanged — it moved from
the `pub use marks::{…}` list to the `pub use camera::{…}` one, so the
COUNT of public paths is unchanged and **no `pub use` shim** was left
behind. It was never one path: `lib.rs:55` is `pub mod camera;`, so
`viewer::camera::cursor_projection` resolves beside
`viewer::cursor_projection`, exactly as both spellings resolved through
`marks` before. `session-shims-and-test-imports`'s hazard is a `pub
use` shim INSIDE a module — a lie about where an item lives — not a
crate-root re-export, and this close first drew that contrast wrongly
(`every-crate-root-reexport-is-a-second-path-not-the-only-one`). The one
production consumer, `gpu.rs`, now imports `crate::camera::cursor_projection`.

**Three structural sentences the move falsified, all fixed:**
`marks.rs`'s *"# `cursor_projection` is not a mark, and is here for
want of a home"* section is deleted — the module is now the three marks
its opening claims and nothing else; `camera.rs`'s opening line said
*"one state value, one typed operation vocabulary, one pure `apply`"*
and now names the algebra too, with a `# The one free transform`
section saying why the function is there; and the README's GQ7 row
listed `cursor_projection` among `marks.rs`'s members and now names
`camera::cursor_projection` separately, as projection algebra rather
than a mark.

**Where it is now**, since the citations above name the pre-fix tree:
`crates/viewer/src/camera.rs:860-890` (doc from `:860`, `pub fn` at
`:876`), imported by `crates/viewer/src/gpu.rs:80` and called at
`gpu.rs:595`; the doc link this item cited at `marks.rs:460` is
`camera.rs:872`.
