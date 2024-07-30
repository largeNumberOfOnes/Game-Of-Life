#!/bin/bash

for image in *.png; do
    convert $image -channel Alpha -evaluate Divide 1.2 $image
done

