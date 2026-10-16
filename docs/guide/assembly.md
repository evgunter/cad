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
`test_assembly_eval.py` and `test_assembly_author.py` (which build
one scene, `crates/pncad-py/tests/bench_scene.py`), and the same scene
in Rust is `demos/tour/src/assembly.rs`, which goes one step further:
it stands the stand on a turntable — the near post's world pin, whose
placement chain carries a step that reads a `swing` parameter — and
sets a crate on the shelf by a mate in the crate's own bundle.

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
its body. Its write side is deliberately small — `create` and
`resave` for the refactorings, and `save_at` and
`save_as_new_document` for the two acts a save is — and there is no
general mutation API. A save at a path KEEPS the document's identity
and refuses typed rather than letting a copy claim it a second time;
saving it as a NEW document mints a fresh id, an explicit fork.

```python
import tempfile

from pncad import (
    ContentPin,
    Doc,
    DocRef,
    Formula,
    Node,
    PIN_MISMATCH_RECOURSE,
    Workspace,
    WorkspaceError,
    content_pin,
    m,
)


def prism(seed, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(seed)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners], plane=doc.sketch_frame()))
    doc.place(doc.insert(Node.extrude(profile, Formula.length_in(height, m))))
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
assert thicker.id == shelf.id, "same seed, same part"
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

Three doors, and one property that explains all three: *an instance is
a placement*.

`Node.instantiate_part(reference, constraints)` is an instance: a
**leaf** whose material crosses the document seam, and a
**placement** — its part's world, moved as one copy by the placement's
**bundle** of constraints. The bundle is what says where the copy
sits. Leave it out and the bundle is one world pin at the identity, so
the instance stands where its part's world stands;
`[Mate.world_pin(pose)]` stands it at `pose`; and `[]` is an empty
bundle, a **loose** copy that lives in a space of its own until a
constraint pins it. An instance pinned to the world is in the product
from the moment it is inserted, because a placement is what the
product gathers: nothing else has to place it.

`Mate(a_at, a, b_at, b, class_, alignment)` is one value carrying both
halves of "these two parts meet here": the placement constraint the
solve folds, and the contact declaration the gate mints. A mate is not
a node. It is a constraint in the bundle of the copy it places: one of
its sides is on that copy, and the other is the target the copy is
placed against. `Mate.world_pin(pose)` is the same kind of value with
the world as its target — one coincidence of the world's frame, moved
by `pose`, with the copy's own.

`Doc.constrain(placement, mate)` adds a mate to a placement's bundle
and answers its address, a `MateRef`: the placement and the id the
edit minted, never a position. `DocEdit.add_constraint` is the same
edit as a recorded value; `DocEdit.remove_constraint` takes a
constraint out by its address, and what it pinned is recomputed from
what remains, so a copy left unpinned is loose rather than refused.
`Doc.bundle(placement)` reads a bundle back.

So a mated assembly is one copy pinned to the world, and every other
copy placed by the mates in its own bundle. A bundle that pins its
copy fully puts it in its target's space; a copy pinned, directly or
through the copies it is placed on, to the world is in the world's
space, and the world's space is the product.

Each side of a mate is TWO things. `a` and `b` are instance-qualified
entity names — the text `Evaluation.select` answers with when you
query it on an instantiate node, so no name is ever composed by hand.
`a_at` and `b_at` are the nodes those names are read at, which is to
say the geometry the mate is talking about: each side is a read of a
face of that body, exactly as any other node reads a variable. They
are the instances themselves in the plain case. They diverge the
moment something moves an instance — a `Node.transform` moves a body
and mints no name of its own, so the name alone cannot tell a mate on
the instance from a mate on the transformed instance, and the body
read is what does. The solve walks from the body read down to the
minting instance and composes the map of everything it passes.

```python
import tempfile

from pncad import (
    Alignment,
    AxisSense,
    CapEnd,
    ContactClass,
    Doc,
    DocRef,
    EntityKind,
    Formula,
    Frame,
    Mate,
    MateFrame,
    MatePrimitive,
    NamePat,
    Node,
    Placement,
    ProductError,
    SegPat,
    SegTag,
    Selector,
    Workspace,
    content_pin,
    evaluate,
    m,
    product,
    relative_freedom_components,
)

POST_SECTION, POST_HEIGHT = 0.12, 0.5
SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS = 0.9, 0.30, 0.04


