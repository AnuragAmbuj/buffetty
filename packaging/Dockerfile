FROM ubuntu:22.04

# Install Build Dependencies
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    libgtk-4-dev \
    libxml2-utils \
    git \
 && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Create working directory
WORKDIR /app

# Copy Cargo files first to cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY alacritty/Cargo.toml alacritty/Cargo.toml
COPY alacritty_terminal/Cargo.toml alacritty_terminal/Cargo.toml
COPY alacritty_config/Cargo.toml alacritty_config/Cargo.toml
COPY alacritty_config_derive/Cargo.toml alacritty_config_derive/Cargo.toml
COPY alacritty_gtk/Cargo.toml alacritty_gtk/Cargo.toml

# Create dummy source files to trigger dependency build
RUN mkdir -p alacritty/src alacritty_terminal/src alacritty_config/src alacritty_config_derive/src alacritty_gtk/src && \
    touch alacritty/src/lib.rs alacritty/src/main.rs && \
    touch alacritty_terminal/src/lib.rs && \
    touch alacritty_config/src/lib.rs && \
    touch alacritty_config_derive/src/lib.rs && \
    touch alacritty_gtk/src/main.rs && \
    echo "fn main() {}" > alacritty_gtk/src/main.rs

# Build dependencies
RUN cargo build --release --workspace || true

# Copy full source
COPY . .

# Force rebuild of our crates (since we touched them properly now)
RUN touch alacritty/src/lib.rs alacritty_gtk/src/main.rs

# Build release
RUN cargo build --release -p alacritty_gtk

# Packaging script
COPY packaging/build_deb.sh /usr/local/bin/build_deb.sh
RUN chmod +x /usr/local/bin/build_deb.sh

ENTRYPOINT ["/usr/local/bin/build_deb.sh"]
