FROM rust:latest


WORKDIR /usr/src/myapp
COPY . .
#RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
#RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
#RUN ls -ltr
#RUN cargo install cargo-dylint dylint-link
#RUN cargo install --path ./apps/cargo-scout-audit/
RUN /bin/bash
RUN apt-get -y update; apt-get -y install curl
RUN apt-get install -y build-essential libssl-dev pkg-config
#RUN curl https://sh.rustup.rs -sSf > rustup.sh
#RUN chmod 755 rustup.sh
#RUN ./rustup.sh -y
RUN cargo install cargo-dylint dylint-link
RUN cargo install --path ./apps/cargo-scout-audit/
RUN cargo install mdbook
RUN export PATH="$HOME/.cargo/bin:$PATH"
COPY ./vesting .
RUN cat ./vesting/Cargo.toml
CMD ["/usr/local/cargo/bin/cargo -h"]
#:CMD ["/bin/bash sleep 180"]
