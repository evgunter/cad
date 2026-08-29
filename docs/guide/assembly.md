# Assemblies: parts, instances, and what holds them together

Up to here the guide has built one document at a time: a profile, a
body, a recipe with parameters. An **assembly** is a document whose
leaves are *other documents* — a bench is a shelf document and a post
document, instantiated three times between them and held together by
mates.

That one step across a document boundary is where a CAD system
usually starts guessing: which version of the post did you mean, where
exactly does the shelf sit, are those two faces touching on purpose.
This kernel answers all three the same way it answers everything else
— by making you say it, and refusing when you have not.

This page is the assembly surface end to end: the store and the
identity/pin/reference split, authoring an assembly, evaluating one
across the document seam, the solve and the at-rest gate that together
are its validity story, and the two refactorings that move a part in
or out of a document.

Everything below runs. The Python blocks are executed by
`crates/pncad-py/tests/test_guide.py`; the doors' own suites are
`crates/pncad-py/tests/test_workspace.py`,
`test_assembly_eval.py` and `test_assembly_author.py`, and the same
scene in Rust is `demos/tour/src/assembly.rs`.

The scene is the tour's bench: two square posts, one shelf resting on
them. Two part documents, and two assemblies built from those — a
**stand** whose parts touch, and a flat-pack **layout** whose parts
do not.

## 1. Three vocabularies: which part, which version, which reference

A document has an **identity** — `Doc.id`, 32 hex digits — and it
answers *which part*. It survives every edit; it is not a hash of
anything.

A **`ContentPin`** answers *which version*: the SHA-256 of the
document's canonical semantic bytes. It moves whenever the content
does.

A **`DocRef`** pairs them, and that pair is what a cross-document
reference carries. The semantics are Cargo.lock's: editing the
referenced document never silently retargets a reference to it. The
store refuses the stale pin instead, and moving a pin is its own
recorded edit.

A **`Workspace`** is where the documents live: a directory of
`*.pncad` files, scanned by each file's `id:` header line and never
its body. Its write side is deliberately two doors — `create` and
`resave` — and there is no general mutation API.

```python
import tempfile

from pncad import (
    PIN_MISMATCH_RECOURSE, ContentPin, Doc, DocRef, Node, Workspace,
    WorkspaceError, content_pin, m,
)


def prism(label, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(label)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(x * m, y * m) for x, y in corners]))
    doc.insert(Node.extrude(profile, height * m))
    return doc


# A workspace is an ordinary directory of `*.pncad` files that you
# keep; this page uses a throwaway one so it runs anywhere.
store = Workspace(tempfile.mkdtemp())

post = prism("bench-post", 0.12, 0.12, 0.5)
shelf = prism("bench-shelf", 0.9, 0.30, 0.04)
store.create(post)
store.create(shelf)
assert len(store) == 2
assert set(store.documents()) == {post.id, shelf.id}

# The reference an assembly will carry: this part, at this version.
shelf_ref = DocRef(shelf.id, content_pin(shelf))
assert store.resolve(shelf_ref).id == shelf.id

# The shelf legitimately changes: same part, new content, new pin.
thicker = prism("bench-shelf", 0.9, 0.30, 0.08)
assert thicker.id == shelf.id, "same label, same part"
assert content_pin(thicker) != content_pin(shelf)
store.resave(thicker)

# The old reference is not retargeted onto it. It refuses, naming
# both versions — and the message ends on the recourse, because
# accepting a new version is an edit you make, not one made for you.
try:
    store.resolve(shelf_ref)
    raise AssertionError("expected a typed refusal")
except WorkspaceError as refusal:
    assert refusal.variant == "pin_mismatch"
    assert refusal.wanted == content_pin(shelf)
    assert refusal.found == content_pin(thicker)
    # The recourse is IN the refusal, not in a doc: the library's own
    # sentence, which the message ends on.
    assert PIN_MISMATCH_RECOURSE in str(refusal)

# `current_pin` is the door that says what the new version IS —
# `resolve` minus the expected pin, so there is nothing to disagree
# with.
assert store.current_pin(shelf.id) == content_pin(thicker)
assert isinstance(store.current_pin(shelf.id), ContentPin)
```

Read the split once and it explains most of this page. A reference
that names an identity alone would follow the part wherever it went;
a reference that named a pin alone could not say *which part* had
moved. Carrying both is what makes "your assembly still means what it
meant yesterday" a fact rather than a hope.

## 2. Authoring an assembly

Three doors — and one property, *placement lives on the cluster*,
that explains the second of them.

`Node.instantiate_part(reference)` is an instance: a **leaf** whose
material crosses the document seam. It takes no frame.

`DocEdit.set_placement(node, frame)` is where the frame goes — and it
places the instance's **cluster**, not the instance. Instances coupled
by mates share one recorded frame, held by the earliest of them in
document order (their **gauge**); every other member's pose is
*solved*. That is why a document can carry three instances and one
frame, and why zero-anchor and multi-anchor states are
unrepresentable here rather than merely refused.

