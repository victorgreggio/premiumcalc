# Premium Calculator

An insurance premium calculator demonstration using the `formcalc` formula engine with a terminal user interface (TUI) built with `ratatui`.

## Features

- **Realistic Premium Calculation**: Uses multiple formulas that work together to calculate insurance premiums
- **Parallel Processing**: Calculates premiums for all applicants in parallel using Rayon
- **Interactive TUI**: Browse applicants and view detailed breakdowns using a terminal UI
- **Formula Dependencies**: Demonstrates formcalc's automatic dependency resolution
- **Real-world Factors**: Considers age, health metrics, lifestyle, and coverage details

## Premium Calculation Factors

The calculator uses the following formulas to determine the monthly premium:

### 1. Base Premium
- Calculated as $0.50 per $1,000 of coverage
- Formula: `(coverage_amount / 1000) * 0.5`

### 2. Age Factor
- Under 30: 1.0x
- 30-39: 1.2x
- 40-49: 1.5x
- 50-59: 2.0x
- 60+: 2.8x

### 3. Health Risk Score
Composite of three sub-factors:

**BMI Risk:**
- Underweight (<18.5): 1.2x
- Normal (18.5-24.9): 1.0x
- Overweight (25-29.9): 1.3x
- Obese (30+): 1.6x

**Blood Pressure Risk:**
- Normal (<120/80): 1.0x
- Elevated (120-139 / 80-89): 1.2x
- High (140+ / 90+): 1.5x

**Cholesterol Risk:**
- Desirable (<200): 1.0x
- Borderline (200-239): 1.15x
- High (240+): 1.35x

### 4. Lifestyle Multiplier
- Smoker + existing conditions: 2.5x
- Smoker only: 1.8x
- Existing conditions only: 1.6x
- Neither: 1.0x

### 5. Family History Factor
- Score 0-2: 1.0x
- Score 3-4: 1.15x
- Score 5-6: 1.3x

### 6. Duration Discount
- 30+ years: 0.95x (5% discount)
- 20-29 years: 0.97x (3% discount)
- Under 20 years: 1.0x (no discount)

### Final Premium Formula
```
final_premium = base_premium × age_factor × health_risk_score × 
                lifestyle_multiplier × family_history_factor × duration_discount
```

## Dataset

The `applicants.csv` file contains sample applicants with varied profiles:

- **Demographics**: Age (18-75), Gender, Occupation
- **Lifestyle**: Smoking status, Annual income
- **Health Metrics**: BMI, Blood pressure, Cholesterol
- **Medical History**: Existing conditions, Family history score
- **Coverage**: Coverage amount ($250k-$1.5M), Coverage duration (10-30 years)

### Generating Test Data

The project includes a data generator to create large datasets for performance testing:

```bash
# Generate 100,000 applicants (default)
cargo run --release --bin generate_data

# This will:
# - Create realistic applicant profiles with correlated health metrics
# - Generate diverse demographics (ages, occupations, genders)
# - Apply weighted distributions for conditions and risk factors
# - Output to applicants.csv (~7.68 MB for 100k records)
# - Complete in ~2-3 seconds
```

The generator creates realistic data with:
- **Age Distribution**: Weighted toward working age (25-55)
- **Health Correlations**: Blood pressure and cholesterol correlate with age and BMI
- **Condition Prevalence**: Higher rates for older applicants
- **Income Variation**: Based on age and occupation
- **Diverse Names**: Randomly combined from name pools

### Managing Dataset Sizes

For convenience, helper scripts are provided to switch between datasets:

**Windows (PowerShell):**
```powershell
# Switch to small dataset (25 applicants) - good for development
.\switch_to_small.ps1

# Switch to large dataset (100k applicants) - good for benchmarking
.\switch_to_large.ps1
```

**Manual Management:**
- `applicants.csv` - Active dataset used by the application
- `applicants_100k.csv` - Pre-generated 100k dataset for benchmarks
- Use small dataset (25) for `cargo run` in debug mode
- Use large dataset (100k) for `cargo run --release -- --benchmark`

## Running the Application

### Standard Mode (with TUI)

```bash
cd premiumcalc
cargo run --release
```

The application will:
1. Load applicants from `applicants.csv`
2. Calculate all premiums in parallel
3. Display performance metrics
4. Launch the interactive TUI

**Note:** For quick development with `cargo run` (debug mode), use the small dataset (25 applicants). Debug mode with 100k records is very slow (~10-20 minutes). The app will warn you if you try to run a large dataset in debug mode.

### Benchmark Mode (no TUI)

For performance testing without the UI:

```bash
cargo run --release -- --benchmark
# or
cargo run --release -- -b
```

