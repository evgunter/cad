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
it stands the stand on a turntable — the near post's world mate, whose
frame turns about the turntable's pivot by a `swing` parameter — and
sets a crate on the shelf by a mate and values in the crate's own
bundle.

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
    profile = doc.insert(Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners]))
    doc.place(doc.insert(Node.extrude(profile, Formula.length_in(height, m))), doc.world)
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

`Node.instantiate_part(reference, target, constraints)` is an
instance: a **leaf** whose material crosses the document seam, and a
**placement** — its part's world, placed as one copy into the
**target** space and moved there by the placement's **bundle** of
constraints. The target is always written: the world (`Doc.world`), a
space of its own (`Doc.new_space()`), or a construction's own space.
The bundle is what says where the copy sits, and every bundle pins its
copy. Left out, it is the **identity** bundle — no mate, and the
constant zero on every freedom — so the copy stands where its part's
world stands: that is the "place part" gesture. A copy is moved from
there by the numbers of a bundle written in its place —
`Placed.unmated(values)`, values on the target space's own axes, a
chain of spins about them and the slides along them. An instance
placed into the world is in the product, because a placement is what
the product gathers: nothing else has to place it. `Doc.place(body,
target)` is the same gesture over a body of this document.

`Mate(on, to)` is a placement constraint the solve folds: it says where
a copy sits, and declares nothing about contact — whether two copies
touch is the geometry's to say, and the at-rest gate records every
touch between copies it finds (section 4). A mate is not a node. It is a
constraint in the bundle of the copy it places, and it equates two
poses of one kind: `on`, read off that copy's geometry, and `to`, a
pose of the placement's target space read off geometry there —
another copy's face, say. A mate reads only geometry, never the
world's frame. The kind is the
primitive: plane to plane rests, axis to axis is coaxial, frame to
frame coincides, point to point is a ball, direction to direction is
parallel. A plain plane mate equates outward normals, so two faces
resting on each other carry a flip on the target
(`PoseDef.flip(PoseDef.plane(...))`).

A mate holds no number. What the bundle's mates leave free — a plane
mate's two slides and its spin, an axis mate's slide along it and its
spin — is set by a **value**, `MateValue(Coord.slide("x"), by)`, which
names no mate. A value is charted on the target space's own axes,
swung by the shortest turn onto the one direction the mates fix, so
zero says the copy's own coordinates agree with the space's as far as
the mates allow. A standoff is not a value: it is the target plane
slid along its own normal (`PoseDef.slide`).

A bundle is written whole. `Doc.set_constraints(placement, [mate,
value, ...])` writes one in place of the bundle a placement holds and
answers each constraint's address, a `MateRef`: the placement and the
id the edit minted, never a position; `Doc.place` and
`Node.instantiate_part` take a bundle the same way, and
`DocEdit.set_constraints` is the same edit as a recorded value. A
bundle that leaves a freedom unwritten refuses (`mate_unpinned`).
`Placed.of(doc, mates, values)` is the one constructor that writes the
constant zero on each freedom its mates leave and no value names, so
its bundle always pins; `Doc.constrain` and `Doc.remove_mate` write a
bundle whole again by the same rule, so the copy stays pinned.
`Doc.bundle(placement)` reads a bundle back.

So a mated assembly is copies placed into the world, one pinned to the
world by its values alone and every other placed by the mates and
values in its own bundle onto the copies beside it. Which space a body is in is
read off the recipe, `Doc.space(node)`, with nothing evaluated, and the
world's space is the product.

A pose is read off a face of a NODE by a NAME — the instance-qualified
text `Evaluation.select` answers with when you query it on an
instantiate node, so no name is ever composed by hand. The node is the
geometry the pose is about: the instance itself in the plain case,
and a placement of it when the pose is read off a placed copy — the
node read is what says which body the face is on.

