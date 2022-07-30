#!/bin/bash

# Print a list of unused standard headers.

# This script does not modify files.

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
    [chrono]="chrono" # chrono is a namespace
    [sstream]="basic_stringbuf basic_istringstream basic_ostringstream basic_stringstream stringbuf wstringbuf istringstream wistringstream ostringstream wostringstream stringstream wstringstream"
    [ios]="ios_base basic_ios ios wios fpos io_errc is_error_code_enum streamoff streamsize iostream_category make_error_code make_error_condition boolalpha noboolalpha showbase noshowbase showpoint noshowpoint showpos noshowpos skipws noskipws uppercase nouppercase unitbuf nounitbuf internal left right dec hex oct fixed scientific hexfloat defaultfloat"
    [ostream]="basic_ostream ostream wostream endl ends flush emit_on_flush noemit_on_flush flush_emit"
    [istream]="basic_istream istream wistream basic_iostream iostream wiostream ws"
    [streambuf]="basic_streambuf streambuf wstreambuf"
    [optional]="optional bad_optional_access nullopt_t make_optional"
    [thread]="this_thread thread jthread" # this_thread is a namespace
)

symbolsPerHeader[iostream]+="cin wcin cout wcout cerr wcerr clog wclog ${symbolsPerHeader[ios]} ${symbolsPerHeader[streambuf]} ${symbolsPerHeader[istream]} ${symbolsPerHeader[ostream]}"

function get_cpp_files() {
    for path in "$@"; do
        if [[ -f "$path" ]]; then
            echo "$path"
        elif [[ -d "$path" ]]; then
            find "$path" -name "*.c" -or -name "*.cpp" -or -name "*.tcc" -or -name "*.h" -or -name "*.hpp"
        else
            echo "Neither ordinary file nor directory: $path"
        fi
    done
}

function check_if_header_is_unused_in_file() {
    local header="$1"
    local file="$2"

    # If the header is not included, just exit
    grep -q "#include.*$header" "$file" || return

    grep_pattern="\\b${symbolsPerHeader[$header]// /\\b\\|\\b}\\b"

    # Remove all #include lines
    grep -v '#include' "$file" |
        # If no symbol from this header is found, report it
        grep -q "$grep_pattern" || echo "Useless header: $header in file $file"
}

if [[ -z "$@" ]]; then
    cpp_files=$(get_cpp_files .)
else
    cpp_files=$(get_cpp_files "$@")
fi

for file in $cpp_files; do
    for header in "${!symbolsPerHeader[@]}"; do
        check_if_header_is_unused_in_file "$header" "$file"
    done
done
