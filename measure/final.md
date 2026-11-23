# wedge-measure, final report (received 2026-11-23T03:06Z)

wedge-measure FINAL' not in line: continue
d=json.loads(line)
def walk(x):
    global out
    if isinstance(x,str):
        if 'wedge-measure FINAL' in x and (out is None or len(x)<len(out)): out=x
    elif isinstance(x,dict):
        for v in x.values(): walk(v)
    elif isinstance(x,list):
        for v in x: walk(v)
walk(d)
i=out.find('wedge-measure FINAL'); j=out.find('