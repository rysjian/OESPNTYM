FROM alpine:3.21.3
#RUN wget -O vscode.tar.gz "https://code.visualstudio.com/sha/download?build=stable&os=cli-alpine-arm64" && tar -xzf vscode.tar.gz && rm vscode.tar.gz && mv code /usr/local/bin/
COPY code /usr/local/bin
RUN apk add --no-cache libstdc++ && mkdir /root/.vscode /workspace
ENTRYPOINT ["code"]
