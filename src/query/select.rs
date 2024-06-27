use crate::QueryBuilder;

impl<'a> QueryBuilder<'a> {
    
    pub fn select(
        mut self,
        columns: &[&str]
    ) -> Self {
        self.columns = columns
            .iter()
            .map(|&col| col.to_string())
            .collect();
        self
    }
}