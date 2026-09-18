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

The layout's placed family is NOT such a difference: `layout` spells
the posts with `Node.pattern`, the tour's own node, whose value is the
plural family. `posts=` switches that one call site to
`Node.placed_union`, which says the same family fused into one body —
so a test can hold the two spellings against each other rather than
assert that they agree. What the switch is for is that comparison
(`test_assembly_author.TestBenchLayout`); what the scene ships is the
pattern.

A `Doc`'s identity is derived from its label, so the two part documents
have the same identities whatever authored them, and a document
re-authored under the same label with different dimensions is the same
id at a different pin — which is how a part legitimately changes on
disk.
"""

import math

import pncad
from pncad import (
    Alignment,
    AxisSense,
    CapEnd,
    ContactClass,
    Doc,
    DocEdit,
    DocRef,
    EntityKind,
    Expr,
    Frame,
    MateFrame,
    MatePrimitive,
    NamePat,
    Node,
    PatternKind,
    SegPat,
    SegTag,
    Selector,
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
#: post is laid on its side (rotated -pi/2 about +y, so POST_HEIGHT
#: runs along x and POST_SECTION along y and z), set FLAT_PACK_GAP
#: along +x so the flat-pack sits beside the assembled bench, and
#: patterned PATTERN_COUNT ways along +y at PATTERN_SPACING.
FLAT_PACK_GAP = 1.4
PATTERN_COUNT = 2
PATTERN_SPACING = 0.2
#: Where the flat-packed shelf sits, relative to the same gap.
FLAT_PACK_SHELF_Y = 0.9

#: The stand's gauge post: the one instance carrying an authored frame,
#: inset in y so the bench top overhangs front and back.
GAUGE_OFFSET_Y = (SHELF_DEPTH - POST_SECTION) / 2.0

POST_LABEL = "pncad-demo-post"
SHELF_LABEL = "pncad-demo-shelf"
LAYOUT_LABEL = "pncad-demo-layout"
STAND_LABEL = "pncad-demo-stand"


# ---- The part documents ----


def prism(label, width, depth, height):
    """A rectangular prism part document, rooted at the origin.

    The extrusion runs +z from the sketch plane at z = 0, so the part's
    SEATING face is its top cap and its datum face is the origin plane.
    Three nodes: the sketch frame, the section drawn on it, the
    extrude that consumes both.
    """
    doc = Doc(label)
    profile = doc.insert(
        Node.polygon(
            [
                (Expr.length_in(0, m), Expr.length_in(0, m)),
                (Expr.length_in(width, m), Expr.length_in(0, m)),
                (Expr.length_in(width, m), Expr.length_in(depth, m)),
                (Expr.length_in(0, m), Expr.length_in(depth, m)),
            ],
            plane=doc.sketch_frame(elevation=Expr.length_in(0, m)),
        )
    )
    doc.insert(Node.extrude(profile, Expr.length_in(height, m)))
    return doc


def post(height=POST_HEIGHT, section=POST_SECTION):
    """The post: a square-section upright. The dimensions are arguments
    so a caller can author the SAME part changed — same label, so same
    identity, at a different pin."""
    return prism(POST_LABEL, section, section, height)


def shelf(thickness=SHELF_THICKNESS, length=SHELF_LENGTH, depth=SHELF_DEPTH):
    """The shelf: the board the posts carry."""
    return prism(SHELF_LABEL, length, depth, thickness)


# ---- Naming and mates ----


def cap_selector(side, wrapper=None):
    """A cap face, optionally seen through one or two name wrappers.

    The whole point of the wrapper argument: a part's own cap name and
    the same face seen through the instance that placed it are the SAME
    query one nesting deeper. Nothing here reads inside a name.
    """
    pat = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Cap).side(side))
    for tag in reversed(wrapper or []):
        pat = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(tag).of([pat]))
    return Selector.of(pat)


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
    found = evaluate(doc, resolver=store).select(
        node, cap_selector(side, [SegTag.InPart])
    )
    return one(found)


def mate_frame(origin):
    """The scene's mate frames: +z axis, +x clocking reference."""
    return MateFrame(
        origin=(origin[0] * m, origin[1] * m, origin[2] * m),
        axis=(0.0, 0.0, 1.0),
        reference=(1.0, 0.0, 0.0),
    )


