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
its stand on a TURNTABLE — the near post's world pin turns by a
document parameter's swing — and sets a crate on the shelf. `stand`
here pins the near post to the world where the turntable puts it at
the tour's authored swing of zero, and authors no crate
(`work/lib/the-turntable-bench-has-no-python-row.md`).

The layout's placed family is NOT such a difference: `layout` spells
the posts with `Node.pattern`, the tour's own node, whose value is the
plural family. `posts=` switches that one call site to
`Node.placed_union`, which says the same family fused into one body —
so a test can hold the two spellings against each other rather than
assert that they agree. What the switch is for is that comparison
(`test_assembly_author.TestBenchLayout`); what the scene ships is the
pattern.

A `Doc`'s identity is derived from its seed, so the two part documents
have the same identities whatever authored them, and a document
re-authored under the same seed with different dimensions is the same
id at a different pin — which is how a part legitimately changes on
disk.
"""

import math

import pncad
from pncad import (
    CapEnd,
    ContactClass,
    Coord,
    Doc,
    DocRef,
    EntityKind,
    Formula,
    Frame,
    Mate,
    NamePat,
    Node,
    PartSelect,
    PatternKind,
    PoseDef,
    SegPat,
    SegTag,
    Selector,
    WorldRead,
    content_pin,
    evaluate,
    m,
)

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

    The extrusion runs +z from the sketch plane at z = 0, so the part's
    SEATING face is its top cap and its datum face is the origin plane.
    Four nodes: the sketch frame, the section drawn on it, the extrude
    that consumes both, and the placement that puts the extrude in the
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
            plane=doc.sketch_frame(elevation=Formula.length_in(0, m)),
        )
    )
    body = doc.insert(Node.extrude(profile, Formula.length_in(height, m)))
    placement = doc.place(body, [])
    on_floor(doc, placement, body, one(evaluate(doc).select(body, cap_selector(CapEnd.Start))))
    return doc


def on_floor(doc, placement, at, base, resolver=None):
    """`placement`'s copy standing on the world's floor by its face
    `base`, read at `at`: a plane mate to the world's `xy` plane
    flipped, so the face's outward normal points down, and three zero
    values — the copy's own coordinates are the world's as far as the
    floor allows. Answers the mate's address."""
    mate = doc.constrain(
        placement,
        Mate(PoseDef.plane(at, base), WorldRead.through("xy").flip(), ContactClass.Rest),
        resolver=resolver,
    )
    for coord in (Coord.slide("x"), Coord.slide("y"), Coord.spin()):
        doc.add_value(mate, coord, 0)
    return mate


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
    instance: the instance's own, round the copy the part's world
    placement makes of its body.
    """
    pat = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Cap).side(side))
    for tag in reversed(wrapper or []):
        pat = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(tag).of([pat]))
    return Selector.of(pat)


#: A part's face as an instance of the part names it: the part's
#: product name — its body's face under its one world placement's copy
#: — worn under the instance.
PART_FACE = [SegTag.InPart, SegTag.Placed]


def one(found):
    assert len(found) == 1, f"expected exactly one name, got {found}"
    return found[0]


def instance_face(store, doc, node, side):
    """A face of an instance's product, in the ASSEMBLY's names.

    The mate-authoring flow, and the reason no name has to be
    hand-composed: instantiate, evaluate against the store, then SELECT
    on the instantiate node. What comes back is the part's own name
    already wrapped at the instance that placed it, which is what a
    mate reference is.
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


def rest(doc, placement, on, to, at, class_=ContactClass.Rest, resolver=None):
    """`placement`'s copy seated by its face `on` resting on the face
    `to`: a plane mate, the target flipped so the two faces face each
    other, and the two slides and the spin that set where on `to` the
    copy rests. `on` and `to` are `(node, face name)` pairs; `at` is
    `(slide x, slide y)` in metres, the spin zero — the two bodies'
    own axes agree.

    Answers the mate's address and its three values' addresses.
    """
    mate = doc.constrain(
        placement,
        Mate(PoseDef.plane(*on), PoseDef.flip(PoseDef.plane(*to)), class_),
        resolver=resolver,
    )
    values = [
        doc.add_value(mate, Coord.slide("x"), at[0] * m),
        doc.add_value(mate, Coord.slide("y"), at[1] * m),
        doc.add_value(mate, Coord.spin(), 0),
    ]
    return mate, values


#: The stand's two seats, as (the copy's seat, the target's seat) in
#: document order, each naming the part it places first — the mate is
#: a constraint in that part's bundle: the shelf's underside on the
#: near post's top, then the far post's top under the shelf. A post's
#: seat is its cap's centre, a shelf's an authored point of its
#: underside.
STAND_SEATS = ((SEAT_A, POST_SEAT), (POST_SEAT, SEAT_B))

