---
id: python-edit-error-carries-no-handle-for-an-unnamed-variable
kind: issue
title: Python's EditError names a variable only by name, so a refusal about an unnamed one carries nothing to act on
status: open
priority: P2
cost: M
opened: 2026-10-09
refs: [4463]
---

`EditPayload` (`crates/pncad-py/src/edit_payload.rs`, `EditPayload`) carries the variable a refusal is about only as `param: Option<&VarName>`, so every arm whose subject is a variable crosses to Python with `param = None` when that variable has no name. About fifteen arms do this: `VarNameUnchanged`, `AnonymousVarUnread`, `SharedVarNeedsName`, `DeleteAnonymousVar`, `VarKindFixed`, `NonFiniteVar`, `InvalidDistribution`, `VarIsAnOutput`, the `Definition*` family, `SlotUnresolvedVar` and `PayloadUnresolvedVar` (the arms in `edit_payload`'s match that set `param: var.name()`).

The one that matters most is `shared_var_needs_name` (FORK-7): its recourse is "name it", and the Python caller gets no `Var` to pass to `DocEdit.rename_var` beyond the hex tag in the message. Today a caller recovers it because they passed the `Var` themselves (`crates/pncad-py/tests/test_slot_variables.py`, `test_an_unnamed_var_passed_twice_refuses_until_it_is_named`). That stops working when the shared variable sits inside a formula the caller did not author, such as a definition read by a path edit.

The fix is a `var: Var | None` attribute on `EditError`, set wherever the kernel arm carries a `SpokenVar`. It needs a new `EditPayload` field, a `presence` row, the pyi docstring, and the `carries` rows in `src/tests.rs` (around `carries(&E::AnonymousVarUnread …)`). `SnapshotError`'s Python payload has the same gap.

- 2026-11-03 — moved from LIB to PYDOOR by `git mv` when LIB closed, id and body unchanged (LIB had three P1 rows left behind 124 closed ones; its library-door rows went to PYDOOR, opened for them, and its tag, stub, census and projection rows to BIND; Ev, in chat).
