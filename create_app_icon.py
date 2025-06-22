#!/usr/bin/env python3
"""
Create a simple app icon for the Hosts Editor application.
"""

try:
    from PIL import Image, ImageDraw, ImageFont
    PIL_AVAILABLE = True
except ImportError:
    PIL_AVAILABLE = False
    print("PIL not available, creating a basic icon using alternative method")

def create_simple_icon_pil():
    """Create icon using PIL."""
    size = 1024
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Draw background circle
    margin = 50
    circle_coords = [margin, margin, size - margin, size - margin]
    draw.ellipse(circle_coords, fill=(70, 130, 180, 255))
    
    # Draw "H" for Hosts Editor
    font_size = size // 3
    try:
        # Try to use a system font
        font = ImageFont.truetype("arial.ttf", font_size)
    except:
        try:
            font = ImageFont.truetype("/System/Library/Fonts/Arial.ttf", font_size)
        except:
            font = ImageFont.load_default()
    
    text = "H"
    bbox = draw.textbbox((0, 0), text, font=font)
    text_width = bbox[2] - bbox[0]
    text_height = bbox[3] - bbox[1]
    
    text_x = (size - text_width) // 2
    text_y = (size - text_height) // 2
    
    draw.text((text_x, text_y), text, fill=(255, 255, 255, 255), font=font)
    
    return img

def create_simple_icon_basic():
    """Create a basic icon without PIL using raw PNG data."""
    # This creates a minimal 1024x1024 transparent PNG
    # PNG file structure: signature + IHDR + IDAT + IEND
    
    width = height = 1024
    
    # PNG signature
    png_signature = bytes([0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A])
    
    # IHDR chunk
    ihdr_data = (
        width.to_bytes(4, 'big') +      # Width
        height.to_bytes(4, 'big') +     # Height
        bytes([8, 6, 0, 0, 0])          # Bit depth, color type, compression, filter, interlace
    )
    ihdr_chunk = create_png_chunk(b'IHDR', ihdr_data)
    
    # Create minimal IDAT chunk (compressed image data for transparent image)
    # This is a very basic deflate stream for a transparent image
    idat_data = bytes([
        0x78, 0x9C,  # Deflate header
        0xED, 0xC1, 0x01, 0x01, 0x00, 0x00, 0x00, 0x80, 0x90, 0xFE, 0x37, 0x03, 0x00, 0x00, 0x00, 0x01
    ])
    # For a real implementation, we'd need proper deflate compression of the image data
    # This is a placeholder that creates a minimal valid PNG
    
    # Create a larger IDAT with proper transparent data
    row_size = width * 4 + 1  # 4 bytes per pixel (RGBA) + 1 filter byte
    total_size = row_size * height
    
    # Simple approach: create deflate data for transparent image
    # Each row starts with filter byte 0, followed by transparent pixels (0,0,0,0)
    import zlib
    raw_data = bytearray()
    for y in range(height):
        raw_data.append(0)  # Filter byte
        for x in range(width):
            raw_data.extend([0, 0, 0, 0])  # Transparent pixel (RGBA)
    
    compressed_data = zlib.compress(raw_data)
    idat_chunk = create_png_chunk(b'IDAT', compressed_data)
    
    # IEND chunk
    iend_chunk = create_png_chunk(b'IEND', b'')
    
    return png_signature + ihdr_chunk + idat_chunk + iend_chunk

def create_png_chunk(chunk_type, data):
    """Create a PNG chunk with length, type, data, and CRC."""
    import zlib
    length = len(data).to_bytes(4, 'big')
    crc = zlib.crc32(chunk_type + data).to_bytes(4, 'big')
    return length + chunk_type + data + crc

def main():
    """Create the app icon."""
    if PIL_AVAILABLE:
        print("Creating icon using PIL...")
        img = create_simple_icon_pil()
        img.save('app-icon.png')
        print("Created app-icon.png using PIL")
    else:
        print("Creating basic transparent icon...")
        png_data = create_simple_icon_basic()
        with open('app-icon.png', 'wb') as f:
            f.write(png_data)
        print("Created app-icon.png (basic transparent)")
    
    print("Icon created successfully!")
    print("Now run: npx tauri icon")

if __name__ == "__main__":
    main()
