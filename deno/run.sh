#!/bin/sh
rustc -o $$ /dev/stdin && ./$$ && rm $$