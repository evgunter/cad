"""The tour's bench, authored from Python — the ONE definition of it.

`demos/tour/src/assembly.rs` is the scene this module is: two part
documents (a post and a shelf), the flat-pack `layout` (one post
instance patterned along +y, plus the shelf, nothing touching) and the
mated `stand` (two posts and a shelf, the shelf seated on both by two
mates). `test_assembly_author.py` authors it to exercise the authoring
doors; `test_assembly_eval.py` writes it into a store and evaluates
what it reads back. Both get their constants, their part shapes and
their two assembly recipes from HERE, so the two files cannot drift
apart from each other, and `test_assembly_eval.py`'s tour guard has one
Python side to compare the tour against.

WHAT THIS SCENE IS, AND WHAT IT IS NOT
--------------------------------------
It is the tour's GEOMETRY, its placements and its mates: the same five
base dimensions, the same derived seats, the same pattern count and
spacing, the same flat-pack offset. Every number here is checked
against `assembly.rs` by `TestTheSceneIsTheToursOwn`.

It is NOT the tour's documents byte for byte, and ONE difference is
deliberate: the tour's prisms are PARAMETRIC — `prism_part` declares
each document's named dimensions and draws its profile from
expressions over them. Putting an expression into an authoring step is
a named gap in the bindings (`pncad.pyi`'s module docstring), so the
prisms here are drawn from literal quantities and declare no
parameters. The bodies are the same; the recipes are not.

A second difference is a job rather than a choice: the tour stands
its stand on a TURNTABLE — the near post's world mate turns by a
document parameter's swing — and sets a crate on the shelf. `stand`
here pins the near post to the world where the turntable puts it at
the tour's authored swing of zero, and authors no crate
(`work/lib/the-turntable-bench-has-no-python-row.md`).

The layout's placed family is NOT such a difference: `layout` spells
the posts as the tour does, a row under the index `k = index(2)`
(`Node.linear_pattern`) whose value is the plural family, each member
`row[i]` placed into the world. `posts="union"` switches that one call
site to placing `Node.union(row)`, the same family fused into one body,
whole — so a test can hold the two spellings against each other rather
than assert that they agree. What the switch is for is that comparison
(`test_assembly_author.TestBenchLayout`); what the scene ships is the
members.

A `Doc`'s identity is derived from its seed, so the two part documents
have the same identities whatever authored them, and a document
re-authored under the same seed with different dimensions is the same
id at a different pin — which is how a part legitimately changes on
disk.
"""

from pncad import (
    CapEnd,
    Coord,
    Doc,
    DocEdit,
    DocRef,
    EntityKind,
    Formula,
    Frame,
    Mate,
    MateValue,
    NamePat,
    Node,
    Placed,
    PoseDef,
    SegPat,
    SegTag,
    Selector,
    VarDecl,
    VarName,
    content_pin,
    evaluate,
    m,
)

from placing import moves, placed, assemble_world_copies

# ---- The scene's dimensions ----

#: The five base dimensions, in metres.
POST_SECTION = 0.12
POST_HEIGHT = 0.5
SHELF_LENGTH = 0.9
SHELF_DEPTH = 0.30
SHELF_THICKNESS = 0.04

#: Derived exactly as the scene derives them: where the shelf's
#: underside meets each post, in SHELF coordinates, and where a post's
#: top meets it in POST coordinates. The posts sit FLUSH with the
#: shelf's two ends, which is the obvious way to draw a bench.
SEAT_A = (POST_SECTION / 2.0, SHELF_DEPTH / 2.0, 0.0)
SEAT_B = (SHELF_LENGTH - POST_SECTION / 2.0, SHELF_DEPTH / 2.0, 0.0)
POST_SEAT = (POST_SECTION / 2.0, POST_SECTION / 2.0, POST_HEIGHT)

POST_VOLUME = POST_SECTION * POST_SECTION * POST_HEIGHT
SHELF_VOLUME = SHELF_LENGTH * SHELF_DEPTH * SHELF_THICKNESS

