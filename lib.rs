use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use pyo3::{basic::CompareOp, prelude::*, wrap_pyfunction, PyObjectProtocol};
use versions::Versioning;
use lazy_static::lazy_static;
use regex::Regex;


lazy_static! {
    static ref SEMVER_PATTERN: Regex = Regex::new(
        r"(?x)
        (?P<major>0|[1-9]\d*)
        (\.
        (?P<minor>0|[1-9]\d*)
        (\.
            (?P<patch>0|[1-9]\d*)
        )?
        (?:[-\.](?P<prerelease>
            (?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)
            (?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*
        ))?
        (?:-\+(?P<build>
            [0-9a-zA-Z-]+
            (?:\.[0-9a-zA-Z-]+)*
        ))?
        )?
        "
    ).unwrap();

    static ref KNOWN_PRERELEASE_STR: [&'static str; 6] = [
        "final", 
        "release", 
        "alpha", 
        "beta", 
        "rc", 
        "latest"
        ];
}


fn convert_to_semver(
    version: &str
) -> Option<Versioning> {
    
    let captured = match SEMVER_PATTERN.captures(version) {
        Some(captured) => captured,
        None => return None,
    };

    // let major = get_named_capture(&captured, "major");
    // let minor = get_named_capture(&captured, "minor");
    // let patch = get_named_capture(&captured, "patch");
    // let prerelease = get_named_capture(&captured, "prerelease");
    // let build = get_named_capture(&captured, "build");
    let start = captured.get(0).unwrap().start();
    let end = captured.get(0).unwrap().end();
    let version = version[start..end].to_string();
    
    let semver = Versioning::new(&version);
    semver
}


#[pyfunction]
fn parse_version(ver: &str) -> Option<RsVersion> {
    convert_to_semver(ver).map(|v| RsVersion { inner: v })
}

#[allow(dead_code)]
fn convert_to_num(
    version: &str
) -> u64 {
    if version.len() == 0 {
        return 0;
    }

    if let Ok(num) = version.parse::<u64>() {
        return num;
    }

    let version = version.to_string();
    let mut version = version.replace(".", "");
    if version.find("-").is_some() {
        version = version.split("-").next().unwrap().to_string();
    }

    if let Ok(num) = version.parse::<u64>() {
        return num;
    }

    0
}

// fn get_named_capture(
//     captured: &regex::Captures,
//     name: &str
// ) -> Option<String> {
//     match captured.name(name) {
//         Some(capture) => Some(capture.as_str().to_string()),
//         None => None
//     }
// }

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RsVersion {
    inner: Versioning,
}

#[pymethods]
impl RsVersion {
    fn is_ideal(&self) -> bool {
        self.inner.is_ideal()
    }

    fn is_general(&self) -> bool {
        self.inner.is_general()
    }

    fn is_complex(&self) -> bool {
        self.inner.is_complex()
    }

    fn nth(&self, n: usize) -> Option<u32> {
        self.inner.nth(n)
    }
}

#[pyproto]
impl PyObjectProtocol for RsVersion {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }

    fn __richcmp__(&self, other: RsVersion, op: CompareOp) -> PyResult<bool> {
        Ok(match op {
            CompareOp::Lt => self.inner.lt(&other.inner),
            CompareOp::Le => self.inner.le(&other.inner),
            CompareOp::Eq => self.inner.eq(&other.inner),
            CompareOp::Ne => self.inner.ne(&other.inner),
            CompareOp::Gt => self.inner.gt(&other.inner),
            CompareOp::Ge => self.inner.ge(&other.inner),
        })
    }

    fn __hash__(&self) -> PyResult<u64> {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        Ok(hasher.finish())
    }
}

#[pymodule]
fn rs_versions(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_version, m)?)?;
    m.add_class::<RsVersion>()?;

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_convert_to_num() {
        assert_eq!(convert_to_num("1"), 1);
        assert_eq!(convert_to_num("1.2"), 12);
        assert_eq!(convert_to_num("1.2.3"), 123);
        assert_eq!(convert_to_num("1.2.3-alpha"), 123);
    }

    #[test]
    fn test_convert_to_semver() {
        assert!(convert_to_semver("resin-1.2.3").is_some());
        assert!(convert_to_semver("resin-1.2.3-alpha").is_some());
        assert!(convert_to_semver("resin-1.2.3-alpha+build").is_some());
        assert!(convert_to_semver("resin-1.2.3-alpha+build.1").is_some());
        assert!(convert_to_semver("resin-1.2.3-alpha+build.1.2.3").is_some());
        assert!(convert_to_semver("resin-1.2.3-alpha+build.1.2.3-beta").is_some());

        assert!(convert_to_semver("2020.1.2").is_some());
        assert!(convert_to_semver("2020.1.2-alpha").is_some());
        
        assert!(convert_to_semver("20220202").is_some());
        assert!(convert_to_semver("20220202-alpha").is_some());
    }
}

