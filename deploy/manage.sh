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
        INPUT=${2:-/dev/stdin}
        SERVER=${3:-"$DEFAULT_SEVER"}
        ./backup/fetch-backup restore "$SERVER" "$INPUT"
        ;;
    *)
        print_help
esac
