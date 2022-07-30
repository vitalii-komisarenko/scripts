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
    [sstream]="basic_stringbuf basic_istringstream basic_ostringstream basic_stringstream stringbuf wstringbuf istringstream wistringstream ostringstream wostringstream stringstream wstringstream"
    [ios]="ios_base basic_ios ios wios fpos io_errc is_error_code_enum streamoff streamsize iostream_category make_error_code make_error_condition boolalpha noboolalpha showbase noshowbase showpoint noshowpoint showpos noshowpos skipws noskipws uppercase nouppercase unitbuf nounitbuf internal left right dec hex oct fixed scientific hexfloat defaultfloat"
    [ostream]="basic_ostream ostream wostream endl ends flush emit_on_flush noemit_on_flush flush_emit"
    [istream]="basic_istream istream wistream basic_iostream iostream wiostream ws"
    [streambuf]="basic_streambuf streambuf wstreambuf"
)

symbolsPerHeader[iostream]+="cin wcin cout wcout cerr wcerr clog wclog ${symbolsPerHeader[ios]} ${symbolsPerHeader[streambuf]} ${symbolsPerHeader[istream]} ${symbolsPerHeader[ostream]}"

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
