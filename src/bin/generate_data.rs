use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

const FIRST_NAMES_MALE: &[&str] = &[
    "James",
    "John",
    "Robert",
    "Michael",
    "William",
    "David",
    "Richard",
    "Joseph",
    "Thomas",
    "Christopher",
    "Daniel",
    "Matthew",
    "Anthony",
    "Mark",
    "Donald",
    "Steven",
    "Andrew",
    "Paul",
    "Joshua",
    "Kenneth",
    "Kevin",
    "Brian",
    "George",
    "Timothy",
    "Ronald",
    "Edward",
    "Jason",
    "Jeffrey",
    "Ryan",
    "Jacob",
];

const FIRST_NAMES_FEMALE: &[&str] = &[
    "Mary",
    "Patricia",
    "Jennifer",
    "Linda",
    "Barbara",
    "Elizabeth",
    "Susan",
    "Jessica",
    "Sarah",
    "Karen",
    "Lisa",
    "Nancy",
    "Betty",
    "Margaret",
    "Sandra",
    "Ashley",
    "Kimberly",
    "Emily",
    "Donna",
    "Michelle",
    "Carol",
    "Amanda",
    "Dorothy",
    "Melissa",
    "Deborah",
    "Stephanie",
    "Rebecca",
    "Sharon",
    "Laura",
    "Cynthia",
];

const LAST_NAMES: &[&str] = &[
    "Smith",
    "Johnson",
    "Williams",
    "Brown",
    "Jones",
    "Garcia",
    "Miller",
    "Davis",
    "Rodriguez",
    "Martinez",
    "Hernandez",
    "Lopez",
    "Gonzalez",
    "Wilson",
    "Anderson",
    "Thomas",
    "Taylor",
    "Moore",
    "Jackson",
    "Martin",
    "Lee",
    "Perez",
    "Thompson",
    "White",
    "Harris",
    "Sanchez",
    "Clark",
    "Ramirez",
    "Lewis",
    "Robinson",
    "Walker",
    "Young",
    "Allen",
    "King",
    "Wright",
    "Scott",
    "Torres",
    "Nguyen",
    "Hill",
    "Flores",
    "Green",
    "Adams",
    "Nelson",
    "Baker",
    "Hall",
    "Rivera",
    "Campbell",
    "Mitchell",
    "Carter",
    "Roberts",
];

const OCCUPATIONS: &[&str] = &[
    "Engineer",
    "Teacher",
    "Developer",
    "Manager",
    "Sales",
    "Nurse",
    "Accountant",
    "Designer",
    "Analyst",
    "Consultant",
    "Administrator",
    "Technician",
    "Specialist",
    "Coordinator",
    "Director",
    "Architect",
    "Lawyer",
    "Doctor",
    "Pharmacist",
    "Therapist",
    "Writer",
    "Artist",
    "Chef",
    "Mechanic",
    "Electrician",
    "Plumber",
    "Carpenter",
    "Driver",
    "Operator",
    "Clerk",
];

const CONDITIONS: &[&str] = &["none", "diabetes", "hypertension", "asthma", "arthritis"];