```python
import tempfile

from pncad import (
    CapEnd,
    Coord,
    Doc,
    DocRef,
    EntityKind,
    Formula,
    Mate,
    MateValue,
    NamePat,
    Node,
    Placed,
    PoseDef,
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
    Settle,
)

POST_SECTION, POST_HEIGHT = 0.12, 0.5
SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS = 0.9, 0.30, 0.04


def prism(seed, width, depth, height):
    """One part: a rectangular block, placed in its world where it is
    drawn by the "place part" gesture."""
    doc = Doc(seed)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners]))
    doc.place(doc.insert(Node.extrude(profile, Formula.length_in(height, m))), doc.world)
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


store = Workspace(tempfile.mkdtemp())
post = prism("bench-post", POST_SECTION, POST_SECTION, POST_HEIGHT)
shelf = prism("bench-shelf", SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS)
store.create(post)
store.create(shelf)
post_ref = DocRef(post.id, content_pin(post))
shelf_ref = DocRef(shelf.id, content_pin(shelf))

stand = Doc("bench-stand")
# Three copies placed into the world, each standing where its part's
# world stands by the identity bundle until a bundle written in its
# place moves it.
post_a = stand.insert(Node.instantiate_part(post_ref, stand.world))
shelf_i = stand.insert(Node.instantiate_part(shelf_ref, stand.world))
post_b = stand.insert(Node.instantiate_part(post_ref, stand.world))
assert len(stand.bundle(post_a)) == 6

# The near post is the one copy pinned to the world, at a pose: inset
# in y so the shelf overhangs front and back. Its bundle is values on
# the world's axes, the rest written zero by the one constructor.
inset = (SHELF_DEPTH - POST_SECTION) / 2
pinned = stand.set_constraints(post_a, Placed.unmated([MateValue(Coord.slide("y"), inset * m)]))
assert len(pinned) == 6

# The faces, SELECTED rather than spelled: evaluate against the store,
# then ask each instantiate node for its face.
ev = evaluate(stand, resolver=store)
a_top = instance_cap(ev, post_a, CapEnd.End)
b_top = instance_cap(ev, post_b, CapEnd.End)
shelf_underside = instance_cap(ev, shelf_i, CapEnd.Start)


def rest(placement, on, to, slides):
    """`placement`'s face `on` resting on the face `to` — the target
    flipped, so the two faces face each other — and the values that
    set where on `to` it rests: two slides, and no spin. The bundle is
    written whole."""
    mate, *_ = stand.set_constraints(placement, [
        Mate(PoseDef.plane(*on), PoseDef.flip(PoseDef.plane(*to))),
        MateValue(Coord.slide("x"), slides[0] * m),
        MateValue(Coord.slide("y"), slides[1] * m),
        MateValue(Coord.spin("z"), 0),
    ])
    return mate


# Each mate goes in the bundle of the copy it PLACES, and reads that
# copy's face as `on`: the shelf on the near post, then the far post
# under the shelf. Each mate's poses are read off the faces at every
# evaluation, so a post whose height changes moves the shelf with it.
# The values put each post's top centre under its seat on the shelf,
# flush with the shelf's ends. A value charts on the target space —
# here the world — never on the body its copy rests on: the shelf's
# slides are where its origin stands on the world's x and y, and the
# far post's on the world's x and its -y, the world's axes swung onto
# the mated normal.
half = POST_SECTION / 2
mate_a = rest(shelf_i, (shelf_i, shelf_underside), (post_a, a_top), (0.0, inset + half - SHELF_DEPTH / 2))
mate_b = rest(post_b, (post_b, b_top), (shelf_i, shelf_underside),
              (SHELF_LENGTH - 2 * half, half - SHELF_DEPTH / 2))

# A constraint is addressed by the placement that holds it. The near
# post's bundle is its values alone, no mate; each other bundle is the
# mate that places it and its three values.
assert (mate_a.placement, mate_b.placement) == (shelf_i, post_b)
assert all(isinstance(v, MateValue) for _, v in stand.bundle(post_a))
held = stand.bundle(shelf_i)
assert held[0][0] == mate_a
assert [type(v) for _, v in held[1:]] == [MateValue] * 3

# Every copy is in the world's space, read off the recipe.
world = stand.output(stand.world)
assert all(stand.space(n) == world for n in (post_a, shelf_i, post_b))
ev = evaluate(stand, resolver=store)

# A mate's poses are reads, like any other node's, and its reads are
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
volume = product(stand, ev).mass_properties(Settle.resolution(1e-6 * m)).met().volume
expected = 2 * POST_SECTION**2 * POST_HEIGHT + SHELF_LENGTH * SHELF_DEPTH * SHELF_THICKNESS
assert abs(volume - expected) < 1e-12

# A copy placed into a space of its own is placed, but not in the
# world, so it is not in the product. A document holding nothing else
# has no product, and the gather says so.
flat = Doc("bench-aside")
aside = flat.new_space()
alone = flat.insert(Node.instantiate_part(post_ref, aside))
assert flat.space(alone) == flat.output(aside)
try:
    product(flat, evaluate(flat, resolver=store))
    raise AssertionError("expected a typed refusal")
except ProductError as refusal:
    assert refusal.variant == "empty_product"
# Only a placement relates two spaces: a copy of `alone` placed into
# the world, by the identity bundle, puts the post in the product.
flat.place(alone, flat.world)
volume = product(flat, evaluate(flat, resolver=store)).mass_properties(Settle.resolution(1e-6 * m)).met().volume
assert abs(volume - POST_SECTION**2 * POST_HEIGHT) < 1e-12
```

