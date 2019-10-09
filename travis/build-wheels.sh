#!/bin/bash -xe

# Compile wheels
#for PYBIN in /opt/python/*/bin
for PYBIN in /opt/python/cp37-cp37m/bin
do
    "${PYBIN}/pip" install --upgrade pip
    "${PYBIN}/pip" install -r /io/requirements.txt
    "${PYBIN}/pip" wheel /io/ -w wheelhouse/
done

# Bundle external shared libraries into the wheels
for whl in wheelhouse/urlquote-*.whl
do
    auditwheel repair "${whl}" -w /io/dist/
done

# Install packages and test
for PYBIN in /opt/python/*/bin
do
    "${PYBIN}/pip" install -r /io/requirements.txt
    "${PYBIN}/pip" install milksnake
    "${PYBIN}/pip" install urlquote --no-index -f /io/dist/
    "${PYBIN}/pip" install -r /io/requirements-test.txt
    (cd "$HOME"; "${PYBIN}/pytest" /io/tests/test_urlquote.py)
done