use serde::Serializer;

pub struct Hessian2Binary {
    data: Vec<u8>
}

impl serde::Serialize for Hessian2Binary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        todo!()
    }
}
