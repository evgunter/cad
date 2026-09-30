"""The selector surface's own behavior rows (LIB-PYSEL).

`test_north_star.py` executes the surface at scene scale (the
`diecomposed` flip); these are the door-level rows: the decided
predicate against a datum, the typed `SelectRefusal`, and the
boundary refusals on the pattern vocabulary. Everything here narrows
by VALUE — no test reads inside a name text, because that is the
contract the surface exists to keep.
"""

import json
import subprocess
import sys
import unittest
from pathlib import Path

from pncad import (
    CapEnd,
    Cmp,
    Doc,
    EntityKind,
    Expr,
    GeomPred,
    NamePat,
    Node,
    SegPat,
    SegTag,
    SelectRefusal,
    Selector,
    evaluate,
    m,
)


def unit_cube(doc):
    square = doc.insert(
        Node.polygon([
            (Expr.length_in(0, m), Expr.length_in(0, m)),
            (Expr.length_in(1, m), Expr.length_in(0, m)),
            (Expr.length_in(1, m), Expr.length_in(1, m)),
            (Expr.length_in(0, m), Expr.length_in(1, m)),
        ], plane=doc.sketch_frame())
    )
    return doc.insert(Node.extrude(square, Expr.length_in(1, m)))


class TestDatumDistance(unittest.TestCase):
    """The DECIDED atom: position is datum-RELATIVE (GS-Q6), the
    comparison is the sign trilean, and the answer agrees with the
    structural description of the same face — the Rust module-docs
    example, crossed."""

    def test_the_top_cap_is_the_face_a_metre_above_the_ground(self):
        doc = Doc()
        cube = unit_cube(doc)
        ground = doc.insert(Node.datum_plane((
            Expr.length_in(0, m),
            Expr.length_in(0, m),
            Expr.length_in(0, m),
        ), (
            Expr.literal(0.0),
            Expr.literal(0.0),
            Expr.literal(1.0),
        )))
        ev = evaluate(doc)

        faces = Selector.of(NamePat.of_kind(EntityKind.Face))
        by_position = ev.select_where(
            cube, faces, [GeomPred.datum_distance(ground, Cmp.Approx, Expr.length_in(1, m))]
        )
        by_role = ev.select(
            cube,
            Selector.of(
                NamePat.of_kind(EntityKind.Face).seg(
                    SegPat.tag(SegTag.Cap).side(CapEnd.End)
                )
            ),
        )
        self.assertEqual(len(by_position), 1)
        self.assertEqual(by_position, by_role)

        # The strict arms: four wall faces sit definitely BELOW the
        # stated metre (their carrier origins are on the walls'
        # centroids), and none sits definitely above.
        below = ev.select_where(
            cube, faces, [GeomPred.datum_distance(ground, Cmp.Less, Expr.length_in(1, m))]
        )
        self.assertEqual(len(below), 5)
        above = ev.select_where(
            cube, faces, [GeomPred.datum_distance(ground, Cmp.Greater, Expr.length_in(1, m))]
        )
        self.assertEqual(above, [])

    def test_a_non_datum_reference_refuses_typed(self):
        """The refusal is the Rust door's own, crossing as the typed
        `SelectRefusal` — reason tag plus the offending node, never a
        silent empty answer."""
        doc = Doc()
        cube = unit_cube(doc)
        ev = evaluate(doc)
        with self.assertRaises(SelectRefusal) as caught:
            ev.select_where(
                cube,
                Selector.of(NamePat.of_kind(EntityKind.Face)),
                [GeomPred.datum_distance(cube, Cmp.Approx, Expr.length_in(1, m))],
            )
        refusal = caught.exception
        self.assertEqual(refusal.reason, "not_a_datum")
        self.assertEqual(refusal.datum, cube)
        self.assertEqual(refusal.found, "body")
        # The stub-promised attributes are PRESENT and None, not
        # missing (the over-promising-stub rule).
        self.assertIsNone(refusal.name)
        self.assertIsNone(refusal.predicate)

    def test_an_in_band_margin_refuses_rather_than_guessing(self):
        """The decided trilean's REFUSAL arm, end to end: a candidate
        whose margin lands strictly inside the ambiguity band
        (ε, K·ε) is neither included nor dropped — the query raises
        the typed refusal naming the funnel site (SELECT-DESIGN §2's
        razor-thin-selection-cliff rule, crossing exactly as Rust's
        `SelectRefusal::InBand`).

        The sliver: state the top face's distance off by
        ε·(1 + K)/2 — the band's midpoint (`DEFAULT_K` = 10; the ε is
        the document's own, so the row survives CI's tolerance
        sweep)."""
        doc = Doc()
        cube = unit_cube(doc)
        ground = doc.insert(Node.datum_plane((
            Expr.length_in(0, m),
            Expr.length_in(0, m),
            Expr.length_in(0, m),
        ), (
            Expr.literal(0.0),
            Expr.literal(0.0),
            Expr.literal(1.0),
        )))
        ev = evaluate(doc)

        sliver = doc.epsilon * (1.0 + 10.0) / 2.0
        with self.assertRaises(SelectRefusal) as caught:
            ev.select_where(
                cube,
                Selector.of(NamePat.of_kind(EntityKind.Face)),
                [GeomPred.datum_distance(ground, Cmp.Approx, Expr.length_in(1.0 + sliver, m))],
            )
        refusal = caught.exception
        self.assertEqual(refusal.reason, "in_band")
        self.assertEqual(refusal.predicate, "sel_datum_distance")
        # The candidate is NAMED — as opaque text, carried not read.
        self.assertIsInstance(refusal.name, str)
        self.assertIsNone(refusal.matched)
        self.assertIsNone(refusal.candidates)
        self.assertIsNone(refusal.datum)
        self.assertIsNone(refusal.dim)


