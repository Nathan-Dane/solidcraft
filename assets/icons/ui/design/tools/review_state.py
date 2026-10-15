"""Shared by icon_status.py and author/gen.py: read and write design/review.json (standard library only).

  {"version": 1, "final": {"<icon id>": "<sha256 of its master svg(s) when accepted>"}, "notes": {"<icon id>": "text"}}

Older files with "final" as a plain list are read too and get their hashes filled in on the next save.
"""
import os, re, json, glob, hashlib, tomllib

D = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))     # .../assets/icons/ui/design
REVIEW = D + '/review.json'

def aliases():
    return tomllib.load(open(D + '/aliases.toml', 'rb')).get('alias', {})
def master_of(icon_id, al=None):
    v = (aliases() if al is None else al).get(icon_id)
    return (v['master'] if isinstance(v, dict) else v) if v else icon_id
def master_hash(master):
    h = hashlib.sha256()
    for f in sorted(glob.glob(D + '/svg/*/' + master + '.svg')):
        h.update(os.path.relpath(f, D).encode()); h.update(open(f, 'rb').read())
    return h.hexdigest()
def load():
    try: r = json.load(open(REVIEW))
    except (OSError, ValueError): r = {}
    fin = r.get('final', {})
    if isinstance(fin, list): fin = {i: '' for i in fin}
    return {'final': dict(fin), 'notes': {k: v for k, v in r.get('notes', {}).items() if isinstance(v, str) and v.strip()}}
def save(final, notes):
    """final: {id: hash or ''}; a missing hash is filled in from the master as it is now."""
    ok = lambda i: re.fullmatch(r'[a-z][a-z0-9_]*', i)
    al = aliases()
    fin = {i: (h or master_hash(master_of(i, al))) for i, h in final.items() if ok(i)}
    nts = {k: v for k, v in notes.items() if ok(k) and isinstance(v, str) and v.strip()}
    with open(REVIEW, 'w') as f:
        json.dump({'version': 1, 'final': dict(sorted(fin.items())), 'notes': dict(sorted(nts.items()))}, f, indent=1); f.write('\n')
def resolve(masters_or_ids):
    """The icons were redrawn: drop their notes, and the notes on aliases of the same masters. Returns the ids cleared."""
    r = load(); al = aliases(); done = set(masters_or_ids)
    cleared = sorted(i for i in r['notes'] if i in done or master_of(i, al) in done)
    for i in cleared: del r['notes'][i]
    if cleared: save(r['final'], r['notes'])
    return cleared
