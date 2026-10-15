#!/usr/bin/env python3
"""Rules every icon master in svg/ must satisfy (stdlib only). usage: lint_icons.py <design dir>"""
import sys, glob, os, re, tomllib, xml.etree.ElementTree as ET
d = sys.argv[1] if len(sys.argv) > 1 else os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
pal = tomllib.load(open(d + '/palette.toml', 'rb'))
light = {r['light'] for r in pal['roles'].values()}
OK_TAGS = {'svg', 'title', 'path', 'circle', 'defs', 'mask', 'clipPath', 'g'}
FAMILIES = {'model', 'small', 'flat', 'workspace'}
bad = []
def err(f, m): bad.append('%s: %s' % (os.path.relpath(f, d), m))
files = sorted(glob.glob(d + '/svg/*/*.svg'))
for f in files:
    fam, name = os.path.basename(os.path.dirname(f)), os.path.basename(f)[:-4]
    if fam not in FAMILIES: err(f, 'unknown family'); continue
    if not re.fullmatch(r'[a-z][a-z0-9_]*', name): err(f, 'file name must be an app icon id (lower_snake_case)')
    t = ET.parse(f).getroot(); ns = '{http://www.w3.org/2000/svg}'
    if t.get('viewBox') != '0 0 64 64': err(f, 'viewBox must be 0 0 64 64')
    for a in ('width', 'height'):
        if t.get(a): err(f, a + ' attribute on svg')
    ids = []
    def walk(e, mask):
        for c in e:
            tag = c.tag.replace(ns, '')
            if tag not in OK_TAGS: err(f, 'element <%s> not allowed' % tag)
            for a in ('transform', 'style', 'class'):
                if c.get(a): err(f, '%s attribute on <%s>' % (a, tag))
            if c.get('id'): ids.append(c.get('id'))
            m = mask or tag in ('mask', 'clipPath')
            for k in ('fill', 'stroke'):
                v = c.get(k)
                if v in (None, 'none') or v.startswith('url('): continue
                if m and v in ('white', 'black'): continue
                if v == 'currentColor':
                    continue
                if v.lower() not in light: err(f, '%s="%s" is not a palette colour' % (k, v))
                if fam == 'flat' and v.lower() != '#ffffff': err(f, 'flat icons are single colour: use currentColor, not %s' % v)
            if c.get('stroke') not in (None, 'none'):
                if not c.get('stroke-width'): err(f, 'stroke without stroke-width')
                if c.get('stroke-linejoin') != 'round' or c.get('stroke-linecap') != 'round': err(f, 'strokes need round joins and caps')
            if tag in ('path', 'circle') and c.get('fill') is None and c.get('stroke') is None and not m: err(f, '<%s> without paint' % tag)
            walk(c, m)
    walk(t, False)
    if len(ids) != len(set(ids)): err(f, 'duplicate ids')
# every small/ master has a model/ counterpart; aliases point at real masters
names = {os.path.basename(f)[:-4] for f in files if '/small/' not in f}
for f in files:
    if '/small/' in f and os.path.basename(f)[:-4] not in names: err(f, 'small variant without a master')
al = tomllib.load(open(d + '/aliases.toml', 'rb')).get('alias', {})
for a, t in al.items():
    if t not in names: err(d + '/aliases.toml', '%s -> %s: no such master' % (a, t))
    if a in names: err(d + '/aliases.toml', '%s is both an alias and a master' % a)
print('\n'.join(bad) if bad else 'icons: %d masters, palette, naming and element rules ok' % len(files))
sys.exit(1 if bad else 0)
