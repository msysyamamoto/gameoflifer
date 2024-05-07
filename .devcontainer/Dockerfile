FROM rust:1.77

ENV DEBIAN_FRONTEND=noninteractive

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Configure apt and install packages
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    sudo \
    git \
    mold \
    2>&1 \
    #
    # Create a non-root user to use if preferred - see https://aka.ms/vscode-remote/containers/non-root-user.
    && groupadd --gid $USER_GID $USERNAME \
    && useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME \
    #
    # Add sudo support for non-root users
    && echo ${USERNAME} ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/${USERNAME} \
    && chmod 0440 /etc/sudoers.d/${USERNAME} \
    #
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# rustup component add
RUN rustup component add clippy \
    && rustup component add rls \
    && rustup component add rust-analysis \
    && rustup component add rust-src \
    && rustup component add rustfmt

ENV DEBIAN_FRONTEND=
ENV PATH="~/.cargo/bin:${PATH}"
USER ${USERNAME}

RUN mold -run cargo install cargo-generate wasm-pack
