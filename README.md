# Usage

```
> # build executable
> cargo build --release
> mv target/release/ss-client /usr/local/bin/ss-client
> # add ss-client to session initialization as
> # /usr/local/bin/ss-client 12080
> curl -s http://127.0.0.1:12080/  # get vpn status
```