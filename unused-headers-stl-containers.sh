#!/bin/bash

# Print a list of unused STL containers hedaers.

# This script does not modify files.

# Internally it uses the fact that an STL container
# and its header file often have the same name.

# Problems:
# set and multiset are defined in <set>
# map and multimap are defined in <map>

containers="array vector deque forward_list list \
            stack queue priority_queue set map \
            unordered_set unordered_map"

for container in $containers; do
    echo
    echo ==== $container ====
    echo
    grep -rnIH include..$container \
        | grep -v ^build \
        | cut -d : -f 1 \
        | xargs grep -nH \\b$container\\b \
        | cut -d : -f 1 \
        | uniq -c \
        | grep ' 1 ' \
        | sed -e 's/^.*1 //g' \
        | xargs grep --color -nH $container
done
