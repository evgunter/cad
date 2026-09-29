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

**The viewer builds kernel values that break their own field docs.**
`viewer::tree::standing_as_drawn` answers `NodeStanding::Poisoned
{ node, through: mate }` for a cluster-refused node. `NodeStanding`'s
docs say `through` is the node's *"nearest failed ancestor"*, and a mate
is not a DAG ancestor of anything. The re-read value reaches
`ResolveIndeterminate` (the properties panel), `InterrogateError` (the
mate tool, the sketch-on-face seat), `NodePickError`/`NameLookupError`
(the pick-index tooltip), and the viewer's `DuplicateFault::NoValue` and
`BlendEvent::TargetHasNoValue`. `NodeStanding`'s `Display` then says
*"poisoned by the failure at node M … the repair is upstream, at node
M"*. A mate is not upstream in the DAG. Only the tree's own pointer
(*"upstream failure at feature M"*) uses the word the same loose way.
A viewer-owned standing type was not taken, because the standing rides
inside kernel containers (`Resolution`, `InterrogateError`) that the
viewer's selection and tool values hold. The product gather is not
re-attributed: `session.product_fault()` hands out the gather's own
value, and only the at-rest badge's words are the tree's
(`viewer::tree::product_refusal_wording`).

The question is a design one. Should `NodeStanding`, or a sibling
reading on `Evaluation`, carry cross-placement blame, with its own
wording (not "ancestor", not "upstream") for a mate? The rule would then
live in the kernel, and the viewer's `blamed_mates` /
`downstream_of_mate` would become a read of it. That would retire
`standing_as_drawn` and its wrappers in `viewer::tree`, and the
`frame::index_refusal_as_drawn` wrapper.
