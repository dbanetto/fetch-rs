FROM node:10 AS frontend

RUN mkdir -p /code/src

WORKDIR /code

COPY package.json /code/package.json
COPY package-lock.json /code/package-lock.json

RUN npm install

COPY webpack.config.js /code/webpack.config.js
COPY tsconfig.json /code/tsconfig.json

COPY src /code/src

RUN npm run build

FROM nginx:alpine

COPY --from=frontend /code/public /www
COPY nginx.conf /etc/nginx/conf.d/default.conf

RUN mkdir -p /etc/nginx/service.d && \
        chown nginx:nginx -R /www

HEALTHCHECK --interval=10m --timeout=3s --retries=3 --start-period=5s \
    CMD wget http://localhost/ -q -O /dev/null