Two things in that block are worth pausing on.

**A pose follows its face.** A mate's poses are definitions read off
geometry — a face read as its plane, with its outward normal; an edge
or a carrier read as its axis; a vertex or a centre read as a point —
and moved, where the author moves them, by slides and spins whose
numbers are lengths and angles. The solve reads each pose off the
evaluation at every evaluation, so nothing is stored twice and the
mate follows the face when the part is edited, when the name is
rebound, and across a split or an inline. A value's number may read a
document parameter, so a parameter can drive where a copy sits. A
mate claims no contact: where its values set two faces apart, they
are apart, and nothing is refuted. The solve constructs each copy's pose from its mates
and values, checking nothing and fitting nothing, and its inputs are
the document plus its mated parts' evaluations.

**The constraint door is structural.** A mate whose `on` does not read
the copy it places (`mate_not_on_the_copy`), whose two poses are of
two kinds (`mate_kinds_differ`) is refused where it is added; so is a constraint any of
whose equations the rest of its bundle already fixes, pinned or not
(`mate_overconstrained`): two plane mates fix a normal twice, whatever
their faces' angle, and a value on a freedom the mates fix is one
equation too many; so is a slide beside a ball and a direction, whose
turn the solve does not choose (`mate_no_chart`); and so is a bundle
that leaves a freedom unwritten (`mate_unpinned`). `Doc.set_constraints`, `Doc.place` and
their `DocEdit` twins raise `EditError` with variant `mate_refused`,
`fault` carrying the `MateFault` whole. A mate whose `to` reads a body
outside the placement's target refuses `space_mismatch`, as does any
node whose reads lie in two spaces: only a placement relates two
spaces, and that is decided from the recipe, before anything is
evaluated. What needs the poses' values — a face that is gone
(`mate_side_unevaluated`), a chart the band cannot swing — is the
placement's own refusal where it evaluates, met as below.

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
    Coord,
    Doc,
    DocRef,
    EvaluationError,
    Formula,
    MateValue,
    Node,
    Placed,
    Workspace,
    content_pin,
    evaluate,
    m,
    product,
    Settle,
)

POST_SECTION, POST_HEIGHT = 0.12, 0.5
SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS = 0.9, 0.30, 0.04


def prism(seed, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(seed)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners]))
    doc.place(doc.insert(Node.extrude(profile, Formula.length_in(height, m))), doc.world)
    return doc


store = Workspace(tempfile.mkdtemp())
post = prism("bench-post", POST_SECTION, POST_SECTION, POST_HEIGHT)
shelf = prism("bench-shelf", SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS)
store.create(post)
store.create(shelf)

# The flat-pack layout: a post and the shelf beside it, nothing
# touching. No mates between them: each instance is pinned to the
# world by its values — the post at the identity, the shelf slid
# beside it — and being pinned to the world is what puts each in the
# product.
layout = Doc("bench-layout")
post_i = layout.insert(Node.instantiate_part(DocRef(post.id, content_pin(post)), layout.world))
shelf_i = layout.insert(Node.instantiate_part(DocRef(shelf.id, content_pin(shelf)), layout.world))
layout.set_constraints(shelf_i, Placed.unmated([MateValue(Coord.slide("y"), 0.5 * m)]))

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

