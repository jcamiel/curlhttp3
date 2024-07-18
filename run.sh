#!/usr/bin/env bash

rm -rfd build
mkdir build
cd build
git clone https://github.com/Orange-OpenSource/hurl
cd hurl


git bisect start
git bisect good e3a8fab4161449e248c885e4820ccbc281f04cbd
git bisect bad 80b499042c887ebf51dd91398e6436bc86d2c77b
git bisect run ../../test.sh
