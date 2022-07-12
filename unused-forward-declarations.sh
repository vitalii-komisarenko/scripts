#!/bin/bash

# Print a list of unused forward declarations in C/C++ files.

# This script does not modify files.


# Get the list of files in a given directory(-ies)
find "$@" -type f |
grep -v '\.svn' |
grep -v '\.git' |

# Find all forward declarations
xargs grep -H '^\s*class .*;$' |

# For each forward declaration
while read -r line
do
    # split into file name and class name parts
    file_name=$(echo "$line" | cut -d : -f 1)
    class_name=$(echo "$line" | sed -e 's/.*class //' -e 's/;$//')

    # if class name is used once - it is an unused declaration
    number_of_occurences=$(grep -c "$class_name" "$file_name")

    if ((number_of_occurences == 1))
    then
        echo "Forward declaration $class_name is redundant in $file_name"
    fi
done