volume = product(layout, ev).mass_properties(Settle.resolution(1e-6 * m)).met().volume
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
    Settle,
)


def prism(seed, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(seed)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners]))
    doc.place(doc.insert(Node.extrude(profile, Formula.length_in(height, m))), doc.world)
    return doc


store = Workspace(tempfile.mkdtemp())
shelf = prism("bench-shelf", 0.9, 0.30, 0.04)
store.create(shelf)

layout = Doc("bench-layout")
shelf_i = layout.insert(Node.instantiate_part(DocRef(shelf.id, content_pin(shelf)), layout.world))

# One node runs: the instance. The world every document holds is
# scheduled nowhere.
before = evaluate(layout, resolver=store)
assert (before.reused, before.recomputed) == (0, 1)
pinned_volume = before.value(shelf_i).body().mass_properties(Settle.resolution(1e-6 * m)).met().volume

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
assert memoized.value(shelf_i).body().mass_properties(Settle.resolution(1e-6 * m)).met().volume == pinned_volume

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
placement's bundle by itself, where the placement evaluates — its
mates' poses read off the evaluated geometry and intersected as exact
cosets, then its values placing the copy inside what the mates leave.
What it produces is each placement's **pose** in its target space —
in the world's coordinates for a copy placed into the world — read back
through `SolvedPoses.pose`; `SolvedPoses.placement` is the world
pose. It is **total** — a
refusing bundle must not fail an unrelated one, so refusals are read
back per placement through `SolvedPoses.fault` rather than raised, and
a refusal reaches only the refusing placement and the placements
pinned to it. It crosses the same `resolver=` seam `evaluate` does,
because the poses are read off the parts' evaluations, and each mated
body's own extent enters as the lever a parallelism verdict is decided
over: a tilt is priced across the bodies that carry it, at their
scale.

What a placement's target decides is a copy's **space**, and
`Doc.space(node)` reads it off the recipe: a construction's body is in
a space of its own, a placed copy in its placement's target, an
operator's result in the space its operands share. Nothing in one
space is compared with another's, so only the copies placed into the
world are in the product.

`assemble(doc, evaluation)` is the **at-rest gate**: it gathers the
product, records every touch between two copies the census finds — a
seat a mate set up and a touch no mate relates alike, each a row of
the product proven from the copies' own cells — and runs the kernel's
own at-rest door over the product and those records together. A
mate declares nothing: whether two faces meet is the geometry's to
say, and an overlap between two copies' material is reported as
interference, never recorded as a touch.

