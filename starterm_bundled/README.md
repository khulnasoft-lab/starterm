# Starterm Bundled Resources

This directory contains bundled resources for Starterm that are ready for distribution and integration into the terminal emulator.

## 📁 Directory Structure

```
starterm_bundled/
├── README.md                 # This file
├── themes/                   # Color themes in YAML format
│   ├── dracula.yaml         # Popular themes converted from TOML
│   ├── catppuccin_mocha.yaml
│   ├── gruvbox_dark.yaml
│   └── ... (152 themes total)
├── config/                   # Default configuration files
├── workflows/                # Pre-built workflow definitions
├── scripts/                  # Utility and setup scripts
└── assets/                   # Icons, fonts, and other assets
```

## 🎨 Themes

The `themes/` directory contains **152 professionally crafted color themes** converted from TOML to YAML format for better readability and modern configuration management.

### Popular Themes Included
- **Dark Themes**: Dracula, Gruvbox Dark, Tokyo Night, Nord, One Dark
- **Light Themes**: Gruvbox Light, Solarized Light, GitHub Light
- **Catppuccin Collection**: Mocha, Macchiato, Frappe, Latte
- **GitHub Collection**: All variants including colorblind and high contrast
- **Material Design**: Material Theme and variants

### Theme Format
All themes follow the YAML structure:
```yaml
colors:
  primary:
    background: '#1e1e2e'
    foreground: '#cdd6f4'
  normal:
    black: '#45475a'
    red: '#f38ba8'
    # ... other colors
```

## 🔧 Configuration

The `config/` directory will contain:
- Default Starterm configuration files
- Key binding presets
- Font configurations
- UI layout presets

## ⚡ Workflows

The `workflows/` directory will contain:
- Pre-built workflow definitions
- Common development tasks
- System administration workflows
- Git operations
- Package management workflows

## 🛠️ Scripts

The `scripts/` directory contains:
- `../convert_themes.py` - Theme conversion utility
- Setup and installation scripts
- Migration utilities
- Development tools

## 🎯 Assets

The `assets/` directory will contain:
- Icons and graphics
- Font files
- Documentation images
- UI component assets

## 🚀 Usage

### Installing Themes
1. Copy desired theme files from `themes/` to your Starterm config directory
2. Update your Starterm configuration to reference the theme
3. Restart Starterm to apply changes

### Theme Selection
Choose from various categories:
- **Professional**: GitHub, Material, VS Code themes
- **Vintage**: Terminal.app, Tango, XTerm
- **Gaming**: Cyberpunk, Synthwave, Rainbow
- **Accessibility**: High contrast, colorblind-friendly variants

## 🔄 Conversion Process

The themes were converted from TOML to YAML using our custom conversion script:

```bash
# Convert all themes
python3 convert_themes.py

# Output: 152/152 themes successfully converted
```

**Benefits of YAML format:**
- More human-readable
- Better IDE support
- Easier to edit and maintain
- Consistent with modern config standards
- Better comment support

## 📊 Statistics

- **Total Themes**: 152
- **Source Format**: TOML
- **Target Format**: YAML
- **Conversion Success Rate**: 100%
- **Categories**: Dark (85), Light (25), High Contrast (8), Gaming (12), etc.

## 🤝 Contributing

To add new themes or resources:

1. **Themes**: Add YAML files to `themes/` following the existing format
2. **Workflows**: Add workflow definitions to `workflows/`
3. **Assets**: Place new assets in appropriate subdirectories
4. **Documentation**: Update this README with new additions

## 📜 License

These bundled resources maintain their original licenses:
- Themes: Various (see individual theme attribution)
- Starterm components: Apache-2.0 / MIT
- Workflows: Apache-2.0 / MIT

## 🔗 References

- [Original Theme Collection](../theme/themes/)
- [Starterm Main Repository](../)
- [Workflow System](../workflows/)
- [Configuration Documentation](../config/)

---

*Generated on 2025-01-13 for Starterm v0.13+*
