use formcalc::Formula;
use std::error::Error;

/// Repository abstraction for loading premium calculation formulas
pub trait FormulaRepository: Send + Sync {
    fn load_all(&self) -> Result<Vec<Formula>, Box<dyn Error>>;
}

/// In-memory formula repository that loads formulas as if from a data source
pub struct InMemoryFormulaRepository;

impl InMemoryFormulaRepository {
    pub fn new() -> Self {
        Self
    }
}

impl Default for InMemoryFormulaRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl FormulaRepository for InMemoryFormulaRepository {
    fn load_all(&self) -> Result<Vec<Formula>, Box<dyn Error>> {
        Ok(vec![
            self.load_base_premium_formula(),
            self.load_age_factor_formula(),
            self.load_bmi_risk_formula(),
            self.load_bp_risk_formula(),
            self.load_cholesterol_risk_formula(),
            self.load_health_risk_score_formula(),
            self.load_lifestyle_multiplier_formula(),
            self.load_family_history_factor_formula(),
            self.load_occupation_factor_formula(),
            self.load_duration_discount_formula(),
            self.load_final_premium_formula(),
        ])
    }
}

impl InMemoryFormulaRepository {
    fn load_base_premium_formula(&self) -> Formula {
        Formula::new("base_premium", r#"
            return rnd((coverage_amount / 1000) * 0.5, 2)
        "#)
    }

    fn load_age_factor_formula(&self) -> Formula {
        Formula::new("age_factor", r#"
            if (age < 30) then
                return 1.0
            else if (age < 40) then
                return 1.2
            else if (age < 50) then
                return 1.5
            else if (age < 60) then
                return 2.0
            else
                return 2.8
            end
        "#)
    }

    fn load_bmi_risk_formula(&self) -> Formula {
        Formula::new("bmi_risk", r#"
            if (bmi < 18.5) then
                return 1.2
            else if (bmi < 25) then
                return 1.0
            else if (bmi < 30) then
                return 1.3
            else
                return 1.6
            end
        "#)
    }

    fn load_bp_risk_formula(&self) -> Formula {
        Formula::new("bp_risk", r#"
            if (blood_pressure_sys < 120 and blood_pressure_dia < 80) then
                return 1.0
            else if (blood_pressure_sys < 140 and blood_pressure_dia < 90) then
                return 1.2
            else
                return 1.5
            end
        "#)
    }

    fn load_cholesterol_risk_formula(&self) -> Formula {
        Formula::new("cholesterol_risk", r#"
            if (cholesterol < 200) then
                return 1.0
            else if (cholesterol < 240) then
                return 1.15
            else
                return 1.35
            end
        "#)
    }

    fn load_health_risk_score_formula(&self) -> Formula {
        Formula::new("health_risk_score", r#"
            return rnd(get_output_from('bmi_risk') * get_output_from('bp_risk') * get_output_from('cholesterol_risk'), 3)
        "#)
    }

    fn load_lifestyle_multiplier_formula(&self) -> Formula {
        Formula::new("lifestyle_multiplier", r#"
            if (smoker and has_conditions) then
                return 2.5
            else if (smoker) then
                return 1.8
            else if (has_conditions) then
                return 1.6
            else
                return 1.0
            end
        "#)
    }

    fn load_family_history_factor_formula(&self) -> Formula {
        Formula::new("family_history_factor", r#"
            if (family_history_score <= 2) then
                return 1.0
            else if (family_history_score <= 4) then
                return 1.15
            else
                return 1.3
            end
        "#)
    }

    fn load_occupation_factor_formula(&self) -> Formula {
        Formula::new("occupation_factor", "return 1.0")
    }

    fn load_duration_discount_formula(&self) -> Formula {
        Formula::new("duration_discount", r#"
            if (coverage_years >= 30) then
                return 0.95
            else if (coverage_years >= 20) then
                return 0.97
            else
                return 1.0
            end
        "#)
    }

    fn load_final_premium_formula(&self) -> Formula {
        Formula::new("final_premium", r#"
            return rnd(
                get_output_from('base_premium') * 
                get_output_from('age_factor') * 
                get_output_from('health_risk_score') * 
                get_output_from('lifestyle_multiplier') * 
                get_output_from('family_history_factor') * 
                get_output_from('occupation_factor') * 
                get_output_from('duration_discount'),
                2
            )
        "#)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use formcalc::FormulaT;

    #[test]
    fn test_load_all_formulas() {
        let repo = InMemoryFormulaRepository::new();
        let formulas = repo.load_all().unwrap();
        assert_eq!(formulas.len(), 11);
    }

    #[test]
    fn test_formula_names() {
        let repo = InMemoryFormulaRepository::new();
        let formulas = repo.load_all().unwrap();
        
        let names: Vec<&str> = formulas.iter().map(|f| f.name()).collect();
        assert!(names.contains(&"base_premium"));
        assert!(names.contains(&"age_factor"));
        assert!(names.contains(&"bmi_risk"));
        assert!(names.contains(&"bp_risk"));
        assert!(names.contains(&"cholesterol_risk"));
        assert!(names.contains(&"health_risk_score"));
        assert!(names.contains(&"lifestyle_multiplier"));
        assert!(names.contains(&"family_history_factor"));
        assert!(names.contains(&"occupation_factor"));
        assert!(names.contains(&"duration_discount"));
        assert!(names.contains(&"final_premium"));
    }
}