`Node.mate(a, b, class_, alignment)` is one node carrying both halves
of "these two parts meet here": the placement constraint the solve
folds, and the contact declaration the gate mints. `a` and `b` are
instance-qualified entity names — the text `Evaluation.select` answers
with when you query it on an instantiate node, so no name is ever
composed by hand.

```python
import tempfile

from pncad import (
    Alignment, AxisSense, CapEnd, ContactClass, Doc, DocEdit, DocRef,
    EditError, EntityKind, Frame, MateFrame, MatePrimitive, NamePat,
    Node, SegPat, SegTag, Selector, Workspace, clusters, content_pin,
    evaluate, gauge_of, m, reading_edges,
)

POST_SECTION, POST_HEIGHT = 0.12, 0.5
SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS = 0.9, 0.30, 0.04


def prism(label, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(label)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(x * m, y * m) for x, y in corners]))
    doc.insert(Node.extrude(profile, height * m))
    return doc


def instance_cap(ev, instance, side):
    """One instance's cap face, in the ASSEMBLY's names.

    The part's own cap name, seen one wrapper deeper: `InPart` is the
    segment the instantiate seam adds. Nothing reads inside a name —
    this is the same query, nested.
    """
    cap = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Cap).side(side))
    through = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.InPart).of([cap]))
    found = ev.select(instance, Selector.of(through))
    assert len(found) == 1, f"expected one face, got {found}"
    return found[0]


def frame_at(x, y, z):
    """A mate frame: +z axis, +x clocking reference."""
    return MateFrame(origin=(x * m, y * m, z * m), axis=(0.0, 0.0, 1.0),
                     reference=(1.0, 0.0, 0.0))


store = Workspace(tempfile.mkdtemp())
post = prism("bench-post", POST_SECTION, POST_SECTION, POST_HEIGHT)
shelf = prism("bench-shelf", SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS)
store.create(post)
store.create(shelf)
post_ref = DocRef(post.id, content_pin(post))
shelf_ref = DocRef(shelf.id, content_pin(shelf))

stand = Doc("bench-stand")
post_a = stand.insert(Node.instantiate_part(post_ref))
# Only the gauge is placed by hand. The other two poses are solved.
stand.apply(
    DocEdit.set_placement(
        post_a,
        Frame.translation((0 * m, (SHELF_DEPTH - POST_SECTION) / 2 * m, 0 * m)),
    )
)
shelf_i = stand.insert(Node.instantiate_part(shelf_ref))
post_b = stand.insert(Node.instantiate_part(post_ref))

# The mate references, SELECTED rather than spelled: evaluate against
# the store, then ask each instantiate node for its face.
ev = evaluate(stand, resolver=store)
a_top = instance_cap(ev, post_a, CapEnd.Top)
b_top = instance_cap(ev, post_b, CapEnd.Top)
shelf_underside = instance_cap(ev, shelf_i, CapEnd.Bottom)

# Where each post's top meets the shelf's underside, each written in
# its OWN part's coordinates. The posts sit flush with the shelf's
# two ends, which is the obvious way to draw a bench.
post_seat = frame_at(POST_SECTION / 2, POST_SECTION / 2, POST_HEIGHT)
seat_a = frame_at(POST_SECTION / 2, SHELF_DEPTH / 2, 0.0)
seat_b = frame_at(SHELF_LENGTH - POST_SECTION / 2, SHELF_DEPTH / 2, 0.0)


def seat(a, b):
    """Two frames meeting outright, axes aligned, no clocking rider."""
    return Alignment(a, b, MatePrimitive.frame_coincidence(), AxisSense.Aligned)


mate_a = stand.insert(
    Node.mate(a_top, shelf_underside, ContactClass.Rest, seat(post_seat, seat_a))
)
mate_b = stand.insert(
    Node.mate(shelf_underside, b_top, ContactClass.Rest, seat(seat_b, post_seat))
)

# The two mates couple all three instances into ONE cluster, gauged
# by the earliest of them.
assert clusters(stand) == [[post_a, shelf_i, post_b]]
assert all(gauge_of(stand, n) == post_a for n in (post_a, shelf_i, post_b))

# ...so exactly one instance carries an authored frame. The other two
# never will.
assert list(stand.placements()) == [post_a]

# A mate's references are NOT recipe edges — inserting one transfers
# no root. What couples the graph is the reading edges, recomputed
# from the name heads every time and never stored.
assert set(reading_edges(stand)) == {
    (mate_a, post_a), (mate_a, shelf_i), (mate_b, shelf_i), (mate_b, post_b),
}

# The roots say what this document IS, in gather order. A root is a
# live node nothing else consumes — so the two MATES are roots too,
# consumed as they are by nothing. `product` gathers the
# body-denoting ones.
assert stand.roots == [post_a, shelf_i, post_b, mate_a, mate_b]

# `set_roots` is the designate door and it is TOTAL: one edit states
# the whole list, so the product's solid order is never inferred from
# an edit sequence. Reordering the bodies means carrying the mates
# along — leave a live node reaching no root and the edit refuses,
# because a silently dead subgraph is not a thing you meant.
try:
    stand.apply(DocEdit.set_roots([shelf_i, post_a, post_b]))
    raise AssertionError("expected a typed refusal")
except EditError as refusal:
    assert refusal.variant == "root_uncovered"
assert stand.roots == [post_a, shelf_i, post_b, mate_a, mate_b], "refused, so untouched"

stand.apply(DocEdit.set_roots([shelf_i, post_a, post_b, mate_a, mate_b]))
assert stand.roots[:3] == [shelf_i, post_a, post_b]
```

