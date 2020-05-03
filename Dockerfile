FROM opensuse/tumbleweed

RUN zypper -n update
RUN zypper -n install cmake
RUN zypper -n install ninja
RUN zypper -n install gcc
RUN zypper -n install gcc-c++

COPY . PaperTrader
WORKDIR PaperTrader

ENTRYPOINT [ "/PaperTrader/docker-entrypoint.sh" ]
