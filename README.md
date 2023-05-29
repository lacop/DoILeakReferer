# DoILeakReferer.com

A simple website to check if your browser is sending the `Referer` header.

Hosted at https://DoILeakReferer.com and https://DoILeakReferrer.com.

## Local development

Just `cargo run`, access at http://localhost:8080.

To test Docker setup, use:
```bash
sudo DOCKER_BUILDKIT=1 docker build --tag doileak . && \
sudo docker run -p 8080:8080 doileak
```

## Deploy

Simply `fly deploy` from this directory.

## Why two domains?

Well, the authors of the `Referer` spec spelled it wrong :) But two domains also allow testing cross-origin behavior.

## What is this for?

Probably nothing to be honest. I just had this idea one day and wanted to try out fly.io for a long time, so I figured why not build it.

## TODO

This is a tiny project which I would consider done, but there are two improvements I would like:

- Serve the image instead of index when Accept headers indicates this was used as `<img src="https://doileakreferer.com"/>`. This avoids the need for `/image` path.
- Show the git hash from which the Fly image was built & deployed.