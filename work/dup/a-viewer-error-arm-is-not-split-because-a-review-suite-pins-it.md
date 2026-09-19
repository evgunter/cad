---
id: a-viewer-error-arm-is-not-split-because-a-review-suite-pins-it
kind: issue
title: camera.rs declines an error-arm split on the ground that a promoted review suite pins the arm — the reading Ev withdrew, in src/
status: open
opened: 2026-09-19
---


## Finding

- **Where**: `crates/viewer/src/camera.rs`, the `UnusableBounds` arm's
  rustdoc:

  > Splitting a `UnusableAspect` arm out was considered and declined:
  > it would buy that one bit at the cost of **a promoted review suite
  > that pins this arm by name**.

- **Why it is wrong**: the cost it names is a review suite's class.
  `memories/review-and-dependency-policy.md` withdrew exactly that
  reading on 2026-09-04 — *"nothing about them is special afterwards …
  trimmed, gated or retired under the same rules as every other row"* —
  so a row that would have to be re-spelt is the same cost any row
  would be, and cannot by itself decide a production error vocabulary.
- **Importance**: medium, and sharper than the test-side carriers: this
  one is in `src/`, and it is the only carrier found that lets the
  withdrawn reading **decide a public API's shape** rather than justify
  a test fixture.
- **Confidence**: sure about the text and the withdrawal. **Not sure
  the arm should be split** — which is why this is a row and not an
  edit.
- **Raised by**: the S-DUP citation-census unit's fix pass,
  2026-09-19.

## Why filed rather than fixed

Two different things sit in that doc comment. The paragraph above the
quoted sentence gives a complete, independent argument for one arm —
*"both are 'the framing request names no view', and the doors that
return it take exactly those two arguments, so the caller's next
question is answered by which door refused."* That argument stands on
its own and this row does not touch it.

The quoted sentence is an **additional** cost claim, and deleting it
would read as licensing the split, while restating it means deciding
what the error vocabulary owes a caller. That is an API judgement on
`viewer`'s ground, not a duplication lane's call.

## What would settle it

The owner deciding one of: the sentence goes and the argument above it
carries the decision unchanged; or the arm is split and the pinning row
is re-spelt like any other row. Either is small; which one it is
belongs to whoever owns the error vocabulary.

## How it was found, and why the earlier census missed it

The unit's first census enumerated **citations of the memory**, and
this file does not cite it. Its second, run on the style review's
instruction (method item 8 — a disclosed blind spot is an instruction
to run another instrument), enumerated the **class-shaped ground**
itself: a generalisation over what a file IS
(*"a/the/every review suite" + a property*, *"reviewer suites are…"*,
*"derives what it needs independently"*, *"the independence is the
value"*, *"protected class"/"promoted as-is"/"keep verbatim"*), over
every tracked file with comment markers stripped and the text joined.
That returned 28 files, of which the live source carriers were this
one, `crates/viewer/tests/common/mod.rs` and
`crates/viewer/tests/review_gui3_r2.rs` — the last two fixed in
PR #2886.

Its blind spot, stated: it matches a ground phrased as a
generalisation. A file that declines a change for the same bad reason
phrased entirely about one named suite would not match, and nothing
distinguishes that from a legitimate instance-specific argument except
reading it.

## Why this sits on S-DUP's slate

`crates/viewer/src/camera.rs` is claimed by `chrome`, `vgeom` and
`view` (`work.py territory`), so there is no single ground-owner, and
the finding's subject — one withdrawn instruction still deciding
things — is S-DUP's charter. Any of the three may claim it by `git mv`.

## The sentence is factually true, and that makes it weaker, not stronger

Both halves measured at `5b4979ef2`, by naming the sites rather than
totalling them:

- **A promoted review suite does pin the arm by name.**
  `crates/viewer/tests/review_gui0_r1.rs:629` and `:643` both assert
  `Err(CameraError::UnusableBounds)`, and `:629` is the **aspect** case
  (`Camera::framing(&plate_bounds(), 0.0)`) — precisely the assertion a
  `UnusableAspect` split would have to re-spell. So the quoted sentence
  is accurate about the world; what is wrong with it is only its
  ground.
- **It is not the cost, though.** Other test-side sites naming the arm:
  `crates/viewer/tests/error_display.rs:71`, `:87`, `:268`, `:340`
  (four constructions) and `:271` (a string assertion over the same
  name), and `crates/viewer/tests/datum_draw.rs:1291`. Prose mentions
  in `src/` carry it too —
  `crates/viewer/src/datums.rs:1066`, `:1073`, and
  `crates/viewer/src/pane/viewport.rs:577`, which discusses exactly
  this conflation.

So a split's real cost is spread over every one of those, of which the
review suite is two lines. Naming the review suite as *the* cost
singles out the one site the withdrawn reading made special and is
silent about the rest — which is the reading's characteristic effect,
not an incidental phrasing.

## The decline survives without the sentence

`crates/viewer/src/camera.rs:184-195` — the paragraph immediately above
— argues the arm entirely from the caller's position: *"both are 'the
framing request names no view', and the doors that return it … take
exactly those two arguments, so the caller's next question — which of
my two arguments was wrong — is answered by which door refused."* It
mentions no suite. **Deleting the review-suite sentence therefore
leaves the decision standing**, which is why the choice this row hands
over is a real one and not a forced split.
