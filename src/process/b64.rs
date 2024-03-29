use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine,
};
use std::io::Read;

use crate::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("{}", encode);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decode = match format {
        Base64Format::Standard => STANDARD.decode(buf.as_bytes()),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf.as_bytes()),
    }?;

    let decoded = String::from_utf8(decode)?;
    println!("{}", decoded);
    Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_decode_not_enough_padding() {
        let encoded = "SGVsbG8gV29ybGQ=";

        let result = process_decode(encoded, Base64Format::Standard);

        assert!(result.is_err());
    }

    #[test]
    fn test_process_decode_invalid_length() {
        let encoded = "SGVsbG8gV29ybGQ";

        let result = process_decode(encoded, Base64Format::Standard);

        assert!(result.is_err());
    }

    #[test]
    fn test_process_decode_non_base64_string() {
        let encoded = "Hello World!";

        let result = process_decode(encoded, Base64Format::Standard);

        assert!(result.is_err());
    }
}
