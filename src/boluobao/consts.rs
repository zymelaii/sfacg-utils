use phf::{phf_map, Map};

pub const AUTH: &'static str = "Basic YW5kcm9pZHVzZXI6MWEjJDUxLXl0Njk7KkFjdkBxeHE=";

pub const APPKEYS: Map<&'static str, &'static str> = phf_map! {
    "4.8.42(android;25)" => "FMLxgOdsfxmN!Dt4",
};
