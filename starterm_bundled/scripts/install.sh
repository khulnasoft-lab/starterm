#!/bin/bash

# Starterm Bundled Resources Installer
# This script installs themes, configs, and workflows to the user's system

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
STARTERM_CONFIG_DIR="${HOME}/.config/starterm"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUNDLE_DIR="$(dirname "$SCRIPT_DIR")"

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${PURPLE}"
    echo "═══════════════════════════════════════════════════════════"
    echo "  🚀 Starterm Bundled Resources Installer"
    echo "═══════════════════════════════════════════════════════════"
    echo -e "${NC}"
}

print_section() {
    echo -e "\n${CYAN}📦 $1${NC}"
    echo "───────────────────────────────────────────────────────────"
}

check_dependencies() {
    print_section "Checking Dependencies"
    
    local deps=("cp" "mkdir" "chmod")
    local missing_deps=()
    
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            missing_deps+=("$dep")
        fi
    done
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        log_error "Missing dependencies: ${missing_deps[*]}"
        exit 1
    fi
    
    log_success "All dependencies found"
}

create_directories() {
    print_section "Creating Directories"
    
    local dirs=(
        "$STARTERM_CONFIG_DIR"
        "$STARTERM_CONFIG_DIR/themes"
        "$STARTERM_CONFIG_DIR/workflows"
        "$STARTERM_CONFIG_DIR/config"
        "$STARTERM_CONFIG_DIR/assets"
    )
    
    for dir in "${dirs[@]}"; do
        if [ ! -d "$dir" ]; then
            mkdir -p "$dir"
            log_info "Created directory: $dir"
        else
            log_info "Directory exists: $dir"
        fi
    done
    
    log_success "Directory structure ready"
}

install_themes() {
    print_section "Installing Themes"
    
    local theme_count=0
    
    if [ -d "$BUNDLE_DIR/themes" ]; then
        for theme_file in "$BUNDLE_DIR/themes"/*.yaml; do
            if [ -f "$theme_file" ]; then
                local theme_name=$(basename "$theme_file")
                cp "$theme_file" "$STARTERM_CONFIG_DIR/themes/"
                log_info "Installed theme: $theme_name"
                ((theme_count++))
            fi
        done
        
        log_success "Installed $theme_count themes"
    else
        log_warning "No themes directory found"
    fi
}

install_workflows() {
    print_section "Installing Workflows"
    
    local workflow_count=0
    
    if [ -d "$BUNDLE_DIR/workflows" ]; then
        # Copy all workflow files and directories
        cp -r "$BUNDLE_DIR/workflows"/* "$STARTERM_CONFIG_DIR/workflows/" 2>/dev/null || true
        
        # Count workflow files
        workflow_count=$(find "$BUNDLE_DIR/workflows" -name "*.yaml" -o -name "*.yml" | wc -l)
        
        log_success "Installed $workflow_count workflow files"
    else
        log_warning "No workflows directory found"
    fi
}

install_config() {
    print_section "Installing Configuration"
    
    if [ -f "$BUNDLE_DIR/config/starterm.yaml" ]; then
        local config_dest="$STARTERM_CONFIG_DIR/starterm.yaml"
        
        if [ -f "$config_dest" ]; then
            # Backup existing config
            local backup_file="$config_dest.backup.$(date +%Y%m%d_%H%M%S)"
            cp "$config_dest" "$backup_file"
            log_warning "Backed up existing config to: $backup_file"
        fi
        
        cp "$BUNDLE_DIR/config/starterm.yaml" "$config_dest"
        log_success "Installed default configuration"
    else
        log_warning "No default configuration found"
    fi
}

install_scripts() {
    print_section "Installing Utility Scripts"
    
    local script_count=0
    
    if [ -d "$BUNDLE_DIR/scripts" ]; then
        for script_file in "$BUNDLE_DIR/scripts"/*.py "$BUNDLE_DIR/scripts"/*.sh; do
            if [ -f "$script_file" ] && [ "$(basename "$script_file")" != "install.sh" ]; then
                local script_name=$(basename "$script_file")
                cp "$script_file" "$STARTERM_CONFIG_DIR/"
                chmod +x "$STARTERM_CONFIG_DIR/$script_name" 2>/dev/null || true
                log_info "Installed script: $script_name"
                ((script_count++))
            fi
        done
        
        log_success "Installed $script_count utility scripts"
    else
        log_warning "No scripts directory found"
    fi
}

show_summary() {
    print_section "Installation Summary"
    
    local theme_count=$(find "$STARTERM_CONFIG_DIR/themes" -name "*.yaml" 2>/dev/null | wc -l || echo "0")
    local workflow_count=$(find "$STARTERM_CONFIG_DIR/workflows" -name "*.yaml" -o -name "*.yml" 2>/dev/null | wc -l || echo "0")
    
    echo -e "${GREEN}Installation completed successfully!${NC}\n"
    echo -e "📊 ${YELLOW}Statistics:${NC}"
    echo -e "   • Themes installed: ${GREEN}$theme_count${NC}"
    echo -e "   • Workflows installed: ${GREEN}$workflow_count${NC}"
    echo -e "   • Configuration: ${GREEN}✓${NC}"
    echo -e "   • Install location: ${CYAN}$STARTERM_CONFIG_DIR${NC}"
    
    echo -e "\n🎨 ${YELLOW}Popular themes available:${NC}"
    echo -e "   • ${PURPLE}catppuccin_mocha${NC} (dark, modern)"
    echo -e "   • ${PURPLE}dracula${NC} (dark, vibrant)"
    echo -e "   • ${PURPLE}gruvbox_dark${NC} (dark, retro)"
    echo -e "   • ${PURPLE}tokyo_night${NC} (dark, elegant)"
    echo -e "   • ${PURPLE}github_light${NC} (light, clean)"
    
    echo -e "\n⚡ ${YELLOW}Next steps:${NC}"
    echo -e "   1. Start Starterm: ${CYAN}starterm${NC}"
    echo -e "   2. Choose a theme in your configuration"
    echo -e "   3. Explore workflows in the sidebar (Ctrl+Shift+Space)"
    echo -e "   4. Customize keybindings and preferences"
    
    echo -e "\n📚 ${YELLOW}Documentation:${NC}"
    echo -e "   • Configuration: ${CYAN}$STARTERM_CONFIG_DIR/starterm.yaml${NC}"
    echo -e "   • Themes: ${CYAN}$STARTERM_CONFIG_DIR/themes/${NC}"
    echo -e "   • Workflows: ${CYAN}$STARTERM_CONFIG_DIR/workflows/${NC}"
}

main() {
    print_header
    
    log_info "Bundle directory: $BUNDLE_DIR"
    log_info "Install directory: $STARTERM_CONFIG_DIR"
    
    check_dependencies
    create_directories
    install_themes
    install_workflows
    install_config
    install_scripts
    show_summary
    
    echo -e "\n${GREEN}🎉 Installation complete! Enjoy using Starterm!${NC}"
}

# Parse command line options
FORCE_INSTALL=false
QUIET=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -f|--force)
            FORCE_INSTALL=true
            shift
            ;;
        -q|--quiet)
            QUIET=true
            shift
            ;;
        -h|--help)
            echo "Starterm Bundled Resources Installer"
            echo ""
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  -f, --force    Force installation (overwrite existing files)"
            echo "  -q, --quiet    Quiet installation (minimal output)"
            echo "  -h, --help     Show this help message"
            echo ""
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Run main function
main
