---
id: join-desync-renders-a-caller-arity-error-as-a-kernel-lockstep-bug
kind: issue
title: topo: BooleanError::JoinDesync renders a graft caller's arity error as 'A/B lockstep invariant violated (kernel bug or corrupt reduction)'
status: open
opened: 2026-10-13
priority: P3
cost: E
---


## Finding

`BooleanError::JoinDesync`'s `Display` (`crates/topo/src/boolean/mod.rs`) renders every `what` as "A/B lockstep invariant violated: {what} (kernel bug or corrupt reduction)". The graft doors raise it for caller errors: `combine.rs` "graft needs exactly one destination solid per source solid" (its own comment: "a count mismatch is a caller error"), `combine.rs` and `instance.rs` "graft source holds no solid to graft", and `instance.rs` "graft source does not hold exactly one solid" from the public `graft_disjoint`. The rendered sentence blames the kernel for the caller's input.

Same shape as band's `BodyNotIntact` (PR 5021). Found by that PR's §5 sweep.

## Slate triage (2026-10-26, CLEAVE)

- Moved to HONEWORD (`work/honeword/`) by `git mv`, id and body unchanged: whose charter is split and boolean refusal text, payloads and endings. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
