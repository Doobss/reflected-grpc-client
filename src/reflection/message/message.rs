use protobuf;

use crate::ReflectedClientResult;

#[derive(Clone)]
pub struct Message {
    descriptor: protobuf::reflect::MessageDescriptor,
}

impl Message {
    pub fn from_descriptor(
        descriptor: protobuf::reflect::MessageDescriptor,
    ) -> ReflectedClientResult<Self> {
        Ok(Self { descriptor })
    }
}

impl core::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("name", &self.descriptor.name())
            .field(
                "fields",
                &self
                    .descriptor
                    .fields()
                    .map(|field| field.name().to_owned())
                    .collect::<Vec<String>>(),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn create_builder() -> ReflectedClientResult<()> {
        Ok(())
    }
}
