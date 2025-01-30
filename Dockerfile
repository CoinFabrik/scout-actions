FROM coinfabrik/scout-tests:latest
SHELL ["/bin/bash", "-c"]
WORKDIR /scoutme
COPY . .