```python
import tempfile

from pncad import (
    CapEnd,
    Coord,
    Doc,
    DocRef,
    EntityKind,
    Formula,
    Mate,
    MateValue,
    NamePat,
    Node,
    Placed,
    PoseDef,
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
    Settle,
)

POST_SECTION, POST_HEIGHT = 0.12, 0.5
SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS = 0.9, 0.30, 0.04
INSET = (SHELF_DEPTH - POST_SECTION) / 2
HALF = POST_SECTION / 2


def prism(seed, width, depth, height):
    """One part: a rectangular block, placed in its world where it is
    drawn by the "place part" gesture."""
    doc = Doc(seed)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners]))
    doc.place(doc.insert(Node.extrude(profile, Formula.length_in(height, m))), doc.world)
    return doc


def instance_cap(ev, instance, side):
    cap = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Cap).side(side))
    copy = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Placed).of([cap]))
    through = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.InPart).of([copy]))
    (found,) = ev.select(instance, Selector.of(through))
    return found


def rest(doc, placement, on, to, slides):
    """`placement`'s face `on` resting on the face `to`, flipped so the
    two face each other, and two slide values and no spin setting where
    on `to` it rests, written whole as its bundle. `on` and `to` are
    (node, face name) pairs."""
    mate, *_ = doc.set_constraints(placement, [
        Mate(PoseDef.plane(*on), PoseDef.flip(PoseDef.plane(*to))),
        MateValue(Coord.slide("x"), slides[0] * m),
        MateValue(Coord.slide("y"), slides[1] * m),
        MateValue(Coord.spin("z"), 0),
    ])
    return mate


store = Workspace(tempfile.mkdtemp())
post = prism("bench-post", POST_SECTION, POST_SECTION, POST_HEIGHT)
shelf = prism("bench-shelf", SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS)
store.create(post)
store.create(shelf)
post_ref = DocRef(post.id, content_pin(post))
shelf_ref = DocRef(shelf.id, content_pin(shelf))


def bench():
    """The stand: two posts, a shelf seated on them by two mates."""
    doc = Doc("bench-stand")
    a = doc.insert(Node.instantiate_part(post_ref, doc.world))
    doc.set_constraints(a, Placed.unmated([MateValue(Coord.slide("y"), INSET * m)]))
    s = doc.insert(Node.instantiate_part(shelf_ref, doc.world))
    b = doc.insert(Node.instantiate_part(post_ref, doc.world))
    ev = evaluate(doc, resolver=store)
    underside = (s, instance_cap(ev, s, CapEnd.Start))
    # Each mate goes in the bundle of the part it places; its values
    # put each post's top centre under the shelf's seat, flush with the
    # shelf's ends.
    m_a = rest(doc, s, underside, (a, instance_cap(ev, a, CapEnd.End)),
               (0.0, INSET + HALF - SHELF_DEPTH / 2))
    m_b = rest(doc, b, (b, instance_cap(ev, b, CapEnd.End)), underside,
               (SHELF_LENGTH - 2 * HALF, HALF - SHELF_DEPTH / 2))
    return doc, (a, s, b), (m_a, m_b)


stand, (post_a, shelf_i, post_b), mates = bench()

solved = solve_document(stand, resolver=store)
assert all(solved.fault(n) is None for n in (post_a, shelf_i, post_b))
# Every bundle pins its copy, and all three are placed into the world.
ev = evaluate(stand, resolver=store)
world = stand.output(stand.world)
assert [stand.space(n) for n in (post_a, shelf_i, post_b)] == [world] * 3


def near(got, want):
    return all(abs(g.meters - w) < 1e-12 for g, w in zip(got, want, strict=True))


# The near post stands at its pin's pose...
assert near(solved.placement(stand, post_a).origin, (0.0, INSET, 0.0))
# ...and each other pose is solved from the mates and values in its
# bundle, never stored. A copy placed into the world is posed in the
# world's coordinates, so its pose is its world placement: the shelf sits on top of the posts, flush with
# the bench's front.
assert near(solved.pose(shelf_i).origin, (0.0, 0.0, POST_HEIGHT))
assert near(solved.placement(stand, shelf_i).origin, (0.0, 0.0, POST_HEIGHT))
assert near(solved.placement(stand, post_b).origin, (SHELF_LENGTH - POST_SECTION, INSET, 0.0))
# The ROTATION, which a translation check cannot see: both mates rest
# a cap on a cap at zero spin, so the far post is not turned. A solve
# that rotated it and still landed its seating point would put the
# part in sideways.
columns = solved.placement(stand, post_b).columns
identity = ((1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0))
assert all(abs(g - w) < 1e-12 for c, i in zip(columns, identity) for g, w in zip(c, i))

# The gate: the flush-seated stand CERTIFIES. Its two seats are
# touches between copies, recorded at the door; nothing overlaps.
assembly = assemble(stand, ev)
assert assembly.interference == []
expected = (2 * POST_SECTION**2 * POST_HEIGHT
            + SHELF_LENGTH * SHELF_DEPTH * SHELF_THICKNESS)
assert abs(assembly.body.mass_properties(Settle.resolution(1e-6 * m)).met().volume - expected) < 1e-12

# `assemble` IS the gather plus the check, so its body is the gathered
# body.
gathered = product(stand, ev)
assert gathered.mass_properties(Settle.resolution(1e-6 * m)).met().volume == assembly.body.mass_properties(Settle.resolution(1e-6 * m)).met().volume
```

## 5. The refusals an assembly author actually meets

Each of these is reached by authoring the mistake. They are the
product working: the kernel declining to pick a pose or to read a
pose off what does not carry one.

