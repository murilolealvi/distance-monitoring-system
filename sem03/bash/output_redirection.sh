#!/bin/bash

echo Hello World! > hello.txt

echo Erased... > hello.txt

echo Hello World, again...! >> hello.txt

cat hello.txt

wc -w hello.txt
wc -w < hello.txt
#not giving it as positional argument, but redirecting inputs

rm hello.txt

wc -w <<< "1 2 3 4 5 6 7 8 9"

#cat << EOF
