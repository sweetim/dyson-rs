use super::api_error::*;
use super::model::*;
use super::util::*;

const DYSON_API_URL: &'static str = "https://appapi.cp.dyson.com";

pub struct DysonClient {
    pub user_credentials: UserCredentials,
    pub account_credentials: AccountCredentials,
    pub device_manifest: Vec<DeviceManifest>
}

impl DysonClient {
    pub async fn login(user_credentials: UserCredentials) -> Result<Self, Box<dyn std::error::Error>> {
        let mut dyson_client = DysonClient{
            user_credentials,
            account_credentials: Default::default(),
            device_manifest: vec![]
        };

        dyson_client.get_account_credentials().await?;
        dyson_client.get_devices_manifest().await?;
        Ok(dyson_client)
    }

    async fn get_account_credentials(&mut self) -> Result<AccountCredentials, Box<dyn std::error::Error>> {
        let url = format!("{}/v1/userregistration/authenticate?country={}",
                          DYSON_API_URL,
                          &self.user_credentials.country_code);

        let client = reqwest::Client::new();
        let response = client.post(&url)
            .json(&self.user_credentials)
            .send()
            .await?;

        let account_credentials: AccountCredentials = response.json().await?;
        self.account_credentials = account_credentials.clone();
        Ok(account_credentials)
    }

    async fn get_devices_manifest(&mut self) -> Result<Vec<DeviceManifest>, Box<dyn std::error::Error>> {
        let url = format!("{}/v2/provisioningservice/manifest", DYSON_API_URL);

        let device_manifest: Vec<DeviceManifest> = self.request_dyson_api_json(&url).await?;
        self.device_manifest = device_manifest.clone();
        Ok(device_manifest)
    }

    pub async fn get_device_environment_data(&self, device: &str) -> Result<EnvironmentData, Box<dyn std::error::Error>> {
        let url = format!("{}/v1/environment/devices/{}/data?language={}",
                            DYSON_API_URL,
                            device,
                            self.user_credentials.country_code);

        Ok(self.request_dyson_api_json(&url).await?)
    }

    pub async fn get_device_environment_data_help(&self, device: &str) -> Result<EnvironmentDataHelp, Box<dyn std::error::Error>> {
        let url = format!("{}/v1/environment/devices/{}/help?language={}",
                          DYSON_API_URL,
                          device,
                          self.user_credentials.country_code);

        Ok(self.request_dyson_api_json(&url).await?)
    }

    pub async fn get_device_environment_data_daily_legacy(&self, device: &str) -> Result<Vec<EnvironmentDataDaily>, Box<dyn std::error::Error>> {
        let url = format!("{}/v1/messageprocessor/devices/{}/environmentdailyhistory",
                          DYSON_API_URL,
                          device);

        Ok(self.request_dyson_api_json(&url).await?)
    }

    pub async fn get_device_environment_data_weekly_legacy(&self, device: &str) -> Result<Vec<EnvironmentDataWeekly>, Box<dyn std::error::Error>> {
        let url = format!("{}/v1/messageprocessor/devices/{}/environmentweeklyhistory",
                          DYSON_API_URL,
                          device);

        Ok(self.request_dyson_api_json(&url).await?)
    }

    pub async fn get_device_environment_data_weekly(&self, device: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/v1/messageprocessor/devices/{}/environmentdata/weekly",
                          DYSON_API_URL,
                          device);

        Ok(self.request_dyson_api_text(&url).await?)
    }

    pub async fn get_device_environment_data_daily(&self, device: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/v1/messageprocessor/devices/{}/environmentdata/daily",
                          DYSON_API_URL,
                          device);

        Ok(self.request_dyson_api_text(&url).await?)
    }

    pub async fn request_dyson_api_json<'a, T>(&self, url: &str) -> Result<T, Box<dyn std::error::Error>>
        where for<'de> T: serde::Deserialize<'de> + 'a
    {
        let client = reqwest::Client::new();
        let response = client.get(url)
            .basic_auth(&self.account_credentials.account, Some(&self.account_credentials.password))
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn request_dyson_api_text(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client.get(url)
            .basic_auth(&self.account_credentials.account, Some(&self.account_credentials.password))
            .send()
            .await?;

        Ok(response.text().await?)
    }

    fn decrypt_local_credentials(local_credentials: &str) -> Result<DecryptedLocalCredentials, DecryptCredentialsError> {
        let key = (0..0x20).map(|x| x + 1)
            .collect::<Vec<u8>>();

        let iv: [u8; 16] = [0; 16];

        let data = base64::decode(&local_credentials)?;
        let decrypted_data = decrypt(&data, key.as_slice(), &iv)?;
        let text = String::from_utf8(decrypted_data)?;
        let output: DecryptedLocalCredentials = serde_json::from_str(&text)?;

        Ok(output)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_decrypt_local_credentials() {
        let local_credentials = String::from("FpW3nD8izChaGMi60A4impGpdTtZKRc6JsMZfx1u5i2NWanM2aF4t34o9l0ScFciO1CC7EYPfjNjr/hszayQJoWB7tOArk8Y0X4gQjYaMhz+mbm+rIl+2nQimb6kxCfzLM92U7EI4Jz7hyRqkfA3coWF0zcixptQ+n5/YZeCxmuBT+CP7gfCzfe38x5VLPhb");
        let actual = DysonClient::decrypt_local_credentials(&local_credentials).unwrap();

        let mut expected = DecryptedLocalCredentials{
            serial: String::from("ABC-DE-FGH1234A"),
            access_point_password_hash: String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/=abcdefghijklmnopqrstuvw")
        };

        assert_eq!(actual, expected);
    }
}