#: The flat-pack's declared PLACEMENTS, which no volume can see: the
#: post is patterned PATTERN_COUNT ways along +y at PATTERN_SPACING,
#: and each copy laid on its side (rotated -pi/2 about +y, so
#: POST_HEIGHT runs along x and POST_SECTION along y and z), set
#: FLAT_PACK_GAP along +x so the flat-pack sits beside the assembled
#: bench.
FLAT_PACK_GAP = 1.4
PATTERN_COUNT = 2
PATTERN_SPACING = 0.2
#: Where the flat-packed shelf sits, relative to the same gap.
FLAT_PACK_SHELF_Y = 0.9

#: The stand's near post: the one instance pinned to the world, inset
#: in y so the bench top overhangs front and back.
ROOT_OFFSET_Y = (SHELF_DEPTH - POST_SECTION) / 2.0

POST_SEED = "pncad-demo-post"
SHELF_SEED = "pncad-demo-shelf"
LAYOUT_SEED = "pncad-demo-layout"
STAND_SEED = "pncad-demo-stand"


# ---- The part documents ----


def prism(seed, width, depth, height):
    """A rectangular prism part document, rooted at the origin.

    The extrusion runs up its own +z from z = 0, so the part's SEATING
    face is its top cap and its base cap lies on the origin plane.
    Three nodes: the section, the extrude that consumes it, and the
    placement that puts the extrude in the
    part's world where it stands — what an instance of the part
    delivers. The placement rests the extrude's base cap on the world's
    floor, a plane mate with three zero values: the body's own
    coordinates are the world's, and an edit that grows the part grows
    it up from its base.
    """
    doc = Doc(seed)
    profile = doc.insert(
        Node.polygon(
            [
                (Formula.length_in(0, m), Formula.length_in(0, m)),
                (Formula.length_in(width, m), Formula.length_in(0, m)),
                (Formula.length_in(width, m), Formula.length_in(depth, m)),
                (Formula.length_in(0, m), Formula.length_in(depth, m)),
            ],
        )
    )
    # A part with no height has no faces to read its seat off: it is
    # authored standing a metre tall, its placement read off that, then
    # edited flat — the part edited until it no longer builds.
    built = height if height > 0 else 1.0
    body = doc.insert(Node.extrude(profile, Formula.length_in(built, m)))
    doc.set_product(doc.place(body, doc.world))
    if built != height:
        doc.apply(DocEdit.set_param(body, "distance", Formula.length_in(height, m)))
    return doc


def on_floor(doc, placement):
    """`placement`'s copy standing on the world's floor by its base
    cap, which lies on its own `xy`: the identity bundle, written whole
    in place of the one the placement holds — the copy's own
    coordinates are the world's."""
    doc.set_constraints(placement, Placed.identity())


def post(height=POST_HEIGHT, section=POST_SECTION):
    """The post: a square-section upright. The dimensions are arguments
    so a caller can author the SAME part changed — same seed, so same
    identity, at a different pin."""
    return prism(POST_SEED, section, section, height)


def shelf(thickness=SHELF_THICKNESS, length=SHELF_LENGTH, depth=SHELF_DEPTH):
    """The shelf: the board the posts carry."""
    return prism(SHELF_SEED, length, depth, thickness)


# ---- Naming and mates ----


def cap_selector(side, wrapper=None):
    """A cap face, optionally seen through one or two name wrappers.

    The whole point of the wrapper argument: a part's own cap name and
    the same face seen through the instance that placed it are the SAME
    query one nesting deeper. Nothing here reads inside a name.

    `PART_FACE` is the wrapper a part's face wears seen through an
    instance: none, since a copy carries its source's names verbatim.
    """
    pat = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Cap).side(side))
    for tag in reversed(wrapper or []):
        pat = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(tag).of([pat]))
    return Selector.of(pat)


#: A part's face as an instance of the part names it: the part's
#: product name, its body's own face, which its one world placement's
#: copy and the instance's copy carry verbatim.
PART_FACE = []


def one(found):
    assert len(found) == 1, f"expected exactly one name, got {found}"
    return found[0]


def instance_face(store, doc, node, side):
    """A face of an instance's product, in the ASSEMBLY's names.

    The mate-authoring flow, and the reason no name has to be
    hand-composed: instantiate, evaluate against the store, then SELECT
    on the instantiate node. What comes back is the part's own name,
    as the instance's copy carries it, read at that node, which is what
    a mate reference is.
    """
    found = evaluate(doc, resolver=store).select(node, cap_selector(side, PART_FACE))
    return one(found)


