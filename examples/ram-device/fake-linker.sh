#!/bin/sh
while [ "$#" -gt 0 ]; do
    if [ "$1" = "-o" ]; then touch "$2"; break; fi
    shift
done
exit 0
