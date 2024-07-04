use protobuf;

use crate::{Message, ReflectedClientResult};

#[derive()]
pub struct Method {
    descriptor: protobuf::reflect::MethodDescriptor,
    input: Message,
    output: Message,
}

impl Method {
    pub fn from_descriptor(
        descriptor: protobuf::reflect::MethodDescriptor,
    ) -> ReflectedClientResult<Self> {
        let input = Message::from_descriptor(descriptor.input_type())?;
        let output = Message::from_descriptor(descriptor.output_type())?;
        Ok(Self {
            descriptor,
            input,
            output,
        })
    }
}

impl core::fmt::Debug for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Method")
            .field("name", &self.descriptor.proto().name)
            .field("input", &self.input)
            .field("output", &self.output)
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
