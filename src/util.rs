//! clamav model — antivirus engine.

#[derive(Debug, Clone)]
pub struct Signature {
    pub name: String,
    pub type_: String,
    pub pattern: String,
}

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub file: String,
    pub infected: bool,
    pub signature: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub signatures: Vec<Signature>,
}

impl Default for Model {
    fn default() -> Self { Self { signatures: vec![
        Signature { name: "Eicar-Test-Signature".into(), type_: "hash".into(), pattern: "X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*".into() },
        Signature { name: "Test-File".into(), type_: "string".into(), pattern: "MALWARE".into() },
    ] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn scan(&self, file: &str, content: &str) -> ScanResult {
        for sig in &self.signatures {
            if content.contains(&sig.pattern) {
                return ScanResult { file: file.into(), infected: true, signature: Some(sig.name.clone()) };
            }
        }
        ScanResult { file: file.into(), infected: false, signature: None }
    }

    pub fn update(&mut self, sig: Signature) { self.signatures.push(sig); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scan_clean_infected() {
        let c = Model::new();
        let clean = c.scan("file.txt", "hello world");
        assert!(!clean.infected);
        let infected = c.scan("file.txt", "MALWARE detected");
        assert!(infected.infected);
    }
}
