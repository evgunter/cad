"""Review lane `paramname-rev` probes (PR #3164): the Python doors a
parameter name arrives through from outside."""

import json
import unittest

from pncad import (
    Doc,
    DocEdit,
    DocParam,
    EditError,
    ParamName,
    PersistError,
    load,
    mm,
)


REFUSED = [
    "", " ", "\n", "width\n", " width", "1a", "a b", "a+b", "a\0", "\0",
    "a·b", "​", "٣x", "a:b", "query:certified-range:1:distance",
]
ADMITTED = [
    "_", "sin", "pi", "mm", "e", "inf", "nan", "δ", "日本",
    "Ⅷ", "x٣", "query_certified_range_1_distance", "a" * 10000,
]


class TestParamNameRev(unittest.TestCase):
    def test_constructor_refuses_typed(self):
        for text in REFUSED:
            with self.subTest(text=text):
                with self.assertRaises(EditError) as caught:
                    ParamName(text)
                self.assertEqual(caught.exception.variant, "param_name_not_an_identifier")

    def test_constructor_admits(self):
        for text in ADMITTED:
            with self.subTest(text=text[:20]):
                self.assertEqual(ParamName(text).name, text)

    def test_a_lone_surrogate_refuses_as_the_name_door(self):
        # A Python `str` the `&str` extraction cannot carry: does the
        # door still answer with its own typed refusal?
        with self.assertRaises(EditError) as caught:
            ParamName("\ud800")
        self.assertEqual(caught.exception.variant, "param_name_not_an_identifier")

    def test_load_door_refuses_a_bad_params_key(self):
        doc = Doc("rev")
        doc.apply(DocEdit.set_doc_param(ParamName("width"), DocParam.length(2 * mm)))
        text = doc.save()
        head, _, body = text.partition("{")
        wire = json.loads("{" + body)
        params = wire["snapshot"]["params"]
        params["1 2"] = params.pop("width")
        with self.assertRaises(PersistError) as caught:
            load(head + json.dumps(wire))
        self.assertEqual(caught.exception.variant, "unreadable")
        self.assertIn('parameter name "1 2"', str(caught.exception))


if __name__ == "__main__":
    unittest.main()
