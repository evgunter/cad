---
id: pick-and-pickindex-are-named-against-their-contents
kind: issue
title: the module called pick holds no picking, and the policy it claims is in pickindex
status: closed
opened: 2026-09-06
refs: [2079, pickindex-holds-the-frames-marks-as-well-as-the-index, renamed-module-leaves-citations-in-three-other-programs]
closed: 2026-09-06
---



Found by the style review of #2079.

## What

`crates/viewer/src/pick.rs:1-2` opens *"Keeping a pick index current
with the session, and refusing a pick when there is none — **the policy
half of picking**"*, and `:21-29` explains that the index is elsewhere
because *"the index and this policy are two layers"*.

The picking policy is not in `pick.rs`. It is in `pickindex.rs`:

- `PickIndex::op_for` / `op_under` (`pickindex.rs:1489`, `:1518`) turn
  a `PickAction` into a `SessionOp` — the miss rule, and what a click
  means;
- `PickIndex::hovered_for` (`pickindex.rs:1280`) is the priority rule,
  *"an edge within `EDGE_PICK_RADIUS_PX` beats the face behind it"*
  (`:1477`);
- `pane/viewport.rs:183` says so from the outside: *"the un-projection,
  the ray service, the miss rule — lives in
  `pickindex::PickIndex::op_for`"*.

What `pick.rs` actually holds is a **cache**: when a build is asked
for, what is done with the answer, and the typed refusal for the window
in between. That is lifecycle, not picking policy — and the split's real
boundary is *stateful, seam-driving* against *pure*, which is a better
boundary than the one the header claims.

The names inherit the mismatch. A reader asking how a pick works opens
`pick`; nothing there answers. `pickindex` reads as *the index*, and it
is the index plus every decision taken over it.

## Confidence

`sure` that `op_for`, `hovered_for` and the priority rule are in
`pickindex` and not in `pick`. `likely` on the naming being worth
revisiting — renaming a module twice in a week is its own cost, and
re-wording both headers may be the whole fix.

## The false sentence is fixed; the rename is DECLINED, and this row is the schedule (#2079 fix pass, 2026-09-06)

The half that was owed is paid. `pick.rs:1-3` no longer claims to be
*"the policy half of picking"*; it now opens *"The pick index's
LIFECYCLE"* and says in its own words that it decides nothing about
picking, that `op_for`, `hovered_for` and the priority rule are
`pickindex`'s, and that **the boundary the split actually landed is
stateful-against-pure**. `pickindex`'s header says the matching thing
from the other side. So no header now asserts the mismatch this item
found.

The rename is declined here, for three reasons in descending weight.

**1. A rename is not a move, and this PR's whole warrant is that it is
a move.** The reviewer certified it with a whitespace-sensitive
sorted-line diff — one line removed, 43 added, all headers and imports;
no signature, `derive`, field or `impl` touched. Renaming `pick` and
`pickindex` rewrites every call site in fifteen source files, twelve
test files and two examples, and that diff is not certifiable the same
way: the check that made this unit safe to read stops working on the
change that would ride it.

**2. The right names are not knowable yet, because the second split is
not taken.** `pickindex-holds-the-frames-marks-as-well-as-the-index`
names ~405 lines — `highlight`, `edge_overlay`, `focus`,
`cursor_projection` and their companions — as a clean second boundary
with a different consumer set. If that comes off, what is left is
genuinely *the index and the cursor questions over it*, at which point
`pickindex` may be the right name after all, and the third module needs
one too. Naming these modules once, after the split that decides what
they hold, is one decision; naming them now is two.

**3. Even the easy half is not clean.** `pick.rs` → `pickcache` would
leave `NotIndexed` and `unindexed` — a refusal vocabulary, not a cache
— in a module named for a cache. The obvious rename does not survive
its own contents either.

**What this row is now**: the schedule, parked behind the second split
rather than behind nothing. A taker should decide the names for both
modules and the third at once.

## Answered by the split: `pick` → `pickcache`, and `pickindex` was right all along (2026-09-06)

The schedule fired. `pickindex-holds-the-frames-marks-as-well-as-the-
index` landed as a pure move in the commit below this one, and the two
arguments that were parked on it expire as predicted — but not with the
answer the parking predicted.

