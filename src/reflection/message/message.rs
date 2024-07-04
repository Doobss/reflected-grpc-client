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
        self.descriptor.fmt(f)
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
