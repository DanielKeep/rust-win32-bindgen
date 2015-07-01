#!/usr/bin/env bash
LIBDIR="$1"

[[ -d "$LIBDIR" ]] || {
    echo "Usage: gen-sym-list-from-stubs STUBDIR"
    exit 1
}

for LIB in $LIBDIR/*.rs;
do
    LIBNAME="$(basename "$LIB")"
    LIBNAME="${LIBNAME/.rs/}"
    cat $LIB \
    | grep -Po '(?<=// pub fn )[A-Za-z0-9_]+' \
    | sed -re 's/^([A-Za-z0-9_]+)$/\1: '"$LIBNAME"'/'
done
