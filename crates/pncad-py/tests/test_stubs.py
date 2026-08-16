"""The stub/module name-for-name check, as a TEST.

§L4: the `.pyi` lattice must not drift from the compiled module.
LIB-U9S verified this by hand and recorded a follow-up ("stub-check
job ... once a pip-capable Python exists"); until that job lands,
this test is the durable form — it needs only the stdlib, so it runs
in the same degraded environment as the rest of the suite.

Scope, stated honestly: NAME-level equality of the top-level surface
(classes, functions, constants), both directions. Signature-level
checking remains `ty`'s job.

LAYOUT-INVARIANT (R1/R2 converged finding): the module is importable
two ways — the repo script's flat `pncad.so` staging, and the wheel's
package layout (`pncad/__init__.py` re-exporting `pncad/pncad.abi3.so`),
where `dir(pncad)` also contains module-typed attributes (the
package's own submodule self-reference). Module-typed attributes are
implementation artifacts of the layout, not API surface, so
`module_names` excludes them; the check must be green under BOTH
layouts.
"""

import ast
import types
import unittest
from pathlib import Path

import pncad

STUB = Path(__file__).resolve().parent.parent / "pncad.pyi"


def stub_names():
    """Every top-level name the stub declares.

    Underscore-prefixed names are the stub's own PRIVATE spelling
    machinery — the TypeVars and type aliases that let one overload
    stand for a family — and are no more surface than a Rust type
    alias inside a signature. The module cannot expose them, and a
    checker never resolves them from outside, so they are excluded on
    the same rule that excludes them from `module_names`.
    """
    tree = ast.parse(STUB.read_text())
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


if __name__ == "__main__":
    unittest.main()
