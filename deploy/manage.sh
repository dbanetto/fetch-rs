#!/bin/sh

cd `dirname $0`
OPTION=$1
DEFAULT_SEVER='http://localhost:3000'

function print_help {
    echo "dev.sh build - build containers"
    echo "dev.sh run   - run the dev environment"
}

case $OPTION in
    build)
        docker-compose -p fetch -f docker/docker-compose-dev.yml build
        ;;
    up|start|run)
        (cd ../services/web && npm run watch) &
        (cd ../services/api && cargo watch -x build --ignore target/ --ignore migrations/) &
        (docker-compose -p fetch -f docker/docker-compose-dev.yml up)
        ;;
    stop)
        docker-compose -p fetch -f docker/docker-compose-dev.yml stop
        ;;
    backup)
        SERVER=${2:-"$DEFAULT_SEVER"}
        ./backup/fetch-backup backup "$SERVER"
        ;;
    restore)
        SERVER=${2:-"$DEFAULT_SEVER"}
        INPUT=${3:/dev/stdin}
        ./backup/fetch-backup restore "$SERVER" "$INPUT"
        ;;
    *)
        print_help
esac
