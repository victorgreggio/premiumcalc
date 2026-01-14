use crate::domain::{Applicant, PremiumResult};
use crate::repository::FormulaRepository;
use formcalc::{Engine, Value};
use std::error::Error;
use std::time::Instant;

/// Service responsible for calculating insurance premiums
/// Follows Single Responsibility Principle - only handles premium calculations
pub struct PremiumCalculationService {
    formula_repository: Box<dyn FormulaRepository>,
}

impl PremiumCalculationService {
    pub fn new(formula_repository: Box<dyn FormulaRepository>) -> Self {
        Self { formula_repository }
    }

    /// Calculate premium for a single applicant
    pub fn calculate(&self, applicant: &Applicant) -> Result<PremiumResult, Box<dyn Error>> {
        let start = Instant::now();

        let mut engine = Engine::new();
        self.set_variables(&mut engine, applicant);

        let formulas = self.formula_repository.load_all()?;
        engine.execute(formulas)?;

        let result = self.extract_results(&engine, applicant)?;
        let calculation_time_ms = start.elapsed().as_secs_f64() * 1000.0;

        Ok(PremiumResult::new(
            applicant.clone(),
            result.base_premium,
            result.age_factor,
            result.health_risk_score,
            result.lifestyle_multiplier,
            result.occupation_factor,
            result.final_premium,
            calculation_time_ms,
        ))
    }

    /// Set applicant data as variables in the formula engine
    fn set_variables(&self, engine: &mut Engine, applicant: &Applicant) {
        engine.set_variable("age".to_string(), Value::Number(applicant.age as f64));
        engine.set_variable(
            "gender".to_string(),
            Value::String(applicant.gender.clone()),
        );
        engine.set_variable("smoker".to_string(), Value::Bool(applicant.smoker));
        engine.set_variable("bmi".to_string(), Value::Number(applicant.bmi));
        engine.set_variable(
            "blood_pressure_sys".to_string(),
            Value::Number(applicant.blood_pressure_sys as f64),
        );
        engine.set_variable(
            "blood_pressure_dia".to_string(),
            Value::Number(applicant.blood_pressure_dia as f64),
        );
        engine.set_variable(
            "cholesterol".to_string(),
            Value::Number(applicant.cholesterol as f64),
        );
        engine.set_variable(
            "family_history_score".to_string(),
            Value::Number(applicant.family_history_score as f64),
        );
        engine.set_variable(
            "coverage_amount".to_string(),
            Value::Number(applicant.coverage_amount),
        );
        engine.set_variable(
            "coverage_years".to_string(),
            Value::Number(applicant.coverage_years as f64),
        );
        engine.set_variable(
            "annual_income".to_string(),
            Value::Number(applicant.annual_income),
        );
        engine.set_variable(
            "has_conditions".to_string(),
            Value::Bool(applicant.has_existing_conditions()),
        );
    }

    /// Extract calculation results from the engine
    fn extract_results(
        &self,
        engine: &Engine,
        _applicant: &Applicant,
    ) -> Result<CalculationResults, Box<dyn Error>> {
        Ok(CalculationResults {
            base_premium: self.extract_number(engine, "base_premium")?,
            age_factor: self.extract_number(engine, "age_factor")?,
            health_risk_score: self.extract_number(engine, "health_risk_score")?,
            lifestyle_multiplier: self.extract_number(engine, "lifestyle_multiplier")?,
            occupation_factor: self.extract_number(engine, "occupation_factor")?,
            final_premium: self.extract_number(engine, "final_premium")?,
        })
    }

    fn extract_number(&self, engine: &Engine, name: &str) -> Result<f64, Box<dyn Error>> {
        match engine
            .get_result(name)
            .ok_or(format!("{} not found", name))?
        {
            Value::Number(n) => Ok(n),
            _ => Err(format!("{} is not a number", name).into()),
        }
    }
}

/// Internal struct for holding calculation results
struct CalculationResults {
    base_premium: f64,
    age_factor: f64,
    health_risk_score: f64,
    lifestyle_multiplier: f64,
    occupation_factor: f64,
    final_premium: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::InMemoryFormulaRepository;