fn generate_applicant(id: u32, rng: &mut impl Rng) -> String {
    let gender = if rng.gen_bool(0.5) { "M" } else { "F" };

    let first_name = if gender == "M" {
        FIRST_NAMES_MALE[rng.gen_range(0..FIRST_NAMES_MALE.len())]
    } else {
        FIRST_NAMES_FEMALE[rng.gen_range(0..FIRST_NAMES_FEMALE.len())]
    };

    let last_name = LAST_NAMES[rng.gen_range(0..LAST_NAMES.len())];
    let name = format!("{} {}", first_name, last_name);

    // Age: weighted towards 25-55
    let age = if rng.gen_bool(0.7) {
        rng.gen_range(25..=55)
    } else {
        rng.gen_range(18..=75)
    };

    let smoker = rng.gen_bool(0.15);

    let occupation = OCCUPATIONS[rng.gen_range(0..OCCUPATIONS.len())];

    // Income based on age
    let base_income = rng.gen_range(35000..=150000);
    let annual_income = if age < 25 {
        (base_income as f64 * 0.6) as u32
    } else if age > 60 {
        (base_income as f64 * 0.7) as u32
    } else {
        base_income
    };
    let annual_income = (annual_income / 1000) * 1000;

    // BMI: normal distribution around 25
    let bmi = (rng.gen_range(17.0_f64..40.0_f64) * 10.0).round() / 10.0;

    // Blood pressure: correlated with age and BMI
    let bp_sys_base = 110.0 + (age as f64 - 18.0) * 0.5 + (bmi - 22.0) * 2.0;
    let blood_pressure_sys = (bp_sys_base + rng.gen_range(-10.0..10.0)).clamp(90.0, 180.0) as u32;
    let blood_pressure_dia =
        ((blood_pressure_sys as f64 * 0.6) + rng.gen_range(-5.0..5.0)).clamp(60.0, 110.0) as u32;

    // Cholesterol: correlated with age
    let chol_base = 160.0 + (age as f64 - 18.0) * 1.2;
    let cholesterol = (chol_base + rng.gen_range(-25.0..25.0)).clamp(130.0, 300.0) as u32;

    // Existing conditions
    let existing_conditions = if age > 60 {
        let weights = [4, 2, 2, 1, 1];
        let dist = WeightedIndex::new(weights).unwrap();
        CONDITIONS[dist.sample(rng)]
    } else if age > 45 {
        let weights = [8, 1, 1, 1, 1];
        let dist = WeightedIndex::new(weights).unwrap();
        CONDITIONS[dist.sample(rng)]
    } else {
        let weights = [16, 1, 1, 1, 1];
        let dist = WeightedIndex::new(weights).unwrap();
        CONDITIONS[dist.sample(rng)]
    };

    // Family history score
    let weights = [20, 20, 15, 12, 10, 8, 5, 2];
    let dist = WeightedIndex::new(weights).unwrap();
    let family_history_score = dist.sample(rng);

    // Coverage amount
    let coverage_amounts = [
        250000, 300000, 350000, 400000, 450000, 500000, 550000, 600000, 750000, 1000000, 1500000,
    ];
    let coverage_amount = coverage_amounts[rng.gen_range(0..coverage_amounts.len())];

    // Coverage years
    let coverage_years_options = [10, 15, 20, 25, 30];
    let coverage_years = coverage_years_options[rng.gen_range(0..coverage_years_options.len())];

    format!(
        "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
        id,
        name,
        age,
        gender,
        smoker,
        occupation,
        annual_income,
        bmi,
        blood_pressure_sys,
        blood_pressure_dia,
        cholesterol,
        existing_conditions,
        family_history_score,
        coverage_amount,
        coverage_years
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let num_records = 100_000;
    let output_file = "applicants.csv";

    println!("Generating {} applicant records...", num_records);
    let start = Instant::now();

    let mut file = File::create(output_file)?;

    // Write header
    writeln!(
        file,
        "id,name,age,gender,smoker,occupation,annual_income,bmi,blood_pressure_sys,blood_pressure_dia,cholesterol,existing_conditions,family_history_score,coverage_amount,coverage_years"
    )?;

    let mut rng = rand::thread_rng();

    for i in 1..=num_records {
        let record = generate_applicant(i, &mut rng);
        writeln!(file, "{}", record)?;

        if i % 10_000 == 0 {
            println!("  Generated {} records...", i);
        }
    }

    let elapsed = start.elapsed();
    println!(
        "✓ Complete! Generated {} records in {:.2}s",
        num_records,
        elapsed.as_secs_f64()
    );
    println!("  Output: {}", output_file);

    Ok(())
}