def seat(a_frame, b_frame, primitive=None):
    """The scene's alignment: two frames meeting, axes aligned, no
    clocking rider."""
    return Alignment(
        mate_frame(a_frame),
        mate_frame(b_frame),
        primitive or MatePrimitive.frame_coincidence(),
        AxisSense.Aligned,
    )


#: The stand's two mates, as (a seat, b seat) in document order: the
#: gauge post's top to the shelf's underside, then the shelf's
#: underside to the far post's top.
STAND_SEATS = ((POST_SEAT, SEAT_A), (SEAT_B, POST_SEAT))


# ---- The assembly documents ----


def layout(post_ref, shelf_ref, posts=Node.pattern):
    """The flat-pack: one post instance patterned along +y, plus the
    shelf, nothing touching — A5's disjoint half.

    Every placement carries FLAT_PACK_GAP along +x, which is how the
    flat-pack shares one montage cell with the assembled bench: a
    layout document's placements ARE its subject, so where the parts
    sit has to be something this document SAYS.

    `posts` is the node the placed family is said with, and the two it
    admits take the same `(input, count, kind)`: `Node.pattern`, the
    tour's own, whose value is the PLURAL family; and
    `Node.placed_union`, whose value is that family fused into one
    body. One count and one rule serve both, so nothing about the
    scene moves with the switch.

    Answers the document and its three nodes, in document order.
    """
    doc = Doc(LAYOUT_LABEL)
    post_i = doc.insert(Node.instantiate_part(post_ref))
    # The post is laid on its SIDE: a rotation, which is why the frame
    # stores a general linear part and not a translation.
    doc.apply(
        DocEdit.set_placement(
            post_i,
            Frame.rotate_then_translate(
                (0.0, 1.0, 0.0),
                -math.pi / 2 * pncad.rad,
                ((FLAT_PACK_GAP + POST_HEIGHT) * m, 0 * m, 0 * m),
            ),
        )
    )
    family = doc.insert(
        posts(
            post_i,
            Expr.count(PATTERN_COUNT),
            PatternKind.linear((Expr.literal(0.0), Expr.literal(1.0), Expr.literal(0.0)), Expr.length_in(PATTERN_SPACING, m)),
        )
    )
    shelf_i = doc.insert(Node.instantiate_part(shelf_ref))
    doc.apply(
        DocEdit.set_placement(
            shelf_i,
            Frame.translation((FLAT_PACK_GAP * m, FLAT_PACK_SHELF_Y * m, 0 * m)),
        )
    )
    return doc, post_i, family, shelf_i


def stand(store, post_ref, shelf_ref, primitive=None, class_=ContactClass.Rest):
    """The assembled bench: a post at each end of the shelf, the shelf
    SEATED on them by mates.

    Only the gauge post carries an authored frame — placement lives on
    the cluster, and the mates place the rest. Answers the document,
    its three instances and its two mates, each in document order.
    """
    doc = Doc(STAND_LABEL)
    post_a = doc.insert(Node.instantiate_part(post_ref))
    doc.apply(
        DocEdit.set_placement(
            post_a, Frame.translation((0 * m, GAUGE_OFFSET_Y * m, 0 * m))
        )
    )
    shelf_i = doc.insert(Node.instantiate_part(shelf_ref))
    post_b = doc.insert(Node.instantiate_part(post_ref))
    a_top = instance_face(store, doc, post_a, CapEnd.End)
    b_top = instance_face(store, doc, post_b, CapEnd.End)
    s_bottom = instance_face(store, doc, shelf_i, CapEnd.Start)
    mate_1 = doc.insert(
        Node.mate(
            post_a, a_top, shelf_i, s_bottom, class_, seat(*STAND_SEATS[0], primitive)
        )
    )
    mate_2 = doc.insert(
        Node.mate(
            shelf_i, s_bottom, post_b, b_top, class_, seat(*STAND_SEATS[1], primitive)
        )
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
    """Author the whole scene into `store` and answer label -> `Doc`.

    The four documents are WRITTEN, so a caller that wants the scene as
    the persistence door hands it back resolves each one out of the
    store rather than keeping these.
    """
    (post_doc, post_ref), (shelf_doc, shelf_ref) = parts(store)
    layout_doc, _, _, _ = layout(post_ref, shelf_ref)
    store.create(layout_doc)
    stand_doc, _, _ = stand(store, post_ref, shelf_ref)
    store.create(stand_doc)
    return {
        "post": post_doc,
        "shelf": shelf_doc,
        "layout": layout_doc,
        "stand": stand_doc,
    }