```python
import tempfile

from pncad import (
    AssemblyError,
    CapEnd,
    Coord,
    Doc,
    DocRef,
    EditError,
    EntityKind,
    EvaluationError,
    Formula,
    Mate,
    MateValue,
    NamePat,
    Node,
    OVERCONSTRAINED_RECOURSE,
    Placed,
    PoseDef,
    ProductError,
    SegPat,
    SegTag,
    Selector,
    UNPINNED_RECOURSE,
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
HALF = POST_SECTION / 2


def prism(seed, width, depth, height):
    """One part: a rectangular block, placed in its world where it is
    drawn by the "place part" gesture."""
    doc = Doc(seed)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners]))
    doc.place(doc.insert(Node.extrude(profile, Formula.length_in(height, m))), doc.world)
    return doc


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


def two_instances():
    """A post standing in the world by the identity bundle — a bundle
    written in its place is what places it — and the shelf, pinned to
    the world at the identity."""
    doc = Doc("bench-refusals")
    a = doc.insert(Node.instantiate_part(post_ref, doc.world))
    s = doc.insert(Node.instantiate_part(shelf_ref, doc.world))
    return doc, a, s


def post_under_shelf(doc, post_i, shelf_i, values=True):
    """The post's top resting on the shelf's underside, the post's
    side first, and — unless `values` is false — the values that set it
    flush under the shelf's near end, written whole as the post's
    bundle."""
    ev = evaluate(doc, resolver=store)
    bundle = [Mate(
        PoseDef.plane(post_i, instance_cap(ev, post_i, CapEnd.End)),
        PoseDef.flip(PoseDef.plane(shelf_i, instance_cap(ev, shelf_i, CapEnd.Start))),
    )]
    if values:
        bundle += [MateValue(Coord.slide("x"), 0 * m),
                   MateValue(Coord.slide("y"), (HALF - SHELF_DEPTH / 2) * m),
                   MateValue(Coord.spin("z"), 0)]
    mate, *_ = doc.set_constraints(post_i, bundle)
    return mate


# 1. UNPINNED. One planar rest fixes the seating plane and nothing
#    else: the post may still slide and spin in it, and no value says
#    where. The solve picks no pose, and every bundle pins its copy, so
#    the door refuses the bundle, naming the placement and the three
#    freedoms it leaves, and ending on the recourse; the post keeps the
#    bundle it had.
doc, post_i, shelf_i = two_instances()
held = doc.bundle(post_i)
try:
    post_under_shelf(doc, post_i, shelf_i, values=False)
    raise AssertionError("expected a typed refusal")
except EditError as refusal:
    fault = refusal.fault
assert fault.variant == "mate_unpinned"
assert (fault.placement, fault.free) == (post_i, 3)
assert UNPINNED_RECOURSE in str(fault)
assert doc.bundle(post_i) == held

# 2. OVERCONSTRAINED. A post rests on a base's top by its underside,
#    and a second plane mate, a side on one of the base's sides, fixes
#    a normal the first already fixes. The door reads the two mates'
#    kinds, measures nothing, and refuses the bundle, naming both and
#    ending on the recourse; the document is as it was. One mate and
#    its values pin the post instead.
doc = Doc("bench-overconstrained")
post_i = doc.insert(Node.instantiate_part(post_ref, doc.world))
base = doc.insert(Node.instantiate_part(post_ref, doc.world))
ev = evaluate(doc, resolver=store)


def facing(node, normal):
    """`node`'s face whose outward normal is `normal`, as its plane."""
    def outward(name):
        pose = ev.face_frame(node, name)
        sign = 1.0 if pose.sense else -1.0
        return tuple(round(sign * c, 9) + 0.0 for c in pose.axis)
    (name,) = [n for n in ev.all_faces(node) if outward(n) == normal]
    return PoseDef.plane(node, name)


floor = Mate(facing(post_i, (0.0, 0.0, -1.0)), PoseDef.flip(facing(base, (0.0, 0.0, 1.0))))
try:
    doc.set_constraints(post_i, [
        floor,
        Mate(facing(post_i, (-1.0, 0.0, 0.0)), PoseDef.flip(facing(base, (1.0, 0.0, 0.0)))),
    ])
    raise AssertionError("expected a typed refusal")
except EditError as refusal:
    fault = refusal.fault
assert fault.variant == "mate_overconstrained"
assert fault.placement == post_i
assert len(fault.constraints) == 2 and fault.free == 1
assert OVERCONSTRAINED_RECOURSE in str(fault)
# The repair the recourse names: the floor mate alone, its three
# freedoms written zero by the one constructor — the post stands on the
# base, pinned.
doc.set_constraints(post_i, Placed.of(doc, [floor], []))
assert solve_document(doc, resolver=store).fault(post_i) is None

# 3. A PLANE READ OFF AN EDGE. A plane is read off a face, and Python
#    holds names as opaque text, so a plane read off an edge's name is
#    built, and the edit door refuses it: a selection's kind is the
#    name's own, held where the selection is written, and the sentence
#    names the side and what the name denotes.
doc, post_i, shelf_i = two_instances()
ev = evaluate(doc, resolver=store)
edge = sorted(ev.all_edges(post_i))[0]
try:
    doc.set_constraints(post_i, [Mate(
        PoseDef.plane(post_i, edge),
        PoseDef.flip(PoseDef.plane(shelf_i, instance_cap(ev, shelf_i, CapEnd.Start))),
    )])
except EditError as err:
    assert err.variant == "selection_name_kind"
    assert "edge, not a face" in str(err)
else:
    raise AssertionError("an edge-read plane is refused at the door")

# 4. NOTHING TO GATHER. Evaluated with no resolver, the instances —
#    placements into the world — produced no copies, so the GATHER
#    refuses before the gate runs — and it refuses under the gather's
#    own tag, not a wrapper's, because which invariant broke is what
#    you branch on. `node` is the first placement that failed, in
#    document order: the post.
doc, post_i, _ = two_instances()
try:
    assemble(doc, evaluate(doc))
    raise AssertionError("expected a typed refusal")
except AssemblyError as refusal:
    assert refusal.variant == "root_failed"
    assert refusal.node == post_i
try:
    product(doc, evaluate(doc))
    raise AssertionError("expected a typed refusal")
except ProductError as refusal:
    assert refusal.variant == "root_failed"
    assert refusal.node == post_i
```

