use dotenv::dotenv;
use llmao::{
    Provider,
    extract::{Error, ErrorKind, Extract},
};

const SCHEMA: &str = r#"
{
  "type": "json_schema",
  "name": "user",
  "strict": true,
  "schema": {
    "type": "object",
    "properties": {
      "username": {
        "type": "string"
      },
      "email": {
        "type": "string"
      }
    },
    "required": [
      "username",
      "email"
    ],
    "additionalProperties": false
  }
}
"#;

#[derive(serde::Deserialize)]
struct User {
    username: String,
    email: String,
}

#[derive(Debug)]
pub enum ProviderError {
    HttpError(ureq::Error),
    ParseError(serde_json::Error),
    NoContent,
    InvalidSchema,
}

impl std::fmt::Display for ProviderError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            ProviderError::HttpError(e) => {
                write!(f, "HTTP error: {}", e)
            }
            ProviderError::ParseError(e) => {
                write!(f, "Parse error: {}", e)
            }
            ProviderError::NoContent => {
                write!(f, "No content in response")
            }
            ProviderError::InvalidSchema => {
                write!(f, "Invalid schema")
            }
        }
    }
}

impl std::error::Error for ProviderError {}

impl Error for ProviderError {
    fn kind(&self) -> ErrorKind {
        match self {
            ProviderError::NoContent => ErrorKind::NoData,
            ProviderError::ParseError(_) => {
                ErrorKind::DeserializationFailed
            }
            ProviderError::InvalidSchema => {
                ErrorKind::BadSchema
            }
            _ => ErrorKind::NoData,
        }
    }
}

// error implementations, useful for converting
// and error propogation
impl From<ureq::Error> for ProviderError {
    fn from(e: ureq::Error) -> Self {
        ProviderError::HttpError(e)
    }
}

impl From<serde_json::Error> for ProviderError {
    fn from(e: serde_json::Error) -> Self {
        ProviderError::ParseError(e)
    }
}

pub struct OpenAI {
    api_key: String,
    base_url: String,
    model: String,
}

impl OpenAI {
    pub fn new(
        api_key: String,
        base_url: String,
        model: String,
    ) -> Self {
        Self {
            api_key,
            base_url,
            model,
        }
    }
}

// per the crate, the only thing
// a Provider trait needs, is its Error types
impl Provider for OpenAI {
    type Error = ProviderError;
}

impl Extract<User> for OpenAI {
    type Prompt = &'static str;
    type Content = &'static str;

    fn extract(
        &mut self,
        prompt: &str,
        content: &str,
    ) -> Result<User, Self::Error> {
        let schema: serde_json::Value =
            serde_json::from_str(SCHEMA)?;

        // Build the api request payload in the format of
        // the /v1/responses endpoint
        let request_body = serde_json::json!({
            "model": self.model,
            "input": [
                {
                    "role": "system",
                    "content": prompt
                },
                {
                    "role": "user",
                    "content": content
                }
            ],
            "text": {
                "format": schema
            }
        });

        //println!("request_body: {:#}", request_body);

        // create the actual HTTP request using ureq and
        // read the response as json
        let response = ureq::post(&self.base_url)
            .header(
                "Authorization",
                &format!("Bearer {}", self.api_key),
            )
            .header("Content-Type", "application/json")
            .send_json(&request_body)?
            .body_mut()
            .read_json()?;

        // converting the response into a valid serde_json Value
        let response_json: serde_json::Value = response;

        println!("{:#}", response_json);

        // extract the content from the OpenAI api response format
        // https://platform.openai.com/docs/guides/structured-outputs
        let content = response_json["output"][1]["content"]
            [0]["text"]
            .as_str()
            .ok_or(ProviderError::NoContent)?;

        println!("content:\n{:#?}", content);

        let extracted: User =
            serde_json::from_str(content)?;

        Ok(extracted)
    }
}

fn main() -> Result<(), ProviderError> {
    dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();
    let base_url = "https://api.openai.com/v1/responses";
    let model = "gpt-5-nano";

    let mut openai = OpenAI::new(
        api_key.to_owned(),
        base_url.to_owned(),
        model.to_owned(),
    );

    let user = openai.extract(
        "Extract the content into the User schema",
        "username: crustacean\nemail: llmao@email.com",
    )?;

    println!("Username: {}", user.username);
    println!("Email: {}", user.email);

    Ok(())
}
