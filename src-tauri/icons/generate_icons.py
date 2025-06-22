#!/usr/bin/env python3
"""
Simple icon generator for Tauri applications.
Creates basic PNG icons and ICNS file from a simple design.
"""

import os
from PIL import Image, ImageDraw

def create_simple_icon(size):
    """Create a simple icon with the given size."""
    # Create a new image with transparent background
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Draw a simple design - a rounded rectangle with "H" for Hosts Editor
    margin = size // 8
    rect_coords = [margin, margin, size - margin, size - margin]
    
    # Draw background rounded rectangle
    draw.rounded_rectangle(rect_coords, radius=size//8, fill=(70, 130, 180, 255))
    
    # Draw "H" letter in white
    font_size = size // 2
    text_x = size // 2
    text_y = size // 2
    
    # Simple "H" using rectangles since we don't have font access
    bar_width = size // 12
    bar_height = size // 3
    h_gap = size // 6
    
    # Left vertical bar
    left_x = text_x - h_gap // 2 - bar_width
    draw.rectangle([left_x, text_y - bar_height // 2, left_x + bar_width, text_y + bar_height // 2], fill=(255, 255, 255, 255))
    
    # Right vertical bar  
    right_x = text_x + h_gap // 2
    draw.rectangle([right_x, text_y - bar_height // 2, right_x + bar_width, text_y + bar_height // 2], fill=(255, 255, 255, 255))
    
    # Horizontal bar
    h_bar_height = bar_width
    draw.rectangle([left_x, text_y - h_bar_height // 2, right_x + bar_width, text_y + h_bar_height // 2], fill=(255, 255, 255, 255))
    
    return img

def main():
    """Generate all required icon files."""
    # Create icons directory if it doesn't exist
    icons_dir = os.path.dirname(os.path.abspath(__file__))
    
    # Generate PNG icons
    sizes = [32, 128, 256, 512]
    for size in sizes:
        img = create_simple_icon(size)
        if size == 128:
            # Also create @2x version
            img.save(os.path.join(icons_dir, f'{size}x{size}@2x.png'))
        img.save(os.path.join(icons_dir, f'{size}x{size}.png'))
    
    # Create a basic ICNS file (simplified - normally would use iconutil on macOS)
    # For now, just create a 512x512 PNG and rename it
    # In a real scenario, you'd use proper ICNS generation
    large_icon = create_simple_icon(512)
    large_icon.save(os.path.join(icons_dir, 'icon.png'))
    
    print("Generated icon files:")
    for size in sizes:
        print(f"  - {size}x{size}.png")
        if size == 128:
            print(f"  - {size}x{size}@2x.png")
    print("  - icon.png")
    print("\nNote: For proper macOS support, you should generate icon.icns using:")
    print("  npm run tauri icon")
    print("or use iconutil on macOS to convert PNG to ICNS format.")

if __name__ == "__main__":
    main()