Two things in that block are worth pausing on.

**Nothing checks a mate's alignment against the faces it names.** A
`MateFrame` is *authored* data — the solve is structural plus decided
predicates over exactly those numbers. So a mate can solve perfectly
and still be refuted at the gate, and that is not a hole: it is the
boundary between "where you said the parts meet" and "where they
actually do", kept visible. (Nothing yet mints a mate frame from a
selected face; that is issue #944.)

**Ask `class_admission` before you author a class.** The solve and the
gate admit different sets, and the table is one value both read:

```python
from pncad import ContactClass, class_admission

rest = class_admission(ContactClass.Rest)
assert rest.variant == "mints" and rest.mints and rest.solves
assert rest.why is None

# The gap the table exists to state: a class the SOLVE folds, that
# the at-rest gate can mint no record for. Asking here costs nothing;
# discovering it after the edit lands costs a round trip.
tangent = class_admission(ContactClass.Tangent)
assert tangent.variant == "no_at_rest_record"
assert tangent.solves and not tangent.mints
assert "at rest" in tangent.why
```

## 3. Evaluating across the seam: `resolver=`

An assembly document does not carry its parts' geometry — it carries
references to them. Evaluating one therefore needs somewhere to look,
and that is `evaluate`'s `resolver=` — `evaluate(doc,
resolver=store)`.

A `Workspace` *is* a resolver, so the store is passed as itself.
Leave it out and you get a kernel-only evaluation, in which every
instantiate node **refuses typed** rather than pretending the part is
empty. Evaluation stays total, as everywhere else in this kernel: the
refusal is read off the node, not raised by the call.

`product(doc, evaluation)` is then what an assembly *is*: every
body-denoting root's solids, gathered in root order into one body. It
is the only useful reading of an assembly document, because an
assembly's nodes are instances and mates and no single node's value is
the assembly.

```python
import tempfile

from pncad import (
    Doc, DocEdit, DocRef, EvaluationError, Frame, Node, Workspace,
    content_pin, evaluate, m, product,
)

POST_SECTION, POST_HEIGHT = 0.12, 0.5
SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS = 0.9, 0.30, 0.04


def prism(label, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(label)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(x * m, y * m) for x, y in corners]))
    doc.insert(Node.extrude(profile, height * m))
    return doc


store = Workspace(tempfile.mkdtemp())
post = prism("bench-post", POST_SECTION, POST_SECTION, POST_HEIGHT)
shelf = prism("bench-shelf", SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS)
store.create(post)
store.create(shelf)

# The flat-pack layout: a post and the shelf beside it, nothing
# touching. No mates, so each instance is its own cluster and each
# carries its own frame.
layout = Doc("bench-layout")
post_i = layout.insert(Node.instantiate_part(DocRef(post.id, content_pin(post))))
shelf_i = layout.insert(Node.instantiate_part(DocRef(shelf.id, content_pin(shelf))))
layout.apply(DocEdit.set_placement(shelf_i, Frame.translation((0 * m, 0.5 * m, 0 * m))))

# With no resolver there is nowhere to look, and the node says so.
# Evaluation is TOTAL — it did not raise; reading the value does.
blind = evaluate(layout)
assert not blind.succeeded(post_i)
assert blind.part_evaluations == 0, "nothing crossed the seam"
try:
    blind.value(post_i)
    raise AssertionError("expected a typed refusal")
except EvaluationError as refusal:
    assert refusal.kind == "part_no_resolver"

# With one, the parts arrive and the product is the material.
ev = evaluate(layout, resolver=store)
assert ev.succeeded(post_i) and ev.succeeded(shelf_i)
assert ev.part_evaluations == 2, "two referenced documents crossed the seam"

