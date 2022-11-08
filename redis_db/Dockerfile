FROM redislabs/rejson

COPY ./rejson.so /opt
COPY ./module.json /opt

CMD ["/bin/bash", "-c", "redis-server --daemonize yes --loadmodule /opt/rejson.so; echo JSON.SET students . '{}' | redis-cli; sleep infinity"]