#!/bin/bash

docker run -it --rm -v ${PWD}:/workspace -w /workspace rust:1.39.0 bash
