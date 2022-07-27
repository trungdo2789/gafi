# Using Docker

There are images that release of Gafi binary, you can pull and run it easy.

```bash
docker run --rm -it grindytech/gafi-node:latest
```

You also can pass any supported arguments.
```bash
docker run --rm -it grindytech/gafi-node:latest --ws-external
```

In case running a Gafi mainnet. You should bind a volume to Gafi node container to persist the database.

`Example:`
```bash
#default data are storaged in /home/gafi/.local
docker run -d -p 30333:30333 -p 9944:9944 -v /my/local/folder:/home/gafi/.local grindytech/gafi-node:latest
```
