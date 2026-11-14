---
id: a-dual-part-bound-under-a-pin-its-resolver-answers-otherwise-reads-record-refused
kind: issue
title: A dual's part run bound to the record's build by pin alone reads RecordRefused where the resolver answered that pin with other content
status: open
opened: 2026-11-10
priority: P3
cost: E
---



Filed from PR 6735's re-review (N1). In 6735, a dual's descent binds each part's run to the record's build of the same part, keyed by the part's `DocRef` (id and pin) and ε (`crates/editor-core/src/eval/parts.rs`, `resolve_and_evaluate`; `RecordBinding::of_part` in `eval/mod.rs`). The record pairing's `Resolver` arm compares only whether each run has a resolver (`RecordBinding::of`, ~`mod.rs:4091`). So the binding trusts that one pin names one content, which is A4's premise: every shipped resolver checks it.

**Reachable only through a resolver that answers a pin with other content.**
- Measured with a scratch row, not kept. It used a stub resolver that ignores pins.
  - The record's resolver answers part `P`, which instantiates `C1`.
  - The dual's resolver answers `P′` under the same id and pin, which instantiates `C2`.
- `f64` over the dual's resolver builds every node.
- The dual's instance of `P` fails `PartProductFailed`, carrying `RecordRefused`. `PartFault::NotEntered` does not surface (`parts.rs`, ~`:938`): the binding refuses the instance of `C2` first, as an instance of a part the record never built (`RecordBinding::partless`).
- So the refusal is mislabelled, though not as the kernel defect the review expected. `RecordRefused` says the record refused the node, but the record built another document and never refused it. That is a mispairing.

**The same premise as `PartFault::ReferenceCycle`**, which A4 makes unconstructible through an honest store and which renders as a kernel defect (`parts.rs`, its doc).

**The fix, if taken.** Pair a part's run with the record's build of it by content as well as by key:
- the `Refusals` tree carries the content pin of the document it was built from;
- `of_part` compares that pin with `content_pin` of the document the run resolved;
- a mismatch refuses the part's every node as `RecordMispaired(Revision)` (or a resolver arm by content).

The cost is one canonicalization per part per dual run. The other route is the kernel checking A4 itself at `Entered::enter`, for every scalar, which would also make `ReferenceCycle` unreachable. That is the broader question, and it is not this row's to decide.
