#!/bin/sh

set -e

cd `dirname $0`

ENV=$1

ENV_CORE=".env_${ENV}_core"
ENV_DB=".env_${ENV}_db"

if [ -f "${ENV_CORE}" ] && [ -f "${ENV_DB}" ]; then
    # secerts already exist, manually delete
    # them if you wish to reseed
    exit 0
fi

# Clean up just in-case one of them exist
stat "${ENV_CORE}" 2>&1 > /dev/null && rm "${ENV_CORE}"
stat "${ENV_DB}" 2>&1 > /dev/null && rm "${ENV_DB}"

PASSWORD=$(openssl rand -base64 45)
USER=$(openssl rand -hex 15)
DATABASE=$(openssl rand -hex 15)

echo "POSTGRES_USER=${USER}" > "${ENV_DB}"
echo "POSTGRES_PASSWORD=${PASSWORD}" >> "${ENV_DB}"
echo "POSTGRES_DB=${DATABASE}" >> "${ENV_DB}"
echo 'Set database secrets'

ENCODED_USER=`python3 -c "import urllib.parse; print(urllib.parse.quote(input(), safe=''))" <<< "${USER}"`
ENCODED_PASSWORD=`python3 -c "import urllib.parse; print(urllib.parse.quote(input(), safe=''))" <<< "${PASSWORD}"`
ENCODED_DATABASE=`python3 -c "import urllib.parse; print(urllib.parse.quote(input(), safe=''))" <<< "${DATABASE}"`

echo "DATABASE_URL=postgres://${ENCODED_USER}:${ENCODED_PASSWORD}@db/${ENCODED_DATABASE}" > "${ENV_CORE}"
echo 'Set core secrets'
