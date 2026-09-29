---
id: kernel-standing-names-a-cluster-refused-node-as-its-own-failure
kind: issue
title: NodeStanding reports a cluster-refused node as its own failure, so every kernel refusal carrying one sends an API reader to the wrong node
status: open
opened: 2026-09-29
priority: P3
cost: M
design: true
---


Found by CHROME while closing
`work/chrome/viewer-panels-disagree-on-a-poisoned-node` (branch
`chrome/poisoned-panels`).

When a mate cluster refuses, the kernel records the fault against every
instance and mate it reached, each as its own `NodeResult::Failed`
(`crates/viewer/src/tree.rs`, module header, second section). So
`Evaluation::usable` answers `NodeStanding::Failed { node }` for an
instance the fault only reached, and its `Display`
(`crates/editor-core/src/eval/mod.rs`, `impl Display for NodeStanding`)
says *"node N failed, so it has no value — fix the node's own
failure"*. The repair is at the mate the fault blames, not at N. A node
DAG-downstream of that instance is `Poisoned { through: N }`, which
points at N too.

The viewer now reads every standing it shows through
`viewer::tree::standing_as_drawn`, which re-points both at the blamed
mate. The kernel's own vocabulary still does not, and every refusal
that carries a `NodeStanding` inherits the misdirection for an API
consumer that is not the viewer:

- `appearance::AppearanceLossCause::Indeterminate`
  (`crates/editor-core/src/appearance.rs`), which is what the CHROME row
  named. The viewer does not surface appearance losses today;
- `resolve::ResolveIndeterminate`, `checks::ChecksError::Root`,
  `names::interrogate::InterrogateError::Standing`,
  `resolve::pick::NodePickError::Standing` and `NameLookupError::Standing`,
  `resolve::hit::HitTestError::Standing`, `stackup`'s no-measure
  refusal, `clearance::SelectionRefusal::NodeDidNotBuild`,
  `names::geompred`'s `NodeHasNoValue`/`DatumHasNoValue`;
- `product::ProductError::RootFailed`, through `From<NodeStanding>`.

One wording defect follows from the viewer's re-read.
`ProductError::RootPoisoned`'s `Display` says *"poisoned through failed
ancestor M"* (`crates/editor-core/src/product.rs`). After
`viewer::tree::product_fault_as_drawn`, M can be a mate, which is not
a DAG ancestor of the root. The viewer's at-rest badge
(`viewer::frame::at_rest_badge`) shows that sentence. `NodeStanding`'s
own wording ("poisoned by the failure at node M … the repair is
upstream") reads correctly for a mate.

The question is a design one: should `NodeStanding` (or a sibling
reading on `Evaluation`) carry cross-placement blame, so that the rule
lives in the kernel and the viewer's `blamed_mates` /
`downstream_of_mate` become a read of it? Doing that would retire
`standing_as_drawn` and its three wrappers in `viewer::tree`.
