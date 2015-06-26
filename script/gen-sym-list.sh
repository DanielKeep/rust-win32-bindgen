#!/usr/bin/env bash
LIBDIR="$1"

[[ -d "$LIBDIR" ]] || {
    echo "Usage: gen-sym-list LIBDIR"
    exit 1
}

for LIB in $LIBDIR/lib*.a;
do
    LIBNAME="$(basename "$LIB")"
    LIBNAME="${LIBNAME/.a/}"
    objdump -t $LIB \
    | grep -Po '(?<= )_[A-Za-z0-9_]*(@[0-9]+)?$' \
    | egrep -v '^__imp_|^__head_|^__lib32_' \
    | sed -re 's/^_([^@]*)(@[0-9]+)?$/\1: '"$LIBNAME"'/'
done
