FROM coinfabrik/scout:latest
SHELL ["/bin/bash", "-c"]
WORKDIR /scoutme
COPY . .

