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
from pncad import Doc, Expr, Node, SurfaceKind, WrittenLength, evaluate, m


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


#: The classes that compare without hashing, each with WHY. A class
#: that lands here without a line is a defect, not a decision, which is
#: what `TestNothingComparesWithoutHashing` is for.
#:
#: Read the values as prose; nothing parses them.
#:
#: THE RULE EVERY ENTRY BELOW APPLIES: a Python value class mirrors its
#: Rust type's derives, and the kernel omits `Hash` on values in favour
#: of the funnel — so a Rust type deriving `PartialEq` and no `Hash`
#: gets a Python class that compares and does not hash. Several classes
#: read like keys anyway; the reason says why the mirror answers that.
#:
#: AND WHY THIS IS STILL A HAND-WRITTEN STRING, not a fact derived from
#: the Rust side. Every reason below names a derive list, which is
#: exactly the kind of claim a reader could check structurally — but
#: nothing here can: the binding census (`test_binding_census.py`)
#: reads the façade's `pub use` LINES, so it sees a type's name and
#: never its `#[derive(...)]` attribute, and no other reader in this
#: suite opens a `.rs` file at all. So a derive list that changes in
#: the kernel silently falsifies the prose beside it, and only the two
#: guards below — which read `__hash__` off the class, not the Rust
#: source — stay true. That is the roster's blind spot, stated so its
#: silence is not read as coverage.
UNHASHABLE = {
    "Expr": "by design: equality is an IEEE comparison of the literals "
    "inside, so `0.0` and `-0.0` are equal trees whose bits are not "
    "(stated on the stub)",
    "MeasureExpr": "by design: `Expr`'s reason, for `Expr`'s trees",
    "Length": "by design: the Rust newtype derives `PartialEq` and "
    "`PartialOrd` and no `Hash`, and this class mirrors its derives — "
    "a magnitude is not a key. The authored record that keys is "
    "`WrittenLength`, which folds `-0.0` and hashes",
    "Angle": "by design: `Length`'s reason, for the angle newtype; "
    "`WrittenAngle` is the authored record that keys",
    "Alignment": "by design: `editor_core::Alignment` derives "
    "`PartialEq` and no `Hash`, and this class mirrors its derives. "
    "The datum reads like a key — \"which alignments did this assembly "
    "use\" is a set — but the kernel never keys on one: it solves poses "
    "from alignments and tallies nothing by them, so a Python `set` of "
    "them is a decision the kernel has not made",
    "AnalysisPolicy": "by design: `editor_core::AnalysisPolicy` derives "
    "`PartialEq` and no `Hash`, and this class mirrors its derives. A "
    "policy is an argument to one run (E2: request configuration, never "
    "a global), not a key the kernel tallies by",
    "CheckEvidence": "by design: `editor_core::CheckEvidence` derives "
    "`PartialEq` and no `Hash`, and this class mirrors its derives. It "
    "is the payload of a finding and carries `f64` measurements, so a "
    "hash would have to answer the float question `Expr` answered "
    "before it could exist at all",
    "CheckFinding": "by design: `editor_core::CheckFinding` derives "
    "`PartialEq` and no `Hash`, and this class mirrors its derives. "
    "Findings read like keys — deduplicating them across runs is the "
    "natural want — but the kernel reports them in a deterministic "
    "ORDER rather than a set, and never keys on one; its evidence "
    "bottoms out in floats besides",
    "ChecksConfig": "by design: `editor_core::ChecksConfig` derives "
    "`PartialEq` and `Eq` and no `Hash`, and this class mirrors its "
    "derives. It is a per-run argument holding a `BTreeMap` of "
    "acknowledgments; a hashable aggregate over a collection is a "
    "decision with a cost, and the kernel has not made it",
    "ChecksReport": "by design: `editor_core::ChecksReport` derives "
    "`PartialEq` and no `Hash`, and this class mirrors its derives. A "
    "report holds the run's findings in order; hashing an aggregate "
    "over a list is the same undecided cost, one rung up",
    "Distribution": "by design: `editor_core::Distribution` derives "
    "`PartialEq` and no `Hash`, and this class mirrors its derives. An "
    "offset band is a magnitude with a dimension, not a key; the "
    "authored record that keys is `DocParam`, which folds `-0.0` "
    "through the kernel's own `fold_signed_zeros` and hashes",
    "DocParamValue": "by design: `editor_core::DocParamValue` derives "
    "`PartialEq` and no `Hash`, and this class mirrors its derives. It "
    "is a parameter's magnitude — `Length`'s reason, one rung up; "
    "`DocParam`, the authored declaration, is the row that keys",
    "FaceCensus": "by design: `step_import::FaceCensus` derives "
    "`PartialEq` and `Eq` and no `Hash`, and this class mirrors its "
    "derives. Three counts read like a key, but the kernel reports a "
    "census beside the region it counted and never tallies by one",
    "FlushFinding": "by design: `editor_core::FlushFinding` (the "
    "document seat's `topo::flush::FlushFinding<(StableName, "
    "StableName)>`) derives `PartialEq` and no `Hash`, and this class "
    "mirrors its derives. `CheckFinding`'s reason, for the contact "
    "verifier's findings",
    "Frame": "by design: `editor_core::Frame` derives `PartialEq` and "
    "no `Hash`, and this class mirrors its derives. `MateFrame`'s "
    "reason for the absolute placement: a pose is a datum the solver "
    "consumes, and it bottoms out in `f64`",
    "MateFrame": "by design: `editor_core::MateFrame` derives "
    "`PartialEq` and no `Hash`, and this class mirrors its derives. A "
    "frame is a pose datum the solver consumes, not a key it tallies "
    "by; and it bottoms out in `f64`, so a hash would owe `Expr`'s "
    "float answer",
    "MatePrimitive": "by design: `editor_core::MatePrimitive` derives "
    "`PartialEq` and no `Hash`, and this class mirrors its derives. "
    "The primitive names which coset of SE(3) a mate pins, and the "
    "kernel MATCHES on it rather than keying by it",
    "McAssertion": "by design: `editor_core::McAssertion` derives "
    "`PartialEq` and no `Hash`, and this class mirrors its derives. It "
    "is one row of a report — the counts an assertion drew — reported "
    "in order beside its measures, never keyed on",
    "McMeasure": "by design: `editor_core::McMeasure` derives "
    "`PartialEq` and no `Hash`, and this class mirrors its derives. A "
    "row of empirical statistics is a magnitude five times over, and a "
    "row nothing could sample carries `NaN`, which equals nothing at "
    "all",
    "SketchPlane": "by design: `profile::SketchPlane<f64>` spells `==` "
    "as `bit_eq` and derives no `Hash`, and this class mirrors it. The "
    "comparison is bit-for-bit on both sides — `-0.0` and `0.0` are "
    "different planes — and neither side keys by one",
    "ValidationFinding": "by design: the binding's own "
    "`validation::Finding` projection derives `PartialEq` and `Eq` and "
    "no `Hash`, and this class mirrors its derives — unhashed to match "
    "the findings beside it, which the kernel reports in a "
    "deterministic ORDER rather than a set",
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
                [
                    (Expr.written_length(WrittenLength.in_unit(0, m)), Expr.written_length(WrittenLength.in_unit(0, m))),
                    (Expr.written_length(WrittenLength.in_unit(1, m)), Expr.written_length(WrittenLength.in_unit(0, m))),
                    (Expr.written_length(WrittenLength.in_unit(1, m)), Expr.written_length(WrittenLength.in_unit(1, m))),
                    (Expr.written_length(WrittenLength.in_unit(0, m)), Expr.written_length(WrittenLength.in_unit(1, m))),
                ],
                plane=doc.sketch_frame(),
            )
        )
        plate = doc.insert(Node.extrude(square, Expr.written_length(WrittenLength.in_unit(1, m))))
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
