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
        docker-compose -p fetch -f docker/docker-compose-dev.yml $@
        ;;
    up)
        docker-compose -p fetch -f docker/docker-compose-dev.yml $@
        ;;
    stop)
        docker-compose -p fetch -f docker/docker-compose-dev.yml $@
        ;;
    start|run)
        docker-compose -p fetch -f docker/docker-compose-dev.yml up
        ;;
    uat)
        case $2 in
            up)
                docker-compose -p fetch_uat -f docker/docker-compose-uat.yml up
                ;;
            build)
                docker-compose -p fetch_uat -f docker/docker-compose-uat.yml build
                ;;
            stop)
                docker-compose -p fetch_uat -f docker/docker-compose-uat.yml stop
                ;;
        esac
        ;;
    prod)
        case $2 in
            up)
                docker-compose -p fetch_prod -f docker/docker-compose-prod.yml up
                ;;
            pull)
                docker-compose -p fetch_prod -f docker/docker-compose-prod.yml pull
                ;;
            stop)
                docker-compose -p fetch_prod -f docker/docker-compose-prod.yml stop
                ;;
        esac
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
    help)
        print_help
        ;;
    *)
        print_help
esac
