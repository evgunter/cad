---
id: rustfmt-does-not-reach-a-macro-wrapped-declaration-block
kind: issue
title: rustfmt does not format a macro invocation's body, so a macro-wrapped declaration block has no formatting gate
status: open
opened: 2026-09-13
priority: P4
cost: E
---


Disclosed by WIRE (PR 2501) at the moment the first instance was
created, and filed as the class because nobody has swept for the rest.

## Finding

**rustfmt does not format the body of a `macro_rules!` invocation.**
Measured by the reviewer of PR 2501 on a scratch file: the same enum
reformats when it sits at module level and is left byte-identical
inside an invocation.

PR 2501 declares `editor-core`'s four document vocabularies —
`ProgramTarget`, `ProgramStep`, `ProgramArcData`, `LoopProgram` —
through one `document_vocabulary!` invocation, so roughly 220 lines of
`crates/editor-core/src/program.rs` (the crate's central payload types)
are now outside rustfmt's reach. It is already visible in the file: the
enums sit at column 0 inside the invocation and the invocation closes
as a bare `}` under the enum's own.

**No gate will ever report drift there.** The PR gate's `rustfmt` step
and `local-scripts/ci-local.sh` both pass over it silently, which is the
property that makes this CIW's rather than a style note: a gate that
cannot see a region reports the same green whether the region is clean
or not.

## Why PR 2501 took the trade anyway

The macro closes a **silent** failure class — a document vocabulary
variant that reaches no census, measured green under a text-scanning
anchor with only `#[doc(hidden)]` added. Formatting drift inside the
block is **visible** to any reader of the diff. Trading a silent failure
for a visible one is the right direction; it is still a cost, and it was
not flagged when the macro landed.

## The class, unswept

Every `macro_rules!`-wrapped declaration block in this tree has the same
property. `profile`'s `transition_table!`, `arc_modes!` and
`target_forms!` are the obvious neighbours — they wrap the kernel step,
mode and target vocabularies the same way and predate this PR — and
nobody has enumerated the rest. **The sweep is the work here**, not the
first instance: a count of how much declaration text in this tree sits
outside the formatter, and then a decision about whether that wants a
gate of its own.

## Shape of a fix, if one is wanted

- A gate that extracts macro-invocation bodies and formats them
  separately, which needs a reader that can find them — the shared
  Rust lexer in `crates/test-utils/src/source.rs` already locates
  balanced bodies.
- Or accept it per site and require the disclosure at each, which is
  what PR 2501 does in `DOCUMENT_VOCABULARIES`'s doc.

Either way the number wants measuring first.
