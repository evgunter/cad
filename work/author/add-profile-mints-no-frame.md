---
id: add-profile-mints-no-frame
kind: issue
title: The add-profile form cannot mint the frame it needs, and names the ones it finds by node number
status: closed
opened: 2026-09-03
refs: [1829]
priority: P0
cost: D
closed: 2026-09-21
pr: 3023
---

## What

A profile is drawn on a `Datum::Frame` node, and `AddProfile` names one
that already exists — deliberately, so one submit inserts one node and
the frame a person drew on is the frame they can see and edit
afterwards. Two things about the form follow from that and are not
right yet.

**1. An empty document cannot draw a sketch.** The form says

> on frame — none in this document — add a frame datum first

which is true and is a dead end: the commonest first act in a new
document is a sketch on world XY, and it now costs a trip to a
different form to author a frame whose numbers are `(0,0,0)`,
`(1,0,0)`, `(0,1,0)`. The chrome states an obligation instead of
offering to discharge it. The op is right; what is missing is a form
affordance — an "on a new XY frame" choice that commits the frame and
the profile as one gesture, or the add-datum form reachable from here.

Whichever it is, it has to keep the property the current shape was
chosen for — **one submit, one undo** — and that is all it has to
keep. See the correction below: the "one committed EDIT" version of
this sentence was wrong, and it was the reason this half read as a
commit-door decision rather than a widget.

**2. The picker names frames by node number.** The combo reads
`feature 3`, `feature 7`. A frame is an oriented plane and the useful
label is what plane it is — "XY at z = 0", "on feature 5's top" — which
is exactly the information the tree row for a `Datum frame` also does
not carry (`tree.rs` labels it `"Datum frame"` and stops). Two frames a
centimetre apart are indistinguishable in the picker today.

## Why it is filed and not fixed

Both are chrome, and both touch the same question the frame-from-face
fork touches (`sketch-frame-from-face`): a frame's LABEL is a different
problem once a frame can be derived from a face, because then the
honest label names the face.

**~~Fixing the label first would be work thrown away if that fork goes
the derived way.~~ The fork went the derived way; this reason is
spent** (2026-09-15). `sketch-frame-from-face` = PR 1829 = DOCM-1,
which built `Datum::FaceFrame`, DM1a, DM1b and DM2
(`crates/editor-core/REFERENCES.md`). A frame CAN be derived from a
face today, so there is no longer a fork to wait on and no work at
risk of being thrown away — the label problem has its answer's shape
and is simply unstarted.

What the derived arm settled and what it did not: `tree.rs` now
distinguishes the two KINDS (`"Datum frame"` against `"Datum frame (on
face)"`), which is more than the bare `"Datum frame"` this row was
filed against. It is not the fix. Neither label says WHICH frame, and
`add_profile_ui`'s ComboBox still reads `format!("feature {}", id.0)`
for every frame of either kind — so "two frames a centimetre apart are
indistinguishable in the picker" is true exactly as written.

(1) does not wait on the fork and could go first.

## Where it stands

~~`gauth` and `gui` are both closed; this is unowned residue until a
program claims it.~~ **Spent.** CHROME claimed it, and AUTHOR has it
since the 2026-09-20 priority-seam cut (`work/author/log.md`). It is
next in `work/author/plan.md`'s order, behind AUTH-1.

