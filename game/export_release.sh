#!/bin/bash
mkdir export
cp target/release/game export/
rsync -avp assets/* export/assets
zip -r export.zip export/*
