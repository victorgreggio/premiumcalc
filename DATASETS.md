# Dataset Management

This directory contains multiple datasets for different use cases.

## Dataset Files

### `applicants.csv` (Active Dataset)
- **Purpose**: The active dataset used by the application
- **Default**: 25 applicants (good for development)
- **Usage**: Modified by running the data generator or switching scripts

### `applicants_100k.csv` (Large Dataset)
- **Purpose**: Pre-generated 100,000 applicant dataset
- **Size**: ~7.68 MB
- **Usage**: Performance benchmarking and stress testing
- **Note**: Only use with `cargo run --release`

## Switching Between Datasets

### Windows (PowerShell)

```powershell
# For development and quick testing (debug mode friendly)
.\switch_to_small.ps1

# For performance benchmarking (requires release mode)
.\switch_to_large.ps1
```

## Generating New Data

To generate a fresh dataset of 100,000 applicants:

```bash
cargo run --release --bin generate_data
```

This will overwrite `applicants.csv` with 100,000 new records.

## Best Practices

### Development Mode (`cargo run`)
- ✅ Use small dataset (25 applicants)
- ✅ Fast compilation, quick iterations
- ❌ Avoid large datasets - will be extremely slow (10-20 minutes)

### Release Mode (`cargo run --release`)
- ✅ Use any dataset size
- ✅ Optimal performance
- ✅ Good for demos and benchmarks

### Benchmark Mode (`cargo run --release -- --benchmark`)
- ✅ Use large datasets (100k)
- ✅ No UI overhead
- ✅ Best for performance metrics

## Dataset Size Recommendations

| Applicants | Mode | Time (approx) | Use Case |
|-----------|------|---------------|----------|
| 25 | Debug | ~150ms | Development, unit testing |
| 25 | Release | ~2ms | Quick verification |
| 1,000 | Release | ~200ms | Medium dataset testing |
| 10,000 | Release | ~2s | Large dataset testing |
| 100,000 | Release | ~45s | Full performance benchmark |

## Warning System

The application will warn you if you attempt to run large datasets (>1000 records) in debug mode:

```
⚠️  WARNING: Running 100000 applicants in DEBUG mode will be very slow!
   For large datasets, use RELEASE mode:
   cargo run --release -- --benchmark

   Press Ctrl+C to cancel, or wait for debug build to complete...
```

Always heed this warning and switch to release mode or use a smaller dataset!