def face_facing(store, doc, node, normal):
    """The face of `node`'s copy whose outward normal is `normal`, read
    off THIS evaluation: a prism's six faces each face one way, so
    the normal picks one. Read through `Evaluation.face_frame`, whose
    outward normal is its sense times its axis."""
    ev = evaluate(doc, resolver=store)

    def outward(name):
        pose = ev.face_frame(node, name)
        sign = 1.0 if pose.sense else -1.0
        return tuple(round(sign * c, 9) + 0.0 for c in pose.axis)

    return one([name for name in ev.all_faces(node) if outward(name) == normal])


def rest(doc, placement, on, to, at):
    """`placement`'s copy seated by its face `on` resting on the face
    `to`: a plane mate, the target flipped so the two faces face each
    other, and the two slides and the spin that set where on `to` the
    copy rests. `on` and `to` are `(node, face name)` pairs; `at` is
    `(slide x, slide y)` in metres, the spin zero — the two bodies'
    own axes agree. The bundle is written whole, in place of the one the
    placement holds.

    Answers the mate's address and its three values' addresses.
    """
    mate, *values = doc.set_constraints(
        placement,
        [
            Mate(PoseDef.plane(*on), PoseDef.flip(PoseDef.plane(*to))),
            MateValue(Coord.slide("x"), at[0] * m),
            MateValue(Coord.slide("y"), at[1] * m),
            MateValue(Coord.spin(), 0),
        ],
    )
    return mate, values


#: The stand's two seats, as (the copy's seat, the target's seat) in
#: document order, each naming the part it places first — the mate is
#: a constraint in that part's bundle: the shelf's underside on the
#: near post's top, then the far post's top under the shelf. A post's
#: seat is its cap's centre, a shelf's an authored point of its
#: underside.
STAND_SEATS = ((SEAT_A, POST_SEAT), (POST_SEAT, SEAT_B))

def shelf_on_post(seat):
    """The slides that rest the shelf on the near post's top with the
    shelf's `seat` over the top's centre: where the shelf's origin
    stands on the world's axes — a value charts on the target space,
    never on the body it rests on — the post's seat less the shelf's,
    from where the near post stands."""
    return (POST_SEAT[0] - seat[0], ROOT_OFFSET_Y + POST_SEAT[1] - seat[1])


def post_under_shelf(seat):
    """The slides that stand a post under the shelf's underside with
    its top's centre at the shelf's `seat`, the shelf standing at the
    world's origin: along the world's own `x` and `y`, the mated
    direction (the underside's flipped normal, `+z`) where the identity
    orientation puts the post's top."""
    return (seat[0] - POST_SEAT[0], seat[1] - POST_SEAT[1])


#: The values each stand mate is set to, from its seats.
SHELF_ON_POST = shelf_on_post(SEAT_A)
POST_UNDER_SHELF = post_under_shelf(SEAT_B)


# ---- The assembly documents ----


def lying():
    """The pose each flat-packed post copy is placed in the world at:
    laid on its SIDE — a rotation, which is why the frame stores a
    general linear part and not a translation — and set beside the
    bench."""
    return Frame.rotate_by_turns_then_translate(
        (0.0, 1.0, 0.0),
        -1,
        4,
        ((FLAT_PACK_GAP + POST_HEIGHT) * m, 0 * m, 0 * m),
    )


