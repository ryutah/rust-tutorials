#!/bin/sh

cd $(dirname $0)/..

set -eux

python -m http.server