def prism(seed, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(seed)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners], plane=doc.sketch_frame()))
    doc.place(doc.insert(Node.extrude(profile, Formula.length_in(height, m))))
    return doc


def instance_cap(ev, instance, side):
    """One instance's cap face, in the ASSEMBLY's names.

    The part's own cap name, seen two wrappers deeper: `Placed` is the
    segment the part's world placement puts round its body's names,
    and `InPart` the one the instantiate seam adds. Nothing reads
    inside a name — this is the same query, nested.
    """
    cap = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Cap).side(side))
    copy = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Placed).of([cap]))
    through = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.InPart).of([copy]))
    found = ev.select(instance, Selector.of(through))
    assert len(found) == 1, f"expected one face, got {found}"
    return found[0]


def frame_at(x, y, z):
    """An AUTHORED mate frame: +z axis, +x clocking reference."""
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
# The near post is the one copy pinned to the world, at a pose: inset
# in y so the shelf overhangs front and back.
near = Placement.literal(
    Frame.translation((0 * m, (SHELF_DEPTH - POST_SECTION) / 2 * m, 0 * m))
)
post_a = stand.insert(Node.instantiate_part(post_ref, [Mate.world_pin(near)]))
# The other two start with empty bundles: loose copies, each in a
# space of its own, until a mate places it.
shelf_i = stand.insert(Node.instantiate_part(shelf_ref, []))
post_b = stand.insert(Node.instantiate_part(post_ref, []))

# The mate references, SELECTED rather than spelled: evaluate against
# the store, then ask each instantiate node for its face.
ev = evaluate(stand, resolver=store)
assert set(ev.loose) == {shelf_i, post_b}
a_top = instance_cap(ev, post_a, CapEnd.End)
b_top = instance_cap(ev, post_b, CapEnd.End)
shelf_underside = instance_cap(ev, shelf_i, CapEnd.Start)

# Where each post's top meets the shelf's underside, each written in
# its OWN part's coordinates. The post's seat IS its top cap face —
# the face the post side's reference already names, so the frame
# takes nothing, and one `own_face` serves every side framed on its
# own head: the solve reads the cap's pose off the post's
# evaluation every time, so a post whose height changes moves the
# seat with it. The shelf's seats are authored numbers in the shelf's
# own coordinates — the part base with one literal step. (Each could
# equally be the underside face with an offset inside it,
# `MateFrame.on_face(...)`; the shelf is modelled from its underside
# up, so no shelf edit moves these.) The posts sit flush with the
# shelf's two ends, which is the obvious way to draw a bench.
own_face = MateFrame.from_face()
seat_a = frame_at(POST_SECTION / 2, SHELF_DEPTH / 2, 0.0)
seat_b = frame_at(SHELF_LENGTH - POST_SECTION / 2, SHELF_DEPTH / 2, 0.0)


def seat(a, b):
    """Two frames meeting outright, axes aligned, no clocking rider."""
    return Alignment(a, b, MatePrimitive.frame_coincidence(), AxisSense.Aligned)


# Each mate goes in the bundle of the copy it PLACES, and names that
# copy's side first: the shelf on the near post, then the far post
# under the shelf. The door asks the solve's own admission of each
# mate, and a side framed on its face is resolved there — through the
# store, since the face is the part's.
mate_a = stand.constrain(
    shelf_i,
    Mate(shelf_i, shelf_underside, post_a, a_top, ContactClass.Rest,
         seat(seat_a, own_face)),
    resolver=store,
)
mate_b = stand.constrain(
    post_b,
    Mate(post_b, b_top, shelf_i, shelf_underside, ContactClass.Rest,
         seat(own_face, seat_b)),
    resolver=store,
)

# A mate is addressed by the placement that holds it. The near post's
# bundle is its world pin; each other bundle is the one mate that
# places it.
assert (mate_a.placement, mate_b.placement) == (shelf_i, post_b)
assert [ref for ref, _ in stand.bundle(shelf_i)] == [mate_a]
assert [ref for ref, _ in stand.bundle(post_b)] == [mate_b]
[(_, pin)] = stand.bundle(post_a)
assert (pin.a, pin.b) == ("world", "copy")
assert pin == Mate.world_pin(near)

# Pinned, through the mates, to the world: no copy is loose, and every
# copy is in the world's space.
ev = evaluate(stand, resolver=store)
assert ev.loose == []
assert all(ev.space(n) == "world" for n in (post_a, shelf_i, post_b))

