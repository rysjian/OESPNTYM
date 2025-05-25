# ========== STAGE 1: Build Tor ==========
FROM alpine:3.21.3 as tor-builder

# Install build dependencies
RUN apk add --no-cache \
        make automake autoconf libtool \
        gcc musl-dev openssl-dev zlib-dev \
        libevent-dev libevent-static

# Copy and extract Tor sources
COPY tor-src.tar.gz /tmp/
RUN tar -xzf /tmp/tor-src.tar.gz -C /tmp/ && \
    cd /tmp/tor-* && \
    if [ -f "./autogen.sh" ]; then ./autogen.sh; else autoreconf -fiv; fi && \
    ./configure \
        --prefix=/usr/local \
        --enable-static-libevent \
        --with-libevent-dir=/usr/lib \
        LDFLAGS="-L/usr/lib" && \
    make -j$(nproc) && \
    make install DESTDIR=/tor-install

# ========== STAGE 2: Build Obfs4proxy ========== 
FROM golang:1.23.9-alpine3.21 as obfs4-builder

ARG TARGETARCH
ENV CGO_ENABLED=0 GOARCH=$TARGETARCH

RUN apk add --no-cache git && \
    git clone https://gitlab.com/yawning/obfs4.git && \
    cd obfs4 && \
    go build -o /obfs4proxy ./obfs4proxy

# ========== STAGE 3: Final Image ==========
FROM alpine:3.21.3

RUN mkdir -p /etc/tor /var/lib/tor

# Copy Tor from stage 1
COPY --from=tor-builder /tor-install/usr/local/ /usr/local/

# Copy Obfs4proxy from stage 2
COPY --from=obfs4-builder /obfs4proxy /usr/local/bin/

# Install runtime dependencies
RUN apk add --no-cache \
        libevent openssl zlib

RUN chown -R nobody:nobody /var/lib/tor && \
    chmod 700 /var/lib/tor

# Configure Tor
RUN echo -e "SocksPort 9050\nControlPort 9051\nDataDirectory /var/lib/tor\nClientTransportPlugin obfs4 exec /usr/local/bin/obfs4proxy" > /etc/tor/torrc

CMD ["tor", "-f", "/etc/tor/torrc"]