volume = product(layout, ev).mass_properties().volume
expected = POST_SECTION**2 * POST_HEIGHT + SHELF_LENGTH * SHELF_DEPTH * SHELF_THICKNESS
assert abs(volume - expected) < 1e-12
```

`part_evaluations` counts *referenced documents* crossed, not
instances: three instances of one post count 1, which is the seam's
own sharing evidence.

### The memo, and what a memo hit skips

`evaluate` also takes `prior=`, a previous evaluation used as a memo:
a node whose content and naming keys match its result in `prior`
reuses that value instead of re-running its op, so only the changed
cone costs anything. `Evaluation.reused` and `.recomputed` count it.

The memo is **per document and node-id-keyed** — ids are minted per
document, so an evaluation of a *different* document is a legal prior
that reuses nothing. Pass the prior evaluation of this document.

And then the part that matters for assemblies, stated at the door and
not softened here:

> **A memo hit is served without re-running the seam's gates.** A
> reused `InstantiatePart` node never asks the resolver, so the
> availability refusals — `part_pin_mismatch`, `part_unresolved`,
> `part_no_resolver` — are raised only for nodes that actually
> re-resolve.

What is served is what the document's own `DocRef` **pins**, certified
by content key: never a different part, and not re-checked against the
store. Both readings of that are true at once — against the store the
value is stale, against the document it is exactly right — and the
consequence is a workflow you will hit on your first day:

```python
import tempfile

from pncad import (
    Doc, DocRef, EvaluationError, Node, Workspace, content_pin,
    evaluate, m,
)


def prism(label, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(label)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(x * m, y * m) for x, y in corners]))
    doc.insert(Node.extrude(profile, height * m))
    return doc


store = Workspace(tempfile.mkdtemp())
shelf = prism("bench-shelf", 0.9, 0.30, 0.04)
store.create(shelf)

layout = Doc("bench-layout")
shelf_i = layout.insert(Node.instantiate_part(DocRef(shelf.id, content_pin(shelf))))

before = evaluate(layout, resolver=store)
assert (before.reused, before.recomputed) == (0, 1)
pinned_volume = before.value(shelf_i).body().mass_properties().volume

# Someone thickens the shelf and saves it. The assembly still pins
# the version it was authored against.
store.resave(prism("bench-shelf", 0.9, 0.30, 0.08))

# An evaluation that ASKS refuses the moved pin. This is the question
# "does my document still resolve against the store as it stands?",
# and the answer is no.
fresh = evaluate(layout, resolver=store)
try:
    fresh.value(shelf_i)
    raise AssertionError("expected a typed refusal")
except EvaluationError as refusal:
    assert refusal.kind == "part_pin_mismatch"

# The SAME call with a prior does not refuse, because it never asks:
# the node is a memo hit, and what it serves is the body the
# document's own DocRef pins.
memoized = evaluate(layout, resolver=store, prior=before)
assert (memoized.reused, memoized.recomputed) == (1, 0)
assert memoized.part_evaluations == 0, "the seam was not crossed"
assert memoized.value(shelf_i).body().mass_properties().volume == pinned_volume

# The limit case of the same rule: with every node a memo hit, the
# resolver is not needed at all.
assert evaluate(layout, prior=before).succeeded(shelf_i)
```

So: **pass no prior when the question is whether the document still
resolves against the store as it stands.** "A pin that moved refuses,
and is never silently retargeted" holds for evaluations that cross the
seam; a run that never asks does not re-assert it. Whether memo
admission *should* know about resolver state is an open kernel-side
question (issue #1185) — this page states the contract as it is rather
than the one it might become.

## 4. The solve and the gate: an assembly's validity story

Two doors, and they check different things.

`solve_document(doc)` folds the mates: per-pair cosets along a
deterministic spanning tree, producing each instance's pose relative
to its cluster gauge. It is **total** — a refusing cluster must not
fail an unrelated one, so refusals are read back per node through
`SolvedPoses.fault` rather than raised. It inspects **no geometry**.

`assemble(doc, evaluation)` is the **at-rest gate**: it gathers the
product, mints every solved mate's declaration into contact records,
and runs the kernel's own at-rest door over the two together. This is
the check the authoring vocabulary can otherwise construct and never
make — the assembly answer to the undeclared-coincidence refusal the
fail-loud tour walks through for a single-document boolean. There, two
slabs sharing a plane refuse until you *declare* the contact. Here, a
mate **is** the declaration, and the gate is where it gets verified
against the geometry it claims.

```python
import tempfile

from pncad import (
    Alignment, AxisSense, CapEnd, ContactClass, Doc, DocEdit, DocRef,
    EntityKind, Frame, MateFrame, MatePrimitive, MateRole, NamePat,
    Node, SegPat, SegTag, Selector, Workspace, assemble, content_pin,
    evaluate, m, product, solve_document,
)

POST_SECTION, POST_HEIGHT = 0.12, 0.5
SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS = 0.9, 0.30, 0.04


def prism(label, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(label)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(x * m, y * m) for x, y in corners]))
    doc.insert(Node.extrude(profile, height * m))
    return doc


