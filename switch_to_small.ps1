# Switch to small dataset (25 applicants)
Copy-Item applicants_100k.csv applicants.csv.backup -Force -ErrorAction SilentlyContinue
Get-Content applicants_100k.csv | Select-Object -First 26 | Set-Content applicants.csv
Write-Host "✓ Switched to SMALL dataset (25 applicants)" -ForegroundColor Green
Write-Host "  Good for: cargo run (debug mode), quick testing" -ForegroundColor Gray
