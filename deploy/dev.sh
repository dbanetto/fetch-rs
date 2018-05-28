#!/bin/sh

cd `dirname $0`
OPTION=$1

function print_help {
    echo "dev.sh build - build containers"
    echo "dev.sh run   - run the dev environment"
}

case $OPTION in
    build)
        docker-compose -p fetch -f docker-compose-dev.yml build
        ;;
    up|start|run)
        (cd ../services/web && npm run watch) &
        (cd ../services/api && cargo watch -x build --ignore target/ --ignore migrations/) &
        (docker-compose -p fetch -f docker-compose-dev.yml up)
        ;;
    stop)
        docker-compose -p fetch -f docker-compose-dev.yml stop
        ;;
    *)
        print_help
esac
