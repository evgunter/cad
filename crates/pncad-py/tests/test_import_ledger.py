"""The STEP import's move ledger read from Python: the one-record join's
two records and a curve promotion's kind, through `StructureNormalization`'s
getters, each `None` on the other kinds."""

import unittest
from pathlib import Path

from pncad import import_step

FIXTURES = Path(__file__).resolve().parents[2] / "step-import" / "tests" / "fixtures"


def ledger(name):
    return import_step((FIXTURES / name).read_text()).normalizations


class TestTheImportLedger(unittest.TestCase):
    def test_a_one_record_join_names_its_two_records(self):
        """A boolean union's top and bottom are each tiled into three
        coplanar records; the two later records of each join the
        earliest, and each join names the record kept and the one its
        faces left."""
        joins = [n for n in ledger("freecad/fuse_boxes.step") if n.kind == "one_record"]
        self.assertEqual(len(joins), 4, "the top's and the bottom's two later records")
        for join in joins:
            self.assertIsInstance(join.kept, int)
            self.assertIsInstance(join.joined, int)
            self.assertNotEqual(join.kept, join.joined)
            self.assertIsInstance(join.moved_by, float)
            self.assertIsNone(join.promoted_to, "a join promotes nothing")
            self.assertIsNone(join.residual)

    def test_a_curve_promotion_names_its_kind(self):
        """A straight degree-1 spline carrier promotes to a line, reported
        on the ledger keyed by the curve, its kind under `promoted_to`."""
        promoted = [
            n for n in ledger("line-beside-spline/mixed_lower.step") if n.kind == "curve_promoted"
        ]
        self.assertEqual(len(promoted), 1, "the one straight spline carrier")
        (row,) = promoted
        self.assertEqual(row.promoted_to, "line")
        self.assertIsInstance(row.residual, float)
        self.assertGreater(row.face, 0, "keyed by the curve entity")
        self.assertIsNone(row.kept, "only a join names records")
        self.assertIsNone(row.joined)


if __name__ == "__main__":
    unittest.main()
