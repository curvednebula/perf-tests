# Go

cd go

go run -ldflags="-s -w" .

# Rust

cd rust

cargo run --release

# Results

100'000 tasks, 10'000 map inserts and gets in each task.

**Go (task = goroutine):**
 - finished in 46.32s, one task avg 23.59s, min 0.02s, max 46.32s
 - RAM: 1.5Gb - 4Gb

**Rust (task = tokio task):**
 - finished in 67.85s, one task avg 33.237s, min 0.007s, max 67.854s
 - RAM: 35Mb - 60Mb


![Chart](assets/chart1.png)
