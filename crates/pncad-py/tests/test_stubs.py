"""The stub/module name-for-name check, as a TEST.

§L4: the `.pyi` lattice must not drift from the compiled module.
LIB-U9S verified this by hand and recorded a follow-up ("stub-check
job ... once a pip-capable Python exists"); until that job lands,
this test is the durable form — it needs only the stdlib, so it runs
in the same degraded environment as the rest of the suite.

Scope, stated honestly: NAME-level equality at TWO depths, both
directions —

1. the module's TOP-LEVEL surface (classes, functions, constants),
   which is what this file checked from the day it was written; and
2. the CLASS ATTRIBUTES of every class the stub declares at top level
   — its methods, its `@property` doors, its `Final` class constants
   (which is how every enum member in this stub is spelled), against
   what the compiled class actually carries.

Depth 2 exists because depth 1 alone was blind to exactly the drift
that matters most in a hand-written stub: a forgotten enum member or
a renamed method changes no top-level name, so the whole Python suite
stayed green while the stub advertised a surface that did not exist
(the issue is `work/lib/stub-check-never-descends-class-attributes.md`,
raised off BLEND-5's review of `RimSupport::{Host, Mate}`).

WHAT IS STILL NOT CHECKED HERE, stated so nobody reads more into a
green run than it earned:

- SIGNATURES. Argument names, arity, types and return types are
  `ty`'s job, over `tests/ty_fixtures/`. This file compares NAMES.
- COVERAGE — whether the Rust façade's curated surface has a Python
  spelling at all — is `test_binding_census.py`'s. This check is
  satisfied by a stub and a module that agree on nothing being bound.
  That file's `stub_surface` reads `Class.attribute` names out of the
  same stub, which looks like the walk below and is not: it asks
  whether a `BOUND_AS` right-hand side is DECLARED anywhere in the
  `.pyi`, and never consults the compiled module at all. Nothing there
  can see a stub member the extension does not carry, which is the
  half this file owns.
- Attributes of NESTED classes. The walk is ONE level deep: a class
  declared inside a top-level class has its NAME compared like any
  other class attribute, and its BODY is not descended into. The stub
  declares no nested classes today, so nothing is being skipped; the
  day one appears, this sentence is the notice that its members are
  unguarded.
- INSTANCE attributes. A bare annotation in a stub class body
  (`variant: str`) declares an attribute that exists on a raised
  instance and never on the class object, so no comparison against the
  class can see it — see `stub_class_names` for the convention that
  makes this decidable, and `test_every_arm_carries_every_attribute`
  in `test_picking.py` / `test_workspace.py` for what does check those
  payloads.
- Most DUNDERS. `test_declared_operators_exist_on_the_compiled_class`
  checks the operator half in one direction only; its docstring says
  exactly which names that reaches and which it cannot.

LAYOUT-INVARIANT (R1/R2 converged finding): the module is importable
two ways — the repo script's flat `pncad.so` staging, and the wheel's
package layout (`pncad/__init__.py` re-exporting `pncad/pncad.abi3.so`),
where `dir(pncad)` also contains module-typed attributes (the
package's own submodule self-reference). Module-typed attributes are
implementation artifacts of the layout, not API surface, so
`module_names` excludes them; the check must be green under BOTH
layouts. The class-attribute walk inherits that invariance for free:
it reads a CLASS's own `__dict__`, and re-exporting a class through a
package `__init__` binds the same class object under a second name
rather than making a second class, so the layout cannot change what
`vars(cls)` answers.
"""

import ast
import types
import unittest
from pathlib import Path

import pncad

STUB = Path(__file__).resolve().parent.parent / "pncad.pyi"


def stub_tree():
    """The stub, parsed once per call — it is ~3.6k lines, and the
    tests below each want their own view of it."""
    return ast.parse(STUB.read_text())


def stub_names():
    """Every top-level name the stub declares.

    Underscore-prefixed names are the stub's own PRIVATE spelling
    machinery — the TypeVars and type aliases that let one overload
    stand for a family — and are no more surface than a Rust type
    alias inside a signature. The module cannot expose them, and a
    checker never resolves them from outside, so they are excluded on
    the same rule that excludes them from `module_names`.
    """
    tree = stub_tree()
    names = set()
    for node in tree.body:
        if isinstance(node, (ast.ClassDef, ast.FunctionDef)):
            names.add(node.name)
        elif isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name):
            names.add(node.target.id)
        elif isinstance(node, ast.Assign):
            for target in node.targets:
                if isinstance(target, ast.Name):
                    names.add(target.id)
    return {
        name
        for name in names
        if not name.startswith("_") or name == "__build_info__"
    }