#: The values each stand mate is set to, from its seats. A value is
#: charted on the target's own frame swung onto its face's outward
#: normal: on the post's top cap that is the post's own x and y, so the
#: shelf's slides are the post's seat less the shelf's; on the shelf's
#: underside it is the half-turn about the shelf's x, so the far post's
#: y slide runs along the shelf's -y.
SHELF_ON_POST = (POST_SEAT[0] - SEAT_A[0], POST_SEAT[1] - SEAT_A[1])
POST_UNDER_SHELF = (SEAT_B[0] - POST_SEAT[0], POST_SEAT[1] - SEAT_B[1])


# ---- The assembly documents ----


def lying():
    """The pose each flat-packed post copy is placed in the world at:
    laid on its SIDE — a rotation, which is why the frame stores a
    general linear part and not a translation — and set beside the
    bench."""
    return Frame.rotate_then_translate(
        (0.0, 1.0, 0.0),
        -math.pi / 2 * pncad.rad,
        ((FLAT_PACK_GAP + POST_HEIGHT) * m, 0 * m, 0 * m),
    )


def layout(store, post_ref, shelf_ref, posts=Node.pattern):
    """The flat-pack: one post instance patterned along +y, plus the
    shelf, nothing touching — A5's disjoint half.

    Every world placement carries FLAT_PACK_GAP along +x, which is how the
    flat-pack shares one montage cell with the assembled bench: a
    layout document's placements ARE its subject, so where the parts
    sit has to be something this document SAYS.

    The post instance's bundle is empty: it is a loose copy, its part's
    own coordinates and no part of the product, which is what a pattern
    master is here. The pattern steps along +y, which the turn about +y
    each copy is laid at leaves where it was.

    `posts` is the node the placed family is said with, and the two it
    admits take the same `(input, count, kind)`: `Node.pattern`, the
    tour's own, whose value is the PLURAL family; and
    `Node.placed_union`, whose value is that family fused into one
    body. One count and one rule serve both, so nothing about the
    scene moves with the switch.

    The product is the posts' copies, then the shelf's: each pattern
    instance is projected out of the family by `Node.part` and placed,
    and a fused `placed_union` family, one body, is placed whole; the
    shelf instance is pinned to the world itself. Each is placed by the
    "place part" gesture at its pose (`Doc.place(..., pose=)`,
    `Doc.pin`), read through `store`.

    Answers the document and its three nodes, in document order.
    """
    doc = Doc(LAYOUT_SEED)
    post_i = doc.insert(Node.instantiate_part(post_ref, []))
    family = doc.insert(
        posts(
            post_i,
            Formula.count(PATTERN_COUNT),
            PatternKind.linear((Formula.literal(0.0), Formula.literal(1.0), Formula.literal(0.0)), Formula.length_in(PATTERN_SPACING, m)),
        )
    )
    shelf_i = doc.insert(Node.instantiate_part(shelf_ref, []))
    doc.pin(
        shelf_i,
        Frame.translation((FLAT_PACK_GAP * m, FLAT_PACK_SHELF_Y * m, 0 * m)),
        resolver=store,
    )
    if posts is Node.pattern:
        for i in range(PATTERN_COUNT):
            doc.place(
                doc.insert(Node.part(family, PartSelect.instance(Formula.count(i)))),
                pose=lying(),
                resolver=store,
            )
    else:
        doc.place(family, pose=lying(), resolver=store)
    return doc, post_i, family, shelf_i


def stand(store, post_ref, shelf_ref, class_=ContactClass.Rest):
    """The assembled bench: a post at each end of the shelf, the shelf
    SEATED on them by mates.

    Only the near post is pinned to the world; each other instance's
    bundle is the one mate that places it and its three values — the
    shelf on the near post, the far post under the shelf. Answers the
    document, its three instances and its two mates' addresses, each in
    document order.
    """
    doc = Doc(STAND_SEED)
    post_a = doc.insert(Node.instantiate_part(post_ref, []))
    doc.pin(post_a, Frame.translation((0 * m, ROOT_OFFSET_Y * m, 0 * m)), resolver=store)
    shelf_i = doc.insert(Node.instantiate_part(shelf_ref, []))
    post_b = doc.insert(Node.instantiate_part(post_ref, []))
    a_top = instance_face(store, doc, post_a, CapEnd.End)
    b_top = instance_face(store, doc, post_b, CapEnd.End)
    s_bottom = instance_face(store, doc, shelf_i, CapEnd.Start)
    mate_1, _ = rest(
        doc,
        shelf_i,
        (shelf_i, s_bottom),
        (post_a, a_top),
        SHELF_ON_POST,
        class_,
        resolver=store,
    )
    mate_2, _ = rest(
        doc,
        post_b,
        (post_b, b_top),
        (shelf_i, s_bottom),
        POST_UNDER_SHELF,
        class_,
        resolver=store,
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
