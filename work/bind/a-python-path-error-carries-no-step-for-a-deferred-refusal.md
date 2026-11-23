---
id: a-python-path-error-carries-no-step-for-a-deferred-refusal
kind: issue
title: A Python PathError carries no step, so a chain user cannot tell which step a deferred refusal is about
status: open
priority: P2
cost: E
design: true
opened: 2026-11-21
---


Found by PROFILE's fix to fillet refusal attribution (PR 7252, branch
`profile/fillet-refusal-names-its-step`).

`profile::PathError` gains `Deferred { step, raised_at, deferral,
refusal }`, a refusal a later step raises about an earlier one: a
fillet resolved where its arrival is fixed, or a fillet arc's stored
form re-read where the loop closes. `PathError::step()` gives the step
it is about, the one whose radius the recourse edits.

The Python chain door's exception (`crates/pncad-py/src/py/path.rs`,
`path_err`) exposes `variant` (the inner refusal's tag, through
`kind()`) and `corners` (peeled through `refusal()`). It exposes no step.
The message does name both steps ("step 3's fillet, not resolved at
step 5: …"), so a reader can see them. A program cannot: it would have
to parse the sentence. The binding census lists `PathError::Deferred` as
bound by `PathError.variant` (`tests/test_binding_census.py`), and that
holds only for the refusal's class.

## What a fix has to decide

Whether the Python `PathError` grows a `step` attribute, `None` for a
refusal that is the raising step's own and the earlier step for a
deferred one, and with it `raised_at` and the `deferral` tag. The
typed chain records its steps (`.step` on every chain state), so the
index has a meaning there.
