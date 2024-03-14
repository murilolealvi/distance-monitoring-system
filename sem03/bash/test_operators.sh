#!/bin/bash

[ hello = hello ]
echo $?
#0 is true and 1 is false

[ hello = hell0 ]
echo $?

[ 1 -eq 1 ] #-eq for integers
echo $?