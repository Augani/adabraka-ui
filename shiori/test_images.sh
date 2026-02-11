#!/bin/bash
# Test script for terminal inline image display
# Run this INSIDE Shiori's terminal to test image protocols

set -e

# Generate a small 4x4 red PNG using Python (available on macOS)
TINY_PNG=$(python3 -c "
import base64, struct, zlib

def make_png(width, height, r, g, b):
    def chunk(ctype, data):
        c = ctype + data
        return struct.pack('>I', len(data)) + c + struct.pack('>I', zlib.crc32(c) & 0xffffffff)

    raw = b''
    for _ in range(height):
        raw += b'\x00'  # filter none
        for _ in range(width):
            raw += bytes([r, g, b, 255])

    sig = b'\x89PNG\r\n\x1a\n'
    ihdr = chunk(b'IHDR', struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0))
    idat = chunk(b'IDAT', zlib.compress(raw))
    iend = chunk(b'IEND', b'')
    return base64.b64encode(sig + ihdr + idat + iend).decode()

print(make_png(32, 32, 255, 80, 80))
")

BLUE_PNG=$(python3 -c "
import base64, struct, zlib

def make_png(width, height, r, g, b):
    def chunk(ctype, data):
        c = ctype + data
        return struct.pack('>I', len(data)) + c + struct.pack('>I', zlib.crc32(c) & 0xffffffff)

    raw = b''
    for _ in range(height):
        raw += b'\x00'
        for _ in range(width):
            raw += bytes([r, g, b, 255])

    sig = b'\x89PNG\r\n\x1a\n'
    ihdr = chunk(b'IHDR', struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0))
    idat = chunk(b'IDAT', zlib.compress(raw))
    iend = chunk(b'IEND', b'')
    return base64.b64encode(sig + ihdr + idat + iend).decode()

print(make_png(64, 32, 80, 120, 255))
")

echo "=== iTerm2 Protocol Tests ==="
echo ""

echo "Test 1: Red 32x32 square (auto size)"
printf '\e]1337;File=inline=1:%s\a' "$TINY_PNG"
echo ""
echo ""

echo "Test 2: Blue rectangle (width=20 cells)"
printf '\e]1337;File=inline=1;width=20:%s\a' "$BLUE_PNG"
echo ""
echo ""

echo "Test 3: Red square (10x5 cells)"
printf '\e]1337;File=inline=1;width=10;height=5:%s\a' "$TINY_PNG"
echo ""
echo ""

echo "Test 4: Non-inline (should NOT display)"
printf '\e]1337;File=inline=0:%s\a' "$TINY_PNG"
echo "(nothing should appear above)"
echo ""

echo "=== Kitty Protocol Tests ==="
echo ""

echo "Test 5: Kitty PNG display (auto size)"
printf '\e_Ga=T,f=100;%s\e\\' "$TINY_PNG"
echo ""
echo ""

echo "Test 6: Kitty PNG with cell size (c=15,r=4)"
printf '\e_Ga=T,f=100,c=15,r=4;%s\e\\' "$BLUE_PNG"
echo ""
echo ""

echo "=== All tests complete ==="
echo "You should see colored rectangles for tests 1,2,3,5,6"
echo "Test 4 should show nothing (non-inline)"
