#!/bin/bash

docker run -it --rm -v ${PWD}:/workspace -w /workspace rust:1.40.0-buster bash
