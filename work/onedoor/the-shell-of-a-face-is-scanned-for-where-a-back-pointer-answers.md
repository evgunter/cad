---
id: the-shell-of-a-face-is-scanned-for-where-a-back-pointer-answers
kind: issue
title: boolean/ops.rs scans every shell's face list for the shell of a face, where Face::shell answers in one lookup
status: closed
opened: 2026-09-19
priority: P1
cost: E
closed: 2026-11-03
pr: 6433
branch: onedoor/one-spelling-trio
---



## Finding

- **Where**: `crates/topo/src/boolean/ops.rs`'s `shell_of` closure
  (~:1996), in the re-cut rotation pass.
- **Importance**: low
- **Confidence**: sure. Read in full; `Face::shell`'s meaning
  confirmed against `Body::solid_of_face` (~:892) and
  `validate.rs`'s pass that checks it.
- **Raised by**: the `solid_of_face` fold's fix pass, 2026-09-19,
  filed on curved because `scripts/work.py territory` puts
  `crates/topo/src/boolean/ops.rs` on curved's ground.

```rust
let shell_of = |face: FaceKey| -> Result<ShellKey, BooleanError> {
    src.shells()
        .find(|(_, sd)| sd.faces.contains(&face))
        .map(|(k, _)| k)
        .ok_or(corrupt("re-cut representative face has no shell"))
};
```

`Face::shell` is a **back-pointer**: `src.get_face(face)` answers the
same question in one arena lookup. This scans every shell of the
operand and, per shell, every face key it lists — quadratic in the
operand where the answer is stored on the face. The two are equivalent
on a body that validates, and `validate.rs` pass 10 is what says so:
the shell that lists a face and the shell that face names are checked
against each other on every validated body.

The comment above it — *"Shell of each group's representative face,
arena order"* — says the ownership list's ORDER is what is wanted. That
is worth reading before the swap: if a face were (invalidly) listed by
two shells, the scan answers with the first in arena order and the
back-pointer answers with the one the face names. On a body that
validates there is no such face; on one that does not, the refusal
`corrupt("re-cut representative face has no shell")` would become a
panic naming the face's dangling `shell` link (D2 row 4) instead, which
is a posture change and not a mechanical substitution.

**Not a member of the face → shell → solid class** — it stops at the
shell, and it is the only site in the tree that spells this hop the
reverse way. The census that found it is recorded on
`work/dup/solid-of-face-has-eleven-hand-written-walks-outside-it.md`.

## Closed (PR 6433, 2026-11-03)

`apply_recuts` now reads `Face::shell` through `live::proven`, and the shell through `live::linked`. The operand's solid read moves the same way, through `live::proven`. All three are row 4, since the operand is an at-rest body that every door keeps tier-1-valid. The face↔shell check is validate's pass 7, not pass 10.