# A mate's sides are reads, like any other node's, and its reads are
# what couple the graph: all three instances are one relatively-free
# component.
(component,) = [c for c in relative_freedom_components(stand) if shelf_i in c]
assert {post_a, shelf_i, post_b} <= set(component)

# The world says what this document IS: its product is every copy its
# placements define in the world's space, in the placements' document
# order. The instances ARE the placements, so the product is the three
# parts, and nothing else placed them. A mate is no body, so it is
# never in the product.
assert stand.placements() == [post_a, shelf_i, post_b]
assert [stand.node_kind(n) for n in stand.placements()] == ["instantiate_part"] * 3
volume = product(stand, ev).mass_properties().volume
expected = 2 * POST_SECTION**2 * POST_HEIGHT + SHELF_LENGTH * SHELF_DEPTH * SHELF_THICKNESS
assert abs(volume - expected) < 1e-12

# A loose copy is placed, but in a space of its own, so it is not in
# the product. A document holding nothing else has no product, and the
# gather says so: no unplaced body, and the loose copy named in the
# message.
flat = Doc("bench-loose")
alone = flat.insert(Node.instantiate_part(post_ref, []))
try:
    product(flat, evaluate(flat, resolver=store))
    raise AssertionError("expected a typed refusal")
except ProductError as refusal:
    assert refusal.variant == "empty_product"
    assert refusal.unplaced_bodies == []
    assert "loose" in str(refusal)
```

Two things in that block are worth pausing on.

**A mate frame is a base and an offset.** The base is the side's
part frame, its own face, or the world's frame; the offset is a
`Placement` written in the base's frame — any rigid motion, the empty
chain by default.
`MateFrame.from_face()` is the face base with no offset: the side's
frame is the face its reference names, and the solve resolves it from
the part's own evaluation at every evaluation — its origin, its chart
axis as local +Z and the carrier's own reference direction as local
+Y — so nothing is stored twice and the mate follows the face when
the part is edited, when the reference is rebound, and across a split
or an inline. `MateFrame.on_face(offset)` adds an offset in that
face's frame: a slide along the face, a turn about its normal, a
set-back from it — and the side follows the face, offset and all.
`MateFrame(origin, axis, reference)` is three authored vectors, the
part base with one literal step, and `MateFrame.on_part(offset)` the
part base with any chain. `MateFrame.on_world(offset)` is the world's
frame with an offset in world coordinates: the frame of a side that
reads the world, which is what a world pin's `a` side is. A rigid step's expressions may read a
document parameter, so a parameter can drive where a side sits. The
offset is any rigid motion, and the mate's contact class says which
are legal: a `Rest` side set back from its face declares a contact
that is not there, and the at-rest gate refutes it. A face with no
canonical frame (a NURBS carrier) refuses typed and keeps taking a
part base. Every frame resolves on every lane: `stackup.sensitivities`'
dual passes read a pose with its tangent, and a certified
`clearance`'s interval leaf an enclosure of it — a face base's pose,
and an offset whose step reads a parameter the lane binds. Two honest
limits stand at the placement door, where a solved pose becomes a
placed body: a box wider than about ε over a placer's translation
refuses there (each placed edge is certified on its placed carrier,
and the box widens both apart), and a boxed rotation refuses
`NotRigid`, so such an interval leaf refuses rather than encloses. The
solve's *algorithm* is coset intersection over decided predicates, no
numeric fitting, and its inputs are the document plus its mated parts'
evaluations. What is still not checked is a PART
frame against the faces the mate names: such a mate can
solve perfectly and still be refuted at the gate, which is the
boundary between "where you said the parts meet" and "where they
actually do", kept visible.

**What the solve would refuse about a mate on its own, the constraint
door refuses.** A head that resolves to no member, one member named
twice, a class outside the vocabulary, a frame offset that does not
evaluate, a frame with no definite direction, a primitive-and-rider
pair the coset table has no row
for, a clocking rider that contradicts the frame coincidence it rides
— each is a fact about the mate alone, which the solve records
against the mate whenever it reads the datum. So `Doc.constrain` (and
`DocEdit.add_constraint`, through `apply`) asks the solve's own
per-mate admission and raises `EditError` with variant
`mate_refused`, `fault` carrying the solve's `MateFault` whole. The
door also asks about the bundle the mate joins: a mate added to a
bundle that already pins its copy refuses `mate_overconstrained`
(§5). The rider on a coincidence is decided over the mated parts'
extent, and a side framed on its face is resolved from the part's own
evaluation, so those two need `resolver=` at the door; everything
else is decided on the datum alone. The doors decide edits and the
solve decides states: a verdict about a bundle's state — two mates
that contradict each other — is the solve's, met at evaluation as
below, and so is a per-mate fault a mate comes to carry after it is
added — a head a rebind or a shrunk pattern strands, a `Part`
re-pointed at another copy, a file whose snapshot holds one. A bundle
that pins less than its copy needs is no verdict at all: that copy is
loose (§5).

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

`product(doc, evaluation)` is then what an assembly *is*: its world,
every copy a placement defines in the world's space, gathered in the
placements' document order into one body. It is the only useful
reading of an assembly document, because an assembly's nodes are
instances, each one copy of its part with its mates in its bundle, and
no single node's value is the assembly.

```python
import tempfile