Read an `AssemblyError`'s `variant` first, and read it as three
groups. `at_rest` is a verdict **against** the document, and it
carries the findings: `str(finding)` is the kernel's own finding, with
its recourse — a pair the census could not decide, or a touch it could
not record. Two copies that touch refuse nothing: the touch is a row of
the product, mated or not, and two copies whose material overlaps are
reported on `Assembly.interference`, never raised. `carried_at_rest`
is the same verdict from a document BELOW this one, crossing whole and
raised first, because that part is the file to open. Everything else is
the gather's own tag, refusing before any verdict exists.

## 6. Moving a part in or out: `split`, `inline`, and the pin door

`split(doc, cut, part_id, resolver=)` cuts a closed node set out into
a new document and leaves **one instance** of it behind, standing where
the part's world stands by the identity bundle, which reads nothing. `inline(doc,
instance, resolver)` is its inverse, by one rule: the referenced
document's recipe is copied verbatim into a new space
(`Doc.new_space()`, the outcome's `space`), and a placement of what the
part's world placed stands where the instance stood, under the
instance's own bundle (the outcome's `placement`) — so nothing moves,
and a variable that bundle reads keeps driving it. `flatten(doc,
placement)` folds such a placement away where its bundle is the
identity, putting what its space holds in its target.

Both are **pure**. They hand back the new document *values* plus the
ordinary recorded edits that produce them, and mutate nothing — which
is what makes each atomic at your single step. Persisting a result is
the store's write side, not the refactoring's.

They differ in one way that matters: `split` reads the store only
for the copies it cuts, to read the geometry its instance is pinned
over, and `inline` crosses the document seam **at the call**, under
the full pin gate. A reference whose pinned version is not what the store holds
refuses `part_pin_mismatch` rather than splicing the version on disk.

```python
import tempfile

