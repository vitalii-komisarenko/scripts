#!/bin/bash

# Print a list of unused standard headers.

# This script does not modify files.

# Internal logic:
# 1. Check that a header exists
# 2. Count the number of lines that contains one of the relevant words (symbols or the header name itself) in the file
# 3. If the number is 1 (only the header name, no symbols defined there), report this file

# Problems:
# * Current implementation relies on the fact, that the name of one of the symbol is the same as the header name

declare -A symbolsPerHeader=(
    [array]="array to_array"
    [vector]="vector"
    [deque]="deque"
    [forward_list]="forward_list"
    [list]="list"
    [stack]="stack"
    [queue]="queue priority_queue"
    [set]="set multiset"
    [map]="map multimap"
    [unordered_set]="unordered_set unordered_multiset"
    [unordered_map]="unordered_map unordered_multimap"
    [string]="char_traits basic_string string u8string u16string u32string wstring getline stoi stol stoll stoul stoull stof stod stold to_string to_wstring"
    [chrono]="chrono" # not a symbol, just a word to search by grep
)

for header in "${!symbolsPerHeader[@]}"; do
    echo
    echo "==== $header ===="
    echo

    grep_pattern="\\b${symbolsPerHeader[$header]// /\\b\\|\\b}\\b"

    grep -rnIH "include..$header" \
        | grep -v ^build \
        | cut -d : -f 1 \
        | xargs grep -nH "$grep_pattern" \
        | cut -d : -f 1 \
        | uniq -c \
        | grep ' 1 ' \
        | sed -e 's/^.*1 //g' \
        | xargs grep --color -nH "\\b$header\\b"
done