from pncad import (
    Doc,
    DocRef,
    EvaluationError,
    Formula,
    Frame,
    Mate,
    Node,
    Placement,
    Workspace,
    content_pin,
    evaluate,
    m,
    product,
)

POST_SECTION, POST_HEIGHT = 0.12, 0.5
SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS = 0.9, 0.30, 0.04


def prism(seed, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(seed)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners], plane=doc.sketch_frame()))
    doc.place(doc.insert(Node.extrude(profile, Formula.length_in(height, m))))
    return doc


store = Workspace(tempfile.mkdtemp())
post = prism("bench-post", POST_SECTION, POST_SECTION, POST_HEIGHT)
shelf = prism("bench-shelf", SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS)
store.create(post)
store.create(shelf)

# The flat-pack layout: a post and the shelf beside it, nothing
# touching. No mates: each instance is pinned to the world — the post
# at the identity, the shelf at a pose beside it — and being pinned to
# the world is what puts each in the product.
beside = Placement.literal(Frame.translation((0 * m, 0.5 * m, 0 * m)))
layout = Doc("bench-layout")
post_i = layout.insert(Node.instantiate_part(DocRef(post.id, content_pin(post))))
shelf_i = layout.insert(
    Node.instantiate_part(DocRef(shelf.id, content_pin(shelf)), [Mate.world_pin(beside)])
)

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
    Doc,
    DocRef,
    EvaluationError,
    Formula,
    Node,
    Workspace,
    content_pin,
    evaluate,
    m,
)


def prism(seed, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(seed)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners], plane=doc.sketch_frame()))
    doc.place(doc.insert(Node.extrude(profile, Formula.length_in(height, m))))
    return doc


store = Workspace(tempfile.mkdtemp())
shelf = prism("bench-shelf", 0.9, 0.30, 0.04)
store.create(shelf)

layout = Doc("bench-layout")
shelf_i = layout.insert(Node.instantiate_part(DocRef(shelf.id, content_pin(shelf))))

# Two nodes run: the instance, and the world every document holds.
before = evaluate(layout, resolver=store)
assert (before.reused, before.recomputed) == (0, 2)
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
assert (memoized.reused, memoized.recomputed) == (2, 0)
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

`solve_document(doc, resolver=store)` folds the bundles: each
placement's bundle by itself, its mates intersected as exact cosets,
in the placements' schedule order. What it produces is each
placement's **pose** in its target's space — where its bundle puts its
copy relative to the copy it is placed against, or to the world —
read back through `SolvedPoses.pose`, and `SolvedPoses.placement`
composes those poses into a world pose. It is **total** — a refusing
bundle must not fail an unrelated one, so refusals are read back per
placement through `SolvedPoses.fault` rather than raised, and a
refusal reaches only the refusing placement and the placements pinned
to it. It inspects no geometry except each mated part's own extent —
an upper bound taken from its evaluated body, which is why it crosses
the same `resolver=` seam `evaluate` does — and that extent enters
only as the lever a parallelism verdict is decided over: a tilt is
priced across the parts that carry it, at their scale. Without a
resolver there is no lever, and a mate on a part faults
`mate_unleverable`.

