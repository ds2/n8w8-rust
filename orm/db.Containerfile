FROM docker.io/postgres:14
ENV POSTGRES_USER=n8w8adm
ENV POSTGRES_PASSWORD=n8w8admpw
ENV POSTGRES_DB=n8w8db
ENV POSTGRES_INITDB_ARGS="--data-checksums"
ENV PGDATA=/db-data
RUN localedef -i de_DE -c -f UTF-8 -A /usr/share/locale/locale.alias de_DE.UTF-8
ENV LANG de_DE.utf8
RUN mkdir -p ${PGDATA}; chmod 777 ${PGDATA}
