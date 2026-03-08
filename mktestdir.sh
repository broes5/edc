#!/usr/bin/bash

rm -r testdir 2> /dev/null
mkdir testdir
cd testdir
touch file.TXT
touch file.txt
touch file\ \({1..15}\).txt

mkdir subdir
cd subdir
touch photo.JPG
touch image.PNG
touch document.ODF

#mkdir subdir{0..5000}
#touch subdir{0..5000}/photo.JPG
#touch subdir{0..5000}/image.PNG
#touch subdir{0..5000}/document.ODF
