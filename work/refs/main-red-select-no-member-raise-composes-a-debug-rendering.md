---
id: main-red-select-no-member-raise-composes-a-debug-rendering
kind: issue
title: main red: select.rs's NoMember refusal composes {member:?} into its message (PR 6555), and the prose census allows no Debug raise
status: closed
closed: 2026-11-12
opened: 2026-11-10
priority: P1
cost: E
---

**What fails.** `pncad-py` `prose_census::tests::no_raise_site_composes_a_debug_rendering_into_its_message` (`crates/pncad-py/src/prose_census.rs:2495`) is red on main `343823b7`, and `select.rs` is unchanged at `328f3671`. It finds `{("crates/pncad-py/src/py/select.rs", "message")}`, and `KNOWN_DEBUG_RAISES` (`prose_census.rs:2088`) is empty.

**Cause.** PR 6555 (`reads/select-indexed-member`, commit `d062407f`) added the `R::NoMember { member, .. }` arm at `crates/pncad-py/src/py/select.rs:861`. Its message is `format!("the read holds no member at {member:?}: …")`, which composes a `Debug` rendering into the raised message. That is the route to the panic the census guards.

**Fix.** Render `member` through its prose (a `Display`/sentence renderer for the member index, or the index spelled as the user wrote it, `xs[i]`) instead of `{member:?}`. Do not list the site in `KNOWN_DEBUG_RAISES`: the allowance is for a site that cannot do better.

Found by the ONEHOME stage-2 lane's full-suite run. Stage 2 does not touch `crates/pncad-py`.

## Closed

`select.rs`'s `select_refusal`, the `R::NoMember` arm, now spells the index values the way the kernel's `MemberOutOfRange` sentence does (`eval/mod.rs`): each value through `u32::to_string`, joined by `", "` inside brackets. The message reads `the read holds no member at [3]: …`, the same text as before, built without a `Debug` rendering. `KNOWN_DEBUG_RAISES` stays empty. No `Display` for a member index exists to reuse: `IndexedRead.__repr__` (`py/doc.rs`) and the kernel spell `[a, b]` inline.

Pins: the census test `no_raise_site_composes_a_debug_rendering_into_its_message` is green. `test_pattern.py`'s `test_a_member_answers_its_own_names`, which already reached `no_member`, now also asserts `no member at [3]:` in the raised text.

Sweep of `crates/pncad-py/src/` for `:?}` and `:#?}` (115 hits, plus `Debug::fmt` and `dbg!`, which have none): no other one reaches `typed_err`. In `py/`, 12 hits are in raised messages. All of them quote a caller's `&str` through `PyValueError`/`PyAttributeError::new_err`, not `typed_err`, so the prose assertion never reads them. The rest are `__repr__`, `__hash__`, an `unreachable!`, and test or census assertion text. No row filed.
