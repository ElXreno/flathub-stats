#!/bin/bash
set -e

if [ -z "$1" ] || [ -z "$2" ]; then
    echo "Usage: $0 GIT_URL GIT_COMMIT"
    echo "Example: $0 https://github.com/ElXreno/flathub-stats.git 4cbd78e"
    exit 1
fi

REPO_URL="$1"
REPO_NAME="`basename $REPO_URL | cut -d . -f 1`"
COMMIT="$2"
NAME="$REPO_NAME-sources-$COMMIT"

rm -rf $REPO_NAME-$COMMIT
git clone $REPO_URL $NAME

pushd $NAME
git checkout $COMMIT
cargo vendor --locked
popd

rm -f $NAME.tar.gz
tar --exclude $NAME/.git -czvf $NAME.tar.gz $NAME
rm -rf $NAME