**What the split decided, and it is not what argument 2 expected.**
Argument 2 read: *"what is left is genuinely the index and the cursor
questions over it, at which point `pickindex` may be the right name
after all"*. That is right, and the reason it is right is not the one
offered. The marks coming off did not narrow `pickindex` to *the
index*: `op_for`, `op_under`, the miss rule and `hovered_for`'s
priority rule are all still there, untouched by the move, so the
finding above — *"it is the index plus every decision taken over
it"* — survives the split word for word.

What the split makes visible is that **the complaint was a complaint
about a type having methods.** After the move, every item left in
`pickindex.rs` is `PickIndex`, one of its keys (`PatchId`, `EdgeId`,
`IdMap`), its construction machinery (`PartWindows` and the `DrawnKind`
pair), its errors (`IdMapError`, `PickIndexError`, `PickError`,
`EdgeNameFault`), its answers (`EdgePick`), the two constants its
queries take (`EDGE_PICK_RADIUS_PX`, `PickKinds`), or a private helper
for one of its methods (`Candidate`, `placement`,
`segment_distance_px`, `ray_segment_closest`). The policy is not
free-standing there and never was: it is `PickIndex::op_for` and
`PickIndex::hovered_for`, inherent methods on the type the module is
named for. **A module named for the type whose inherent `impl` is its
spine is named correctly**, and before the split it was not — six
items (`Highlight`, `EdgeOverlay`, `highlight`, `edge_overlay`,
`focus`, `cursor_projection`) were neither `PickIndex` nor about it.
Those are `marks` now. So `pickindex` keeps its name, and this is the
first point at which that could be said with evidence rather than
hoped.

**`pick` does not keep its name, and that half of the finding is
unchanged by everything above.** The module called `pick` still holds
no picking; the split touched nothing in it. With `pickindex`
established as correctly named, `pick` is a squatter on a name whose
subject belongs to a module that already has a better one — so the
answer is not a swap (which would put `PickIndex::scene_focused`, the
drawable-scene builder, in a module called `pick`, a new false sentence
pointed the other way) but a straight rename with no successor:

    pick.rs  →  pickcache.rs        pick  →  pickcache

**Where `NotIndexed`/`unindexed` end up: exactly where they are, and
the objection dissolves on their contents.** Argument 3 held that
`pickcache` strands *"a refusal vocabulary, in a module named for a
cache"*. That reads `refusal` and `cache` as different subjects. They
are not, here. `NotIndexed`'s two arms are `Building` and `Absent`;
`Building` is defined as `PickCache::indexing` and `Absent` as *"no
index, and no build under way — the last attempt refused (its reason is
`PickCache::error`)"*. Its `Display` says *"the picture on screen has
no pick index and none is being built"*, and `unindexed`'s own doc
names `PickCache::indexing` as *"the one value that knows"*. This
refusal exists **because this cache can be empty** and is computed from
nothing but the cache's state. `pickcache` is precisely where a reader
looks for *what happens when the cache holds nothing*. The objection
was written against the word, and the word is not what the module
holds.

**The naming rule the three modules now follow**, stated so the next
module does not have to re-derive it: each is named for the type it is
built around — `pickindex`/`PickIndex`, `pickcache`/`PickCache` — and
`marks`, which has no spine type (two small value types and five free
functions), is named for what it produces.

**Why this is a separate commit from the move.** Argument 1 does not
expire and was never claimed to: a rename is not a move, and the
sorted-line certification that makes a move readable does not survive
call-site churn. So the move landed alone and certified (34 lines
removed, 75 added, no code line among them, 144 declarations identical),
and this rename rides its own commit where a reviewer reads it as what
it is. The churn is ~40 references across five source files, one test
file and `lib.rs` — an order of magnitude smaller than the fifteen-
source-file rewrite argument 1 was weighing, because that estimate was
dominated by `pickindex`'s surface and `pickindex` is not being renamed.

`work/view/renamed-module-leaves-citations-in-three-other-programs` is
the residue this rename creates outside VIEW's fence.

## Closed