What a bundle decides is a copy's **space**. `Evaluation.space(node)`
answers `"world"` for a copy pinned to the world, directly or through
the copies it is placed on, and otherwise the loose copy whose space
it is; `Evaluation.loose` lists the loose copies. Nothing in one space
is compared with another's, so a copy is either in the product or
checked by itself, never half of each.

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
    Alignment,
    AxisSense,
    CapEnd,
    ContactClass,
    Doc,
    DocRef,
    EntityKind,
    Formula,
    Frame,
    Mate,
    MateFrame,
    MatePrimitive,
    NamePat,
    Node,
    Placement,
    SegPat,
    SegTag,
    Selector,
    Workspace,
    assemble,
    content_pin,
    evaluate,
    m,
    product,
    solve_document,
)

POST_SECTION, POST_HEIGHT = 0.12, 0.5
SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS = 0.9, 0.30, 0.04
INSET = (SHELF_DEPTH - POST_SECTION) / 2


def prism(seed, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(seed)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners], plane=doc.sketch_frame()))
    doc.place(doc.insert(Node.extrude(profile, Formula.length_in(height, m))))
    return doc


def frame_at(x, y, z):
    return MateFrame(origin=(x * m, y * m, z * m), axis=(0.0, 0.0, 1.0),
                     reference=(1.0, 0.0, 0.0))


def instance_cap(ev, instance, side):
    cap = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Cap).side(side))
    copy = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Placed).of([cap]))
    through = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.InPart).of([copy]))
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
    near = Placement.literal(Frame.translation((0 * m, INSET * m, 0 * m)))
    a = doc.insert(Node.instantiate_part(post_ref, [Mate.world_pin(near)]))
    s = doc.insert(Node.instantiate_part(shelf_ref, []))
    b = doc.insert(Node.instantiate_part(post_ref, []))
    ev = evaluate(doc, resolver=store)
    post_seat = frame_at(POST_SECTION / 2, POST_SECTION / 2, POST_HEIGHT)
    seat_a = frame_at(POST_SECTION / 2, SHELF_DEPTH / 2, 0.0)
    seat_b = frame_at(SHELF_LENGTH - POST_SECTION / 2, SHELF_DEPTH / 2, 0.0)
    fold = primitive or MatePrimitive.frame_coincidence()

    def align(x, y):
        return Alignment(x, y, fold, AxisSense.Aligned)

    # Each mate goes in the bundle of the part it places.
    m_a = doc.constrain(
        s,
        Mate(s, instance_cap(ev, s, CapEnd.Start),
             a, instance_cap(ev, a, CapEnd.End),
             class_, align(seat_a, post_seat)),
    )
    m_b = doc.constrain(
        b,
        Mate(b, instance_cap(ev, b, CapEnd.End),
             s, instance_cap(ev, s, CapEnd.Start),
             class_, align(post_seat, seat_b)),
    )
    return doc, (a, s, b), (m_a, m_b)


stand, (post_a, shelf_i, post_b), mates = bench()

solved = solve_document(stand, resolver=store)
assert all(solved.fault(n) is None for n in (post_a, shelf_i, post_b))
# Every bundle pins its copy, and the near post is pinned to the
# world, so all three copies are in the world's space.
ev = evaluate(stand, resolver=store)
assert ev.loose == []
assert [ev.space(n) for n in (post_a, shelf_i, post_b)] == ["world"] * 3

# The near post is pinned to the world, so its world placement is its
# pin's pose verbatim...
assert solved.placement(stand, post_a).origin == (0 * m, INSET * m, 0 * m)
# ...and each other pose is solved in its TARGET's space and composed
# outward along the mates, never stored. The shelf's pose is relative
# to the near post it is placed on; composed onto that post's pin, it
# sits on top of the posts.
on_post = solved.pose(shelf_i).origin
assert abs(on_post[1].meters + INSET) < 1e-12
assert abs(on_post[2].meters - POST_HEIGHT) < 1e-12
in_world = solved.placement(stand, shelf_i).origin
assert abs(in_world[1].meters) < 1e-12
assert abs(in_world[2].meters - POST_HEIGHT) < 1e-12
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
# alphabet the mates were authored in — each naming its mate by its
# address.
assembly = assemble(stand, ev)
assert [d.mate for d in assembly.minted] == list(mates)
assert [d.class_ for d in assembly.minted] == [ContactClass.Rest] * 2
expected = (2 * POST_SECTION**2 * POST_HEIGHT
            + SHELF_LENGTH * SHELF_DEPTH * SHELF_THICKNESS)
assert abs(assembly.body.mass_properties().volume - expected) < 1e-12

