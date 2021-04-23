#!/bin/sh

set -ex

# When using this script, make sure you start the container with --init
# so that it dies properly
npm install
npm run relay:watch &
npm run start
