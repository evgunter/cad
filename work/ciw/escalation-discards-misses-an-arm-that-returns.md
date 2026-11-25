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

Moved from GUARD to CIW on 2026-11-20, by `git mv`, when GUARD merged into CIW: GUARD held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CIW carries.

**Three more shapes the gate does not see** (LATHE, 2026-11-25, PR 7575's
review). Each passes over a refusal unread, and none matches a one-line
`Err(_) => continue | break | {} | ()` arm:
- **a closure that discards its argument**, `.map_err(|_| …)`:
  `crates/topo/src/merge_faces.rs`, `curved_outline_ring`, read a
  pcurve mint refusal, escalations included, as `PeriodClosure`. PR 7575
  now carries the refusal as `MergeCoplanarError::Pcurve { source }`.
- **`.ok()?`**: `crates/topo/src/boolean/ops/interior_cuts.rs`,
  `loop_cut_in`, read the section's and the march's escalations as "no
  cut". PR 7575 now escalates them, and reads only a structural refusal
  as no cut.
- **a tuple or or-pattern arm**, where `Err(_)` is not first in the
  pattern or is beside an `Ok` arm. Two sites stand, each with its reason:
  - `interior_cuts.rs`, `cut_in`'s plane arm, `Ok((NonzeroSign::Negative, _)) | Err(_) => None`:
    a ball against a plane read in band takes no cut, and the guard's
    refusal stands. It is a value discard of the onecopy row's kind.
  - `crates/topo/src/boolean/ops.rs`, `boolean_op_recut`'s
    `(Err(_), Some(refusal)) => return Err(refusal)`: escalations take
    the arm ahead of it, so it maps only a structural cut-in refusal to
    the section pass's.

  Neither is counted in `escalation-discards.counts`.

Owed beside the above: widen the pattern to these shapes, or name them
under "WHAT IT CANNOT SEE", then re-sweep.
