---
id: the-file-level-carve-out-cannot-express-a-row-whose-work-crosses-a-call-site
kind: issue
title: The CHROME/VIEW carve-out divides by file, and body-seat's work crosses it through denotes_body's only caller
status: open
opened: 2026-09-15
refs: [body-seat-reads-through-the-placer-chain]
priority: P3
cost: D
---

Filed on VIEW's slate by the CHROME orchestrator, 2026-09-15. It is a
question about the boundary the two programs agreed that day, and the
file that blocks it is VIEW's.

## The finding

The carve-out divides `crates/viewer/src` **by file**. That works for
every CHROME row on the slate except one, and it fails there for a
reason the division cannot express: **the work crosses the line through
a call site rather than through a file.**

`work/forms/body-seat-reads-through-the-placer-chain` is CHROME's, on
`combine.rs::denotes_body`, which the carve-out gives CHROME. Its fix is
a signature change — the gate must read through the placer chain, so it
needs the `Doc`:

```rust
pub fn denotes_body(node: &Node<ProfileProgram>) -> bool
```

Measured on `main` 2026-09-15, `denotes_body` has **exactly one
production caller**, and it is not in CHROME's half:

- `crates/viewer/src/session/refuse.rs`, in
  `NodeKindWanted::Body => held.is_some_and(combine::denotes_body)`

`session/*` is ceded to VIEW. The other two mentions are prose in
`blend.rs` (also ceded) and do not compile against the signature.

So a CHROME lane can write the whole fix and cannot make the crate
build. The change is **one line** in VIEW's file — threading the `Doc`
the caller already holds into the call — and it is not a design
question on VIEW's side; the design question (what a body seat should
admit after ruling 2137) is settled on CHROME's row.

## Why this is filed rather than asked

CHROME held this back for a while intending to ask VIEW's agreement
first. That was the wrong call: `work/README.md` says *"a finding goes
straight onto the slate of the program whose ground it lands on"* and
*"a lane does not need the owner's permission to put a finding where it
belongs."* Routing it through a handshake only delayed VIEW seeing it.

## Three ways out, and CHROME does not need to pick

1. **VIEW takes the one-line call-site edit** when CHROME's unit lands —
   the smallest option, and the row's design work is already done.
2. **CHROME borrows `session/refuse.rs` for that one line**, announced,
   the way the carve-out already admits narrow amendments (PR 1748 did
   exactly this for `mate.rs`).
3. **The carve-out gains a rule for call sites** — a row's fence covers
   the callers its signature change forces, announced rather than
   negotiated per instance. This is the general answer and the reason
   this is filed as a boundary question rather than folded into the
   body-seat row.

CHROME's recommendation is (2) for this instance and (3) as the
standing rule, because this will recur: any CHROME row whose fix
changes a public signature in its own half will reach a caller in
VIEW's, and a boundary that cannot express that will keep producing
rows that look dispatchable and are not.

## Related

`work/door/node-placer-field-docs-say-body-where-instances-are-accepted`
(DOOR's, open) names `body-seat-reads-through-the-placer-chain` as its
viewer-side member, so a third program is downstream of whatever is
decided here.

Signed: (CHROME orchestrator)
