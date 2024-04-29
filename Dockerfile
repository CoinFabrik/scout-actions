FROM coinfabrik/citest:latest
SHELL ["/bin/bash", "-c"]
WORKDIR /scoutme
COPY . .

