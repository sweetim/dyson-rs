#[derive(Debug)]
pub enum DecryptCredentialsError {
    Crypto(crypto::symmetriccipher::SymmetricCipherError),
    Base64Decoding(base64::DecodeError),
    UtfEncoding(std::string::FromUtf8Error),
    Json(serde_json::error::Error)
}

impl From<crypto::symmetriccipher::SymmetricCipherError> for DecryptCredentialsError {
    fn from(err: crypto::symmetriccipher::SymmetricCipherError) -> DecryptCredentialsError {
        DecryptCredentialsError::Crypto(err)
    }
}

impl From<std::string::FromUtf8Error> for DecryptCredentialsError {
    fn from(err: std::string::FromUtf8Error) -> DecryptCredentialsError {
        DecryptCredentialsError::UtfEncoding(err)
    }
}

impl From<serde_json::error::Error> for DecryptCredentialsError {
    fn from(err: serde_json::error::Error) -> DecryptCredentialsError {
        DecryptCredentialsError::Json(err)
    }
}

impl From<base64::DecodeError> for DecryptCredentialsError {
    fn from(err: base64::DecodeError) -> DecryptCredentialsError {
        DecryptCredentialsError::Base64Decoding(err)
    }
}
