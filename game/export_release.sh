#!/bin/bash
mkdir export
cp target/release/ludumdare42 export/
rsync -avp assets/* export/assets
zip -r export.zip export/*
