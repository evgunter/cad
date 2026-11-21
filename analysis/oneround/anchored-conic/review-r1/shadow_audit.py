import subprocess,re
files=subprocess.run(['git','diff','--name-only','ec8749552','8d5978300','--','crates/*/src/**'],capture_output=True,text=True).stdout.split()
bind=re.compile(r'(\{\s*frame\s*[,}]|\{[^}]*\bframe\s*,|\bframe\s*:\s*&?Frame|\blet\s+(mut\s+)?frame\b|\|frame\||Some\(frame\)|\(frame,)')
for f in files:
    if 'rows' in f or 'test' in f or 'fuzz' in f or 'probe' in f or f.endswith('conic.rs'): continue
    try: L=open(f).read().split('\n')
    except: continue
    diff=subprocess.run(['git','diff','-U0','ec8749552','8d5978300','--',f],capture_output=True,text=True).stdout
    added=set()
    for h in re.finditer(r'^@@ -\d+(?:,\d+)? \+(\d+)(?:,(\d+))? @@',diff,re.M):
        s=int(h.group(1)); n=int(h.group(2) or 1)
        added.update(range(s,s+n))
    for i in sorted(added):
        if i-1>=len(L): continue
        l=L[i-1]
        if re.search(r'let\s+frame\s*=',l):
            # find fn start
            j=i-1
            while j>0 and not re.match(r'\s*(pub(\([^)]*\))?\s+)?(const\s+)?fn\s',L[j]): j-=1
            k=i
            depth=None
            # function end: naive, next line starting with same indent '}' as fn
            ind=len(L[j])-len(L[j].lstrip())
            k=j+1
            while k<len(L) and not (L[k].startswith(' '*ind+'}') and len(L[k])-len(L[k].lstrip())==ind): k+=1
            others=[m+1 for m in range(j,k) if m!=i-1 and bind.search(L[m])]
            if others:
                print(f"{f}:{i}: fn@{j+1}-{k+1} other frame bindings at {others}")
