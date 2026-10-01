---
id: split-hands-out-a-body-without-running-tier-3
kind: issue
title: split_direct runs no tier-3 check on its output, so an invalid half is handed out silently (shell.rs's verb validates its own output)
status: open
opened: 2026-09-28
priority: P2
cost: E
---


Found by CONTACT-6's reviewer (2026-09-28). `splitting/mod.rs`,
`split_direct` (~735), hands its halves out without
`validate_geometric`. At base, a cut through a reversed-sense cavity
wall shipped a half failing `LoopRoleInverted`, and point-in-solid then
answered 1,274 false `Out` inside it (CONTACT-6). `shell.rs` closes its
own verb with `validate_geometric`, which is the precedent. No sentence
in `split`'s docs says that callers validate. Decide whether split
validates its own output, and what it costs.
