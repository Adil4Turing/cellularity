FROM rust:1.90-slim-bookworm AS builder

# Install required system dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy dependency manifests first for better Docker layer caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source files for dependency caching
# This allows cargo to build dependencies without the actual source
RUN mkdir -p src/core/rules src/utils && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub mod core; pub mod error; pub mod utils;" > src/lib.rs && \
    echo "#[derive(Debug, Clone)] pub enum Error {}" > src/error.rs && \
    echo "impl std::fmt::Display for Error {" >> src/error.rs && \
    echo "    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result { Ok(()) }" >> src/error.rs && \
    echo "}" >> src/error.rs && \
    echo "impl std::error::Error for Error {}" >> src/error.rs && \
    echo "pub type Result<T> = std::result::Result<T, Error>;" >> src/error.rs && \
    echo "pub mod automata; pub mod boundary; pub mod cell;" > src/core/mod.rs && \
    echo "pub mod dense_grid; pub mod grid; pub mod moore_neighborhood;" >> src/core/mod.rs && \
    echo "pub mod neighborhood; pub mod rules; pub mod toroidal_boundary;" >> src/core/mod.rs && \
    echo "pub mod von_neumann_neighborhood; pub mod walled_boundary;" >> src/core/mod.rs && \
    echo "" > src/core/automata.rs && \
    echo "" > src/core/boundary.rs && \
    echo "" > src/core/cell.rs && \
    echo "" > src/core/dense_grid.rs && \
    echo "" > src/core/grid.rs && \
    echo "" > src/core/moore_neighborhood.rs && \
    echo "" > src/core/neighborhood.rs && \
    echo "" > src/core/toroidal_boundary.rs && \
    echo "" > src/core/von_neumann_neighborhood.rs && \
    echo "" > src/core/walled_boundary.rs && \
    echo "pub mod trait_def; pub mod conway;" > src/core/rules/mod.rs && \
    echo "" > src/core/rules/trait_def.rs && \
    echo "" > src/core/rules/conway.rs && \
    echo "pub mod error;" > src/utils/mod.rs && \
    echo "pub use crate::error::{Error, Result};" > src/utils/error.rs

# Build dependencies only (this layer will be cached if dependencies don't change)
RUN cargo build --release && \
    rm -rf src target/release/deps/cellularity*

# Copy actual source code
COPY src ./src

# Build the actual application (using cached dependencies)
RUN touch src/main.rs src/lib.rs && \
    cargo build --release

# Stage 2: Runtime stage - minimal image with just the binary
FROM debian:bookworm-slim

# Install runtime dependencies if needed
# (egui/eframe may need some system libraries if running GUI)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    # X11 + Wayland client libraries commonly required by winit/egui
    libx11-6 \
    libx11-xcb1 \
    libxcb1 \
    libxcb-render0 \
    libxcb-shape0 \
    libxcb-xfixes0 \
    libxrandr2 \
    libxi6 \
    libxcursor1 \
    libxkbcommon0 \
    libwayland-client0 \
    libwayland-egl1 \
    # OpenGL/EGL/GLX + software rendering fallback (Mesa)
    libgl1 \
    libglx0 \
    libglx-mesa0 \
    libgl1-mesa-dri \
    libegl1 \
    libgbm1 \
    libgles2 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user for security
RUN useradd -m -u 1000 appuser && \
    mkdir -p /app && \
    chown -R appuser:appuser /app

# Set working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/cellularity /app/cellularity

# Change ownership to non-root user
RUN chown appuser:appuser /app/cellularity && \
    chmod +x /app/cellularity

# Switch to non-root user
USER appuser

# Set the binary as the entrypoint
ENTRYPOINT ["/app/cellularity"]

# Default command (can be overridden)
CMD []
