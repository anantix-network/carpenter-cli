#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Print with colors
info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Check for required tools
check_requirements() {
    info "Checking requirements..."
    
    if ! command -v git &> /dev/null; then
        error "git is required but not installed. Please install git first."
    fi

    if ! command -v cargo &> /dev/null; then
        error "cargo is required but not installed. Please install Rust first: https://rustup.rs"
    fi
}

# Clone repository
clone_repo() {
    local tmp_dir="$HOME/.carpet-install"
    info "Creating temporary directory: $tmp_dir"
    rm -rf "$tmp_dir"
    mkdir -p "$tmp_dir"

    info "Cloning carpenter-cli repository..."
    git clone --depth 1 https://github.com/your-username/carpenter-cli.git "$tmp_dir" || error "Failed to clone repository"
    
    echo "$tmp_dir"
}

# Build from source
build_source() {
    local install_dir="$1"
    cd "$install_dir"
    
    info "Building carpenter-cli..."
    cargo build --release || error "Failed to build carpet-cli"
}

# Install binary
install_binary() {
    local install_dir="$1"
    
    info "Installing carpet to /usr/local/bin..."
    sudo mkdir -p /usr/local/bin
    sudo cp "${install_dir}/target/release/carpenter-cli" /usr/local/bin/carpet || error "Failed to copy binary"
    sudo chmod +x /usr/local/bin/carpet || error "Failed to set executable permissions"
}

# Cleanup installation files
cleanup() {
    local install_dir="$1"
    info "Cleaning up installation files..."
    rm -rf "$install_dir"
}

# Verify installation
verify_installation() {
    if command -v carpet &> /dev/null; then
        info "Successfully installed carpet-cli!"
        info "Testing installation:"
        carpet --version || error "Failed to run carpet --version"
        info "You can now use the 'carpet' command"
    else
        error "Installation failed. Please check your PATH and try again"
    fi
}

# Main installation process
main() {
    # Don't run as root
    if [ "$(id -u)" = "0" ]; then
        error "Do not run this script as root or with sudo"
    fi

    info "Starting carpet-cli installation..."
    
    check_requirements
    
    local install_dir
    install_dir=$(clone_repo)
    
    build_source "$install_dir"
    install_binary "$install_dir"
    cleanup "$install_dir"
    verify_installation
}

# Run main function
main
