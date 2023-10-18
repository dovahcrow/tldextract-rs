use crate::errors::Result;
use crate::TldExtractError;
use log::debug;
use serde_json::{from_reader, to_vec};
use std::collections::HashSet;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::Write;

pub fn get_tld_cache<'a, O>(cache_path: O) -> Result<HashSet<String>>
where
    O: Into<Option<&'a str>>,
{
    debug!("Trying using local cached TLD data");
    let cache_path = cache_path.into().unwrap_or(".tld_cache");
    let f = File::open(cache_path)?;
    Ok(from_reader(f)?)
}

pub fn get_tld_from_local_file<O>(
    local_file_path: O,
    private_domain: bool,
) -> Result<HashSet<String>>
where
    O: Into<Option<String>>,
{
    if let Some(local_file) = local_file_path.into() {
        debug!("Trying using local public suffix file");
        let f = read_to_string(local_file)?;
        match super::parse_public_suffix_list(&f, private_domain) {
            Ok(list) if !list.is_empty() => Ok(list),
            _ => Err(TldExtractError::Io(std::io::Error::from(
                std::io::ErrorKind::InvalidData,
            ))),
        }
    } else {
        Err(TldExtractError::Io(std::io::Error::from(
            std::io::ErrorKind::InvalidInput,
        )))
    }
}

pub fn set_tld_cache<'a, O>(cache_path: O, tld_cache: &HashSet<String>) -> Result<()>
where
    O: Into<Option<&'a str>>,
{
    let cache_path = cache_path.into().unwrap_or(".tld_cache");
    let data = to_vec(tld_cache).expect("cannot serialize tld cache");
    let mut f = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open(cache_path)?;
    f.write_all(&data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn get_tld_from_local_file_option_none() {
        let result = get_tld_from_local_file(None, false);
        assert!(result.is_err());

        let expected = std::io::ErrorKind::InvalidInput;
        let kind = match result.unwrap_err() {
            TldExtractError::Io(kind) => kind.kind(),
            _ => std::io::ErrorKind::Other,
        };
        assert_eq!(kind, expected)
    }

    #[test]
    fn get_tld_from_local_file_invalid_file() {
        let result = get_tld_from_local_file(Some("".to_string()), false);
        assert!(result.is_err());

        let expected = std::io::ErrorKind::NotFound;
        let kind = match result.unwrap_err() {
            TldExtractError::Io(kind) => kind.kind(),
            _ => std::io::ErrorKind::Other,
        };
        assert_eq!(kind, expected)
    }

    #[test]
    fn get_tld_from_local_file_empty_file() {
        let file_path: PathBuf = [
            env!("CARGO_MANIFEST_DIR"),
            "tests",
            "public_suffix_list-empty_local_file.dat",
        ]
        .iter()
        .collect();
        let result = get_tld_from_local_file(Some(file_path.display().to_string()), true);
        assert!(result.is_err());

        let expected = std::io::ErrorKind::InvalidData;
        let kind = match result.unwrap_err() {
            TldExtractError::Io(kind) => kind.kind(),
            _ => std::io::ErrorKind::Other,
        };
        assert_eq!(kind, expected)
    }

    #[test]
    fn get_tld_from_local_file_raw_public_suffix_list() {
        let file_path: PathBuf = [
            env!("CARGO_MANIFEST_DIR"),
            "tests",
            "public_suffix_list-custom_local_file.dat",
        ]
        .iter()
        .collect();
        let result = get_tld_from_local_file(Some(file_path.display().to_string()), true);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty())
    }
}
