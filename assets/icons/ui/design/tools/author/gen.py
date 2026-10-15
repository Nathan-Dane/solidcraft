import sys, os, json, importlib
HERE=os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0,HERE)
names=sys.argv[1:]
mods=['i1','i2','i3','i4','i5','i6','i7','i8']
allic={}
for m in mods:
    try: mod=importlib.import_module(m)
    except ModuleNotFoundError: continue
    allic.update(mod.ICONS)
import lib
REVIEW=os.path.join(os.path.dirname(os.path.dirname(HERE)),'review.json')
final=set(json.load(open(REVIEW)).get('final',{})) if os.path.exists(REVIEW) else set()
skipped=[]; written=set()
for k,fn in allic.items():
    ic=fn()
    if ic.name in final: skipped.append(ic.name); continue
    if not names or ic.name in names or k in names: ic.write(ic.fam); written.add(ic.name); print('wrote',ic.name)

if names and written:
    sys.path.insert(0,os.path.dirname(HERE))
    import review_state
    cleared=review_state.resolve(written)
    if cleared: print('cleared notes on: '+' '.join(cleared))
if skipped: print('kept %d final icons untouched (review.json): %s'%(len(skipped),' '.join(skipped)))
