"""A comparable value is a KEY: every tag this module exposes goes in
a set and comes back out of a dict.

WHY THIS IS A TEST AND NOT A REVIEW NOTE. Python's rule is that a type
defining `__eq__` and no `__hash__` is unhashable, and PyO3 spells the
two as separate `#[pyclass]` options, so a mirror that asks for `eq`
and forgets `hash` compiles, compares, reads correctly in a REPL and
raises `TypeError` the first time someone tallies by it. Every mirror
in the crate was in exactly that state, and nothing failed. The check
below is what makes the next one fail.

ENUMERATED FROM THE MODULE, never from a written-down list. A fieldless
mirror is recognised by the shape PyO3 gives it — a class carrying its
own variants as class attributes — so a mirror added tomorrow joins
these rows automatically and fails them if it does not hash. A written
roster would have had to be updated by the same hand that forgot the
option.

WHAT THE MIRROR PROBE CANNOT MATCH, stated so its silence is not read
as coverage: a class whose instances come only from a door, never from
a class attribute, is invisible to it — there is nothing to hash
without building one. `Denotation` is exactly that shape (a
hand-written `__eq__` on a value the read-back doors mint) and gets its
own row below; the general case is covered from the other side by
`TestNothingComparesWithoutHashing`, which reads `__hash__` off the
class and needs no instance at all.
"""

import inspect
import unittest

import pncad
from pncad import Doc, Node, SurfaceKind, evaluate, m


def enum_mirrors():
    """Every fieldless enum mirror the module exposes, as
    `{class name: {member name: member}}`.

    The recogniser is PyO3's own shape for a simple enum: the class
    carries each variant as a public class attribute that is an
    INSTANCE of the class. Nothing else in this module has that shape —
    a value class's class attributes are constructors and properties,
    which are not instances of it.
    """
    mirrors = {}
    for name, cls in sorted(vars(pncad).items()):
        if not inspect.isclass(cls):
            continue
        members = {
            attr: value
            for attr, value in vars(cls).items()
            if not attr.startswith("_") and isinstance(value, cls)
        }
        if members:
            mirrors[name] = members
    return mirrors


#: The item holding the classes below that are undecided rather than
#: deliberate.
#:
#: Written WITHOUT its `.md` suffix, and that is load-bearing: a `.md`
#: string literal in a python file under `crates/` is a page
#: `scripts/ci-filter.py` has to resolve to a repo path before it can
#: decide the change tier, and it fails closed on one it cannot. This
#: is prose naming a tracker item, not a page any suite reads.
FILED = "undecided — work/lib/pncad-py-value-classes-compare-without-hashing"

#: The classes that compare without hashing, each with WHY. A class
#: that lands here without a line is a defect, not a decision, which is
#: what `TestNothingComparesWithoutHashing` is for.
#:
#: Read the values as prose; nothing parses them.
UNHASHABLE = {
    "Expr": "by design: equality is an IEEE comparison of the literals "
    "inside, so `0.0` and `-0.0` are equal trees whose bits are not "
    "(stated on the stub)",
    "MeasureExpr": "by design: `Expr`'s reason, for `Expr`'s trees",
    "Alignment": FILED,
    "AnalysisPolicy": FILED,
    "CheckEvidence": FILED,
    "CheckFinding": FILED,
    "ChecksConfig": FILED,
    "ChecksReport": FILED,
    "FlushFinding": FILED,
    "MateFrame": FILED,
    "MatePrimitive": FILED,
}