class TestPatternBoundary(unittest.TestCase):
    """The pattern vocabulary's runtime boundary: the same shapes the
    ty fixtures pin statically, refused for interpreter-only users."""

    def test_an_empty_selector_matches_nothing(self):
        doc = Doc()
        cube = unit_cube(doc)
        self.assertEqual(evaluate(doc).select(cube, Selector.any_of([])), [])

    def test_matches_takes_a_name_not_prose(self):
        with self.assertRaises(ValueError):
            Selector.of(NamePat.any()).matches("the top edge")

    def test_the_side_argument_is_a_side_vocabulary_value(self):
        with self.assertRaises(TypeError):
            SegPat.tag(SegTag.Cap).side("top")

    def test_a_kind_set_holds_its_own_family(self):
        with self.assertRaises(TypeError):
            GeomPred.curve_kind(EntityKind.Edge)

    def test_builders_return_new_values(self):
        """Patterns are immutable: narrowing `any()` yields a NEW
        pattern; the original still matches everything it did."""
        doc = Doc()
        cube = unit_cube(doc)
        ev = evaluate(doc)
        base = NamePat.any()
        narrowed = base.seg(SegPat.tag(SegTag.Cap))
        self.assertEqual(len(ev.select(cube, Selector.of(narrowed))), 2)
        # 6 faces + 12 edges + 8 vertices + 1 body.
        self.assertEqual(len(ev.select(cube, Selector.of(base))), 27)

    def test_the_union_is_additive(self):
        doc = Doc()
        cube = unit_cube(doc)
        ev = evaluate(doc)
        caps = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Cap))
        rims = NamePat.of_kind(EntityKind.Edge).seg(SegPat.tag(SegTag.RimEdge))
        self.assertEqual(len(ev.select(cube, Selector.of(caps))), 2)
        self.assertEqual(len(ev.select(cube, Selector.of(caps).or_(rims))), 10)


# A chain of patterns, each over the one before, names its faces one
# level deeper per pattern; a pattern built by wrapping nests one level
# per wrap. Both are walked on a `threading.Thread` given the wasm32
# build's one-mebibyte stack, less than any thread the binding runs on.
_NESTED = r"""
import json
import sys
import threading

from pncad import Doc, Expr, NamePat, Node, PatternKind, SegPat, evaluate, m

copies, wraps = int(sys.argv[1]), int(sys.argv[2])
said = {}


def nest(pat, levels):
    for _ in range(levels):
        pat = NamePat.any().seg(SegPat.any().of([pat]))
    return pat


def run():
    doc = Doc()
    zero, one = Expr.length_in(0, m), Expr.length_in(1, m)
    square = doc.insert(
        Node.polygon(
            [(zero, zero), (one, zero), (one, one), (zero, one)],
            plane=doc.sketch_frame(),
        )
    )
    node = doc.insert(Node.extrude(square, one))
    step = PatternKind.linear(
        (Expr.literal(1.0), Expr.literal(0.0), Expr.literal(0.0)),
        Expr.length_in(2, m),
    )
    for _ in range(copies):
        node = doc.insert(Node.pattern(node, Expr.count(1), step))
    face = evaluate(doc).all_faces(node)[0]
    try:
        said["as_deep"] = nest(NamePat.any(), copies).matches(face)
        said["one_deeper"] = nest(NamePat.any(), copies + 1).matches(face)
    except ValueError as refused:
        said["refused"] = str(refused)[-300:]
    deep = nest(NamePat.any(), wraps)
    said["levels_shown"] = repr(deep).count("NamePat {")
    said["kept"] = deep.matches(face)


threading.stack_size(1 << 20)
thread = threading.Thread(target=run)
thread.start()
thread.join()
print(json.dumps(said))
"""


class TestNestingPastEveryStack(unittest.TestCase):
    """A name and a pattern nested past every stack are read, matched,
    printed and dropped on a `threading.Thread`, in a child interpreter:
    the failure this row guards against is a dead process."""

    COPIES = 50
    WRAPS = 2_000

    def test_a_deep_name_and_a_deep_pattern_walk_on_a_thread(self):
        child = subprocess.run(
            [sys.executable, "-c", _NESTED, str(self.COPIES), str(self.WRAPS)],
            cwd=Path(__file__).resolve().parent,
            capture_output=True,
            text=True,
            timeout=600,
            check=False,
        )
        self.assertEqual(
            child.returncode,
            0,
            f"the interpreter survives the walks: {child.stderr[-2000:]}",
        )
        said = json.loads(child.stdout)
        self.assertNotIn("refused", said, "the name's own text reads back")
        self.assertTrue(
            said["as_deep"],
            "a name as deep as the chain reads back from its text and "
            "matches a pattern as deep",
        )
        self.assertFalse(said["one_deeper"], "and not one a level deeper")
        self.assertEqual(said["levels_shown"], self.WRAPS + 1, "repr shows every level")
        self.assertFalse(said["kept"], "a pattern deeper than the name does not match it")


if __name__ == "__main__":
    unittest.main()
