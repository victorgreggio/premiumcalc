# Switch to large dataset (100k applicants)
Copy-Item applicants.csv applicants_small.csv -Force -ErrorAction SilentlyContinue
Copy-Item applicants_100k.csv applicants.csv -Force
Write-Host "✓ Switched to LARGE dataset (100,000 applicants)" -ForegroundColor Green
Write-Host "  Good for: cargo run --release -- --benchmark, performance testing" -ForegroundColor Gray
Write-Host "  WARNING: Use 'cargo run --release' for large datasets!" -ForegroundColor Yellow
