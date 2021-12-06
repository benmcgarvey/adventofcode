#!/bin/bash

set -euxo pipefail

mkdir "src/lib/$1"

cp random/template/* "src/lib/$1/"