from pncad import (
    Coord,
    Doc,
    DocRef,
    Formula,
    InlineError,
    MateValue,
    Node,
    Placed,
    SplitError,
    Workspace,
    content_pin,
    evaluate,
    flatten,
    inline,
    m,
    product,
    random_document_id,
    split,
    Settle,
)


def prism(seed, width, depth, height):
    """One part: a rectangular block, rooted at its own origin."""
    doc = Doc(seed)
    corners = [(0, 0), (width, 0), (width, depth), (0, depth)]
    profile = doc.insert(Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners]))
    doc.place(doc.insert(Node.extrude(profile, Formula.length_in(height, m))), doc.world)
    return doc


store = Workspace(tempfile.mkdtemp())
post = prism("bench-post", 0.12, 0.12, 0.5)
shelf = prism("bench-shelf", 0.9, 0.30, 0.04)
store.create(post)
store.create(shelf)

layout = Doc("bench-layout")
post_i = layout.insert(Node.instantiate_part(DocRef(post.id, content_pin(post)), layout.world))
shelf_i = layout.insert(Node.instantiate_part(DocRef(shelf.id, content_pin(shelf)), layout.world))
layout.set_constraints(shelf_i, Placed.unmated([MateValue(Coord.slide("y"), 0.5 * m)]))


def volume(doc):
    return product(doc, evaluate(doc, resolver=store)).mass_properties(Settle.resolution(1e-6 * m)).met().volume


before = volume(layout)

# Split the shelf instance out into a part of its own. A part delivers
# only its world, and the shelf instance is a placement pinned to the
# world, so it is a whole cut: the part's world is that placement.
# Identity is never defaulted: the new document's id is supplied by
# the caller, and a fresh one is what lets both documents live in one
# store.
outcome = split(layout, [shelf_i], random_document_id(), resolver=store)
assert volume(layout) == before, "pure: the original is untouched"
# The remainder holds one instance of the new part, pinned to the
# world where the part's world stands — where the shelf already did —
# beside the post.
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

# ...and inlining it back returns the material exactly: the part's
# recipe in a space of its own, placed where the instance stood. Its
# bundle is the identity, so flatten folds that placement away.
spliced = inline(outcome.remainder, outcome.instance, store)
assert volume(spliced.doc) == before
assert len(spliced.edits) > 0
assert spliced.doc.space(spliced.placement) == spliced.doc.output(spliced.doc.world)
flat = flatten(spliced.doc, spliced.placement)
assert volume(flat.doc) == before

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
    profile = doc.insert(Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners]))
    doc.place(doc.insert(Node.extrude(profile, Formula.length_in(height, m))), doc.world)
    return doc


store = Workspace(tempfile.mkdtemp())
shelf = prism("bench-shelf", 0.9, 0.30, 0.04)
store.create(shelf)
old_pin = content_pin(shelf)

doc = Doc("bench-layout")
first = doc.insert(Node.instantiate_part(DocRef(shelf.id, old_pin), doc.world))

# The shelf is thickened on disk; a second instance is added at the
# NEW version while the first still names the old one. That is a
# staged migration, and it is a legal state.
thicker = prism("bench-shelf", 0.9, 0.30, 0.08)
store.resave(thicker)
new_pin = content_pin(thicker)
second = doc.insert(Node.instantiate_part(DocRef(shelf.id, new_pin), doc.world))

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

**A bundle's mates read its placement's target.** A mate whose `to`
reads a body of another space refuses `space_mismatch` at the door,
naming both spaces. No space has roles a mate can read: a copy placed
in the world by numbers alone is placed by values on the world's axes,
which work on any placement. And the solve reads a face only of an
instance, or of a pattern's copy of one: a face side on a copy
`Doc.place` made of a body refuses typed.

**Only a placement places.** The product is the copies the
document's placements place into the world — each instance placed
there, and each copy `Doc.place` has made of a body — and the
placements' document order is the solid order. A mate is a constraint
in a placement's bundle and no body, so it is never in the product,
and no insert places a body as a side effect. A boolean of two placed
bodies is a new body nobody placed until you place it.

For the wider picture of what the Python surface can and cannot author
today, `docs/guide/north-star-audit.md` keeps the row-by-row account.
