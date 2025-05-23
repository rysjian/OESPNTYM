FROM alpine:latest as tor-builder

RUN apk add --no-cache \
	make \
	automake \
	autoconf \
	libtool \
	gcc \
	musl-dev \
	openssl-dev \
	zlib-dev \
	libevent-dev

COPY tor-src.tar.gz /tmp/

RUN tar -xzf /tmp/tor-src.tar.gz -C /tmp/ && \
	cd /tmp/tor-* && \
	./autogen.sh && \
	./configure \
	--prefix=/usr/local \
	--enable-static-libevent && \
	make -j$(nproc) && \
	make install DESTDIR=/tmp/tor-install

FROM golang:1.21-alpine as obfs4-builder

ARG TARGETARCH

RUN echo "Building for ARCH: $TARGETARCH"

		    # Копируем исходники obfs4proxy
		    COPY obfs4-src.tar.gz /tmp/
		    RUN tar -xzf /tmp/obfs4-src.tar.gz -C /tmp/ && \
		        cd /tmp/obfs4-* && \
			    CGO_ENABLED=0 GOARCH=$TARGETARCH go build -o /obfs4proxy ./obfs4proxy

			    # === Этап 3: Финальный образ ===
			    FROM alpine:latest

			    # Копируем бинарники
			    COPY --from=tor-builder /tmp/tor-install/usr/local/ /usr/local/
			    COPY --from=obfs4-builder /obfs4proxy /usr/local/bin/

			    # Запускаем Tor
			    CMD ["tor", "-f", "/etc/tor/torrc"]

