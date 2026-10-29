"""One body out of a multi-body value, from Python (LIB-B-PART).

A split's half is an output of the split, read as `doc.output(split,
port)` — "the upper half of that split"; one member of a family is
`row[i]` (`tests/test_pattern.py`). The Python mirror of
`crates/editor-core/tests/corpus/part_select.rs` and the acceptance
rows in `crates/editor-core/tests/docm2_part.rs`, asserted through the
bound doors rather than restated as prose.

WHAT A READ OF A HALF MEANS, and what the rows below check. Nothing is
moved, re-stamped or recomputed: a reader of the half reads the
split's own side, so a reader of it weighs what the side `Value.split`
hands back weighs (`TestTheHalfIsItsPort`), the two halves unioned are
the box again (`test_the_halves_rejoin_into_the_box`), and the names
pass through, so every face a reader names carries a face name of the
split's half unchanged (`test_the_names_pass_through`).
"""

import json
import unittest

from pncad import (
    Doc,
    EvaluationError,
    Formula,
    Node,
    evaluate,
    m,
)

from placing import plane_on
from measuring import pinned

#: The box's footprint half-extent and height, metres — the corpus
#: document's own constants, so the dyadic pins below are its pins.
BOX_HALF = 1.0
BOX_H = 1.0
#: The tool plane's height: a cut at mid-height.
CUT_Z = 0.5


def box(doc, half=BOX_HALF, height=BOX_H):
    """The box [-half, half]^2 x [0, height], standing on the ground."""
    square = doc.insert(
        Node.polygon(
            [
                (Formula.length_in(-half, m), Formula.length_in(-half, m)),
                (Formula.length_in(half, m), Formula.length_in(-half, m)),
                (Formula.length_in(half, m), Formula.length_in(half, m)),
                (Formula.length_in(-half, m), Formula.length_in(half, m)),
            ],
        )
    )
    return doc.insert(Node.extrude(square, Formula.length_in(height, m)))


def split_at(doc, target, z=CUT_Z):
    """`target` cut by a horizontal plane at height `z`."""
    tool = plane_on(doc, target, (0.0, 0.0, z), (0.0, 0.0, 1.0))
    return doc.insert(Node.split(target, tool))


def still(doc, read):
    """A reader of `read` that changes nothing: a union of that one
    member, which holds its body."""
    return doc.insert(Node.union([read]))


def canonical(name):
    """A face name's wire form, keys sorted, so two spellings of one
    name compare equal."""
    return json.dumps(name, sort_keys=True)


def mass_of(ev, node):
    body = ev.value(node).body()
    body.validate()
    return pinned(body)


def refusal(testcase, doc, node):
    """The typed refusal `node` evaluates to, as its stable tag."""
    with testcase.assertRaises(EvaluationError) as caught:
        evaluate(doc).value(node)
    return caught.exception.kind


class TestTheHalfIsItsPort(unittest.TestCase):
    """A split's half, read by port — the corpus document's first half.

    The oracle is the split's OWN value: `Value.split` hands back the
    two sides, and a reader of a port must hold exactly what that side
    weighs. Nothing here transcribes a number a read is then checked
    against; the two readings of the same body are compared.
    """

    def build(self):
        doc = Doc()
        cube = box(doc)
        split = split_at(doc, cube)
        above = still(doc, doc.output(split, 0))
        below = still(doc, doc.output(split, 1))
        return doc, split, above, below

    def test_each_half_weighs_what_the_split_says_it_weighs(self):
        doc, split, above, below = self.build()
        ev = evaluate(doc)
        self.assertEqual(ev.value(split).kind, "split")
        upper, lower = ev.value(split).split()
        for read, side, name in ((above, upper, "above"), (below, lower, "below")):
            with self.subTest(half=name):
                self.assertIsNotNone(side)
                self.assertEqual(ev.value(read).kind, "body")
                self.assertEqual(
                    mass_of(ev, read).volume, pinned(side).volume
                )
                self.assertEqual(
                    mass_of(ev, read).surface_area,
                    pinned(side).surface_area,
                )

    def test_the_two_halves_are_the_box_cut_where_the_plane_is(self):
        """The dyadic pin, stated once: a 2 x 2 x 1 box cut at 0.5 is
        two 2 x 2 x 0.5 slabs. Both oracles are exact binary fractions,
        so the comparison is `==` rather than a tolerance."""
        doc, _, above, below = self.build()
        ev = evaluate(doc)
        for read in (above, below):
            mass = mass_of(ev, read)
            self.assertEqual(mass.volume, 2.0 * 2.0 * 0.5)
            self.assertEqual(mass.surface_area, 2 * 2.0 * 2.0 + 4 * 2.0 * 0.5)

    def test_the_halves_rejoin_into_the_box(self):
        """The document's own statement that a port IS the half:
        nothing was moved, re-stamped or lost on the way through, so
        unioning the two back together is the box again.

        The two halves REST on each other across the section, and the
        box's four walls are each cut into two coplanar pieces facing
        the same way out. The union glues all five pairs, as it glues
        any touching members: that it decides them AT ALL across the
        two halves is itself the claim that each reader holds its half
        whole.
        """
        doc, _, above, below = self.build()
        whole = doc.insert(Node.union([above, below]))
        mass = mass_of(evaluate(doc), whole)
        self.assertEqual(mass.volume, 2.0 * 2.0 * 1.0)
        self.assertEqual(mass.surface_area, 2 * 2.0 * 2.0 + 4 * 2.0 * 1.0)

    def test_the_names_pass_through(self):
        """A read of a half hands on that half's table: every face name
        a reader answers with is the reader's own `From` layer over a
        face name of the split's half, unchanged, and the names it
        carries are exactly that half's. A split's names spell no half:
        each half is its own table, so the section face and each wall's
        fragment are ONE name the two halves' tables each hold."""
        doc, split, above, below = self.build()
        ev = evaluate(doc)
        upper = set(ev.all_faces(doc.output(split, 0)))
        lower = set(ev.all_faces(doc.output(split, 1)))
        for reader, half in ((above, upper), (below, lower)):
            carried = []
            for name in ev.all_faces(reader):
                (seg,) = json.loads(name)["path"]
                carried.append(canonical(seg["From"]["of"]))
            self.assertEqual(len(carried), len(half), "one name per face")
            self.assertEqual(set(carried), {canonical(json.loads(n)) for n in half})
        # What the halves share is what the split's names no longer
        # tell apart by half: the section face and the four walls'
        # fragments, each one name that each half's table holds. The
        # cap each half keeps whole is its own.
        shared = upper & lower
        self.assertEqual(len(shared), 5)
        self.assertEqual(sum('"SectionFace"' in n for n in shared), 1)
        self.assertEqual(sum('"SplitFragment"' in n for n in shared), 4)
        # A cut box is a 6-faced slab per side: four walls, the
        # original face, and the section.
        self.assertEqual(len(upper), 6)
        self.assertEqual(len(lower), 6)


class TestTheRefusalsAreTyped(unittest.TestCase):
    """A half's own refusal, `empty_half`, reached from Python."""

    def test_a_half_with_no_material_refuses(self):
        """A tool plane that misses the box entirely: a reader of the
        empty side refuses, and a reader of the side WITH material still
        evaluates — the refusal is the read's, not the split's."""
        doc = Doc()
        cube = box(doc)
        split = split_at(doc, cube, z=2.0)
        above = still(doc, doc.output(split, 0))
        below = still(doc, doc.output(split, 1))
        self.assertEqual(refusal(self, doc, above), "empty_half")
        self.assertTrue(evaluate(doc).succeeded(below))


if __name__ == "__main__":
    unittest.main()
