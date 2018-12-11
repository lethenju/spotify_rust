use super::super::EasyAPI;
use super::super::files;

impl EasyAPI {
    /// Refreshes the access token by requesting a new one
    pub fn refresh(&mut self) -> Result<(), std::io::Error> {
        let mut refresh_token = String::new();
        let mut base_64_secret = String::new();
        match files::load_keys(&mut refresh_token, &mut base_64_secret) {
            Ok(()) => {}
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No file :(",
                ));
            }
        }
        return self
            .command
            .refresh(base_64_secret.as_str(), refresh_token.as_str());
    }
    /// Retrieves a refresh token
    /// takes the base64 of <clientid:clientsecret>
    /// and an authorization code
    pub fn retrieve_refresh_token(
        &mut self,
        base_64_secret: String,
        authorization_code: String,
    ) -> Result<String, std::io::Error> {
        let refresh_token = self
            .command
            .retrieve_refresh_token(base_64_secret.as_str(), authorization_code.as_str());
        Ok(refresh_token.unwrap())
    }
}
