FROM coinfabrik/scout-image:latest

SHELL ["/bin/bash", "-c"]

WORKDIR /usr/src/myapp
# Copy source code
COPY . .

COPY entrypoint.sh /usr/src/myapp/entrypoint.sh
RUN chmod +xr /usr/src/myapp/entrypoint.sh

ENTRYPOINT ["/usr/src/myapp/entrypoint.sh"]

