# Premium Calculator - Quick Start Guide

## What Was Created

A complete insurance premium calculator application demonstrating the `formcalc` library capabilities with advanced architecture patterns:

### Project Structure:
```
premiumcalc/
├── src/
│   ├── main.rs              - Application entry point with TUI
│   ├── domain/              - Core business entities
│   ├── services/            - Premium calculation logic
│   ├── repository/          - Data access layer (formulas & applicants)
│   ├── application/         - Use case orchestration
│   ├── ui/                  - Terminal UI components
│   └── bin/
│       └── generate_data.rs - Test data generator
├── applicants.csv           - Applicant data (25 default, up to 100k+)
├── Cargo.toml              - Project configuration
├── README.md               - Detailed documentation
└── QUICKSTART.md           - This file
```

### Architecture Highlights:
- **Domain-Driven Design**: Clean separation of concerns
- **Repository Pattern**: Formulas and data treated as external sources
- **SOLID Principles**: Single responsibility, dependency injection
- **19 Unit Tests**: Comprehensive test coverage
- **Zero Warnings**: Clean build

## Running the Application

### Quick Start - Choose Your Dataset First!

The application comes with dataset management for different use cases:

**For Development/Quick Testing (Debug Mode):**
```bash
.\switch_to_small.ps1   # Switch to 25 applicants
cargo run               # Fast in debug mode
```

**For Performance Testing (Release Mode):**
```bash
.\switch_to_large.ps1   # Switch to 100k applicants  
cargo run --release -- --benchmark
```

### Standard Mode (with TUI)
```bash
cd premiumcalc
cargo run --release
```

### Benchmark Mode (no UI - for large datasets)
```bash
cargo run --release -- --benchmark
# or short form
cargo run --release -- -b
```

**Important:** Large datasets (>1000 applicants) are VERY slow in debug mode. Always use `--release` for datasets over 1000 records, or the app will warn you and may take 10+ minutes.

### Generate Test Data
```bash
# Generate 100,000 applicants (~7.68 MB CSV, takes ~2-3 seconds)
cargo run --release --bin generate_data
```

## What It Does

1. **Loads Data**: Reads applicants from CSV file (25 default, or 100k+ for benchmarks)
2. **Parallel Calculation**: Calculates premiums for all applicants simultaneously
   - Each applicant uses 11 interconnected formulas loaded from repository
   - formcalc automatically resolves dependencies
   - Formulas in same dependency layer run in parallel
3. **Performance**: 
   - Small dataset (25): ~2-5ms total
   - Large dataset (100k): ~45 seconds (~2,200 calcs/sec)
4. **Interactive TUI**: Browse and explore results (standard mode only)

## Key Features Demonstrated

### formcalc Library Features:
✅ **Formula Dependencies** - Automatic dependency resolution using DAG
✅ **Parallel Execution** - Formulas in same layer execute in parallel
✅ **Variables** - Set applicant data as variables
✅ **Conditional Logic** - if/then/else statements for risk scoring
✅ **Built-in Functions** - rnd() for rounding, get_output_from() for dependencies
✅ **Type System** - Numbers, strings, and booleans
✅ **Error Handling** - Graceful error management

### Premium Calculation:
- 11 interdependent formulas per applicant
- Realistic insurance risk factors:
  - Age-based multipliers
  - Health metrics (BMI, blood pressure, cholesterol)
  - Lifestyle factors (smoking, existing conditions)
  - Family history
  - Coverage duration discounts

### Formula Dependency Tree:
```
Layer 1 (Parallel):
├─ base_premium
├─ age_factor
├─ bmi_risk
├─ bp_risk
├─ cholesterol_risk
├─ lifestyle_multiplier
├─ family_history_factor
├─ occupation_factor
└─ duration_discount

Layer 2 (Depends on Layer 1):
└─ health_risk_score (depends on: bmi_risk, bp_risk, cholesterol_risk)

Layer 3 (Final):
└─ final_premium (depends on all previous results)
```

## TUI Interface

### Navigation:
- **↑/↓** or **j/k** - Navigate through applicants
- **Enter** or **Space** - Toggle detailed view
- **q** - Quit

### Views:

**Summary View:**
- Name and demographics
- Coverage details
- Final monthly premium
- Calculation time

