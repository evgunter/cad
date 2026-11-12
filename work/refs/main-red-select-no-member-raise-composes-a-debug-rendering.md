---
id: main-red-select-no-member-raise-composes-a-debug-rendering
kind: issue
title: main red: select.rs's NoMember refusal composes {member:?} into its message (PR 6555), and the prose census allows no Debug raise
status: open
opened: 2026-11-10
priority: P1
cost: E
---

**What fails.** `pncad-py` `prose_census::tests::no_raise_site_composes_a_debug_rendering_into_its_message` (`crates/pncad-py/src/prose_census.rs:2495`) is red on main `343823b7`, and `select.rs` is unchanged at `328f3671`. It finds `{("crates/pncad-py/src/py/select.rs", "message")}`, and `KNOWN_DEBUG_RAISES` (`prose_census.rs:2088`) is empty.

**Cause.** PR 6555 (`reads/select-indexed-member`, commit `d062407f`) added the `R::NoMember { member, .. }` arm at `crates/pncad-py/src/py/select.rs:861`. Its message is `format!("the read holds no member at {member:?}: …")`, which composes a `Debug` rendering into the raised message. That is the route to the panic the census guards.

**Fix.** Render `member` through its prose (a `Display`/sentence renderer for the member index, or the index spelled as the user wrote it, `xs[i]`) instead of `{member:?}`. Do not list the site in `KNOWN_DEBUG_RAISES`: the allowance is for a site that cannot do better.

Found by the ONEHOME stage-2 lane's full-suite run. Stage 2 does not touch `crates/pncad-py`.