    fn create_test_applicant() -> Applicant {
        Applicant {
            id: 1,
            name: "John Doe".to_string(),
            age: 35,
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

    #[test]
    fn test_calculate_premium_healthy_applicant() {
        let formula_repo = Box::new(InMemoryFormulaRepository::new());
        let service = PremiumCalculationService::new(formula_repo);
        let applicant = create_test_applicant();

        let result = service.calculate(&applicant);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.applicant.id, 1);
        assert!(result.base_premium > 0.0);
        assert!(result.final_premium > 0.0);
        assert!(result.calculation_time_ms >= 0.0);
    }

    #[test]
    fn test_calculate_premium_smoker() {
        let formula_repo = Box::new(InMemoryFormulaRepository::new());
        let service = PremiumCalculationService::new(formula_repo);
        let mut applicant = create_test_applicant();
        applicant.smoker = true;

        let result = service.calculate(&applicant).unwrap();
        assert!(result.lifestyle_multiplier >= 1.8);
    }

    #[test]
    fn test_calculate_premium_with_conditions() {
        let formula_repo = Box::new(InMemoryFormulaRepository::new());
        let service = PremiumCalculationService::new(formula_repo);
        let mut applicant = create_test_applicant();
        applicant.existing_conditions = "diabetes".to_string();

        let result = service.calculate(&applicant).unwrap();
        assert!(result.lifestyle_multiplier >= 1.6);
    }

    #[test]
    fn test_calculate_premium_age_factors() {
        let formula_repo = Box::new(InMemoryFormulaRepository::new());
        let service = PremiumCalculationService::new(formula_repo);

        let mut young = create_test_applicant();
        young.age = 25;
        let young_result = service.calculate(&young).unwrap();
        assert_eq!(young_result.age_factor, 1.0);

        let mut middle = create_test_applicant();
        middle.age = 45;
        let middle_result = service.calculate(&middle).unwrap();
        assert_eq!(middle_result.age_factor, 1.5);

        let mut senior = create_test_applicant();
        senior.age = 65;
        let senior_result = service.calculate(&senior).unwrap();
        assert_eq!(senior_result.age_factor, 2.8);
    }

    #[test]
    fn test_base_premium_calculation() {
        let formula_repo = Box::new(InMemoryFormulaRepository::new());
        let service = PremiumCalculationService::new(formula_repo);
        let applicant = create_test_applicant();

        let result = service.calculate(&applicant).unwrap();
        let expected = (500000.0 / 1000.0) * 0.5;
        assert_eq!(result.base_premium, expected);
    }

    #[test]
    fn test_health_risk_score_healthy() {
        let formula_repo = Box::new(InMemoryFormulaRepository::new());
        let service = PremiumCalculationService::new(formula_repo);
        let mut applicant = create_test_applicant();
        // Set to optimal blood pressure
        applicant.blood_pressure_sys = 110;
        applicant.blood_pressure_dia = 70;

        let result = service.calculate(&applicant).unwrap();
        // Healthy applicant should have health_risk_score of 1.0
        assert_eq!(result.health_risk_score, 1.0);
    }

    #[test]
    fn test_high_bmi_increases_premium() {
        let formula_repo = Box::new(InMemoryFormulaRepository::new());
        let service = PremiumCalculationService::new(formula_repo);

        let mut normal = create_test_applicant();
        normal.bmi = 23.0;
        let normal_result = service.calculate(&normal).unwrap();

        let mut obese = create_test_applicant();
        obese.bmi = 32.0;
        let obese_result = service.calculate(&obese).unwrap();

        assert!(obese_result.final_premium > normal_result.final_premium);
    }

    #[test]
    fn test_high_cholesterol_increases_premium() {
        let formula_repo = Box::new(InMemoryFormulaRepository::new());
        let service = PremiumCalculationService::new(formula_repo);

        let mut normal = create_test_applicant();
        normal.cholesterol = 190;
        let normal_result = service.calculate(&normal).unwrap();

        let mut high = create_test_applicant();
        high.cholesterol = 250;
        let high_result = service.calculate(&high).unwrap();

        assert!(high_result.health_risk_score > normal_result.health_risk_score);
    }

    #[test]
    fn test_duration_discount() {
        let formula_repo = Box::new(InMemoryFormulaRepository::new());
        let service = PremiumCalculationService::new(formula_repo);

        let mut short_term = create_test_applicant();
        short_term.coverage_years = 10;
        let short_result = service.calculate(&short_term).unwrap();

        let mut long_term = create_test_applicant();
        long_term.coverage_years = 30;
        let long_result = service.calculate(&long_term).unwrap();

        // 30-year coverage gets 5% discount
        assert!(long_result.final_premium < short_result.final_premium);
    }
}
