---
id: parked-on-an-int-is-invisible-to-the-fired-trigger-rule
kind: issue
title: A parked row whose blocked_on is a PR/issue number is invisible to the fired-trigger rule
status: closed
opened: 2026-09-06
closed: 2026-09-11
---


**Filed by the LIB orchestrator, 2026-09-06, onto META's slate per
Ev's 2026-09-04 ruling (file straight onto the owner).**

`work/README.md`'s fired-trigger rule ("every blocker closed — a lint
ERROR") resolves item ids only; `blocked_on` also admits ints, which
are PR or issue numbers and "are not checked". A row parked on an
int therefore never reds when its trigger fires. The instance:
`work/lib/LIB-G17.md` sat `parked` on `blocked_on: [1202]` for two
days after `work/shell/shell-needs-shellnaming-birth-channel.md`
(the tracker file for that number, `github: 1202`) closed on
2026-09-04, and was found by a SEAT lane reading by eye
(`work/lib/lib-g17-is-parked-on-a-fired-trigger.md`, now closed).

Two shapes of fix, META's to choose: (a) lint resolves an int in
`blocked_on` against every item's `github:` field and applies the
fired-trigger rule when the match is closed, treating an int that
matches nothing as unchecked as today; (b) lint refuses an int in
`blocked_on` on a `parked` row when an item with that `github:`
exists, telling the author to name the item. (a) catches the case
silently-later, (b) at filing time.

## Closed (2026-09-11)

**Shape (a) taken — lint resolves an int through `github:` — with one
narrowing the item did not name: it warns, it does not error.**

What landed in `scripts/work.py`: a number in `blocked_on` that matches
exactly one item's `github:` is read as that item, and if the item is
closed the row is named, in both of the fired-trigger rule's shapes
(nothing else gating it, and a fired entry beside a live one). A number
matching no `github:` stays unchecked, as the item asks.

**Why a warning and not the error the id case gets.** The author wrote
a number and the tracker matched it to a row: a naming is a claim about
that row, a match is an inference about it. An inference is allowed to
name a row and is not allowed to red `main` for a program that cannot
see how it was drawn. `work/README.md` now says this at the rule. The
fix the warning asks for — name the item instead of the number — puts
the row back under the error, which is the right end state and the
author's to choose.

The cost of being wrong is not symmetric either: the error's cost is
already on file (`work/README.md`: a closing PR can red `main` for rows
it does not own, accepted deliberately by Ev on 2026-09-04), and paying
it a second time for an inferred match, on a program whose `keep_out`
forbids it from fixing the row it just reddened, is not the same trade.

**Two true positives on the first run**, both `work/code-quality/`'s and
both routed there rather than fixed across the fence:

- **`S190`** — parked on `[855]`; `work/fix/census-decline-consults-one-face-of-pair.md`
  carries `github: 855` and closed on 2026-09-04. Same subject (the
  pair-carrying `CensusUnsupported`), so the trigger has genuinely
  fired and nothing else gates the row. Under the id spelling this
  would be the ERROR.
- **`S79`** — parked on `[757, 758, 759]`; `#759` resolves to
  `facade-polygon-door-demoted-without-replacement`, closed. Still
  waits on `#757` and `#758`, so only the entry is stale.

**Two ambiguous mappings found in the same pass**, and they are why the
resolution refuses a number claimed twice rather than picking the first
writer: `github: 1374` is on both `work/chrome/add-profile-placement-on-picked-face-frame.md`
(open) and `work/docm/sketch-frame-from-face.md` (closed) — where
picking wrongly would invent or suppress a fired trigger — and
`github: 1607` on both `work/ciw/render-lanes-red-at-missing-merge-ref.md`
and `work/issues/render-lanes-checkout-merge-ref-vanishes.md` (both
closed, one `refs:` the other: a duplicate filing). Each is named by its
own warning. Both are other programs' rows; routed, not edited.

**Shape (b) is not foreclosed** and reads better once the tree is clean:
refusing an int on a `parked` row when an item carries that `github:`
catches it at filing. It is worth doing after the rows above are fixed,
not before — today it would refuse two rows whose correct spelling
nobody has written yet.
