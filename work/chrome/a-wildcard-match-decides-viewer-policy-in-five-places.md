---
id: a-wildcard-match-decides-viewer-policy-in-five-places
kind: issue
title: A wildcard match arm decides viewer policy over a crate-owned enum in five places, and none of them can red when the enum grows
status: closed
opened: 2026-09-22
priority: P2
cost: E
closed: 2026-09-24
branch: chrome/subset-policy
pr: 3140
---


## Finding

The residue of `matches-subset-policy-survives-in-four-viewer-modules`,
given its own file because a residue disclosed inside another row's
prose dies when that row closes (`work/README.md`; and
`work/meta/a-stated-sweep-blind-spot-is-never-swept`, which records two
lanes where the stated gap was exactly where the finding was).

That row swept for `matches!`. The same construct spelled as a `match`
with a `_ =>` arm is silent in exactly the same way, and the sweep
declared the spelling unsearched on the ground that the instrument was
a clippy lint. **It is not.** `grep -rn '^\s*_ =>' crates/viewer/src`
is 41 hits and takes a second, and one of those hits is inside the set
that sweep's own lane had just measured by hand.

## The hit list

Filed — a policy over an enum this crate owns, where a new variant
silently gets an answer nobody chose:

- **`pane/features.rs:120`, `feature_row`** — `match &row.status {
  RowStatus::Poisoned { through, .. } => Some(*through), _ => None }`,
  twelve lines below the *"Exhaustive on purpose"* badge match that
  DOES red. A new `RowStatus` silently has no row to link to: its
  message draws with `ui.weak` and the eye is sent nowhere, which is
  issue 1463's symptom. Recorded on
  `band-refusal-still-badges-every-row`, whose taker meets it.
- **`tree.rs:305`, `frame_pose`** — `_ => None` over `Node`, ninety
  lines from `node_kind`, whose doc argues *"one arm per variant so a
  new node type cannot fall into a wildcard and draw as something it
  is not"*. `TreeRow::pose`'s own doc says `None` is *"a node kind that
  has no such sentence"* — a claim the wildcard cannot keep. A new
  datum flavour draws with no pose, which is `feature_row`'s named
  defect: *"a tree of rows reading `Datum frame` twice asks a person to
  tell two frames apart by clicking"*.
- **`tree.rs:451`, `node_note`** — `_ => None` over `Node`. A node kind
  that grows a standing caveat silently has none.
- **`sketch.rs:1233`, `fresh_step_at`** — `_ => {}` over `Step`. A new
  step carrying an `ArcData` is silently not freshened, so it *"starts
  in a mode its row refuses"* — the exact thing the function's own doc
  says it exists to prevent.
- **`frame.rs:2113`, `creation_offer`** — `_ => None` over
  `ParseError`. Which refusals get the ratified refuse-then-offer is a
  policy; a new parse error that names an undeclared identifier
  silently gets no offer.

One more, test-only and cheaper: `widgets.rs:1415`'s `kind(op)` reads
a new gesture op as `"other"`, which is a test's own vocabulary rather
than the chrome's.

## Not defects, so the negative result is a receipt

The other 35 hits, by the criterion that excludes them:

- **The catch-all is forced by a guard** — a `match` whose named arms
  carry `if` guards needs a catch-all whatever the enum does;
  exhaustiveness is not available to decline. `pickcache.rs:354`,
  `session/refuse.rs:425`, `session/select.rs:332`, `frame.rs:671`,
  `frame.rs:714`, `evalseam.rs:977`, `session.rs:2828`,
  `platform.rs:225`, `sketch.rs:1205`, `app.rs:2188`.
  (`select.rs:332` is silent — but at the `matches!` inside its guard,
  which the sibling row files.)
- **Not an enum this crate can exhaust** — tuples and arrays
  (`app.rs:1880`, `evalseam.rs:208`, `props.rs:575`, `session.rs:2389`,
  `pane/viewport.rs:830`, `pane/create.rs:274`, `tree.rs:329`,
  `tree.rs:349`), and `egui::Shape` (`pane.rs:111`, `pane.rs:148`),
  whose variant list is epaint's.
- **Identity downcast** — `Some(E::V(x)) => Some(x), _ => None`, where
  a new variant correctly answers "not that one": `tools.rs` ×8 (the
  open-tool read doors), `widgets.rs:625`, `display.rs:340`,
  `drafts.rs:585`, `frame.rs:2138`, `session.rs:2096`.
- **Test-only assertions naming one shape**: `widgets.rs:1493`,
  `pane/profile.rs:481`.

## Gap (d), and why it is one line rather than a second row

The sibling row also names a helper predicate whose body is the subset
(`foo.is_bar()` reads as one token at the call site). There is a
worked instance of the spelling and it is not a defect:
`session::refuse::admits`'s `Body` arm reads
`crate::combine::denotes_body`, whose body is an exhaustive `match`
carrying *"a new node variant does not compile until someone decides
which side of this line it is on"*. So the spelling hides the
construct from a grep but does not decide whether it is silent — which
means the gap is real and needs a pattern of its own (every
`fn … -> bool` in the crate, read), not that it is empty. That is a
sitting, and it is this row's remaining half; it is written here
rather than in a PR body so it survives this row.

## Evidence 2026-09-22 (`chrome/badge-attribution`, PR 3090)

The first hit, `pane/features.rs`'s `feature_row` link decision, is
fixed: an exhaustive `match` with an arm per status. Four hits remain;
the class was not swept on that branch (its ground is live this
wave).

## Closed 2026-09-24 (`chrome/subset-policy`)

The four remaining hits are exhaustive: `tree::frame_pose`,
`tree::node_note` (whose inner `other =>` over `ClassAdmission` was a
fifth wildcard spelled as a binding), `sketch::fresh_step_at`, and
`frame::creation_offer`, both its `ParseError` arm and the outer
`Option<&Refusal>` wildcard the hit list did not name.

`widgets.rs`'s test `kind(op)` is left as it is. It is the test's own
vocabulary, not chrome policy.

**Gap (d), the helper predicate.** It was swept by its shape rather
than by reading every `fn … -> bool`. A viewer predicate whose body is
a subset appears at its definition in the `matches!`, `_ =>`, `==` and
binding passes, so those passes are that sitting. The second pass found
one more: `MateTool::proposal`'s `admission ==
ClassAdmission::NotAdmitted`, now a match. It also found one fenced out
of this lane, filed as `create-pane-words-mate-admission-through-a-binding-catch-all`.
The full 40-line census, the second pass and what it still cannot see
are in the PR body.

**At review, the "guard-forced catch-all" exemption was found to let
policy through**, and it is tightened in the README: a guard's `_` may
stand only for the guard's `false` case. Where the arms name variants,
the fallback names them again. Converted under it: `frame::frame_status`
(`StatusUpdate`), `Standing::unresolved` (`Standing`) and `pickcache`'s
attempt step (`Attempt`). `Withdrawal`'s `fused` test over
`AdmissionFault` was also reclassified from identity to policy and
converted. The two `Refusal` lists in `frame.rs` became one home,
`Refusal::parse_error`. Two more are filed rather than fixed because
they sit on other lanes' ground:
`create-pane-hides-one-face-frame-fault-through-a-not-equal` and
`assembly-shaped-reads-a-document-as-an-assembly-off-one-node-kind`.
