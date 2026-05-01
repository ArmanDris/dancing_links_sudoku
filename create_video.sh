# Create SVG frames
rm tmp/dlx_frames/*.svg
cargo run --example visualize

# Convert to png
# NOTE Currently width of png is 800, final draft should increase this
mkdir -p tmp/dlx_png
i=0
rm tmp/dlx_png/*.png
for f in tmp/dlx_frames/*.svg; do
    printf -v out "tmp/dlx_png/frame_%04d.png" "$i"
    echo "converting $f -> $out"
    rsvg-convert --width 800 "$f" -o "$out"
    i=$((i + 1))
done

# Turn png's into video
ffmpeg -framerate 8 -i tmp/dlx_png/frame_%04d.png \
    -vf "scale=trunc(iw/2)*2:trunc(ih/2)*2" \
    -c:v libx264 -crf 18 -pix_fmt yuv420p \
    "tmp/dlx_visualization_$(date +%Y%m%d_%H%M%S).mp4"

