import sys
import pncad
n = int(sys.argv[1])
nested = "(" * n + "1" + ")" * n
text = (
    "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\n"
    "FILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\n"
    f"#1=CARTESIAN_POINT('',{nested});\nENDSEC;\nEND-ISO-10303-21;\n"
)
try:
    pncad.import_step(text)
    print("imported")
except Exception as e:
    print("refused:", type(e).__name__, str(e)[:160])
