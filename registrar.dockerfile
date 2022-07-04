FROM node:latest AS pjs

FROM pjs
RUN apt-get update && apt-get install curl netcat -y && \
    curl -sSo /wait-for-it.sh https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh && \
    chmod +x /wait-for-it.sh
COPY ./register/ /var/tmp/register
RUN cd /var/tmp/register && yarn && chmod +x index.js
COPY ./scripts/docker-register-para.sh /usr/bin
# unset the previous stage's entrypoint
ENTRYPOINT []
CMD [ "/usr/bin/docker-register-para.sh" ]

