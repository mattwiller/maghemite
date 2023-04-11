use super::*;

#[derive(Debug, Clone)]
pub enum DataNode {
    Object(Type, Vec<Box<DataNode>>),
    Value(Value),
}

impl DataNode {
    pub fn data_type(&self) -> Type {
        match self {
            Self::Object(data_type, _) => data_type,
            Self::Value(value) => value.data_type(),
        }
    }
}

impl PartialEq for DataNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DataNode::Object(t1, v1), DataNode::Object(t2, v2)) => t1 == t2 && v1 == v2,
            (DataNode::Value(v1), DataNode::Value(v2)) => v1 == v2,
            _ => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
