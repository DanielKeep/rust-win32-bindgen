#!/usr/bin/env python3

import sys

syms = {}
cons = {}

last_lib = None

for line in sys.stdin.readlines():
    line = line.strip()
    if line == "": continue
    sym, lib = line.split(':', 1)
    sym = sym.strip()
    lib = lib.strip()

    if last_lib != lib:
        print("%s..." % lib, file=sys.stderr)
        last_lib = lib

    if sym in syms and syms[sym] != lib:
        sym_cons = cons.get(sym, set([syms[sym]]))
        sym_cons.add(lib)
        cons[sym] = sym_cons
        del syms[sym]
    else:
        syms[sym] = lib

print('# Symbols')
for (sym, lib) in sorted(syms.items()):
    print('%s: %s' % (sym, lib))

print('\n# Conflicts')
for (sym, libs) in sorted(cons.items()):
    print('%s: %s' % (sym, ' '.join(sorted(libs))))

print('Got %d symbols with %d conflicts.' % (len(syms), len(cons)), file=sys.stderr)
