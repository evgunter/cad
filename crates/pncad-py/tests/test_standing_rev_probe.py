"""Review probe (standing-rev): the binding's projection of a node's
standing at the doors PR 3463 retyped — the name doors (now
`NameLookupError` kernel-side, still `HitTestError` here), the pick
build, and the distance query's datum — on a failed and a poisoned
node. Prints each refusal's class, tag, fields and message."""

import unittest

from pncad import (
    Cmp,
    Doc,
    DocEdit,
    EntityKind,
    Expr,
    GeomPred,
    HitTestError,
    NamePat,
    Node,
    NodePick,
    NodePickError,
    SelectRefusal,
    Selector,
    evaluate,
    m,
    rad,
)

DELTA = 0.1 * m


def fields(err, names):
    return {n: getattr(err, n, "<absent>") for n in names}


class TestStandingProjection(unittest.TestCase):
    def setUp(self):
        doc = Doc()
        square = doc.insert(
            Node.polygon([
                (Expr.length_in(0, m), Expr.length_in(0, m)),
                (Expr.length_in(1, m), Expr.length_in(0, m)),
                (Expr.length_in(1, m), Expr.length_in(1, m)),
                (Expr.length_in(0, m), Expr.length_in(1, m)),
            ], plane=doc.sketch_frame())
        )
        self.square = square
        self.failed = doc.insert(Node.extrude(square, Expr.length_in(1, m)))
        z = Expr.length_in(0, m)
        self.poisoned = doc.insert(Node.transform(
            self.failed,
            (Expr.length_in(2, m), z, z),
            (Expr.literal(0.0), Expr.literal(0.0), Expr.literal(1.0)),
            Expr.literal(0.0 * rad),
        ))
        self.good = evaluate(doc)
        self.picks = {
            n: NodePick.build(self.good, n, 0, DELTA)
            for n in (self.failed, self.poisoned)
        }
        doc.apply(DocEdit.set_param(self.failed, "distance", Expr.length_in(0, m)))
        self.doc = doc
        self.broken = evaluate(doc)

    def test_name_doors_raise_hit_test_error_under_the_standing_tags(self):
        for node, word, through in (
            (self.failed, "node_failed", None),
            (self.poisoned, "node_poisoned", self.failed),
        ):
            for door in ("patch_names", "boundary_names"):
                with self.assertRaises(HitTestError) as caught:
                    getattr(self.picks[node], door)(self.broken)
                err = caught.exception
                f = fields(err, ["variant", "node", "through", "kind", "body", "hits"])
                print("PYPROBE", door, word, f, str(err))
                self.assertEqual(err.variant, word)
                self.assertEqual(err.node, node)
                self.assertEqual(err.through, through)
                self.assertIn("name lookup:", str(err))
                self.assertNotIn("hit test", str(err))

    def test_pick_build_raises_under_the_standing_tags(self):
        for node, word, through in (
            (self.failed, "node_failed", None),
            (self.poisoned, "node_poisoned", self.failed),
        ):
            with self.assertRaises(NodePickError) as caught:
                NodePick.build(self.broken, node, 0, DELTA)
            err = caught.exception
            print("PYPROBE build", word, fields(err, ["variant", "node", "through"]), str(err))
            self.assertEqual(err.variant, word)
            self.assertEqual(err.node, node)
            self.assertEqual(err.through, through)

    def test_a_poisoned_datum_projects_its_node_and_through(self):
        with self.assertRaises(SelectRefusal) as caught:
            self.broken.select_where(
                self.square,
                Selector.of(NamePat.of_kind(EntityKind.Face)),
                [GeomPred.datum_distance(self.poisoned, Cmp.Approx, Expr.length_in(0, m))],
            )
        err = caught.exception
        f = fields(err, ["reason", "datum", "found", "through"])
        print("PYPROBE datum", f, str(err))
        self.assertEqual(err.reason, "datum_has_no_value")
        self.assertEqual(err.datum, self.poisoned)
        # The standing's `through` rides the message only (the PR's
        # deviation 5); a caller cannot read the repair node as a value.
        self.assertEqual(getattr(err, "through", None), self.failed)


if __name__ == "__main__":
    unittest.main()
