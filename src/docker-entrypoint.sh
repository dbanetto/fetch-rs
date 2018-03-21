#!/bin/sh

for i in 1 2 3 4 5; do
    /usr/local/bin/diesel migration run && break
    echo "Attempt $i"
    sleep 5
done

cd /opt
./fetch --config /etc/fetch.toml
