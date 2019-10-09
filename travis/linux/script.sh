#!/bin/bash -xe
docker run --rm -it -v ${PWD}:/io cargodock bash /io/travis/build-wheels.sh
ls dist