def module_names():
    """Every public top-level API name the compiled module exposes.

    Module-typed attributes are excluded: under the wheel's package
    layout `dir(pncad)` carries the extension submodule itself, which
    is layout plumbing, not surface.
    """
    return {
        name
        for name in dir(pncad)
        if (not name.startswith("_") or name == "__build_info__")
        and not isinstance(getattr(pncad, name), types.ModuleType)
    }


def stub_classes():
    """Every top-level `class` the stub declares, by name.

    Underscore-prefixed classes would be private spelling machinery on
    the same rule `stub_names` states; the stub declares none today,
    and the filter is here so that adding one does not silently mint a
    class-attribute comparison against a name the module cannot have.
    """
    return {
        node.name: node
        for node in stub_tree().body
        if isinstance(node, ast.ClassDef) and not node.name.startswith("_")
    }


#: The statement kinds `stub_class_names` knows how to read.
#:
#: THIS IS A FAIL-LOUD LIST, not a convenience. A walk that ignores
#: what it does not recognise is the exact blindness this file's
#: second depth was added to close: put an `if TYPE_CHECKING:` block,
#: a `match`, or a `for` loop in a stub class body and every name
#: inside it would silently stop being compared, with the suite still
#: green. So an unrecognised statement fails
#: [`TestStubClassDrift.test_every_stub_class_body_uses_a_statement_this_walk_understands`]
#: instead, and whoever adds the construct decides — in the same PR —
#: whether the walk should descend it or whether the names inside it
#: are genuinely out of scope.
#:
#: `Expr` is docstrings and the `...` bodies a stub is made of; it
#: declares no name and is read for nothing. `Pass` is the same. The
#: top-level walk in `stub_names` deliberately has NO such list: a
#: module body legitimately carries `import`/`from` statements, whose
#: names are typing vocabulary rather than surface.
KNOWN_CLASS_BODY_STATEMENTS = (
    ast.Expr,
    ast.Pass,
    ast.FunctionDef,
    ast.AnnAssign,
    ast.Assign,
    ast.ClassDef,
)


def stub_class_names(node):
    """The names a stub class declares AS CLASS ATTRIBUTES.

    **The annotation convention, which this reading depends on.** In a
    `.pyi` an annotation is a DECLARATION, and its syntax alone does
    not say whether the name lives on the class or on an instance —
    `x: int` in a class body creates no attribute at runtime either
    way. This stub holds one convention throughout:

    - `Host: Final[RimSupport]` — a CLASS-level constant. Every enum
      member in the file is spelled this way, and every one of them is
      a real attribute of the compiled class.
    - `variant: str` — an INSTANCE attribute, which is how every
      refusal payload on every `PncadError` subclass is declared. It
      exists on a raised exception and never on the class object, so
      comparing it against the class would be comparing against
      something that cannot be there.

    So `Final[...]` annotations are read as class attributes and bare
    ones are not. The convention is SELF-ENFORCING rather than
    trusted: spell a real class constant bare and the compiled class
    carries a name the stub no longer declares (module-only drift);
    spell an instance payload `Final` and the stub declares a name the
    class does not carry (stub-only drift). Both fail
    [`TestStubClassDrift.test_class_attributes_agree_name_for_name`].

    Methods, `@property` doors, `@staticmethod`s and nested classes
    are all just names here — the decorators change what the name IS,
    not whether it exists, and what it is belongs to `ty`. Repeated
    `@overload` declarations of one name collapse into that one name,
    which is right: the module carries one attribute per name however
    many overloads describe it.

    Underscore-prefixed names are excluded on the rule `stub_names`
    states, one level down: the private aliases and TypeVars, and the
    dunders, which are argued separately at
    `test_declared_operators_exist_on_the_compiled_class`.
    """
    names = set()
    for stmt in node.body:
        if isinstance(stmt, (ast.FunctionDef, ast.ClassDef)):
            names.add(stmt.name)
        elif isinstance(stmt, ast.AnnAssign) and isinstance(stmt.target, ast.Name):
            annotation = ast.unparse(stmt.annotation)
            if annotation == "Final" or annotation.startswith("Final["):
                names.add(stmt.target.id)
        elif isinstance(stmt, ast.Assign):
            for target in stmt.targets:
                if isinstance(target, ast.Name):
                    names.add(target.id)
    return {name for name in names if not name.startswith("_")}


