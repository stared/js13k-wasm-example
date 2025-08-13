#!/bin/bash

echo "Creating submission.zip..."
rm -f submission.zip

zip -9 submission.zip index.html g.wasm

echo ""
echo "=== JS13K Submission Size Check ==="
echo ""

SIZE=$(stat -f%z submission.zip 2>/dev/null || stat -c%s submission.zip 2>/dev/null)
SIZE_KB=$(echo "scale=2; $SIZE / 1024" | bc)
LIMIT=13312  # 13KB in bytes

echo "Submission size: $SIZE bytes ($SIZE_KB KB)"
echo "Limit: $LIMIT bytes (13.00 KB)"
echo ""

if [ $SIZE -le $LIMIT ]; then
    REMAINING=$((LIMIT - SIZE))
    REMAINING_KB=$(echo "scale=2; $REMAINING / 1024" | bc)
    echo "✅ PASS - Within limit!"
    echo "Remaining space: $REMAINING bytes ($REMAINING_KB KB)"
else
    OVER=$((SIZE - LIMIT))
    OVER_KB=$(echo "scale=2; $OVER / 1024" | bc)
    echo "❌ FAIL - Over limit!"
    echo "Over by: $OVER bytes ($OVER_KB KB)"
fi

echo ""
echo "=== File Breakdown ==="
echo "index.html: $(stat -f%z index.html 2>/dev/null || stat -c%s index.html 2>/dev/null) bytes"
echo "g.wasm: $(stat -f%z g.wasm 2>/dev/null || stat -c%s g.wasm 2>/dev/null) bytes"