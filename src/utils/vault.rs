use homedir::get_my_home;
use securestore::{KeySource, SecretsManager};

pub struct Vault;

impl Vault {
    pub fn init() -> SecretsManager {
        let password = std::env::var("CCP_VAULT_PASS").expect("ENV variable => CCP_VAULT_PASS");
        let secrets_path = get_my_home().unwrap().unwrap().as_path().join(".ccp").join("secrets.json");
        let sman = SecretsManager::load(secrets_path, KeySource::Password(password.as_str())).expect("Failed to load secrets store!");
        sman
    }
}
