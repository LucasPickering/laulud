#!/bin/sh

set -e

GIT_TAG=deployed
REPO_ROOT=$(git rev-parse --show-toplevel)
BRANCH=$(git rev-parse --abbrev-ref HEAD)
DEPLOYED_VERSION_SHA=$(git rev-parse $GIT_TAG)

# If on the master branch, deploy the new code, otherwise deploy the pre-existing
# latest. If deploying from any branch other than master, let's just assume that
# we're making infra changes, and don't want to change the code.
if [ $BRANCH = "master" ]; then
    # Deploy latest code
    VERSION_SHA=$(git rev-parse origin/master)
else
    # Re-deploy current version
    echo "!!!! Not on master, re-deploying current app version !!!!"
    VERSION_SHA=$DEPLOYED_VERSION_SHA
fi

echo "Deploying version $VERSION_SHA"
terraform -chdir=$REPO_ROOT/deploy/terraform apply -var version_sha=$VERSION_SHA

# If we actually deployed new code, update the git tag
if [ $VERSION_SHA != DEPLOYED_VERSION_SHA ]; then
    git tag -f $GIT_TAG $VERSION_SHA
    git push -f origin $GIT_TAG
    echo "Updated git tag '$GIT_TAG' to '$VERSION_SHA'"
fi
