apt-get install -yqq entr
ls /opt | entr -r /opt/fetch-api --config /etc/fetch.toml
