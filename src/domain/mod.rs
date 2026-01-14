use serde::Deserialize;

/// Domain model representing an insurance applicant
#[derive(Debug, Clone, Deserialize)]
pub struct Applicant {
    #[allow(dead_code)]
    pub id: u32,
    pub name: String,
    pub age: u32,
    pub gender: String,
    pub smoker: bool,
    pub occupation: String,
    pub annual_income: f64,
    pub bmi: f64,
    pub blood_pressure_sys: u32,
    pub blood_pressure_dia: u32,
    pub cholesterol: u32,
    pub existing_conditions: String,
    pub family_history_score: u32,
    pub coverage_amount: f64,
    pub coverage_years: u32,
}

impl Applicant {
    pub fn has_existing_conditions(&self) -> bool {
        self.existing_conditions != "none"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_applicant() -> Applicant {
        Applicant {
            id: 1,
            name: "John Doe".to_string(),
            age: 30,
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
    fn test_has_existing_conditions_none() {
        let applicant = create_test_applicant();
        assert!(!applicant.has_existing_conditions());
    }

    #[test]
    fn test_has_existing_conditions_diabetes() {
        let mut applicant = create_test_applicant();
        applicant.existing_conditions = "diabetes".to_string();
        assert!(applicant.has_existing_conditions());
    }

    #[test]
    fn test_annual_premium() {
        let applicant = create_test_applicant();
        let result = PremiumResult::new(
            applicant,
            250.0,
            1.2,
            1.0,
            1.0,
            1.0,
            300.0,
            0.5,
        );
        assert_eq!(result.annual_premium(), 3600.0);
    }

    #[test]
    fn test_premium_result_creation() {
        let applicant = create_test_applicant();
        let result = PremiumResult::new(
            applicant.clone(),
            250.0,
            1.2,
            1.0,
            1.0,
            1.0,
            300.0,
            0.5,
        );
        
        assert_eq!(result.base_premium, 250.0);
        assert_eq!(result.age_factor, 1.2);
        assert_eq!(result.final_premium, 300.0);
        assert_eq!(result.calculation_time_ms, 0.5);
        assert_eq!(result.applicant.id, applicant.id);
    }
}

/// Value object representing the premium calculation result
#[derive(Debug, Clone)]
pub struct PremiumResult {
    pub applicant: Applicant,
    pub base_premium: f64,
    pub age_factor: f64,
    pub health_risk_score: f64,
    pub lifestyle_multiplier: f64,
    pub occupation_factor: f64,
    pub final_premium: f64,
    pub calculation_time_ms: f64,
}

impl PremiumResult {
    pub fn new(
        applicant: Applicant,
        base_premium: f64,
        age_factor: f64,
        health_risk_score: f64,
        lifestyle_multiplier: f64,
        occupation_factor: f64,
        final_premium: f64,
        calculation_time_ms: f64,
    ) -> Self {
        Self {
            applicant,
            base_premium,
            age_factor,
            health_risk_score,
            lifestyle_multiplier,
            occupation_factor,
            final_premium,
            calculation_time_ms,
        }
    }

    #[allow(dead_code)]
    pub fn annual_premium(&self) -> f64 {
        self.final_premium * 12.0
    }
}
