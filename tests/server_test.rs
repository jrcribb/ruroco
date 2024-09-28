#[cfg(test)]
mod tests {
    use ruroco::config_server::ConfigServer;
    use ruroco::server::Server;
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn test_create_invalid_pid() {
        env::set_var("LISTEN_PID", "12345");

        let conf_dir_path =
            env::current_dir().unwrap_or(PathBuf::from("/tmp")).join("tests").join("conf_dir");

        let result = Server::create(ConfigServer {
            address: String::from("127.0.0.1:8082"),
            config_dir: conf_dir_path,
            ..Default::default()
        });

        assert!(result.is_ok());
    }

    #[test]
    fn test_create_from_invalid_path() {
        let path = env::current_dir()
            .unwrap_or(PathBuf::from("/tmp"))
            .join("tests")
            .join("files")
            .join("config_invalid.toml");

        let result = Server::create_from_path(path);

        assert!(result.is_err());
        assert!(result.err().unwrap().contains("TOML parse error at line 1, column 1"));
    }

    #[test]
    fn test_create_from_invalid_toml_path() {
        let result = Server::create_from_path(PathBuf::from("/tmp/path/does/not/exist"));

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            r#"Could not read "/tmp/path/does/not/exist": No such file or directory (os error 2)"#
        );
    }
}
