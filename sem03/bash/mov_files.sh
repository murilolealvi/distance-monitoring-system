#!/bin/bash
# ! represents the root directory

LOC_FROM=/my/location/from
LOC_TO=/my/location/to

cp $LOC_FROM $LOC_TO
cp "$LOC_FROM/here" "$LOC_TO/there"