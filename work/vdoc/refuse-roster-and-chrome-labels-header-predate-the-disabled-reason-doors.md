---
id: refuse-roster-and-chrome-labels-header-predate-the-disabled-reason-doors
kind: issue
title: chrome_labels.rs's header was already false at its own merge base, and the viewer README's refuse roster is short by four members
status: open
opened: 2026-09-20
priority: P4
cost: E
---

Filed by VNEWS's `vnews/app-controls-read-their-refusals` (#2960).
**Filed rather than fixed** because `crates/viewer/README.md` and
`crates/viewer/tests/*` are named specifically in VNEWS's `keep_out` —
*"a prose or census or citation defect found here is filed on vdoc and
never fixed across that fence"*. Both defects below are **older than
that diff**; an earlier draft of this item said the diff caused the
first one and that was wrong, so the cause is stated plainly here
rather than left for VDOC to re-derive.

## 1. `crates/viewer/tests/chrome_labels.rs`'s header, already false

The header says:

> Two of the names a user reads are pure functions of state rather than
> pixels, so they are pinned here: the toolbar's name for the open
> document, and the initial layout's shape. The rest of the chrome's
> wording lives inside widget calls and is not testable without a
> window; this suite claims only what it can see.

**The second sentence was false before this item was filed and before
#2960 touched anything.** Two headless egui harnesses already existed,
and one of them lays out *this very toolbar*:

- `crates/viewer/src/app.rs`'s `toolbar_row` runs the real
  `ViewerApp::toolbar_ui` through an `egui::Context::default()` with no
  window at all, for the two wrapping rows.
- `crates/viewer/src/pane/view.rs` injects real `egui::Event::Key`
  values and reads the draft that results.

So "not testable without a window" is not a property of the chrome's
wording; it is a property of what anybody had tried. #2960 is the
demonstration — it reads `on_disabled_hover_text`'s words back off the
painted frame — but it is evidence, not cause.

**What the repair is not.** Bumping "two" to "three" leaves the second
sentence saying the thing the harnesses disprove, and the second
sentence is the load-bearing one: it is the reason a reader stops
looking for chrome rows here. The honest version says what this suite
claims and why, without a claim about what is reachable.

## 2. `crates/viewer/README.md`'s `session::refuse` roster, short by four

The vocabulary table's row (~`:341`) enumerates the module's members:

> `Refusal` with its `rank`/`preferred` ladder, its `Display`, and the
> recourse composers `affordance`/`exists_wording`/`offer_wording`;
> `NodeKindWanted` and `admits`, since they are a `Refusal` payload and
> its predicate

It was already short by `Refusal::self_instance` — which is the tree's
canonical statement of the very rule the module's newer doors follow —
before #2960, and is now short by `Step`, `Refusal::nothing_to_step`
and `Refusal::new_document_name` as well. The row's own shape says what
to do with `Step`: it is there *"since"* it is a payload and
`nothing_to_step` is its predicate, which is the clause the row already
carries for `NodeKindWanted`/`admits`.

## Where the evidence is

- `crates/viewer/tests/chrome_labels.rs` — the header.
- `crates/viewer/src/app.rs` — `toolbar_row`, and (from #2960)
  `a_disabled_toolbar_control_says_what_its_own_operation_refuses`.
- `crates/viewer/src/pane/view.rs` — the key-event harness.
- `crates/viewer/src/session/refuse.rs` — `Step`, `nothing_to_step`,
  `new_document_name`, `self_instance`, and the module header, which
  #2960 DID update because that file is VNEWS's own ground.
- `crates/viewer/README.md` — the `session::refuse` vocabulary row.
