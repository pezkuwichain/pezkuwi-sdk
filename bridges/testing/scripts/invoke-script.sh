#!/usr/bin/env bash

INVOKE_LOG=`mktemp -p $TEST_FOLDER invoke.XXXXX`

pushd $PEZKUWI_SDK_PATH/bridges/testing/environments/rococo-westend
./bridges_rococo_westend.sh $1 >$INVOKE_LOG 2>&1
popd
