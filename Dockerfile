FROM rust:latest

# Install dependencies pkgs
RUN apt-get update && \
    apt-get install -y libsdl2-dev

# Main working directory
ENV DIRNAME tinychip

WORKDIR ${DIRNAME}

RUN groupadd rustgroup && \
    useradd -m -g rustgroup tinychip

# Copy files
COPY src src
COPY Cargo.toml Cargo.toml

# Build and Install the binary
RUN cargo build --release && \
    cargo install --path . && \
    rm -rf /${DIRNAME}

USER tinychip

ENTRYPOINT [ "tinychip" ]
