### Traefik 反向代理容器
文档参考：https://blog.cthudson.com/2023-11-02-running-traefik-with-podman/
``` shell
podman run -d \
  --name=traefik \
  --net podman \
  --security-opt label=type:container_runtime_t \
  -v /run/user/1000/podman/podman.sock:/var/run/docker.sock \
  -v /home/suxin/traefik/acme.json:/acme.json \
  -p 1080:80 \
  -p 1443:443 \
  -p 9000:8080 \
  docker.io/library/traefik:latest \
  --api.dashboard=true \
  --api.insecure=true \
  --certificatesresolvers.lets-encrypt.acme.email="qazwsxrrrrr@outlook.com" \
  --certificatesresolvers.lets-encrypt.acme.storage=/acme.json \
  --certificatesresolvers.lets-encrypt.acme.tlschallenge=true \
  --entrypoints.http.address=":80" \
  --entrypoints.http.http.redirections.entryPoint.to=https \
  --entrypoints.http.http.redirections.entryPoint.scheme=https \
  --entrypoints.https.address=":443" \
  --providers.docker=true
```

### Podman 容器镜像源配置
编辑文件 `/etc/containers/registries.conf`
```
unqualified-search-registries = ["docker.io"]
[[registry]]
prefix = "docker.io"
location = "docker.1ms.run"
```