def frame_at(x, y, z):
    return MateFrame(origin=(x * m, y * m, z * m), axis=(0.0, 0.0, 1.0),
                     reference=(1.0, 0.0, 0.0))


def instance_cap(ev, instance, side):
    cap = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Cap).side(side))
    through = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.InPart).of([cap]))
    (found,) = ev.select(instance, Selector.of(through))
    return found


store = Workspace(tempfile.mkdtemp())
post = prism("bench-post", POST_SECTION, POST_SECTION, POST_HEIGHT)
shelf = prism("bench-shelf", SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS)
store.create(post)
store.create(shelf)
post_ref = DocRef(post.id, content_pin(post))
shelf_ref = DocRef(shelf.id, content_pin(shelf))


def bench(primitive=None, class_=ContactClass.Rest):
    """The stand: two posts, a shelf seated on them by two mates."""
    doc = Doc("bench-stand")
    a = doc.insert(Node.instantiate_part(post_ref))
    doc.apply(
        DocEdit.set_placement(
            a, Frame.translation((0 * m, (SHELF_DEPTH - POST_SECTION) / 2 * m, 0 * m))
        )
    )
    s = doc.insert(Node.instantiate_part(shelf_ref))
    b = doc.insert(Node.instantiate_part(post_ref))
    ev = evaluate(doc, resolver=store)
    post_seat = frame_at(POST_SECTION / 2, POST_SECTION / 2, POST_HEIGHT)
    seat_a = frame_at(POST_SECTION / 2, SHELF_DEPTH / 2, 0.0)
    seat_b = frame_at(SHELF_LENGTH - POST_SECTION / 2, SHELF_DEPTH / 2, 0.0)
    fold = primitive or MatePrimitive.frame_coincidence()

    def align(x, y):
        return Alignment(x, y, fold, AxisSense.Aligned)

    m_a = doc.insert(
        Node.mate(instance_cap(ev, a, CapEnd.Top), instance_cap(ev, s, CapEnd.Bottom),
                  class_, align(post_seat, seat_a))
    )
    m_b = doc.insert(
        Node.mate(instance_cap(ev, s, CapEnd.Bottom), instance_cap(ev, b, CapEnd.Top),
                  class_, align(seat_b, post_seat))
    )
    return doc, (a, s, b), (m_a, m_b)


stand, (post_a, shelf_i, post_b), mates = bench()

solved = solve_document(stand)
assert all(solved.fault(n) is None for n in (post_a, shelf_i, post_b, *mates))
# Both mates PLACED a child — that is what `Determining` means. A
# mate that solved nothing and is carried to evaluation as a pure
# contact declaration is `Declaring`.
assert [solved.role(mate) for mate in mates] == [MateRole.Determining] * 2

# The gauge's relative pose is the identity, bit-exactly, so its
# world placement is its recorded frame verbatim...
assert solved.placement(stand, post_a).origin == stand.placement(post_a).origin
# ...and the other two are composed outward along the mate tree,
# never stored. The shelf sits on top of the posts.
assert abs(solved.placement(stand, shelf_i).origin[2].meters - POST_HEIGHT) < 1e-12
far = solved.placement(stand, post_b).origin
assert abs(far[0].meters - (SHELF_LENGTH - POST_SECTION)) < 1e-12
# The ROTATION, which a translation check cannot see: both mates
# align +z with +z at zero clocking, so the far post is not turned. A
# solve that rotated it and still landed its seating point would put
# the part in sideways.
assert solved.placement(stand, post_b).columns == (
    (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0),
)

# The gate: the flush-seated stand CERTIFIES, and hands back one
# minted declaration per solved mate, at face granularity, in the
# alphabet the mates were authored in.
assembly = assemble(stand, evaluate(stand, resolver=store))
assert [d.mate for d in assembly.minted] == list(mates)
assert [d.class_ for d in assembly.minted] == [ContactClass.Rest] * 2
expected = (2 * POST_SECTION**2 * POST_HEIGHT
            + SHELF_LENGTH * SHELF_DEPTH * SHELF_THICKNESS)
assert abs(assembly.body.mass_properties().volume - expected) < 1e-12

# `assemble` IS the gather plus the check, so its body is the gathered
# body — and a mate-less, disjoint layout passes the same gate
# outright, with nothing minted.
gathered = product(stand, evaluate(stand, resolver=store))
assert gathered.mass_properties().volume == assembly.body.mass_properties().volume
```

## 5. The refusals an assembly author actually meets

Each of these is reached by authoring the mistake. They are the
product working: the kernel declining to pick a pose, invent a record,
or widen a declaration you did not make.

```python
import tempfile

from pncad import (
    Alignment, AssemblyError, AxisSense, CapEnd, ContactClass, Doc,
    DocEdit, DocRef, EntityKind, Frame, MateFrame, MatePrimitive,
    MateSide, NamePat, Node, ProductError, SegPat, SegTag, Selector,
    UNDER_RECOURSE, Workspace, assemble, content_pin, evaluate, m,
    product, solve_document,
)

