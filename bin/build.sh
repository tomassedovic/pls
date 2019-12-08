#!/bin/bash

set -eu
set -o pipefail

source venv/bin/activate
fbs freeze
