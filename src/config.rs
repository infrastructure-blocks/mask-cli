use std::env;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub tokens: String,
}

impl Config {
    // TODO: env-parse library.
    pub fn from_env() -> Result<Config, String> {
        let tokens = match env::var("MASK_TOKENS") {
            Ok(tokens_var) => tokens_var,
            Err(_) => {
                return Err(format!(
                    "required environment variable \"{}\" missing",
                    "MASK_TOKENS"
                ))
            }
        };

        Ok(Config { tokens })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod from_env {
        use super::*;

        mod mask_tokens {
            use super::*;

            #[test]
            fn test_missing() {
                assert!(Config::from_env().is_err())
            }

            #[test]
            fn test_empty_string() {
                // TODO: reset env vars.
                env::set_var("MASK_TOKENS", "");
                assert_eq!(
                    Config::from_env(),
                    Ok(Config {
                        tokens: "".to_string()
                    })
                )
            }
        }
    }
}
