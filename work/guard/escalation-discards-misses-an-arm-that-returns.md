---
id: escalation-discards-misses-an-arm-that-returns
kind: issue
title: "escalation-discards.sh misses an arm that returns on an escalation: `Err(_) => return …`"
status: open
opened: 2026-11-14
priority: P2
---



Found by PR 6949's review: the gate did not see an escalation dropped unread in `crates/sweep/src/skin.rs`'s `still`, which read `Ok(_) | Err(_) => return Ok(false)`. PR 6949 rewrites it as an explicit `Err(Indeterminate { .. }) => false`, with the reason at the site.

**The blind spot.** `scripts/gates/escalation-discards.sh` counts an arm on one line whose pattern holds `Err(_)` only when its body is `continue`, `break`, `{}` or `()`, the arms a search moves on by. Its header lists its other blind spots:
- `Err(_e)`, `_ =>` and multi-line arms;
- the value discard (`=> None`, `=> false`), which goes to `work/onecopy/value-discards-read-a-refusal-as-nothing-found.md`.

It does not name an arm whose body RETURNS: `Err(_) => return Ok(false)` or `=> return None`. That arm leaves the function on the refusal, unread, just as a search's `continue` moves past it.

**Owed.**
- Either widen the pattern to `return` bodies and re-sweep the register, or name the shape under "WHAT IT CANNOT SEE" with why it is out of scope (perhaps beside the value discards).
- Then sweep for it, and record each hit's disposition in `escalation-discards.counts`.
