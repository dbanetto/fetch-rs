apt-get install -yqq entr
ls /opt | entr -r /opt/fetch-web --config /etc/fetch.toml
