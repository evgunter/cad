"""The STATIC half of §L4's two-layer type story.

`tests/test_stubs.py` proves the stub and the compiled module agree
name for name; nothing there looks at a signature. This file is the
signature check: it hands `ty` the stub plus two fixtures and demands
that legal authoring typechecks clean and that every deliberately
off-lattice line draws a diagnostic. That is the Python analog of the
Rust `compile_fail` probes — the lattice is only a type story if a
type checker can be shown enforcing it.

`ty` is not vendored: the test uses `$PNCAD_TY` if set, else a `ty` on
PATH, and FAILS otherwise: an environment without a type checker
cannot report this check green.
"""

import os
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
FIXTURES = HERE / "ty_fixtures"
STUB = HERE.parent / "pncad.pyi"

# The interpreter version the wheel targets in CI; the stubs use PEP
# 585 builtin generics, so the checker must not be told 3.8.
PYTHON_VERSION = "3.12"


def ty_binary():
    ty = os.environ.get("PNCAD_TY") or shutil.which("ty")
    if ty is None:
        raise RuntimeError("no `ty` on PATH and PNCAD_TY unset")
    return ty


class TestTheStubLatticeTypechecks(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        # A search path holding the stub ALONE: the crate directory
        # would also expose `tests` and `examples` as importable, which
        # is not the shape a wheel installs.
        cls._stage = tempfile.TemporaryDirectory(prefix="pncad-ty-")
        cls.search = Path(cls._stage.name)
        shutil.copy(STUB, cls.search / "pncad.pyi")

    @classmethod
    def tearDownClass(cls):
        cls._stage.cleanup()

    def check(self, fixture):
        return subprocess.run(
            [
                ty_binary(),
                "check",
                "--output-format",
                "concise",
                "--color",
                "never",
                "--python-version",
                PYTHON_VERSION,
                "--extra-search-path",
                str(self.search),
                str(FIXTURES / fixture),
            ],
            capture_output=True,
            text=True,
            check=False,
        )

    def test_legal_authoring_typechecks_clean(self):
        done = self.check("legal.py")
        self.assertEqual(
            done.returncode, 0, f"ty rejected legal authoring:\n{done.stdout}{done.stderr}"
        )

    def test_every_off_lattice_line_is_a_type_error(self):
        source = (FIXTURES / "illegal.py").read_text(encoding="utf-8").splitlines()
        marked = {
            n for n, line in enumerate(source, 1) if line.rstrip().endswith("# ty: error")
        }
        self.assertGreaterEqual(len(marked), 9, "the illegal fixture lost its rows")

        done = self.check("illegal.py")
        self.assertNotEqual(done.returncode, 0, "ty accepted off-lattice authoring")
        flagged = set()
        for row in done.stdout.splitlines():
            parts = row.split(":", 3)
            if len(parts) >= 3 and parts[0].endswith("illegal.py") and parts[1].isdigit():
                flagged.add(int(parts[1]))
        self.assertEqual(
            sorted(marked - flagged),
            [],
            f"ty accepted lines the lattice forbids:\n{done.stdout}",
        )
        self.assertEqual(
            sorted(flagged - marked),
            [],
            f"ty flagged lines the fixture calls legal:\n{done.stdout}",
        )


if __name__ == "__main__":
    unittest.main()
