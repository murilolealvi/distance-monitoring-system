#!/bin/bash

showuptime(){
    local up=$(uptime -p | cut -c4-)
    local since=$(uptime -s)
    cat << EOF
--------
Up for ${up}
Since ${since}
--------
EOF
}
showuptime