use serde::{Deserialize,Serialize};
#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct HealthDependency {
      pub component:String,
      pub url:String,
      pub is_available:bool,
      pub description:String,
      pub availability_message:String,
      pub auth_type:String,
}

impl HealthDependency {
      pub fn example(available:bool) ->Self {
            HealthDependency {
                  component:String::from("ExampleComponent"),
                  url:String::from("https://dep-service.com"),
                  is_available: available,
                  description:String::from("Main app has dependency on this service"),
                  availability_message:String::from("Is available for testing"),
                  auth_type:String::from("LDAP"),
            }
      }
}