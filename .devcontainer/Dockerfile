FROM rust:1.83-bullseye

# 安装基本工具
RUN apt-get update && apt-get install -y \
    git \
    curl \
    wget \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

# 设置工作目录
WORKDIR /workspace

# 切换到非 root 用户
USER $USERNAME

# 预安装一些常用的 Rust 工具
RUN cargo install cargo-watch cargo-edit 