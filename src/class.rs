use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Hessian2Class<T> {
    class_path: String,
    data: T
}

impl<T> Hessian2Class<T> {
    pub fn new(class_path: String, data: T) -> Self {
        Hessian2Class {
            class_path,
            data,
        }
    }
}
