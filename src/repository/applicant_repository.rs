use crate::domain::Applicant;
use std::error::Error;

/// Repository for loading applicant data
/// Follows Interface Segregation Principle and Dependency Inversion Principle
pub trait ApplicantRepository {
    fn load_all(&self) -> Result<Vec<Applicant>, Box<dyn Error>>;
}

/// CSV-based implementation of ApplicantRepository
pub struct CsvApplicantRepository {
    file_path: String,
}

impl CsvApplicantRepository {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

impl ApplicantRepository for CsvApplicantRepository {
    fn load_all(&self) -> Result<Vec<Applicant>, Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(&self.file_path)?;
        let mut applicants = Vec::new();
        
        for result in rdr.deserialize() {
            let applicant: Applicant = result?;
            applicants.push(applicant);
        }
        
        Ok(applicants)
    }
}
