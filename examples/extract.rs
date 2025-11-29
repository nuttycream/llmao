const schema: &str = r#"
{
  "name": "user"
  "strict": true,
  "schema": {
    "type": "object",
    "properties": {
      "username": {
        "type": "string"
      },
      "email": {
        "type": "string"
      },
    },
    "required": ["username", "email"],
    "additionalProperties": false
  }
}
"#;

fn main() {}