class TestEveryMirrorIsAKey(unittest.TestCase):
    """The mirrors, enumerated from the module and used as keys."""

    def setUp(self):
        self.mirrors = enum_mirrors()
        # A probe that found nothing would pass every row below in
        # silence. It has to find the surface it is measuring.
        self.assertGreater(len(self.mirrors), 20)

    def test_every_member_of_every_mirror_hashes(self):
        unhashable = sorted(
            f"{name}.{attr}"
            for name, members in self.mirrors.items()
            for attr, member in members.items()
            if type(member).__hash__ is None
        )
        self.assertEqual(
            unhashable, [], "comparable enum mirrors that cannot be a dict key"
        )

    def test_every_member_of_every_mirror_reads_back_through_a_dict(self):
        """One set over the WHOLE surface, then one dict read back
        member by member: the two operations a tag is for."""
        by_member = {
            member: f"{name}.{attr}"
            for name, members in self.mirrors.items()
            for attr, member in members.items()
        }
        total = sum(len(members) for members in self.mirrors.values())
        self.assertEqual(len(by_member), total, "two members collided as one key")
        self.assertEqual(len({*by_member}), total)
        for name, members in self.mirrors.items():
            for attr, member in members.items():
                self.assertEqual(by_member[member], f"{name}.{attr}")

    def test_hash_agrees_with_equality_over_every_pair_of_members(self):
        """The contract, over every ordered pair of every mirror: equal
        values hash equal. Unequal values MAY collide and this does not
        ask them not to — a hash that separated everything would be a
        stronger claim than Python makes."""
        for name, members in self.mirrors.items():
            for left_attr, left in members.items():
                for right_attr, right in members.items():
                    if left == right:
                        self.assertEqual(
                            hash(left),
                            hash(right),
                            f"{name}: .{left_attr} == .{right_attr}"
                            " and the two hash differently",
                        )

    def test_a_mirror_read_off_a_door_keys_the_same_as_the_class_attribute(self):
        """The case the workaround was written for: a tag that came out
        of a door, not off the class, finds the entry the class
        attribute put there."""
        doc = Doc()
        square = doc.insert(
            Node.polygon(
                [(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m), (0 * m, 1 * m)],
                plane=doc.sketch_frame(),
            )
        )
        plate = doc.insert(Node.extrude(square, 1 * m))
        ev = evaluate(doc)
        tally = {}
        for face in ev.all_faces(plate):
            kind = ev.face_carrier_kind(plate, face)
            tally[kind] = tally.get(kind, 0) + 1
        self.assertEqual(tally, {SurfaceKind.Plane: 6})


class TestADenotationIsAKey(unittest.TestCase):
    """`Denotation` compares by hand, so it hashes by hand, and the two
    are checked against each other on values a DOOR minted — two reads
    of one name are distinct objects that must key the same."""

    def setUp(self):
        from test_readback import end_cap, unit_cube

        self.doc = Doc()
        self.cube = unit_cube(self.doc)
        self.ev = evaluate(self.doc)
        self.cap = end_cap(self.ev, self.cube)

    def test_two_reads_of_one_name_are_one_key(self):
        first = self.ev.denotation(self.cube, self.cap)
        second = self.ev.denotation(self.cube, self.cap)
        self.assertIsNot(first, second)
        self.assertEqual(first, second)
        self.assertEqual(hash(first), hash(second))
        self.assertEqual(len({first, second}), 1)
        self.assertEqual({first: "unique"}[second], "unique")


class TestNothingComparesWithoutHashing(unittest.TestCase):
    """The other direction, over EVERY class the module exposes and
    without building an instance of any of them: CPython sets
    `__hash__` to `None` on a type that defines `__eq__` and no
    `__hash__`, so the defect is readable off the class object."""

    def setUp(self):
        self.classes = {
            name: cls for name, cls in vars(pncad).items() if inspect.isclass(cls)
        }

    def test_every_unhashable_class_is_on_the_roster_with_a_reason(self):
        unhashable = sorted(
            name for name, cls in self.classes.items() if cls.__hash__ is None
        )
        self.assertEqual(
            sorted(name for name in unhashable if name not in UNHASHABLE),
            [],
            "classes that compare without hashing and say nowhere why",
        )

    def test_the_roster_carries_no_stale_entry(self):
        stale = sorted(
            f"{name} ({'hashes now' if name in self.classes else 'no such class'})"
            for name in UNHASHABLE
            if name not in self.classes or self.classes[name].__hash__ is not None
        )
        self.assertEqual(stale, [], "stale roster entries — remove them")


if __name__ == "__main__":
    unittest.main()
