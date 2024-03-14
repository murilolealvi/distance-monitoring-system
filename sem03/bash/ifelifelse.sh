#!/bin/bash

if [ ${1,,} = murilo ]; then
    echo "E AI MURILO"
# ,, parameters expansion to ignore upper and lowercases
elif [ ${1,,} = help ]; then
    echo "COLOCA TEU NOME"
else
    echo "QUEM É VOCÊ?"
fi