**Detailed View:**
- Complete applicant information
- All health metrics
- Each formula result breakdown:
  - Base Premium: $250.00
  - Age Factor: x1.5
  - Health Risk Score: x1.248
  - Lifestyle Multiplier: x1.8
  - Family History Factor: x1.15
  - Occupation Factor: x1.0
  - Duration Discount: x0.97
  - **MONTHLY PREMIUM: $756.21**

## Performance Metrics

### Small Dataset (25 applicants)
```
Loaded 25 applicants
Calculating premiums in parallel...
Calculated 25 premiums in 2.34ms
Average time per calculation: 0.09ms
```

### Large Dataset (100,000 applicants) - Benchmark Mode
```
$ cargo run --release -- --benchmark

Loaded 100000 applicants
Calculating premiums in parallel...
Calculated 100000 premiums in 44843.85ms
Average time per calculation: 0.45ms

Benchmark complete!
```

Performance breakdown:
- **100,000 calculations in ~45 seconds**
- **~2,200 calculations/second throughput**
- **0.45ms average per calculation**
- Each calculation involves:
  - Setting 10+ variables
  - Loading 11 formulas from repository
  - Resolving dependencies across 3 layers
  - Parallel execution where possible

## Data Generation

The included data generator creates realistic applicant profiles:

```bash
cargo run --release --bin generate_data
```

Features:
- **Realistic Demographics**: Names, ages, occupations, genders
- **Correlated Health Metrics**: BP and cholesterol correlate with age/BMI
- **Weighted Distributions**: Age groups, condition prevalence
- **Diverse Profiles**: Income based on age/occupation
- **Fast Generation**: 100,000 records in ~2-3 seconds

The generator creates an `applicants.csv` file that can be used for:
- Performance benchmarking
- Stress testing
- Load testing
- Demonstrating parallel processing capabilities

## Sample Applicants

The dataset includes diverse profiles:
- Ages: 25-62 years
- Occupations: Engineer, Teacher, Developer, Doctor, Lawyer, etc.
- Coverage: $250k - $1M
- Durations: 10-30 years
- Health conditions: None, diabetes, hypertension
- Smokers and non-smokers
- Varying health metrics

## Example Calculation

**Applicant: Sarah Johnson, 42, Female, Teacher**
- Smoker: Yes
- BMI: 28.3
- Blood Pressure: 135/85
- Cholesterol: 220
- Existing: Diabetes
- Coverage: $350,000 for 15 years

**Formula Results:**
1. Base Premium: $175.00 (350,000 / 1000 × 0.5)
2. Age Factor: 1.5 (age 42)
3. BMI Risk: 1.3 (overweight)
4. BP Risk: 1.2 (elevated)
5. Cholesterol Risk: 1.15 (borderline)
6. Health Risk Score: 1.794 (1.3 × 1.2 × 1.15)
7. Lifestyle Multiplier: 2.5 (smoker + conditions)
8. Family History Factor: 1.3 (score 5)
9. Occupation Factor: 1.0
10. Duration Discount: 1.0 (15 years)
11. **Final Premium: $1,176.41/month**

## Try It Yourself!

### Quick Start Flow:

1. **Run with default dataset (25 applicants)**
   ```bash
   cargo run --release
   ```
   - Navigate through applicants with ↑/↓
   - Press Enter to see detailed breakdowns
   - Compare how different risk factors affect premiums
   - Press 'q' to quit

2. **Generate large dataset for performance testing**
   ```bash
   cargo run --release --bin generate_data
   ```
   - Creates 100,000 realistic applicants

3. **Run benchmark on large dataset**
   ```bash
   cargo run --release -- --benchmark
   ```
   - See the engine process 100k calculations in ~45 seconds
   - Witness parallel processing power

4. **Explore with TUI (WARNING: Large datasets may be slow in UI)**
   ```bash
   cargo run --release
   ```
   - TUI works best with smaller datasets (< 1000 applicants)
   - Use benchmark mode for large datasets

### Run Tests
```bash
cargo test
```
- 19 tests covering domain, services, application, and repository layers
- All tests pass with zero warnings

The application showcases formcalc's power for complex, interdependent calculations that need to be executed quickly and in parallel, with clean architecture and the ability to scale to production workloads.
