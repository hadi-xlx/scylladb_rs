use crate::{
    QueryBuilder,
    Operations,
};

impl<'a> QueryBuilder<'a> {
    
    pub fn delete(mut self) -> Self {
        self.operation = Operations::Delete;
        self
    }

}