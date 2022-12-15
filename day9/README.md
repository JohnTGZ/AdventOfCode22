# Day 9

## Building
```bash
# Create cmake configuration
cmake -S . -B build
# Build the files
cmake --build build
```

## Run tests
```bash
ctest --test-dir build --verbose
```