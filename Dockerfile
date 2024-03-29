# 使用Rust官方镜像作为基础镜像
FROM rust:latest

# 设置工作目录
WORKDIR /usr/src/app

# 复制项目的Cargo.toml和Cargo.lock文件到工作目录
COPY Cargo.toml Cargo.lock ./

# 使用Cargo安装项目依赖
RUN cargo install --locked

# 复制项目的源代码到工作目录
COPY . .

# 构建Rust项目
RUN cargo build --release

# 从基础镜像中获取可执行文件到当前目录
COPY --from=0 /usr/local/cargo/bin/hello-world.exe .

# 设置容器启动时运行的命令
CMD ["./hello-world.exe"]