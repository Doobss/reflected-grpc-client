use protobuf;
use std::collections::HashMap;

use crate::{Method, ReflectedClientResult};

#[derive()]
pub struct Service {
    descriptor: protobuf::reflect::ServiceDescriptor,
    methods: HashMap<String, Method>,
}

impl Service {
    pub fn from_descriptor(
        descriptor: protobuf::reflect::ServiceDescriptor,
    ) -> ReflectedClientResult<Self> {
        let method_descriptors: Vec<protobuf::reflect::MethodDescriptor> =
            descriptor.methods().collect();
        let mut methods = HashMap::with_capacity(method_descriptors.len());
        for method_descriptor in method_descriptors {
            let method_name = method_descriptor.proto().name().to_owned();
            let method = Method::from_descriptor(method_descriptor)?;
            methods.insert(method_name, method);
        }
        Ok(Self {
            descriptor,
            methods,
        })
    }
}

impl core::fmt::Debug for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Service")
            .field("name", &self.descriptor.proto().name)
            .field("methods", &self.methods)
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
