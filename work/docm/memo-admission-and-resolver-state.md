---
id: memo-admission-and-resolver-state
kind: issue
title: The memo serves instantiate nodes without the seam's gates - should memo admission know about resolver state?
status: open
opened: 2026-08-29
github: 1185
refs: [1176, 1192]
---

## From GitHub issue 1185

Opened 2026-08-29; 1 comment.

Raised out of LIB-G18a (#1176), where `evaluate`'s `prior=` reached Python. **Not fixable in that unit's fences** (no kernel changes), and the unit documents the behaviour at the door instead. This issue is the design question underneath.

## The behaviour, measured

`crates/editor-core/src/eval/mod.rs` looks the memo up before anything asks the resolver:

```rust
if let Some(NodeResult::Ok(v)) = prior.and_then(|p| p.nodes.get(&id))
    && v.content_key == content_key
    && v.naming_key == naming_key
{ return NodeStep { result: NodeResult::Ok(v.clone()), reused: true }; }
```

For an `InstantiatePart` node the content key is the node's own content — id, pin, placement — so a hit means "same reference, same placement". The seam is then never crossed. Executed on the tour's bench corpus, through the Python door:

| run | result |
| --- | --- |
| fresh `evaluate(layout, resolver=moved_store)` after the shelf is resaved | refuses `part_pin_mismatch` |
| same call with `prior=` from before the resave | **succeeds**, `reused=3 recomputed=0`, `part_evaluations=0`, serving the pre-resave body |
| fresh `evaluate(layout, resolver=store_missing_post)` | refuses `part_unresolved` ×2 |
| same call with a prior | **succeeds**, zero refusals |

Both reachable in Rust too — nothing about this is a binding artifact.

## The two framings, and why both belong here

The two LIB-G18a reviews framed the served value differently, and the difference is the whole question:

- **"Stale wrong answer."** The natural memo workflow is *edit a part, re-evaluate with the prior*. That workflow silently serves the old body. A caller who reaches for `prior=` for the reason `prior=` exists gets the answer the store no longer supports.
- **"Pin-correct but unrefused."** What is served is exactly what the document's own `DocRef` pins, certified by content key. It is never a *different* part and nothing is retargeted. What is bypassed is the **A4 refusal contract** — "a pin that moved refuses" — which now holds only for evaluations that actually ask the seam.

Both are true of the same behaviour. The design question is which one the kernel intends:

1. **Memo admission learns about resolver state** — e.g. an instantiate node's memo entry is admissible only when the resolver that produced it is the resolver now in hand, or when the reference still resolves. Restores A4 unconditionally; costs a seam crossing (or an identity check) per reused instantiate node.
2. **Pin-serving is correct by design**, and the refusal contract is narrowed in the spec to "an evaluation that crosses the seam refuses a moved pin" — i.e. the memo is a pure function of the document, and checking the store is what *not* passing a prior is for.

**Precedent worth weighing for (1):** the viewer already gates memo priming on resolver identity — `same_resolver` / `Arc::ptr_eq` in `crates/viewer/src/evalseam.rs`, landed with GUI-4's fix pass. So a consumer has already decided it cannot reuse across a resolver change. That is evidence the kernel's default surprises the layer above it.

## The class this belongs to

**"An argument that silently voids another argument's gate."** `prior=` voids `resolver=`'s gate; nothing in either parameter's own contract says so, and the void is invisible in the result except through `part_evaluations == 0`. Two other places a reviewer said to look:

- `import_step`'s options — whether an option can disable a check the caller believes is running.
- **G18b's future `update_references`** — moving a pin at its sites against a differently-timed workspace snapshot is the same shape: two arguments whose timing relationship is unstated.

Worth a sweep for the class once this one is decided, rather than three separate findings later.

## What LIB-G18a did in the meantime

Documented it at the door (`evaluate`'s `prior=` in `py/value.rs` and `pncad.pyi`), qualified the audit page's A4 sentence the same way, and pinned the behaviour with contract-asserting tests (adopted from both reviewers' probe branches) so a change here goes red in a named place rather than silently.

## Comments

**2026-08-29** — comment:

(LIB orchestrator) The class has a second live site, measured by the G18b unit (PR #1192) during its sweep of this issue's pattern: `product` / `assemble` / `SolvedPoses.placement` each take a document plus a second value that must be OF that document (an evaluation, a solve), and nothing can check the pairing — an `Evaluation` carries no identity of the document it was computed from. Mispairing is silent misbehavior, not a refusal. The unit stated the obligation at each door's docstring; the structural fix (an evaluation carrying its document's id, checked at the doors) is a kernel-shape question and belongs to this issue's resolution, not to a binding unit.

The same sweep executed the class as contract tests on three doors where the answer IS checkable (`update_to_store` snapshots the store; `update_references` reads no store; `inline` crosses at the call under the pin gate) — `crates/pncad-py/tests/test_assembly_author.py`. Sweep blind spot recorded there: the pattern only sees doors whose second state is an argument, and only at the Python boundary.

---
_Generated by [Claude Code](https://claude.ai/code)_

## Home

Raised by LIB-G18a at the `evaluate` door; LIB's `keep_out` names evaluate's signature and the resolver door as design conversations before they are units, which is exactly this.
