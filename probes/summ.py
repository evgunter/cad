import re,sys
for l in open(sys.argv[1]):
    m=re.match(r'^((ArcSplit|FullTurn) s\S+ (off|radius|tilt|far)\S* ?\S* posed\S+|ball s\S+ \S+ ?\S*):(.*)',l)
    if not m: continue
    ops=re.findall(r'\[(\S+) (Empty|Body vol \S+|Refused \w+)',m.group(4))
    print(m.group(1).ljust(42),' '.join(f'{o}={v.replace("Refused ","R:").replace("Body vol ","B")}' for o,v in ops))
