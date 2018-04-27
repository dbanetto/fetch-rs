FROM node:8 AS frontend

RUN mkdir -p /code/src/frontend

WORKDIR /code

COPY package.json /code/package.json
COPY package-lock.json /code/package-lock.json

RUN npm install

COPY webpack.config.js /code/webpack.config.js
COPY tsconfig.json /code/tsconfig.json

COPY src/frontend /code/src/frontend

RUN npm run build

FROM nginx:alpine

COPY --from=frontend /code/public /www
COPY src/deploy/web/web.conf /etc/nginx/conf.d/default.conf

RUN mkdir -p /etc/nginx/service.d && \
        chown nginx:nginx -R /www