# `assemble` IS the gather plus the check, so its body is the gathered
# body — and a mate-less, disjoint layout passes the same gate
# outright, with nothing minted.
gathered = product(stand, ev)
assert gathered.mass_properties().volume == assembly.body.mass_properties().volume
```

## 5. The refusals an assembly author actually meets

Each of these is reached by authoring the mistake. They are the
product working: the kernel declining to pick a pose, invent a record,
or widen a declaration you did not make. The first is not a refusal
at all: it is the kernel declining to pick a pose without refusing
anything.

```python
import tempfile

from pncad import (
    Alignment,
    AssemblyError,
    AxisSense,
    CapEnd,
    ContactClass,
    Doc,
    DocEdit,
    DocRef,
    EditError,
    EntityKind,
    EvaluationError,
    Formula,
    MateFrame,
    Mate,
    MatePrimitive,
    NamePat,
    Node,
    OVERCONSTRAINED_RECOURSE,
    ProductError,
    SegPat,
    SegTag,
    Selector,
    Workspace,
    assemble,
    content_pin,
    evaluate,
    m,
    product,
    solve_document,
)

POST_SECTION, POST_HEIGHT = 0.12, 0.5
SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS = 0.9, 0.30, 0.04


def prism(seed, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(seed)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners], plane=doc.sketch_frame()))
    doc.place(doc.insert(Node.extrude(profile, Formula.length_in(height, m))))
    return doc


def frame_at(x, y, z):
    return MateFrame(origin=(x * m, y * m, z * m), axis=(0.0, 0.0, 1.0),
                     reference=(1.0, 0.0, 0.0))


def instance_cap(ev, instance, side):
    cap = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Cap).side(side))
    copy = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Placed).of([cap]))
    through = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.InPart).of([copy]))
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
    """A loose post — its bundle empty, so a mate is what places it —
    and the shelf, pinned to the world at the identity."""
    doc = Doc("bench-refusals")
    a = doc.insert(Node.instantiate_part(post_ref, []))
    s = doc.insert(Node.instantiate_part(shelf_ref))
    return doc, a, s


def post_on_shelf(doc, post_i, shelf_i, class_, primitive):
    """A mate placing the post under the shelf: the post's side first."""
    ev = evaluate(doc, resolver=store)
    return Mate(
        post_i, instance_cap(ev, post_i, CapEnd.End),
        shelf_i, instance_cap(ev, shelf_i, CapEnd.Start),
        class_, Alignment(post_seat, seat_a, primitive, AxisSense.Aligned),
    )


# 1. A LOOSE COPY. One planar rest fixes the seating plane and nothing
#    else: the post may still slide and spin in it. That is no
#    refusal, and the solve picks no pose: a bundle that pins less than
#    its copy needs makes a LOOSE copy, a space of its own. Nothing
#    outside that space is compared with it, so it has no world pose
#    and is not in the product — the world here is the shelf alone.
doc, post_i, shelf_i = two_instances()
doc.constrain(
    post_i,
    post_on_shelf(doc, post_i, shelf_i, ContactClass.Rest,
                  MatePrimitive.planar_rest(0 * m)),
    resolver=store,
)
solved = solve_document(doc, resolver=store)
assert solved.fault(post_i) is None, "a loose copy is not a refusal"
ev = evaluate(doc, resolver=store)
assert ev.loose == [post_i]
assert (ev.space(post_i), ev.space(shelf_i)) == (post_i, "world")
try:
    solved.placement(doc, post_i)
    raise AssertionError("expected a typed refusal")
except EvaluationError as refusal:
    assert refusal.kind == "space_mismatch"
shelf_volume = SHELF_LENGTH * SHELF_DEPTH * SHELF_THICKNESS
assert abs(product(doc, ev).mass_properties().volume - shelf_volume) < 1e-12

# 2. OVERCONSTRAINED. A mate added to a bundle that already pins its
#    copy says nothing the bundle does not, and the door refuses it.
#    The commonest way in: an instance inserted with no bundle given is
#    pinned to the world at the identity, and a mate added to it on
#    top of that pin over-determines it. The fault names the placement
#    and every constraint whose removal leaves the copy as pinned as it
#    was, and ends on the recourse.
doc = Doc("bench-overconstrained")
post_i = doc.insert(Node.instantiate_part(post_ref))
shelf_i = doc.insert(Node.instantiate_part(shelf_ref))
seated = post_on_shelf(doc, post_i, shelf_i, ContactClass.Rest,
                       MatePrimitive.frame_coincidence())
