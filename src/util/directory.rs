pub fn create_directory_name(name: &str) -> String {
    name.to_lowercase().replace(" ", "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_directory_name() {
        assert_eq!(create_directory_name("System Name"), "system_name");
        assert_eq!(create_directory_name("System"), "system");
        assert_eq!(create_directory_name("System Name 2"), "system_name_2");
    }
}
