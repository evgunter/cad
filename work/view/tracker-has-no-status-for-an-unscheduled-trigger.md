---
id: tracker-has-no-status-for-an-unscheduled-trigger
kind: ruling
title: A row whose trigger is neither an item nor a PR has no honest status: parked lies and open overstates
status: open
needs_ev: true
opened: 2026-09-04
refs: [pick-priority-filter-vocabulary, session-shims-and-test-imports]
---

## The question

`work/README.md` gives `parked` one meaning — *waits on a named
trigger* — and enforces it by requiring `blocked_on` to be non-empty,
with every id resolving to a file and every int read as a PR or issue
number. So a row may be parked only on **an item or a PR**.

Three rows in two programs now wait on something that is neither, and
each had to pick between two false statuses:

1. **`pick-priority-filter-vocabulary`** (VIEW). Its trigger is a
   third asymmetric tool — a vertex pick — that does not exist and is
   not scheduled, and `crates/viewer/README.md` GQ7 **ratifies the
   deferral**. There is no item to name, so it is `open`, which says
   it is available to dispatch. It is not.
2. **`session-shims-and-test-imports`** (VIEW). Its blocker closed on
   2026-09-04 and the board went on carrying it as parked behind a
   closed item for a day. What it now waits on is a cross-program
   reach — CHROME owns `crates/viewer/tests/*` and has no item for the
   sweep — which is a decision, not a row. Un-parked 2026-09-04 for
   want of anything truer.
3. **CHROME's nine rows** parked on `viewer-session-god-module-split`,
   which closed the same day. `lint` does not object: a closed
   `blocked_on` resolves fine, so nothing goes red when a trigger
   fires and the rows keep reading as blocked.

The failure mode is the same in all three and it is quiet: a status
that has gone false is indistinguishable from one that is still true,
so it is believed unread. Both VIEW rows were.

## Why this is a ruling and not work

The vocabulary is `work/README.md`'s, and the fix is a change to the
tracker contract every program reads. Four shapes, and the choice is
Ev's:

1. **A prose trigger.** Let `blocked_on` carry a free-text reason
   alongside ids, so *"a vertex-pick tool exists"* is sayable. Cheapest;
   loses `lint`'s reference-resolution for that entry, and a prose
   trigger is exactly the thing nothing can check.
2. **A `deferred` status**, distinct from `parked`: ratified as
   not-now, with the ratification cited and no `blocked_on` required.
   Says what (1) says without weakening `blocked_on`; adds a fifth
   status to a vocabulary that is currently small enough to hold in
   the head.
3. **File the trigger as an item.** A `ruling` or an issue for "the
   vertex-pick tool", parked on itself forever. Keeps the vocabulary
   as it is and makes the board carry rows nobody intends to do, which
   is the thing `work/` exists not to do.
4. **Nothing** — accept that these are `open` rows whose first
   paragraph is the truth, and rely on a reader opening the file.
   That is the status quo, and it is what happened here.

## The adjacent defect, whichever way this goes

**`lint` accepts a `parked` row whose `blocked_on` names a CLOSED
item.** That is a trigger that has fired, and it is mechanically
checkable today with no vocabulary change at all: all nine of the
CHROME rows would have gone red on the commit that closed the split
(measured, see the correction below).
Whether that check should WARN or BLOCK is the only question in it —
a program closing an item would otherwise have to un-park other
programs' rows in the same PR, which `work/README.md`'s one-file-one-item
rule makes a merge conflict by design. A warning names the rows and
leaves the un-parking to the owner.

This half does not need the ruling above and can land first.

## Corrected — the counts are measured now (2026-09-04)

The two counts above were estimated when this file was written this
morning. They are now measured, by running the check that this item's
adjacent-defect half asks for against the tree:

- *"three of the nine CHROME rows would have gone red"* — it is **nine
  of nine**. Every row `work.py lint` flags is a CHROME row parked on
  `viewer-session-god-module-split`, and each is parked on that and
  nothing else, bar `parameter-row-field-has-no-text-door`, which also
  waits on `doc-param-unit-edit-has-no-door` (open) and so is still
  genuinely blocked.
- The list of three rows above names `session-shims-and-test-imports`
  (VIEW). That row was un-parked later the same day and is `open` now,
  so it does **not** trip the fired-trigger check. It remains a live
  instance of the vocabulary gap this ruling is about — a row whose
  trigger is a cross-program decision, not an item — but the
  fired-trigger count is nine, all CHROME's.

Only these factual claims are corrected. The four candidate shapes
above are unchanged: they are the question, and they are Ev's.

Signed: (VIEW orchestrator)