def stub_class_operators(node):
    """The dunder methods a stub class declares that `object` does not
    itself provide — `__add__`, `__mul__`, `__len__` and their family.

    WHY THE FILTER. Checking a dunder against the compiled class means
    asking `hasattr`, and `hasattr(cls, "__eq__")` is True for every
    class ever written, so a check over ALL dunders would be loudly
    vacuous for `__init__`, `__eq__`, `__hash__`, `__repr__` and the
    comparison family — it would report success having asked nothing.
    The names `object` does NOT provide are the ones where the answer
    is a fact about THIS binding: if `Length.__mul__` were dropped from
    the pymethods block, `hasattr` says so.

    `vars(cls)` is not usable for this direction either, because PyO3
    spells construction as `__new__` while the stub spells it
    `__init__` (17 classes do), so an own-dict test would fail on
    every one of them for a difference that is not drift.
    """
    return {
        stmt.name
        for stmt in node.body
        if isinstance(stmt, ast.FunctionDef)
        and stmt.name.startswith("_")
        and not hasattr(object, stmt.name)
    }


def stub_class_dunders(node):
    """EVERY dunder a stub class declares, whatever `object` provides.

    [`stub_class_operators`] is this walk narrowed to the names
    `object` does not carry, because that row's question is answered
    with `hasattr`. The row below asks a question `hasattr` cannot
    answer and the compiled class's own `__dict__` can, so it needs
    the unnarrowed set.
    """
    return {
        stmt.name
        for stmt in node.body
        if isinstance(stmt, ast.FunctionDef) and stmt.name.startswith("_")
    }


def stub_enum_member_names(node):
    """The `Final[<this class>]` constants a stub class declares.

    That annotation is the stub's one spelling for a FIELDLESS MIRROR's
    members — a Rust enum whose Python class is its variants and
    nothing else. Read here rather than assumed from a name list, so a
    mirror that grows a variant or a value class that grows a
    self-typed constant moves the classification with it.
    """
    return {
        stmt.target.id
        for stmt in node.body
        if isinstance(stmt, ast.AnnAssign)
        and isinstance(stmt.target, ast.Name)
        and ast.unparse(stmt.annotation) == f"Final[{node.name}]"
    }


def module_class_names(cls):
    """The public names a compiled class carries IN ITS OWN `__dict__`.

    OWN, not `dir()`, and that is the whole subtlety of this direction.
    `dir()` walks the MRO, so every `PncadError` subclass would report
    `args`, `add_note` and `with_traceback` — `BaseException`'s
    machinery, which the stub does not declare and must not be made to
    declare, since restating the standard exception protocol in 30
    class bodies documents nothing. Reading the class's own dict drops
    them at the source rather than by an exclusion list that would go
    stale with the next CPython release (`add_note` is 3.11's).

    The two sides are therefore compared as OWN declarations: the stub
    class's own body against the compiled class's own dict. That is
    symmetric, and it is the reason a name INHERITED on one side and
    declared on the other would read as drift — no such case exists in
    this stub today (the only inheritance here is the error hierarchy,
    where PncadError declares nothing to inherit).

    Underscore-prefixed names are excluded for the reason
    `stub_class_names` gives: they are PyO3's unconditional machinery
    (`__doc__`, `__module__`, `__new__`, `__repr__`, `__weakref__`,
    and the whole comparison family minted from one `__richcmp__`),
    which a stub does not and should not restate.

    **The filter stays, and ONE name is checked around it rather than
    through it.** Restating the exception protocol and PyO3's slots in
    130 class bodies documents nothing, so widening this reader is the
    wrong repair — but `__eq__` is not machinery. A name in the own
    dict is a decision this binding made, and it is the ONE comparison
    dunder whose presence there is evidence: `__lt__` sits in the dict
    of a class whose `<` raises `TypeError` and `__hash__` sits there
    as `None` on a class that is unhashable, both minted alongside,
    while `__eq__` compares by value whenever it is present. So it
    gets its own row —
    [`TestStubClassDrift.test_a_class_that_compares_by_value_declares_it`]
    — and this set keeps answering the question it was written for.
    """
    return {name for name in vars(cls) if not name.startswith("_")}


class TestStubDrift(unittest.TestCase):
    def test_stub_and_module_agree_name_for_name(self):
        stub = stub_names()
        module = module_names()
        self.assertEqual(
            sorted(module - stub),
            [],
            "the compiled module exposes names the stub does not declare",
        )
        self.assertEqual(
            sorted(stub - module),
            [],
            "the stub declares names the compiled module does not expose",
        )

    def test_the_check_is_not_vacuous(self):
        # A scanner bug that returned nothing would pass equality on
        # two empty sets; pin a few names that must be present.
        names = stub_names()
        for expected in ("Doc", "evaluate", "load", "import_step", "mm"):
            self.assertIn(expected, names)


