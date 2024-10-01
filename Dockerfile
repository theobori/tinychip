FROM rust:latest

# Install dependencies pkgs
RUN apt-get update && \
    apt-get install -y libsdl2-dev

# Main working directory
ENV DIRNAME tinychip

WORKDIR ${DIRNAME}

# Copy files
COPY . .

# Build and Install the binary
RUN cargo build --release && \
    cargo install --path . && \
    rm -rf /${DIRNAME}

ENTRYPOINT [ "tinychip" ]
