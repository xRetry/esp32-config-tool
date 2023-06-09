FROM microros/base:humble
ARG DEBIAN_FRONTEND=noninteractive
SHELL ["/bin/bash", "-c"]

RUN apt-get update && apt-get install -y curl
RUN curl https://raw.githubusercontent.com/xRetry/dev-env/main/setup_env.sh | sh -s -- rust python

# Install dependencies
RUN apt-get update && apt-get install -y \
    libclang-dev \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

ENV PATH=/root/.cargo/bin:$PATH
RUN cargo install cargo-ament-build
RUN pip install --upgrade pytest 

# Install the colcon-cargo and colcon-ros-cargo plugins
RUN pip install git+https://github.com/colcon/colcon-cargo.git git+https://github.com/colcon/colcon-ros-cargo.git

WORKDIR /

# Create directory for projects (there should be mounted from host).
RUN mkdir -p /ws

# Set default location after container startup.
WORKDIR /ws
