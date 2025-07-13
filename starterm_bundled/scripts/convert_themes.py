#!/usr/bin/env python3
"""
Theme Converter: TOML to YAML
Converts Starterm theme files from TOML to YAML format
"""

import os
import sys
import toml
import yaml
from pathlib import Path

def convert_toml_to_yaml(toml_file, yaml_file):
    """Convert a TOML file to YAML format"""
    try:
        # Read TOML file
        with open(toml_file, 'r') as f:
            data = toml.load(f)
        
        # Write YAML file
        with open(yaml_file, 'w') as f:
            yaml.dump(data, f, default_flow_style=False, sort_keys=False, indent=2)
        
        print(f"✅ Converted: {toml_file.name} -> {yaml_file.name}")
        return True
    except Exception as e:
        print(f"❌ Error converting {toml_file}: {e}")
        return False

def main():
    # Define paths
    script_dir = Path(__file__).parent
    themes_dir = script_dir / "theme" / "themes"
    output_dir = script_dir / "starterm_bundled" / "themes"
    
    # Ensure output directory exists
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # Check if themes directory exists
    if not themes_dir.exists():
        print(f"❌ Themes directory not found: {themes_dir}")
        sys.exit(1)
    
    # Get all TOML files
    toml_files = list(themes_dir.glob("*.toml"))
    
    if not toml_files:
        print(f"❌ No TOML theme files found in {themes_dir}")
        sys.exit(1)
    
    print(f"🎨 Converting {len(toml_files)} theme files from TOML to YAML...")
    print(f"📁 Source: {themes_dir}")
    print(f"📁 Output: {output_dir}")
    print("=" * 60)
    
    converted_count = 0
    
    for toml_file in sorted(toml_files):
        # Generate YAML filename
        yaml_file = output_dir / f"{toml_file.stem}.yaml"
        
        if convert_toml_to_yaml(toml_file, yaml_file):
            converted_count += 1
    
    print("=" * 60)
    print(f"🎉 Conversion complete: {converted_count}/{len(toml_files)} files converted")
    
    if converted_count < len(toml_files):
        sys.exit(1)

if __name__ == "__main__":
    main()
