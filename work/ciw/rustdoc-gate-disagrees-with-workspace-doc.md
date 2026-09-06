---
id: rustdoc-gate-disagrees-with-workspace-doc
kind: issue
title: Workspace `cargo doc -D warnings` and the hosted rustdoc gate disagree about topo: a broken intra-doc link fails the workspace pass
status: closed
opened: 2026-09-01
github: 1504
refs: [1502, 1317, doc-gate-two-unread-axes]
closed: 2026-09-04
---

## From GitHub issue 1504

Opened 2026-09-01; 0 comments.

(SMELL-UV orchestrator) Found by lane uv-h (PR 1502's comment carries the discovery) and filed here as the durable home — both halves are outside SMELL-UV's fences.

**The instance**: `crates/topo/src/boolean/mod.rs:27` links `SweepStrategy::Idealized`, a variant the enum does not have. A workspace-wide `cargo doc` under `-D warnings` fails on it.

**The class**: the hosted rustdoc gate documents `topo` green, so the hosted configuration and a local workspace doc pass disagree about the same tree — either the gate's scope/flags differ from what a contributor runs, or the broken link is invisible at the gate's configuration. Per the run-record-is-the-instrument rule, worth establishing which before trusting either side. The one-line `topo` fix is Track Q's fence; the gate-configuration question looks like S-QA's (`scripts/`/workflow ground — compare with issue 1317's rustdoc-gate blind-spot list).

## Home

`work/issues/` — the gate-configuration half is S-QA's workflow/`scripts/` ground and S-QA is closed, and no open program's territory covers `.github/workflows/` or the rustdoc gate.

## Closed (2026-09-04, CIW): the disagreement is a feature selection, measured

The issue asked which side is right before trusting either. Neither is
wrong; they document different feature selections, and the instance is
one of the two sites `scripts/doc-gate.sh`'s pass-3 header already
names as correct-under-the-primary-selection.

`SweepStrategy::Idealized` is `#[cfg(feature = "sweep-testing")]`
(`crates/topo/src/boolean/reduce.rs:83`). The hosted gate's pass 1
documents at `--all-features`, where the variant exists and the link
resolves — which is why the gate reports `topo` green, and it is right
to. A contributor's plain `cargo doc` runs DEFAULT features, where the
variant is compiled out and both prose sites are unresolvable:

```
$ RUSTDOCFLAGS="-D warnings -A rustdoc::private_intra_doc_links" \
  cargo doc --no-deps --document-private-items -p topo
error: unresolved link to `SweepStrategy::Idealized`
  --> crates/topo/src/boolean/mod.rs:27:39
error: unresolved link to `SweepStrategy::Idealized`
  --> crates/topo/src/boolean/reduce.rs:18:38
error: could not document `topo`
EXIT=101
```

So there is no gate misconfiguration to fix and no flag to align: the
gate reading a superset of the features is the whole reason it is the
gate. What the finding leaves behind is smaller than it looked and is
one sentence — a reader who runs `cargo doc -D warnings` by hand gets a
red the gate does not, and nothing tells them why. That residue rides
`work/ciw/doc-gate-two-unread-axes`, whose axis (a) is this same
cross-half seam pointing the other way, and is recorded there.

The one-line `topo` prose fix (link the feature-gated variant in a form
that resolves at both selections, or stop linking it) stays Track Q's
fence, as this issue said at filing.
