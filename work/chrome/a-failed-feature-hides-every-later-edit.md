---
id: a-failed-feature-hides-every-later-edit
kind: issue
title: after one feature fails, later edits commit and read Ok but nothing is drawn or picked
status: open
opened: 2026-10-31
priority: P0
cost: H
refs: [one-failed-root-refuses-the-whole-pick-index, 6197]
---

## Finding

Found driving the viewer (2026-10-31), and hit by Ev making a sphere: a revolve that failed earlier stopped the sphere from appearing, which read as the sphere's own revolve failing. Ev: "the failed feature did *not* halt the ability to take actions in the gui that looked like they'd have an effect, which is i think the real problem here."

After a revolve fails (`Revolve 1 … FAILED`, its Place `POISONED`), a second revolve of the same profile commits and its tree row reads Ok (`Revolve 2 … Ok placed`, `Place … Ok`), but the viewport draws nothing new: the picture and the pick index wait on the failed node. The only notice is a grey toolbar line: `pick index: waits on Revolve "Revolve 1" (…), which failed — until the index builds, no pick is answered and the picture is not redrawn`. Every edit after the failure goes on being accepted, so a person keeps working on a picture that does not change.

Either a failed feature should not hold up the picture of everything else, or the chrome should stop presenting later edits as taking effect while it does (say it where the edit is made, loudly). Which one is the design question.

Replay with `cargo run --release -p viewer --features app --example drive -- script.txt` (window 2400 wide, so the path rows' fields are on screen):

```text
window 2400 1600
click "New…"
click "document name"
type "ball"
click "Create" 0
settle
click "Add feature"
click "path"
click "line_to" 0
click "arc_to"
click "0.50"
type "1"
key Enter
click "×" 2
click "×" 2
settle
click "Add profile"
settle
click "Revolve tool…"
click "Profile 1 *"
settle
click "Commit revolve"
settle
click "Revolve tool…"
click "Profile 1 *"
settle
click "0.500"
type "0"
key Enter
click "Commit revolve"
settle
tree
picture after.png
```

The first revolve uses the default axis heading (½π, the profile's y axis) on a semicircle along x, which refuses; the second, at heading 0, is the sphere, and is not drawn.