[(pin, _)] = doc.bundle(post_i)
try:
    doc.constrain(post_i, seated, resolver=store)
    raise AssertionError("expected a typed refusal")
except EditError as refusal:
    assert (refusal.variant, refusal.inner_variant) == (
        "mate_refused", "mate_overconstrained",
    )
    assert refusal.fault.placement == post_i
    assert refusal.fault.constraints
    assert all(c.placement == post_i for c in refusal.fault.constraints)
    assert OVERCONSTRAINED_RECOURSE in str(refusal)
assert doc.bundle(post_i) == [(pin, Mate.world_pin())], "the refused edit left the bundle"
# The repair is the one the recourse names: remove the pin, and the
# mate is what places the post.
doc.apply(DocEdit.remove_constraint(pin))
joint = doc.constrain(post_i, seated, resolver=store)
assert [ref for ref, _ in doc.bundle(post_i)] == [joint]
assert evaluate(doc, resolver=store).space(post_i) == "world"

# 3. A CLASS THE GATE CANNOT MINT. `Tangent` solves and mints nothing
#    at rest — exactly what `class_admission` said before the edit
#    landed.
doc, post_i, shelf_i = two_instances()
mate = doc.constrain(
    post_i,
    post_on_shelf(doc, post_i, shelf_i, ContactClass.Tangent,
                  MatePrimitive.frame_coincidence()),
)
try:
    assemble(doc, evaluate(doc, resolver=store))
    raise AssertionError("expected a typed refusal")
except AssemblyError as refusal:
    # `refusals` is EVERY mate that did not mint, in document order,
    # so two broken mates are two repairs from one call.
    assert refusal.variant == "unminted_mates"
    (row,) = refusal.refusals
    assert row.variant == "no_at_rest_record"
    assert row.mate == mate
    assert row.class_ == ContactClass.Tangent

# 4. A HEAD THAT IS NOT A FACE — refused where the mate is BUILT, not
#    at the gate. A mate declares a FACE PAIR, and the kernel says so
#    in the TYPE of a head, so a Rust caller cannot write this mate at
#    all. Python holds names as opaque text, so `Mate` asks the
#    same constructor on your behalf and refuses at the call.
doc, post_i, shelf_i = two_instances()
ev = evaluate(doc, resolver=store)
edge = sorted(ev.all_edges(post_i))[0]
try:
    Mate(
        post_i, edge,
        shelf_i, instance_cap(ev, shelf_i, CapEnd.Start),
        ContactClass.Rest,
        Alignment(post_seat, seat_a, MatePrimitive.frame_coincidence(),
                  AxisSense.Aligned),
    )
    raise AssertionError("expected a typed refusal")
except EditError as refusal:
    assert refusal.variant == "mate_head_not_a_face"
    assert "edge" in str(refusal)

# 5. NOTHING TO GATHER. Evaluated with no resolver, the shelf instance
#    — a placement pinned to the world — produced no copies, so the
#    GATHER refuses before the gate runs — and it refuses under the
#    gather's own tag, not a wrapper's, because which invariant broke
#    is what you branch on. `node` is the placement that failed.
doc, _, shelf_i = two_instances()
try:
    assemble(doc, evaluate(doc))
    raise AssertionError("expected a typed refusal")
except AssemblyError as refusal:
    assert refusal.variant == "root_failed"
    assert refusal.node == shelf_i and refusal.refusals is None
try:
    product(doc, evaluate(doc))
    raise AssertionError("expected a typed refusal")
except ProductError as refusal:
    assert refusal.variant == "root_failed"
    assert refusal.node == shelf_i
