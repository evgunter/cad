"""Review B of the datum (PR 7382): `Node.datum` through Python — insert,
read a plane off it, mate it through a role and read the sides back,
save and load."""

import unittest

from pncad import Placed, Coord, Doc, Formula, Mate, MateValue, Node, PoseDef, RoleRead, evaluate, load, m

from placing import plane_on


def block(doc):
    corners = [(0, 0), (2, 0), (2, 2), (0, 2)]
    profile = doc.insert(
        Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners])
    )
    return doc.insert(Node.extrude(profile, Formula.length_in(1.0, m)))


class DatumReviewTest(unittest.TestCase):
    def test_a_datum_round_trips_through_python(self):
        doc = Doc()
        body = block(doc)
        space = doc.space(body)
        datum = doc.insert(
            Node.datum(space, Placed.unmated([MateValue(Coord.slide("x"), Formula.length_in(4.0, m))]))
        )
        self.assertEqual(doc.node_kind(datum), "datum")
        frame = doc.output(datum)
        self.assertIsNotNone(frame)
        image = doc.insert(Node.mirror(body, RoleRead(frame, "yz")))
        ev = evaluate(doc)
        self.assertIsNotNone(ev.value(image).body(), "the mirror across the datum builds")
        # The datum flush on the block's top: its `on` through its xy plane.
        top = plane_on(doc, body, (0.0, 0.0, 1.0), (0.0, 0.0, 1.0))
        mate = Mate(RoleRead(frame, "xy"), top)
        on = mate.on
        self.assertIsInstance(on, RoleRead)
        self.assertEqual(on.role, "xy")
        self.assertIsInstance(mate.to, PoseDef)
        self.assertIn("RoleRead(", repr(mate))
        doc.set_constraints(datum, Placed.of(doc, [mate], []))
        text = doc.save()
        back = load(text)
        self.assertEqual(back.save(), text, "saves and loads equal")
        self.assertEqual(back.node_kind(datum), "datum")
        # RED at 1c7255b5: `Doc.bundle` matches `Place | InstantiatePart`
        # only (`crates/pncad-py/src/py/doc.rs:1474`), so a datum's
        # bundle, written by `set_constraints`, cannot be read back.
        written = [c for _, c in doc.bundle(datum) if isinstance(c, Mate)]
        self.assertEqual(len(written), 1)
        self.assertIsInstance(written[0].on, RoleRead)
        self.assertEqual(written[0].on.role, "xy")

    def test_a_copys_far_side_on_a_datum_reads_back_a_role_read(self):
        doc = Doc()
        body = block(doc)
        datum = doc.insert(Node.datum(doc.world))
        frame = doc.output(datum)
        copy = doc.place(body, doc.world)
        start = plane_on(doc, body, (0.0, 0.0, 0.0), (0.0, 0.0, 1.0))
        doc.set_constraints(copy, Placed.of(doc, [Mate(start, RoleRead(frame, "xy"))], []))
        [mate] = [c for _, c in doc.bundle(copy) if isinstance(c, Mate)]
        self.assertIsInstance(mate.to, RoleRead)
        self.assertEqual(mate.to.role, "xy")
        self.assertIsInstance(mate.on, PoseDef)

    def test_a_datums_mate_off_its_own_frame_refuses(self):
        doc = Doc()
        body = block(doc)
        space = doc.space(body)
        datum = doc.insert(Node.datum(space))
        other = doc.insert(Node.datum(space))
        top = plane_on(doc, body, (0.0, 0.0, 1.0), (0.0, 0.0, 1.0))
        with self.assertRaises(Exception) as caught:
            doc.set_constraints(datum, Placed.of(doc, [Mate(RoleRead(doc.output(other), "xy"), top)], []))
        self.assertIn("not its datum's own frame", str(caught.exception))
        self.assertEqual(caught.exception.fault.variant, "mate_not_the_datums_own")


if __name__ == "__main__":
    unittest.main()
