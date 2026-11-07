//! clamav model — antivirus toolkit.

#[derive(Debug, Clone)]
pub struct Signature {
    pub name: String,
    pub type_: String,
    pub severity: u32,
}

#[derive(Debug, Clone)]
pub struct Scan {
    pub path: String,
    pub status: String,
    pub threats: Vec<Signature>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub signatures: Vec<Signature>,
    pub scans: Vec<Scan>,
}

impl Default for Model {
    fn default() -> Self { Self { signatures: vec![], scans: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_signature(&mut self, name: &str, type_: &str, severity: u32) {
        self.signatures.push(Signature { name: name.into(), type_: type_.into(), severity });
    }

    pub fn scan(&mut self, path: &str) -> usize {
        let id = self.scans.len();
        self.scans.push(Scan { path: path.into(), status: "scanned".into(), threats: vec![] });
        id
    }

    pub fn add_threat(&mut self, scan_id: usize, name: &str, type_: &str, severity: u32) -> bool {
        if let Some(s) = self.scans.get_mut(scan_id) {
            s.threats.push(Signature { name: name.into(), type_: type_.into(), severity });
            true
        } else { false }
    }

    pub fn infected(&self, scan_id: usize) -> bool {
        self.scans.get(scan_id).map(|s| !s.threats.is_empty()).unwrap_or(false)
    }

    pub fn list_scans(&self) -> &[Scan] { &self.scans }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scan_threat() {
        let mut c = Model::new();
        c.add_signature("Eicar-Test", "Test", 0);
        let id = c.scan("/tmp/test.txt");
        assert!(c.add_threat(id, "Eicar-Test", "Test", 0));
        assert!(c.infected(id));
    }
}
