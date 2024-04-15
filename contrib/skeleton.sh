#!/bin/bash
# Name: Example
# Description: Example skeleton
# Version: 0.0.1
# Author: John Smith
# Role: 
#       [role.example]
#          version = 4
#          prompt = "this is an example"
# 

if [ -z "$@" ] 
then
    echo "Please provide an argument"
    exit
fi

gptx -r example "$@"
