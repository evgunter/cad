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
- `resolve::ResolveIndeterminate`, `checks::ChecksError::Placement`,
  `names::interrogate::InterrogateError::Standing`,
  `resolve::pick::NodePickError::Standing` and `NameLookupError::Standing`,
  `resolve::hit::HitTestError::Standing`, `stackup`'s no-measure
  refusal, `clearance::SelectionRefusal::NodeDidNotBuild`,
  `names::geompred`'s `NodeHasNoValue`/`DatumHasNoValue`;
- `product::ProductError::Read`, which carries the read's standing
  whole and renders it as `product: the body the slot reads: {standing}`, so it inherits
  the standing's recourse, *"fix the node's own failure"* included.

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
value, and no chrome surface draws its standing: the tree badges the
refused read at its row, and the at-rest badge takes no verdict on a
gather refusal (since 2026-10-01; before that it drew the read's
standing re-read by `standing_as_drawn`).

The question is a design one. Should `NodeStanding`, or a sibling
reading on `Evaluation`, carry cross-placement blame, with its own
wording (not "ancestor", not "upstream") for a mate? The rule would then
live in the kernel, and the viewer's `blamed_mates` /
`downstream_of_mate` would become a read of it. That would retire
`standing_as_drawn` and its wrappers in `viewer::tree`, and the
`frame::index_refusal_as_drawn` wrapper.

## Parked on the D10 hold (2026-10-17, WIRE triage)

Read against the tree at `fd218851f9`. Mate-cluster blame is stage 3 C's ground. Read together with INTENT's `a-mate-is-poisoned-through-a-member-its-own-solve-faulted`, the same standing-versus-mate-blame question from the mate side. Parked on `a-mate-relates-two-poses`, the INTENT row whose landing reaches this ground, and moved to WIREHOLD by WIRE's priority-seam cut, keeping its id.

Signed (WIRE orchestrator).

**Census entries that wait on it (WIRE, PR 5298, 2026-10-18).** `crates/editor-core/tests/refusal_concision_chains.rs` `FILED_NO_RECOURSE` keeps `MeasureRefUnreadable/NodeFailed` and `MeasureRefUnreadable/NodePoisoned` unlabelled under a comment citing this row. Their only recourse is `NodeStanding`'s "fix the node's own failure" / "the repair is upstream", which is false for a node a cluster refused, so it is not certified as a `Recourse:` until this lands.

## Unparked (2026-10-20, INTENT stage 3 D lane)

Its trigger fired: `a-mate-relates-two-poses` (stage 3 C, PR #5267) merged on 2026-10-18, so mate-cluster blame now stands on the tree it waited for and the design question is open to weigh.
