#!/bin/bash
image_id=$(docker build --quiet .)
if [ $? -ne 0 ]; then
  exit $?
fi;
docker run --rm "$image_id"