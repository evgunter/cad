---
id: at-rest-badge-reports-an-empty-document-as-a-refusal
kind: issue
title: DocSession's at_rest badge reports a body-less document as a Refused product, three lines below the landing that classifies it as not-a-fault
status: closed
branch: chrome/empty-doc-badge
opened: 2026-09-15
priority: P1
cost: E
closed: 2026-09-24
pr: 3135
---


## Finding

`crates/viewer/src/session.rs`, the landing. The line that routes the
gather refusal asks `means_no_body()` and treats a body-less document as
an absence rather than a fault — that is the classification WIRE's PR
2629 gave one home, and this site cites it correctly.

**Three lines below, `at_rest` does the opposite.** It builds
`AtRestBadge::Refused { message: AssemblyError::product_refusal(&fault) }`
for **every** fault, `NoBodyRoots` included, whenever the document is
assembly-shaped. So a body-less assembly document gets an at-rest badge
reading *"assembly: product: no product root denotes a body"* — exactly
the *"deleting the last feature looks like a failure"* outcome that the
line above it, and `frame::product_badge`, and `pickindex.rs` all exist
to avoid. The viewer says both things at once about one document.

## It is constructible, and here is the derivation

Measured against the tree by WIRE's implementer lane (`wire-n1`) at the
WIRE orchestrator's request. **Derived, not built** — nobody has driven
this document through the GUI.

- `roots::is_sink` makes the root set exactly the sink set.
- `Node::Measure`'s `inputs()` are the nodes its references are read
  **at**, so a measure over the instantiate **de-sinks** it.
- `product::sources_of` returns `None` for `ValuePayload::Measure`, so
  the measure denotes no body.

A document holding an `InstantiatePart` **plus a `Measure` whose refs are
read at it** therefore has the instantiate de-sunk, the measure as its
only root, and gathers `NoBodyRoots` — while `session::assembly_shaped`
is **true**, because it scans `order()` for any `InstantiatePart` and
never looks at roots. `at_rest` then carries the refusal badge.

**Two routes closed on the way, recorded so nobody re-walks them:**

- A `BooleanValue::Empty` root **still** sets `any_body_denoting`:
  `sources_of` returns `Some(vec![])` and the flag is set on `Some`, not
  on a non-empty vec.
- `Node::Mate`'s `inputs()` is `Vec::new()`, so a mate never de-sinks an
  instance.

**What was not checked:** whether the GUI will author a measure over the
only body-producing node. The document is constructible through
`DocEdit::InsertNode` either way, so the question is reachability by a
user rather than reachability at all.

## Why it is filed and not fixed

Pre-existing — it predates PR 2629, which only made it visible by putting
the correct classification on the line above it. `crates/viewer/src/session.rs`
is CHROME's and VIEW's, not WIRE's, and 2629's seam was explicitly the
minimum that makes the citation work. Widening it to a behaviour fix in
another program's file is what an announced seam is not for.

Read beside
`viewer-states-the-empty-document-rule-in-four-places-and-the-one-that-gates-cannot-red`,
filed the same day: that row is the structural half (the rule restated in
places the citation did not reach, and a `matches!` that cannot red on a
new arm). **This row is the behavioural half** — a site that classifies
the arm the other way and shows a user a failure that is not one. A taker
doing either should read both.

## What a taker owes

A decision about whether `at_rest` should badge a refusal the landing has
already classified as an absence — and if not, whether the guard belongs
at `at_rest` or in `assembly_shaped`, which today answers a question
about node kinds while the caller needs one about roots. Plus a row that
goes red on it: the shape is a body-less assembly-shaped document, and
the derivation above says how to build one.

## Filed from outside the fence

Filed by the WIRE orchestrator under
`docs/prompts/implementer-discipline.md` §6, out of PR 2629's style
review (the reviewer named the site; the reachability question was put to
the implementer lane and answered above). `session.rs` is claimed by
CHROME and VIEW jointly; filed on CHROME because the deliverable is a
badge. Re-home to VIEW if that reading is wrong.

## Closed

Closed by PR 3135 (`chrome/empty-doc-badge`). The premise held
against the tree: `DocSession::land`'s `Err(fault)` arm built
`AtRestBadge::Refused` for every class, and the derivation above is
constructible — `landing_gathers::a_body_less_assembly_takes_no_at_rest_badge`
builds it (one `InstantiatePart` plus a `Measure` read at it), gathers
`NoBodyRoots`, and read `Some(Refused { "assembly: product: no product
root denotes a body …" })` before the fix.

The guard sits at the landing, not in `assembly_shaped`: the arm now
asks `fault.kind().means_no_body()` once and both the registry's subject
and the A5 badge read that one answer, so the landing cannot say the
two things at once. `frame::badge_site` was not the route — it is
private to `frame`, it answers which channel reports a refusal rather
than whether there is a product for the gate to judge, and `session`
sits below `frame`. `assembly_shaped` keeps answering the node-kind
question it names; `AtRestBadge`'s doc states the second condition.

The class sweep found the neighbouring half — the at-rest badge also
repeats the gather refusals `badge_site` sends to the feature tree or to
`product_badge` — and filed it as
`at-rest-badge-repeats-a-gather-refusal-another-channel-carries`
rather than deciding it here.