(At DOCM's exit sweep, `refs` names the PRs `sketch-frame-from-face` stood for: `sketch-frame-from-face` = #1829 — the unit rows left the tracker with `work/docm/`; `docs/DOC-LEDGER.md` sweep 14.)


## The one-submit premise was false, on a ratified page that names this row (2026-09-15, `chrome/citation-repoint`)

Half 1 above treated "a two-node gesture" as *"a real decision about
the commit door"*. It is not one. `crates/editor-core/REFERENCES.md`,
DM1's chrome-consequence bullet, names this row and its sibling by id
and rules the opposite:

> "on a new XY frame" is two inserts in one committed action
> (`commit_action`); "on this face" mints one `FaceFrame` and one
> profile the same way.

`DocSession::commit_action` (`crates/viewer/src/session.rs`) is
shipped: a `Vec<DocEdit>` applied in order, the whole run recorded as
ONE history state (*"one user action is one undo"*), all-or-nothing on
any refusal. `delete_node`'s cascade already uses it. So half 1's "add
a frame datum first" dead end is discharged by two edits through a
door that exists, and the invariant it has to keep is one submit / one
UNDO, which `commit_action` gives it.

Corrected here rather than only in the sibling because this row is
where the claim was first made and the sibling inherited it; a
correction living only in the row that copied it leaves the source
standing.

**Where it stands** (above) said this was unowned residue awaiting a
claim; it is AUTHOR's, which is the claim, and that section now says
so directly.

## A third label site, from AUTH-1's correctness review (2026-09-21)

This row's half 2 is *"the picker names frames by node number"*. The
same defect now has a third site, and a taker should close all three
together rather than discovering the third afterwards.

AUTH-1 (PR 2955) renders the add-datum form's held FACE pick as
`format!("feature {} body {}", node, body)` — **the same text for all
six faces of a box**, and it stays on screen after the selection is
cleared, so an author can commit against a pick nothing in the
viewport is showing and cannot tell from the form which face it is
(the reviewer's NOTE-6). That is this row's complaint exactly, one
door over: a node id where the useful label is what the thing IS.

So the three sites are:

- `add_profile_ui`'s ComboBox — `format!("feature {}", id.0)` for
  every frame of either kind (this row, half 2);
- `tree.rs`'s rows — `"Datum frame"` / `"Datum frame (on face)"`,
  which distinguish the KINDS and never say WHICH frame (this row,
  already);
- and now `add_datum_ui`'s face pick.

**The third one raises this row's value rather than changing it.** A
face frame's honest label is the face it sits on, which this row
already said; AUTH-1 makes that true of a second form and of a pick
that is not even a node. Note the label work also wants
`Display for BlendTarget` (`crates/viewer/src/blend.rs`), whose doc
already argues the sentence belongs in one home — AUTH-1's fix pass
routes its own site through it, so the shape is in the tree to copy.

## The third site has moved (2026-09-21, verified against the tree)

The section above says `add_datum_ui` renders the held face pick as a
hand-rolled `format!("feature {} body {}", node, body)`. **That is no
longer true.** AUTH-1's fix pass (PR 2955, merged `2cf83b500`) routed
it through `BlendTarget::of_face(face).to_string()`
(`crates/viewer/src/pane/create.rs:495`), and the node-number spelling
now lives in `impl Display for BlendTarget`
(`crates/viewer/src/blend.rs:146-150`), which writes
`"feature {} body {body}"`.

**The defect is unchanged and the fix got cheaper.** A person still
reads a node number where they need to know which face; but the
spelling has ONE home instead of two, so fixing that home moves this
site and the blend sites together. The row's own note that the label
work "also wants `Display for BlendTarget`" anticipated this — what it
did not anticipate is that the face site would already be behind it.

Also confirmed unchanged by the same check: site 1 is
`create.rs:55` and `:60` — the `format!("feature {}", id.0)` appears
TWICE, in the closed combo text and in the options — and site 2 is
`tree.rs:215-216`.

Dispatched as **AUTH-3** (`docs/AUTH-3-SPEC.md`, branch
`author/profile-frame`), both halves in one unit because both live in
`add_profile_ui`'s ComboBox and splitting them would put two lanes in
the same widget.

## Closed 2026-09-21 — PR 3023 merged (`58fe4023`)

**Both halves.** The add-profile form offers "a new xy frame" first
and unconditionally, minting the frame and the profile as ONE
committed action and one undo, so the dead end this row was filed
against — *"none in this document — add a frame datum first"* — is
unreachable from the form. And a frame says which frame it is:
`feature 3 — xy at (0, 0, 0) m` in the picker, `Datum frame — xy at
(0, 0, 0) m` in the tree.

**The design calls the spec left open, and how they went.**
`ProfilePlane::{Existing, NewXy}` as an enum on `AddProfile`'s plane
rather than a second op — two ops would be two commits and two undos,
and a distinct op would re-declare the loops and the insert-door
contract and answer three exhaustive matches twice. The label reads
the NODE and never an evaluation, because `tree::rows` draws a row for
every node including the unevaluated and the failed, so an
evaluation-sourced pose goes blank on exactly the rows a person is
diagnosing. When the node cannot say, the label says LESS rather than
guessing: `xy, origin driven` for a non-literal component, the bare
origin for axes that are not the world's, `on feature 4's face` for a
face frame — which says whose and cannot say which.

**Site 3 of the label defect is NOT closed** and is filed as
`face-pick-cannot-name-which-face`. The spec asserted that fixing
`Display for BlendTarget` would move the face-pick site with the blend
sites; the lane falsified it. True of the node half, false of the face
half, and the face half is the whole defect — `BlendTarget` is
deliberately a body scope, and a face's identity is its role path,
which `RoleSeg` has no `Display` for and whose comment rules that
prose never renders it. Both outside AUTHOR's ground.

**What the reviews caught that the unit had not.** The feature-tree
half's label composition was held by nothing — deleting it reddened no
test in 674 — in the unit whose brief warned that a unit-tested helper
is not a wired one; it now has a `pane::headless` drive and four rows.
The create-pane suite never opened the combo, so "the mint goes first
and unconditionally" was unasserted; a mutation making the mint appear
only in empty documents passed all four rows. And two `NewXy` submits
minted two coincident frames, which needed `OpOutcome` to carry
`minted` so the pick settles onto the frame the first submit made —
not the doc fix it was first taken for.

**Residue**: `chrome-calls-one-node-two-names` (the chrome says
`feature N` in widgets and `node N` in refusals, including in two
widgets), `held-face-pick-is-invisible-in-the-viewport`, and rows on
VIEW, CHROME and CIW.
