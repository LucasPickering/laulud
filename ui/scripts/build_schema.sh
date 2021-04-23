#!/bin/sh

set -e

# Concat the API's multi-file schema definition into one big file, since Relay
# doesn't support multi-file schemas
INPUT_PATH=../api/schema/*.graphql
OUTPUT_PATH=./schema.graphql
echo "# Auto-generated from $INPUT_PATH\n" | cat /dev/stdin $INPUT_PATH > $OUTPUT_PATH
