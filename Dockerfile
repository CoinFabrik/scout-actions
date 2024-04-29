FROM coinfabrik/scout-image:latest
SHELL ["/bin/bash", "-c"]
WORKDIR /scoutme
COPY . .

