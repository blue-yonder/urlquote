#!/bin/bash -xe
python setup.py sdist bdist_wheel
pip install -e .
pytest tests