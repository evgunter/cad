---
id: refuse-roster-and-chrome-labels-header-predate-the-disabled-reason-doors
kind: issue
title: The refuse-module roster in the viewer README and chrome_labels.rs's header both predate the disabled-reason doors
status: open
opened: 2026-09-20
priority: P4
cost: E
---

Filed by VNEWS's `vnews/app-controls-read-their-refusals`, which is the
diff that falsified both sentences. **Filed rather than fixed** because
`crates/viewer/README.md` and `crates/viewer/tests/*` are named
specifically in VNEWS's `keep_out` — *"a prose or census or citation
defect found here is filed on vdoc and never fixed across that fence"*
— and `work/vnews/plan.md` §Dispatch rules settles that the carve-out
beats the general rule that prose one's own diff falsified is one's own
to repair.

## Two sentences, one cause

The unit added two predicate doors to
`crates/viewer/src/session/refuse.rs` — `Refusal::nothing_to_step` and
`Refusal::empty_name` — plus the `Step` payload enum, so that the
toolbar's Undo, Redo and Create buttons gate on, and show the words of,
the same refusal value their doors raise.

**1. `crates/viewer/README.md`, the `session::refuse` row of the
vocabulary table** (~`:341`) enumerates the module's members:

> `Refusal` with its `rank`/`preferred` ladder, its `Display`, and the
> recourse composers `affordance`/`exists_wording`/`offer_wording`;
> `NodeKindWanted` and `admits`, since they are a `Refusal` payload and
> its predicate

It was already incomplete before this unit — `Refusal::self_instance`
has never been in it, and it is the tree's canonical statement of
exactly the rule the new doors follow. It is now short by `Step`,
`nothing_to_step` and `empty_name` as well. The sentence's own shape
tells VDOC what to do with them: `Step` is there *"since"* it is a
payload and `nothing_to_step` is its predicate, which is the clause the
row already carries for `NodeKindWanted`/`admits`.

**2. `crates/viewer/tests/chrome_labels.rs`'s module header** says:

> Two of the names a user reads are pure functions of state rather than
> pixels, so they are pinned here: the toolbar's name for the open
> document, and the initial layout's shape. The rest of the chrome's
> wording lives inside widget calls and is not testable without a
> window; this suite claims only what it can see.

The unit added a third row to that suite,
`a_disabled_toolbar_control_says_what_its_own_operation_refuses`, and
it is exactly a name a user reads that has become a pure function of
state: three controls' disabled reasons, read off
`Refusal::nothing_to_step` / `Refusal::empty_name` rather than composed
at the widget. So "two" is short by one subject and "the rest … is not
testable without a window" is now false of the class the new row holds.

**What the repair is not.** The header's real claim — *this suite
claims only what it can see* — is still true and is the reason the file
is worth having; what moved is which wording a test can see, and the
count in front of it. A repair that only bumps "two" to "three" leaves
the second sentence saying the thing the new row disproves.

## Where the evidence is

- `crates/viewer/src/session/refuse.rs` — `Step`,
  `Refusal::nothing_to_step`, `Refusal::empty_name`,
  `Refusal::self_instance`, and the module header, which this unit DID
  update because that file is VNEWS's own ground.
- `crates/viewer/tests/chrome_labels.rs` — the new row and the header
  above it.
- `crates/viewer/README.md` — the `session::refuse` vocabulary row.