POST_SECTION, POST_HEIGHT = 0.12, 0.5
SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS = 0.9, 0.30, 0.04


def prism(label, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(label)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(x * m, y * m) for x, y in corners]))
    doc.insert(Node.extrude(profile, height * m))
    return doc


def frame_at(x, y, z):
    return MateFrame(origin=(x * m, y * m, z * m), axis=(0.0, 0.0, 1.0),
                     reference=(1.0, 0.0, 0.0))


def instance_cap(ev, instance, side):
    cap = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Cap).side(side))
    through = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.InPart).of([cap]))
    (found,) = ev.select(instance, Selector.of(through))
    return found


store = Workspace(tempfile.mkdtemp())
post = prism("bench-post", POST_SECTION, POST_SECTION, POST_HEIGHT)
shelf = prism("bench-shelf", SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS)
store.create(post)
store.create(shelf)
post_ref = DocRef(post.id, content_pin(post))
shelf_ref = DocRef(shelf.id, content_pin(shelf))

post_seat = frame_at(POST_SECTION / 2, POST_SECTION / 2, POST_HEIGHT)
seat_a = frame_at(POST_SECTION / 2, SHELF_DEPTH / 2, 0.0)


def two_instances():
    doc = Doc("bench-refusals")
    a = doc.insert(Node.instantiate_part(post_ref))
    doc.apply(DocEdit.set_placement(a, Frame.translation((0 * m, 0 * m, 0 * m))))
    s = doc.insert(Node.instantiate_part(shelf_ref))
    return doc, a, s


# 1. UNDER-DETERMINED. One planar rest fixes the seating plane and
#    nothing else: the pair may still slide and spin in it. The solve
#    refuses and names the RESIDUAL in class vocabulary rather than
#    picking a pose — and the residual is point-FREE, because no base
#    point distinguishes one plane from a parallel one.
doc, post_i, shelf_i = two_instances()
ev = evaluate(doc, resolver=store)
mate = doc.insert(
    Node.mate(
        instance_cap(ev, post_i, CapEnd.Top),
        instance_cap(ev, shelf_i, CapEnd.Bottom),
        ContactClass.Rest,
        Alignment(post_seat, seat_a, MatePrimitive.planar_rest(0 * m),
                  AxisSense.Aligned),
    )
)
fault = solve_document(doc).fault(mate)
assert fault.variant == "mate_under"
assert fault.residual.variant == "planar"
assert fault.residual.normal == (0.0, 0.0, 1.0)
assert fault.residual.point is None
assert (fault.parent, fault.child) == (post_i, shelf_i)
# The kernel's own prose, ending on the recourse.
assert UNDER_RECOURSE in str(fault)

# 2. A CLASS THE GATE CANNOT MINT. `Tangent` solves and mints nothing
#    at rest — exactly what `class_admission` said before the edit
#    landed.
doc, post_i, shelf_i = two_instances()
ev = evaluate(doc, resolver=store)
mate = doc.insert(
    Node.mate(
        instance_cap(ev, post_i, CapEnd.Top),
        instance_cap(ev, shelf_i, CapEnd.Bottom),
        ContactClass.Tangent,
        Alignment(post_seat, seat_a, MatePrimitive.frame_coincidence(),
                  AxisSense.Aligned),
    )
)
try:
    assemble(doc, evaluate(doc, resolver=store))
    raise AssertionError("expected a typed refusal")
except AssemblyError as refusal:
    assert refusal.variant == "no_at_rest_record"
    assert refusal.mate == mate
    assert refusal.class_ == ContactClass.Tangent

# 3. A REFERENCE THAT IS NOT A FACE. A mate declares a FACE PAIR; an
#    edge is a different statement, refused rather than widened — and
#    the refusal says which side, and what the name did denote.
doc, post_i, shelf_i = two_instances()
ev = evaluate(doc, resolver=store)
edge = sorted(ev.all_edges(post_i))[0]
mate = doc.insert(
    Node.mate(
        edge,
        instance_cap(ev, shelf_i, CapEnd.Bottom),
        ContactClass.Rest,
        Alignment(post_seat, seat_a, MatePrimitive.frame_coincidence(),
                  AxisSense.Aligned),
    )
)
try:
    assemble(doc, evaluate(doc, resolver=store))
    raise AssertionError("expected a typed refusal")
except AssemblyError as refusal:
    assert refusal.variant == "mate_reference_refused"
    assert refusal.mate == mate and refusal.side == MateSide.A
    assert refusal.why.variant == "ref_not_a_face"
    assert refusal.why.kind == "edge"
    assert refusal.why.width is None      # a tie would carry one

