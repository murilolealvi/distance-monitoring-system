#!/bin/bash

LIST=(one two three four five)

echo $LIST #print first element
echo ${LIST[@]} #print all
echo ${LIST[2]} #print third element