def layout(store, post_ref, shelf_ref, posts="members"):
    """The flat-pack: one post instance patterned along +y, plus the
    shelf, nothing touching — A5's disjoint half.

    Every world placement carries FLAT_PACK_GAP along +x, which is how the
    flat-pack shares one montage cell with the assembled bench: a
    layout document's placements ARE its subject, so where the parts
    sit has to be something this document SAYS.

    The post instance is placed into a space of its own
    (`Doc.new_space`), standing at its part's own coordinates there and
    no part of the product, which is what a pattern master is here. The pattern steps along +y, which the turn about +y
    each copy is laid at leaves where it was.

    `posts` is how the placed family reaches the world, and the two it
    admits read the same row: `"members"`, the tour's own, places each
    member `row[i]` of the PLURAL family; and `"union"` places
    `Node.union(row)`, that family fused into one body, whole. One index
    and one pitch serve both, so nothing about the scene moves with the
    switch.

    The product is the posts' copies, then the shelf's; the shelf
    instance is set beside them in the world. Each is placed by
    the values that put it at its pose (`placing.placed`, and
    `Doc.set_constraints` for the shelf).

    Answers the document, the post instance, the row and the shelf
    instance.
    """
    doc = Doc(LAYOUT_SEED)
    # The pattern's master is placed into a space of its own: no part
    # of the product, only the copies placed into the world are.
    post_i = doc.insert(Node.instantiate_part(post_ref, doc.new_space()))
    doc.apply(DocEdit.declare_var(VarName("k"), VarDecl.index(PATTERN_COUNT)))
    family = doc.insert(
        Node.linear_pattern(
            VarName("k"),
            post_i,
            doc.space(post_i),
            [("y", Formula.length_in(PATTERN_SPACING, m))],
        )
    )
    shelf_i = doc.insert(Node.instantiate_part(shelf_ref, doc.world))
    doc.set_constraints(
        shelf_i,
        Placed.unmated(moves(doc, Frame.translation((FLAT_PACK_GAP * m, FLAT_PACK_SHELF_Y * m, 0 * m)))),
    )
    if posts == "members":
        for i in range(PATTERN_COUNT):
            placed(doc, family[i], lying())
    else:
        placed(doc, doc.insert(Node.union(family)), lying())
    assemble_world_copies(doc)
    return doc, post_i, family, shelf_i


def stand(store, post_ref, shelf_ref):
    """The assembled bench: a post at each end of the shelf, the shelf
    SEATED on them by mates.

    Only the near post is pinned to the world; each other instance's
    bundle, written whole in place of the identity it is inserted with,
    is the one mate that places it and its three values — the
    shelf on the near post, the far post under the shelf. Answers the
    document, its three instances and its two mates' addresses, each in
    document order.
    """
    doc = Doc(STAND_SEED)
    post_a = doc.insert(Node.instantiate_part(post_ref, doc.world))
    doc.set_constraints(post_a, Placed.unmated(moves(doc, Frame.translation((0 * m, ROOT_OFFSET_Y * m, 0 * m)))))
    shelf_i = doc.insert(Node.instantiate_part(shelf_ref, doc.world))
    post_b = doc.insert(Node.instantiate_part(post_ref, doc.world))
    # What the stand delivers: its three copies side by side.
    assemble_world_copies(doc)
    a_top = instance_face(store, doc, post_a, CapEnd.End)
    b_top = instance_face(store, doc, post_b, CapEnd.End)
    s_bottom = instance_face(store, doc, shelf_i, CapEnd.Start)
    mate_1, _ = rest(
        doc,
        shelf_i,
        (shelf_i, s_bottom),
        (post_a, a_top),
        SHELF_ON_POST,
    )
    mate_2, _ = rest(
        doc,
        post_b,
        (post_b, b_top),
        (shelf_i, s_bottom),
        POST_UNDER_SHELF,
    )
    return doc, (post_a, shelf_i, post_b), (mate_1, mate_2)


# ---- The whole scene, on disk ----


def parts(store):
    """Author the two part documents into `store` and answer them with
    the references an assembly carries — the `(id, pin)` pair naming
    which part and which version of it."""
    post_doc, shelf_doc = post(), shelf()
    store.create(post_doc)
    store.create(shelf_doc)
    return (
        (post_doc, DocRef(post_doc.id, content_pin(post_doc))),
        (shelf_doc, DocRef(shelf_doc.id, content_pin(shelf_doc))),
    )


def write(store):
    """Author the whole scene into `store` and answer seed -> `Doc`.

    The four documents are WRITTEN, so a caller that wants the scene as
    the persistence door hands it back resolves each one out of the
    store rather than keeping these.
    """
    (post_doc, post_ref), (shelf_doc, shelf_ref) = parts(store)
    layout_doc, _, _, _ = layout(store, post_ref, shelf_ref)
    store.create(layout_doc)
    stand_doc, _, _ = stand(store, post_ref, shelf_ref)
    store.create(stand_doc)
    return {
        "post": post_doc,
        "shelf": shelf_doc,
        "layout": layout_doc,
        "stand": stand_doc,
    }
