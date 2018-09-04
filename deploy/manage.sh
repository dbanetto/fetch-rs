#!/bin/sh

set -e

cd `dirname $0`
OPTION=$1
DEFAULT_SEVER='http://localhost:3000'

function print_help {
    echo "Dev - local code with live reloading"
    echo "manage.sh build - build containers"
    echo "manage.sh run   - run dev containers with reloading"
    echo "manage.sh stop  - stop dev containers"
    echo ""
    echo "UAT - local code, no live reloading"
    echo "manage.sh uat up    - run local containers with no reloading"
    echo "manage.sh uat build - build containers"
    echo "manage.sh uat stop  - stop uat containers"
    echo ""
    echo "Prod - pre-built containers"
    echo "manage.sh prod up   - run pre-built containers"
    echo "manage.sh prod pull - pull pre-built containers"
    echo "manage.sh prod stop - stop prod containers"
    echo ""
    echo "Utility"
    echo "manage.sh backup <url>         - dump database to JSON"
    echo "manage.sh restore <url> <file> - re-creates resources from a backup"
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
                sh docker/init-secerts.sh uat
                docker-compose -p fetch_uat -f docker/docker-compose-uat.yml up
                ;;
            build)
                sh docker/init-secerts.sh uat
                docker-compose -p fetch_uat -f docker/docker-compose-uat.yml build
                ;;
            stop)
                sh docker/init-secerts.sh uat
                docker-compose -p fetch_uat -f docker/docker-compose-uat.yml stop
                ;;
            *)
                print_help
                ;;
        esac
        ;;
    prod)
        case $2 in
            up)
                sh docker/init-secerts.sh prod
                docker-compose -p fetch_prod -f docker/docker-compose-prod.yml up
                ;;
            pull)
                sh docker/init-secerts.sh prod
                docker-compose -p fetch_prod -f docker/docker-compose-prod.yml pull
                ;;
            stop)
                sh docker/init-secerts.sh prod
                docker-compose -p fetch_prod -f docker/docker-compose-prod.yml stop
                ;;
            *)
                print_help
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
