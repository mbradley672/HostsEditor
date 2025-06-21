# Create a minimal ICO file
$iconData = @(
    # ICO header
    0, 0,      # Reserved, must be 0
    1, 0,      # Type: 1 for ICO
    1, 0,      # Number of images
    
    # Image directory entry
    16,        # Width: 16
    16,        # Height: 16
    0,         # Color count: 0 (no palette)
    0,         # Reserved
    1, 0,      # Color planes: 1
    32, 0,     # Bits per pixel: 32
    64, 4, 0, 0, # Size of image data: 1088 bytes
    22, 0, 0, 0  # Offset to image data: 22
)

# BITMAPINFOHEADER + image data
$imageData = @(
    40, 0, 0, 0,    # Size of BITMAPINFOHEADER
    16, 0, 0, 0,    # Width
    32, 0, 0, 0,    # Height (doubled for ICO)
    1, 0,           # Planes
    32, 0,          # Bits per pixel
    0, 0, 0, 0,     # Compression
    0, 0, 0, 0,     # Image size (can be 0 for uncompressed)
    0, 0, 0, 0,     # X pixels per meter
    0, 0, 0, 0,     # Y pixels per meter
    0, 0, 0, 0,     # Colors used
    0, 0, 0, 0      # Important colors
)

# 16x16 RGBA pixels (transparent)
$pixels = ,0 * (16 * 16 * 4)

# AND mask (all transparent)
$andMask = ,0 * (16 * 16 / 8)

# Combine all data
$allData = $iconData + $imageData + $pixels + $andMask

# Convert to byte array and write to file
$bytes = [byte[]]$allData
[System.IO.File]::WriteAllBytes("icon.ico", $bytes)
