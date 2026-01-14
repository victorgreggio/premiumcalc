use crate::domain::{Applicant, PremiumResult};
use crate::repository::{ApplicantRepository, FormulaRepository};
use crate::services::PremiumCalculationService;
use rayon::prelude::*;
use std::error::Error;
use std::time::{Duration, Instant};

/// Application service orchestrating the premium calculation workflow
/// Follows Single Responsibility Principle - coordinates use cases
pub struct PremiumCalculationApp {
    repository: Box<dyn ApplicantRepository + Send + Sync>,
    calculator: PremiumCalculationService,
}

impl PremiumCalculationApp {
    pub fn new(
        repository: Box<dyn ApplicantRepository + Send + Sync>,
        formula_repository: Box<dyn FormulaRepository>,
    ) -> Self {
        Self {
            repository,
            calculator: PremiumCalculationService::new(formula_repository),
        }
    }

    /// Load applicants from repository
    pub fn load_applicants(&self) -> Result<Vec<Applicant>, Box<dyn Error>> {
        self.repository.load_all()
    }

    /// Calculate premiums for all applicants in parallel
    pub fn calculate_all_premiums(
        &self,
        applicants: Vec<Applicant>,
    ) -> (Vec<PremiumResult>, Duration) {
        let start = Instant::now();

        let results: Vec<PremiumResult> = applicants
            .par_iter()
            .filter_map(|applicant| self.calculator.calculate(applicant).ok())
            .collect();

        let duration = start.elapsed();
        (results, duration)
    }

    /// Calculate premium for a single applicant
    #[allow(dead_code)]
    pub fn calculate_premium(&self, applicant: &Applicant) -> Result<PremiumResult, Box<dyn Error>> {
        self.calculator.calculate(applicant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Applicant;
    use crate::repository::InMemoryFormulaRepository;
    use std::error::Error;

    struct MockRepository {
        applicants: Vec<Applicant>,
    }

    impl MockRepository {
        fn new(applicants: Vec<Applicant>) -> Self {
            Self { applicants }
        }
    }

    impl ApplicantRepository for MockRepository {
        fn load_all(&self) -> Result<Vec<Applicant>, Box<dyn Error>> {
            Ok(self.applicants.clone())
        }
    }

    fn create_test_applicant(id: u32, age: u32) -> Applicant {
        Applicant {
            id,
            name: format!("Test User {}", id),
            age,
            gender: "M".to_string(),
            smoker: false,
            occupation: "Engineer".to_string(),
            annual_income: 75000.0,
            bmi: 22.5,
            blood_pressure_sys: 120,
            blood_pressure_dia: 80,
            cholesterol: 180,
            existing_conditions: "none".to_string(),
            family_history_score: 2,
            coverage_amount: 500000.0,
            coverage_years: 20,
        }
    }

    fn create_app(applicants: Vec<Applicant>) -> PremiumCalculationApp {
        let repository = Box::new(MockRepository::new(applicants));
        let formula_repository = Box::new(InMemoryFormulaRepository::new());
        PremiumCalculationApp::new(repository, formula_repository)
    }

    #[test]
    fn test_load_applicants() {
        let applicants = vec![
            create_test_applicant(1, 30),
            create_test_applicant(2, 40),
        ];
        let app = create_app(applicants.clone());
        
        let loaded = app.load_applicants().unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].id, 1);
        assert_eq!(loaded[1].id, 2);
    }

    #[test]
    fn test_calculate_premium_single() {
        let applicants = vec![create_test_applicant(1, 30)];
        let app = create_app(applicants.clone());
        
        let result = app.calculate_premium(&applicants[0]).unwrap();
        assert_eq!(result.applicant.id, 1);
        assert!(result.final_premium > 0.0);
    }

    #[test]
    fn test_calculate_all_premiums() {
        let applicants = vec![
            create_test_applicant(1, 30),
            create_test_applicant(2, 40),
            create_test_applicant(3, 50),
        ];
        let app = create_app(applicants.clone());
        
        let (results, duration) = app.calculate_all_premiums(applicants);
        
        assert_eq!(results.len(), 3);
        assert!(duration.as_nanos() > 0);
        assert_eq!(results[0].applicant.id, 1);
        assert_eq!(results[1].applicant.id, 2);
        assert_eq!(results[2].applicant.id, 3);
    }

    #[test]
    fn test_parallel_calculation_performance() {
        let applicants: Vec<Applicant> = (1..=100)
            .map(|id| create_test_applicant(id, 30 + (id % 40)))
            .collect();
        
        let app = create_app(applicants.clone());
        
        let (results, _duration) = app.calculate_all_premiums(applicants);
        assert_eq!(results.len(), 100);
    }
}
