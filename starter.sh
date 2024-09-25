
export LIP=zilezia.dev
echo "Building frontend"
cd ./front
trunk build --release
echo "Frontend built"

cd ..
echo "Running app"
cargo run --release

# nie zmieniam już tego w ogóle /srs