class TestStubClassDrift(unittest.TestCase):
    """Depth 2: the class attributes, both directions.

    The module docstring states what this reaches and what it does
    not; each test below restates its own half.
    """

    def test_every_stub_class_body_uses_a_statement_this_walk_understands(self):
        """A construct the walk cannot read fails HERE, not silently.

        See `KNOWN_CLASS_BODY_STATEMENTS`: the point of the list is
        that a stub class body which grows an `if TYPE_CHECKING:` — or
        anything else — does not quietly take its contents out of the
        comparison.
        """
        unreadable = sorted(
            f"{name}: {type(stmt).__name__} at line {stmt.lineno}"
            for name, node in stub_classes().items()
            for stmt in node.body
            if not isinstance(stmt, KNOWN_CLASS_BODY_STATEMENTS)
        )
        self.assertEqual(
            unreadable,
            [],
            "a stub class body carries a statement the attribute walk "
            "does not descend; teach `stub_class_names` to read it, or "
            "argue at the site why the names inside it are out of scope",
        )

    def test_class_attributes_agree_name_for_name(self):
        """The claim: for every class the stub declares at top level,
        the names it declares as CLASS attributes and the names the
        compiled class carries in its own dict are the same set.

        Both directions, because both are real failures: a member the
        stub declares and the module does not have is a promise to a
        caller that `ty` will happily typecheck and the interpreter
        will refuse at runtime; a member the module has and the stub
        does not declare is surface nobody can reach through the typed
        layer, which is the same as unbound to a user reading the stub.

        A class the stub declares and the module does not expose at all
        is NOT reported here — `test_stub_and_module_agree_name_for_name`
        owns that failure, and reporting it twice would make one drift
        read as two.
        """
        stub_only, module_only = [], []
        for name, node in sorted(stub_classes().items()):
            cls = getattr(pncad, name, None)
            if not isinstance(cls, type):
                continue
            declared = stub_class_names(node)
            carried = module_class_names(cls)
            stub_only += [f"{name}.{attr}" for attr in sorted(declared - carried)]
            module_only += [f"{name}.{attr}" for attr in sorted(carried - declared)]
        self.assertEqual(
            module_only,
            [],
            "the compiled class carries attributes the stub does not declare",
        )
        self.assertEqual(
            stub_only,
            [],
            "the stub declares class attributes the compiled class does not carry",
        )

    def test_declared_operators_exist_on_the_compiled_class(self):
        """ONE DIRECTION ONLY, and the docstring of
        `stub_class_operators` says why: a stub-declared `__add__`,
        `__mul__`, `__truediv__`, `__neg__`, `__len__` and their kin
        must resolve on the compiled class, because those are names
        `object` does not hand out for free.

        What this does NOT check, said plainly: the reverse direction
        (an operator the binding grows and the stub never learns
        about), and every dunder `object` does provide — `__init__`,
        `__eq__`, `__hash__`, `__lt__` and the rest — where `hasattr`
        cannot tell an implementation from an inheritance. Those stay
        on `ty` and on the tests that actually exercise the arithmetic.
        """
        missing = sorted(
            f"{name}.{op}"
            for name, node in stub_classes().items()
            for op in stub_class_operators(node)
            if isinstance(getattr(pncad, name, None), type)
            and not hasattr(getattr(pncad, name), op)
        )
        self.assertEqual(
            missing,
            [],
            "the stub declares an operator the compiled class does not "
            "implement",
        )

    def test_a_class_that_compares_by_value_declares_it(self):
        """The claim: a class whose OWN `__dict__` carries `__eq__`
        declares `__eq__` in the stub.

        THE DIRECTION `hasattr` CANNOT REACH.
        `test_declared_operators_exist_on_the_compiled_class` filters
        `__eq__` out and says why — `hasattr(cls, "__eq__")` is True
        for every class ever written, so a check over it reports
        success having asked nothing. The own dict answers what
        `hasattr` cannot: `object` puts nothing in a subclass's dict,
        so a name there is a decision this binding made. It is the one
        comparison dunder where presence is evidence, for the reason
        `module_class_names` states.

        SO THE STUB CAN SAY THE WRONG THING AND NOTHING NOTICES: a
        class that compares by VALUE, declared without `__eq__`, reads
        as comparing by identity, and a caller writes the
        list-of-comparisons workaround for a comparison that works.
        That is the failure this row exists to make loud, and it
        reaches all three routes a class in this crate gets `__eq__`
        by — a `fn __eq__`, a `fn __richcmp__`, and `#[pyclass(eq)]`
        — because it reads the compiled class rather than the Rust.

        THE FIELDLESS MIRRORS ARE OUT, AND THE EXEMPTION IS CHECKED
        RATHER THAN CLAIMED. A mirror's stub body is its variants as
        `Final[<the class>]` constants and nothing else of that kind,
        and the stub declares no dunder on any of them — the
        convention, not drift. It is honest only because the members
        are SINGLETONS: `is` and `==` answer the same question, so a
        reader who takes the dunder-free stub for identity comparison
        is not misled. That is a fact about the compiled module and it
        is asserted below, per member, so the day a mirror stops
        interning its variants this exemption fails instead of
        covering for it.
        """
        undeclared, aliased, compared, exempt = [], [], 0, 0
        for name, node in sorted(stub_classes().items()):
            cls = getattr(pncad, name, None)
            if not isinstance(cls, type) or "__eq__" not in vars(cls):
                continue
            compared += 1
            members = stub_enum_member_names(node)
            if members:
                exempt += len(members)
                aliased += [
                    f"{name}.{member}"
                    for member in sorted(members)
                    if getattr(cls, member) is not getattr(cls, member)
                ]
                continue
            if "__eq__" not in stub_class_dunders(node):
                undeclared.append(name)
        # TWO EMPTY LISTS ARE A PASS, so the reach is pinned as well:
        # a walk that found no comparing class, or no mirror member to
        # check the exemption on, would report success having asked
        # nothing. Floors with room for churn — a rise needs no
        # permission, a DROP through one is the signal.
        self.assertGreater(
            compared, 40, "the scan found almost no class carrying its own `__eq__`"
        )
        self.assertGreater(
            exempt, 80, "the mirror exemption is checked on almost no member"
        )
        self.assertEqual(
            aliased,
            [],
            "a fieldless mirror's member is not interned, so `is` and "
            "`==` no longer answer alike and the stub's silence about "
            "`__eq__` on that class has stopped being harmless",
        )
        self.assertEqual(
            undeclared,
            [],
            "the compiled class compares by value and the stub does not "
            "declare `__eq__`, so it reads as comparing by identity",
        )

    def test_the_class_walk_is_not_vacuous(self):
        """The same argument `test_the_check_is_not_vacuous` makes, at
        the new depth, and it has more ways to go wrong: a walk that
        returned no CLASSES, or read every class body as empty, would
        pass every equality above on two empty sets and report a
        surface it never looked at.

        So pin the shapes the depth exists for: the enum whose members
        are `Final` class constants (the BLEND-5 case that raised the
        issue), an ordinary pyclass's method, a `@property` door, and a
        `@staticmethod`.
        """
        classes = stub_classes()
        self.assertGreater(len(classes), 100, "the stub declares ~136 classes")
        for cls_name, attr in (
            ("RimSupport", "Host"),
            ("RimSupport", "Mate"),
            ("Doc", "insert"),
            ("Datum", "origin"),
            ("Open", "at"),
        ):
            self.assertIn(cls_name, classes, cls_name)
            self.assertIn(attr, stub_class_names(classes[cls_name]), f"{cls_name}.{attr}")
            self.assertIn(
                attr,
                module_class_names(getattr(pncad, cls_name)),
                f"{cls_name}.{attr} on the compiled class",
            )
        # And the operator scan must actually be finding operators,
        # or its one-directional check is a no-op that reads as a pass.
        operators = {
            op for node in classes.values() for op in stub_class_operators(node)
        }
        self.assertIn("__mul__", operators)
        self.assertIn("__len__", operators)
        # THE COUNT THE NAMED PINS ABOVE CANNOT HOLD. Roughly a
        # quarter of the stub's classes compare as two EMPTY sets, and
        # correctly so: the whole `PncadError` hierarchy declares its
        # payload as instance attributes (see `stub_class_names`), and
        # a handful of value classes declare only `__init__`. Those
        # are vacuous rows by construction, not by defect — but they
        # mean a future edit could hollow the walk out to a fraction
        # of its reach with every pin above still green. So the number
        # of classes where at least ONE side is non-empty is itself
        # pinned. It was 98 of 136 when this was written; the floor is
        # set below that with room for ordinary churn, and a DROP
        # through it is the signal — a rise needs no permission.
        compared = sum(
            1
            for name, node in classes.items()
            if stub_class_names(node) or module_class_names(getattr(pncad, name))
        )
        self.assertGreater(
            compared,
            90,
            "the class walk is comparing far fewer classes non-vacuously than "
            "the 98 it reached when this floor was written — something has "
            "stopped reading a side rather than the surface having shrunk",
        )


if __name__ == "__main__":
    unittest.main()