# 4. NOTHING TO GATHER. Evaluated with no resolver, the instance
#    produced no body, so the GATHER refuses before the gate runs —
#    and it refuses under the gather's own tag, not a wrapper's,
#    because which invariant broke is what you branch on.
doc, _, _ = two_instances()
try:
    assemble(doc, evaluate(doc))
    raise AssertionError("expected a typed refusal")
except AssemblyError as refusal:
    assert refusal.variant == "root_failed"
    assert refusal.node is not None and refusal.mate is None
try:
    product(doc, evaluate(doc))
    raise AssertionError("expected a typed refusal")
except ProductError as refusal:
    assert refusal.variant == "root_failed"
```

Read an `AssemblyError`'s `variant` first, and read it as three
groups. `at_rest` is a verdict **against** the document, and it
carries the findings: each `AtRestFinding`'s `attribution.relation`
says which of the three things happened — `refuted` (the faces do not
meet as the declaration claims), `declined` (no certifier lane for a
face the declaration names, so nothing was decided either way), or
`unattributed` (no declaration answers at all — an **undeclared
contact**, which is the hard error by definition, and the same refusal
the fail-loud tour meets on a single-document boolean). Where a
finding is `refuted` or `unattributed`, `attribution.declaration`
names the minted declaration it is about, by the two stable names the
mate was authored in — the recourse is in the error. `uncertified` is the declared
direction's **frontier**: nothing refuted, nothing undeclared, the
census simply declined to certify, so nothing was decided either way.
Everything else — `mate_reference_refused`, `no_at_rest_record`, and
the gather's own tags — refuses before any verdict exists.

That middle group is worth internalising, because it is the one place
on this page where a refusal is not a statement about your model. A
frontier is the library telling you what it cannot yet decide, which
is a different sentence from "your assembly is wrong", and it is
spelled differently on purpose.

## 6. Moving a part in or out: `split`, `inline`, and the pin door

`split(doc, cut, part_id)` cuts a closed node set out into a new
document and leaves **one instance** of it behind. `inline(doc,
instance, resolver)` is its inverse: splice the referenced document's
nodes back in.

Both are **pure**. They hand back the new document *values* plus the
ordinary recorded edits that produce them, and mutate nothing — which
is what makes each atomic at your single step. Persisting a result is
the store's write side, not the refactoring's.

They differ in one way that matters: `split` reads no store, and
`inline` crosses the document seam **at the call**, under the full pin
gate. A reference whose pinned version is not what the store holds
refuses `part_pin_mismatch` rather than splicing the version on disk.

```python
import tempfile

from pncad import (
    Doc, DocEdit, DocRef, Frame, InlineError, Node, SplitError,
    Workspace, content_pin, evaluate, inline, m, product,
    random_document_id, split,
)


def prism(label, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(label)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(x * m, y * m) for x, y in corners]))
    doc.insert(Node.extrude(profile, height * m))
    return doc


store = Workspace(tempfile.mkdtemp())
post = prism("bench-post", 0.12, 0.12, 0.5)
shelf = prism("bench-shelf", 0.9, 0.30, 0.04)
store.create(post)
store.create(shelf)

layout = Doc("bench-layout")
post_i = layout.insert(Node.instantiate_part(DocRef(post.id, content_pin(post))))
shelf_i = layout.insert(Node.instantiate_part(DocRef(shelf.id, content_pin(shelf))))
layout.apply(DocEdit.set_placement(shelf_i, Frame.translation((0 * m, 0.5 * m, 0 * m))))


def volume(doc):
    return product(doc, evaluate(doc, resolver=store)).mass_properties().volume


before = volume(layout)

# Split the shelf out into a part of its own. Identity is never
# defaulted: the new document's id is supplied by the caller, and a
# fresh one is what lets both documents live in one store.
outcome = split(layout, [shelf_i], random_document_id())
assert volume(layout) == before, "pure: the original is untouched"
assert outcome.instance in outcome.remainder.roots
assert outcome.remainder.reference(outcome.instance) is not None
assert len(outcome.node_map) == 1
# No mate spanned the cut, so nothing crossed the new seam. An
# instance you authored by hand crosses nothing either — a non-empty
# interface record is mintable only by a split that OBSERVED
# declarations crossing.
assert outcome.remainder.interface(outcome.instance).crossings == []

# Persisting is the store's job, and then the remainder measures the
# same material through one more document boundary.
store.create(outcome.part)
assert volume(outcome.remainder) == before

# ...and inlining it back returns the material exactly.
spliced = inline(outcome.remainder, outcome.instance, store)
assert volume(spliced.doc) == before
assert len(spliced.edits) > 0

# The refusals name what they refused.
try:
    split(layout, [], random_document_id())
    raise AssertionError("expected a typed refusal")
except SplitError as refusal:
    assert refusal.variant == "empty_cut"
try:
    split(layout, [post_i], layout.id)
    raise AssertionError("expected a typed refusal")
