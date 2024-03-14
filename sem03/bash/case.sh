#!/bin/bash

case ${1,,} in 
    murilo | adm) 
        echo "E AI MURILO"
        ;;
    help)
        echo "COLOCA TEU NOME"
        ;;
    *)
        echo "QUEM É VOCÊ?"
esac
    