```

Read an `AssemblyError`'s `variant` first, and read it as three
groups. `at_rest` is a verdict **against** the document, and it
carries the findings: each `AtRestFinding`'s `attribution` is the
declaration it names, `None` where it names none, and
`attribution.relation` says which of two things happened — `refuted`
(the faces do not meet as the declaration claims) or `declined` (no
certifier lane for a face the declaration names, so nothing was
decided either way). `attribution.declaration` names the minted
declaration, by the two stable names the mate was authored in — the
recourse is in the error. Two copies that touch where no mate relates
them refuse nothing: the touch is a row of the product, and
`run_checks` reports it as an `unproven_coincidence` finding whose
`copies` are the two. `uncertified` is the declared direction's
**frontier**: nothing refuted, the census simply declined to certify,
so nothing was decided either way.
Everything else refuses before any verdict exists:
`unminted_mates` (this document's own mates that did not mint),
`carried_mint_refusal` (the same for mates of documents below it), and
the gather's own tags. The two mint arms carry `refusals` — **every**
mate that did not mint, in document order, never just the first — and
each row carries its own word, `mate_reference_refused` or
`no_at_rest_record`.

That middle group is worth internalising, because it is the one place
on this page where a refusal is not a statement about your model. A
frontier is the library telling you what it cannot yet decide, which
is a different sentence from "your assembly is wrong", and it is
spelled differently on purpose.

The gate runs the same check over each loose copy's space, by itself:
copies placed on a loose copy are compared with each other and never
with the world, so two parts that interpenetrate inside a loose
sub-assembly refuse even where the world beside them certifies.

## 6. Moving a part in or out: `split`, `inline`, and the pin door

`split(doc, cut, part_id)` cuts a closed node set out into a new
document and leaves **one instance** of it behind, pinned to the
world at the identity, where the part's world stands. `inline(doc,
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
    Doc,
    DocRef,
    Formula,
    Frame,
    InlineError,
    Mate,
    Node,
    Placement,
    SplitError,
    Workspace,
    content_pin,
    evaluate,
    inline,
    m,
    product,
    random_document_id,
    split,
)


def prism(seed, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(seed)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners], plane=doc.sketch_frame()))
    doc.place(doc.insert(Node.extrude(profile, Formula.length_in(height, m))))
    return doc


store = Workspace(tempfile.mkdtemp())
post = prism("bench-post", 0.12, 0.12, 0.5)
shelf = prism("bench-shelf", 0.9, 0.30, 0.04)
store.create(post)
store.create(shelf)

beside = Placement.literal(Frame.translation((0 * m, 0.5 * m, 0 * m)))
layout = Doc("bench-layout")
post_i = layout.insert(Node.instantiate_part(DocRef(post.id, content_pin(post))))
shelf_i = layout.insert(
    Node.instantiate_part(DocRef(shelf.id, content_pin(shelf)), [Mate.world_pin(beside)])
)


def volume(doc):
    return product(doc, evaluate(doc, resolver=store)).mass_properties().volume


before = volume(layout)

# Split the shelf instance out into a part of its own. A part delivers
# only its world, and the shelf instance is a placement pinned to the
# world, so it is a whole cut: the part's world is that placement.
# Identity is never defaulted: the new document's id is supplied by
# the caller, and a fresh one is what lets both documents live in one
# store.
outcome = split(layout, [shelf_i], random_document_id())
assert volume(layout) == before, "pure: the original is untouched"
# The remainder holds one instance of the new part, pinned to the
# world at the identity — the part's world already stands where the
# shelf did — beside the post.
assert len(outcome.remainder.placements()) == 2
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
    Doc,
    DocRef,
    Formula,
    Node,
    UpdateError,
    Workspace,
    content_pin,
    m,
    mixed_pins,
    update_references,
)


def prism(seed, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(seed)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners], plane=doc.sketch_frame()))
    doc.place(doc.insert(Node.extrude(profile, Formula.length_in(height, m))))
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
door that is not there, and one rule worth stating outright.

**Mates and patterns do not compose** (issue #945). A patterned family
is one node whose placements are a rule; a mate names an instance. The
bench's flat-pack layout and its assembled stand are two documents for
this reason, not one.

**A part base is not checked against the faces the mate names**,
which is why the gate exists, and why a mate that solves is not yet a
mate that certifies. A face base's in-plane axes are the witness
ladder's: the carrier's reference is local +Y, not +X, so an offset
along the face's reference is written along y.

**A bundle's mates all target one copy, or all the world.** A copy
held by mates onto two different copies refuses `mate_targets_differ`,
naming the placement and the constraints whose targets differ. And the
solve reads a face only of an instance, or of a pattern's copy of one:
a face side on a copy `Doc.place` made of a body refuses typed, while
a world pin works on any placement.

**Only a placement places.** The product is the copies the
document's placements define in the world's space — each instance
pinned there, and each copy `Doc.place` has made of a body — and the
placements' document order is the solid order. A mate is a constraint
in a placement's bundle and no body, so it is never in the product,
and no insert places a body as a side effect. A boolean of two placed
bodies is a new body nobody placed until you place it.

For the wider picture of what the Python surface can and cannot author
today, `docs/guide/north-star-audit.md` keeps the row-by-row account.
