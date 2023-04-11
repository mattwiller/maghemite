use crate::evaluation::EvaluationError;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Collection(Vec<Value>);

impl Collection {
    pub fn new() -> Self {
        Collection(Vec::new())
    }

    pub fn singleton(&self, t: Type) -> Result<&Value, EvaluationError> {
        if self.0.len() != 1 {
            return Err(EvaluationError::ExpectedSingleton(t));
        } else if self.0.get(0).unwrap().data_type() != t {
            return Err(EvaluationError::ExpectedSingleton(t));
        }
        Ok(self.0.get(0).unwrap())
    }
}

impl std::ops::Deref for Collection {
    type Target = Vec<Value>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Collection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<Value> for Collection {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        let items = iter.into_iter().collect();
        Collection(items)
    }
}

impl From<Value> for Collection {
    fn from(value: Value) -> Self {
        Collection(vec![value])
    }
}
