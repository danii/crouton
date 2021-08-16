#[derive(Debug, Default, Clone)]
pub struct RepoStatus {
    pub deleted: usize,
    pub renamed: usize,
    pub modified: usize,
    pub staged: usize,
}

impl RepoStatus {
    pub fn is_deleted(status: &str) -> bool {
        status.starts_with("1 .D") || status.starts_with("1 D")
    }

    pub fn is_renamed(status: &str) -> bool {
        status.starts_with("2 ")
    }

    pub fn is_modified(status: &str) -> bool {
        status.starts_with("1 .M") || status.starts_with("1 .A")
    }

    pub fn is_staged(status: &str) -> bool {
        status.starts_with("1 M") || status.starts_with("1 A")
    }

    pub fn add(&mut self, s: &str) {
        self.deleted += Self::is_deleted(s) as usize;
        self.renamed += Self::is_renamed(s) as usize;
        self.modified += Self::is_modified(s) as usize;
        self.staged += Self::is_staged(s) as usize;
    }
}
