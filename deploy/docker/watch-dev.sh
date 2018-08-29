#!/bin/bash -xec

BULTIN=$1
DEV_BUILD=$2
CONFIG=$3

if which apk > /dev/null ; then
    apk add --no-cache entr
else
    apt-get install -yqq entr
fi

printf "${BULTIN}\n${DEV_BUILD}\n" | entr -r sh -xec "if type ${DEV_BUILD} ; then ${DEV_BUILD} --config ${CONFIG} ; else ${BULTIN} --config ${CONFIG} ; fi"

