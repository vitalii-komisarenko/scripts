#!/bin/bash

# Remove unused #include <iostream>
# Please note that this script MODIFIES FILES IN PLACE.

# The main idea is that if <iosteam> is included and
# there is no other word containing "stream"
# (e.g. istream), this header can be removed.

# TODO: take into account cin / cout / cerr.

grep -rH include..iostream * \
    | cut -d : -f 1 \
    | while read file; do \
        num=$(grep stream $file | wc -l)
        [ $num == "1" ] && sed -i '/include..iostream/d' $file
    done
