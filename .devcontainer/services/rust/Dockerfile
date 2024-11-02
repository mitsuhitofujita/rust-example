FROM rust:1.82.0-bookworm

ARG RUST_UID=1000
ARG RUST_USER=vscode
ARG RUST_HOME=/home/vscode
ARG WORKSPACE_DIR=/workspace
RUN useradd -u ${RUST_UID} -d ${RUST_HOME} -m ${RUST_USER}