except SplitError as refusal:
    assert refusal.variant == "part_id_collides"
    assert refusal.id == layout.id

# Inline crosses the seam AT THIS CALL: the shelf moves on disk and
# the stale pin refuses under the seam's own tag, rather than
# splicing whatever the store now holds.
store.resave(prism("bench-shelf", 0.9, 0.30, 0.08))
try:
    inline(layout, shelf_i, store)
    raise AssertionError("expected a typed refusal")
except InlineError as refusal:
    assert refusal.variant == "part_pin_mismatch"
```

### Accepting a new version, honestly

A pin that moved refuses, everywhere. Accepting the new version is
therefore its own recorded edit, and there are three doors, separated
by *what each one reads and when*:

| door | reads the store | what it gives you |
|---|---|---|
| `update_references(doc, id, new_pin)` | **never** | the edits moving every site of `id` onto `new_pin`, taken as given |
| `Workspace.update_to_store(doc, id)` | **once, at the call** | the same edits, with the pin computed from disk — a snapshot, not a subscription |
| `mixed_pins(doc)` | never | the lint: every referenced id carrying more than one pin |

The middle row is the one that bites if you skim it. The edits carry
the pin as a *literal*; nothing re-reads later, so a resave between
the call and your `apply` leaves the applied pin naming the older
version. That is a snapshot by design — an edit whose meaning depended
on which store was mounted when it replayed would not be a recorded
edit at all.

`mixed_pins` **reports and never gates**. Nothing calls it from
`apply`, `load` or evaluation: a document in mixed-pin state is valid
at all three and stays that way, because a staged migration *is* that
state.

```python
import tempfile

from pncad import (
    Doc, DocRef, Node, UpdateError, Workspace, content_pin, m,
    mixed_pins, update_references,
)


def prism(label, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(label)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(x * m, y * m) for x, y in corners]))
    doc.insert(Node.extrude(profile, height * m))
    return doc


store = Workspace(tempfile.mkdtemp())
shelf = prism("bench-shelf", 0.9, 0.30, 0.04)
store.create(shelf)
old_pin = content_pin(shelf)

doc = Doc("bench-layout")
first = doc.insert(Node.instantiate_part(DocRef(shelf.id, old_pin)))

# The shelf is thickened on disk; a second instance is added at the
# NEW version while the first still names the old one. That is a
# staged migration, and it is a legal state.
thicker = prism("bench-shelf", 0.9, 0.30, 0.08)
store.resave(thicker)
new_pin = content_pin(thicker)
second = doc.insert(Node.instantiate_part(DocRef(shelf.id, new_pin)))

(report,) = mixed_pins(doc)
assert report.id == shelf.id
# Its pins ascend, each with the sites holding it.
assert {sites.pin for sites in report.pins} == {old_pin, new_pin}
assert {n for sites in report.pins for n in sites.nodes} == {first, second}
doc.save()          # it still saves, and it still evaluates

# "Update everywhere" stays usable FROM the staged state: the site
# that already moved contributes no edit. Apply the whole list or
# none of it — that all-or-nothing is what atomic means here.
edits = update_references(doc, shelf.id, new_pin)
assert len(edits) == 1
for edit in edits:
    doc.apply(edit)
assert mixed_pins(doc) == []

# The two empty arms are separate refusals because the recourses
# differ: one says "you are already there", the other "nothing here
# references that document at all".
try:
    update_references(doc, shelf.id, new_pin)
    raise AssertionError("expected a typed refusal")
except UpdateError as refusal:
    assert refusal.variant == "already_pinned"
    assert refusal.pin == new_pin
try:
    update_references(doc, Doc("elsewhere").id, new_pin)
    raise AssertionError("expected a typed refusal")
except UpdateError as refusal:
    assert refusal.variant == "no_such_reference"
    assert refusal.pin is None
```

## Where the edges still are

Three honest limits, so you do not spend an afternoon looking for a
door that is not there.

**Mates and patterns do not compose** (issue #945). A patterned family
is one node whose placements are a rule; a mate names an instance. The
bench's flat-pack layout and its assembled stand are two documents for
this reason, not one.

**A mate frame is not minted from a face** (issue #944). You select
the face for the reference and *author* the frame separately, and
nothing checks that the two agree — which is why the gate exists, and
why a mate that solves is not yet a mate that certifies.

**A mate is a product root.** Roots are the live nodes nothing else
consumes, and a mate is consumed by nothing, so `Doc.roots` on the
bench stand is three instances *and* two mates. `product` gathers only
the body-denoting ones, so this costs nothing until you call
`set_roots` — which is total, and therefore wants the mates listed
alongside the bodies just to reorder the solids. It is coherent (the
alternative is a live node reaching no root, which is a silently dead
subgraph) and it is still a surprise the first time.

For the wider picture of what the Python surface can and cannot author
today, `docs/guide/north-star-audit.md` keeps the row-by-row account.
