use serde::Serializer;

#[allow(dead_code)]
pub struct Binary {
    data: Vec<u8>,
}

impl serde::Serialize for Binary {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}
