#!/bin/bash

LIST=(one two three four five)

for item in ${LIST[@]}; do
     echo -n $item | wc -c; #count characters
     #echo -n ignores all the null characters
done