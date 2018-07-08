#!/bin/sh

apt-get install -yqq entr
echo "/bin/fetch-api\n/opt/fetch-api" | entr -r bash -xec 'if [ -f /opt/fetch-api ] ; then /opt/fetch-api --config /etc/fetch.toml ; else /bin/fetch-api --config /etc/fetch.toml ; fi'
