FROM archlinux

RUN pacman --noconfirm -Syu
RUN pacman --noconfirm -S base-devel
RUN pacman --noconfirm -S cmake
RUN pacman --noconfirm -S catch2
COPY . PaperTrader
WORKDIR PaperTrader

ENTRYPOINT [ "/PaperTrader/docker-entrypoint.sh" ]