This mode:
- Loads applicants from CSV
- Calculates all premiums in parallel
- Displays timing statistics
- Exits without launching the TUI
- Perfect for testing with large datasets (100k+ records)

## TUI Controls

- **↑/↓** or **j/k**: Navigate through applicants
- **Enter** or **Space**: Toggle between summary and detailed view
- **q**: Quit the application

## TUI Interface

The interface shows:

### Header
- Total number of applicants processed
- Total calculation time
- Average time per calculation

### Left Panel
- List of all applicants with their calculated monthly premiums
- Highlighted selection

### Right Panel (Summary View)
- Applicant name and basic demographics
- Coverage details
- Final monthly premium
- Calculation time

### Right Panel (Expanded View)
- Complete applicant information
- All health metrics
- Detailed breakdown of each calculation factor
- Step-by-step premium calculation
- Final premium and calculation time

## Performance

The formcalc engine demonstrates:
- **Automatic Dependency Resolution**: Formulas are executed in the correct order
- **Parallel Execution**: Formulas in the same dependency layer run in parallel
- **Multi-applicant Parallelism**: All applicants are processed simultaneously using Rayon

### Benchmark Results (100,000 applicants)

```
Dataset Size:        100,000 applicants
CSV File Size:       7.68 MB
Formula Count:       11 interdependent formulas

PERFORMANCE METRICS (Release Mode):
- Total Time:        ~45 seconds (best run)
- Throughput:        ~2,200 calculations/second
- Avg per Calc:      0.45 ms

SYSTEM DETAILS:
- Parallel processing with Rayon
- Multi-core utilization
- In-memory formula repository
```

### Small Dataset Performance (25 applicants)

Expected performance with the default small dataset:
- Individual calculation: ~0.1-0.5ms per applicant
- 25 applicants in parallel: ~1-5ms total (depending on hardware)

## Architecture

### Repository Pattern

The application uses the Repository pattern for both data and formulas:

**Applicant Repository:**
- `ApplicantRepository` trait - abstraction for data access
- `CsvApplicantRepository` - loads applicants from CSV files
- Easily extensible to database, API, or other sources

**Formula Repository:**
- `FormulaRepository` trait - abstraction for formula loading
- `InMemoryFormulaRepository` - loads formulas as if from a data source
- Formulas treated as data, enabling dynamic updates without recompilation
- Can be extended to load from database, JSON files, or configuration management systems

### Domain-Driven Design

The codebase follows DDD and SOLID principles:
- **Domain Layer**: Core business entities (`Applicant`, `PremiumResult`)
- **Service Layer**: Business logic (`PremiumCalculationService`)
- **Repository Layer**: Data access abstractions
- **Application Layer**: Use case orchestration (`PremiumCalculationApp`)
- **UI Layer**: Presentation (`AppState`, rendering)

## Formula Dependencies

The calculator demonstrates formcalc's dependency management:

```
base_premium (no deps)
├─ final_premium (depends on all)

age_factor (no deps)
├─ final_premium

bmi_risk (no deps)
├─ health_risk_score
   └─ final_premium

bp_risk (no deps)
├─ health_risk_score
   └─ final_premium

cholesterol_risk (no deps)
├─ health_risk_score
   └─ final_premium

lifestyle_multiplier (no deps)
├─ final_premium

family_history_factor (no deps)
├─ final_premium

occupation_factor (no deps)
├─ final_premium

duration_discount (no deps)
├─ final_premium
```

The engine automatically:
1. Executes independent formulas in parallel (bmi_risk, bp_risk, cholesterol_risk)
2. Waits for dependencies (health_risk_score waits for its components)
3. Computes the final premium once all dependencies are ready

## Example Output

### With Small Dataset (25 applicants)
```
Loaded 25 applicants
Calculating premiums in parallel...
Calculated 25 premiums in 2.34ms
Average time per calculation: 0.09ms
```

### With Large Dataset (100,000 applicants) - Benchmark Mode
```
$ cargo run --release -- --benchmark

Loaded 100000 applicants
Calculating premiums in parallel...
Calculated 100000 premiums in 44843.85ms
Average time per calculation: 0.45ms

Benchmark complete!
```

Then the TUI launches (in standard mode) showing all applicants with their calculated premiums, allowing you to explore the detailed breakdown of each calculation.

## Testing

The project includes comprehensive unit tests covering:

- **Domain Layer** (4 tests): Entity behavior, value objects
- **Service Layer** (9 tests): Premium calculations, formula application
- **Application Layer** (4 tests): Use case orchestration, parallel processing
- **Repository Layer** (2 tests): Formula loading, repository contracts

Run tests with:
```bash
cargo test
```

All tests pass with zero warnings:
```
running 19 tests
test result: ok. 19 passed; 0 failed; 0 ignored
```
