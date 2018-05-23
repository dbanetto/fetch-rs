#!/bin/sh

cd `dirname $0`

( cd ../services/web && npm run watch ) &
( cd ../services/api && cargo watch -x build --ignore target/ --ignore migrations/) &

docker-compose -p fetch -f docker-compose-dev.yml $@
