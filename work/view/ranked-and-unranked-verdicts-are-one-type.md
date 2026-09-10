---
id: ranked-and-unranked-verdicts-are-one-type
kind: issue
title: A ranked verdict and a policy's verdict are the same type, so which door a call site must use is carried only by prose
status: open
opened: 2026-09-06
refs: [status-line-writers-bypass-the-ranking, camera-fold-clears-status-line, 2026]
---

Found by #2026's style review, on the unit that created the second
door.

## What this is

`crates/viewer/src/frame.rs` has two doors onto the status line and the
rule for choosing between them is written down in three doc comments
and in nothing else:

- `frame::apply(status, update)` — the verdict goes to the field.
- `frame::deliver(notices, status, update)` — a `Show` joins the
  frame's notices and meets `frame_status`'s ranking; the other three
  arms go to the field.

**Both take `StatusUpdate` and both are correct for some callers**, so
picking the wrong one compiles, passes every row, and reintroduces the
defect the sweep was for. `crates/viewer/src/pane/viewport.rs` has both,
ten lines apart, in one file:

```rust
// :46, in `land`
frame::deliver(notices, status, frame::fold_status(folded));
// :178, in the id pass
frame::apply(self.status, frame::cursor_status(step));
```

Nothing at either call site says which is which. The rule is *"can this
policy ever answer `Show`?"* — a fact about the callee's arms, invisible
from here, enforced by no type. Give `frame::cursor_status` a `Show` arm
one day (the news vocabulary has been extended twice already) and
`:178` silently becomes a writer that puts a sentence on the line the
ranking never saw. There is no diff at which that looks wrong: the
diff that breaks it is in `cursor_status`, and the site that breaks is
in another file and unchanged.

That is `status-line-writers-bypass-the-ranking`'s own defect class,
moved up one level. That item swept eighteen sites where a writer could
skip the ranking; the sweep's own fix leaves ONE site where a writer can
skip the ranking, and it is spelled the same as the site that must not.

## The asymmetry is real, so "one door" is not free

The two doors are not redundant. `app::ViewerApp::perform_batch`
(`crates/viewer/src/app.rs`) computes `frame::frame_status(...)` — the
RANKED answer, a `Show` that has already been weighed against
everything the frame said — and hands it to `apply_status`. That `Show`
must reach the field. Routed through `deliver` it would be pushed back
onto `notices` to be ranked a second time, against the very list it was
the winner of.

So there genuinely are two kinds of `Show` here:

1. **A policy's `Show`** — one candidate sentence, must be ranked.
2. **The ranking's `Show`** — the frame's answer, must not be.

## The proposal (reviewer's, #2026)

**Make them different types.** A ranked verdict arrives as something
other than `StatusUpdate` — the natural shape is for `frame_status` to
return a distinct `LineVerdict` (or for `StatusUpdate` to lose its
ranked role entirely and the field-writing door to take the new type)
— at which point:

- one door suffices for policies, and it is `deliver`;
- `apply` becomes private to `frame`, or takes the ranked type;
- the choice a call site now makes in prose is made by the compiler,
  and a `Show` arm added to `cursor_status` fails to build at
  `viewport.rs:178` instead of silently changing behaviour.

## What has to be decided, and by whom

- Whether the ranked type is a new enum or a newtype over `Message` —
  `frame_status` can only answer `Keep`, `Clear` or `Show` today
  (`Expire` is a policy's answer and never the ranking's), so the
  ranked type is a strictly smaller vocabulary and that is worth
  stating in it.
- What happens to `apply`'s public surface: it is called from
  `app::ViewerApp::apply_status` and from `pane::viewport`'s cursor
  path, and the second of those is exactly the site this item is about.
- Whether `frame::deliver`'s `Clear` arm survives the change. **No
  policy that reaches `deliver` answers `Clear` at all now**:
  `fold_status` is the only one that does reach it — `deliver`'s single
  call site is `pane::viewport`'s `land` — and it answers `Show` or
  `Expire`; `dialog_status`, the other policy of that shape, is deleted
  (the ruling
  `was-the-status-route-supposed-to-fire-for-an-absent-chooser`). Only
  `batch_status` answers `Clear`, and `batch_status` is the ranking's
  input, so it reaches the field through `apply_status` and never
  through this door. `deliver`'s own header argues the arm stays
  regardless — a wildcard there would route a variant added later to
  the field by default — and that argument is about the compiler
  carrying the rule, not about the arm having a producer.

Sequence after `frame-module-has-eight-concerns-and-no-holds-row`'s
split question is answered, or independently of it — the two touch the
same file but not the same argument.

