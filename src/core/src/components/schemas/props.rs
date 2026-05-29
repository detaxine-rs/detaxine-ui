#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum ColorTemperature {
    Danger,
    Success,
    Warning,
    Info,
    Primary,
    Gray,
}

#[derive(Debug, Clone)]
pub struct StringVec(pub Vec<String>);

impl From<Vec<String>> for StringVec {
    fn from(v: Vec<String>) -> Self {
        Self(v)
    }
}

impl From<&[&str]> for StringVec {
    fn from(slice: &[&str]) -> Self {
        Self(slice.iter().map(|s| s.to_string()).collect())
    }
}

impl<const N: usize> From<[&str; N]> for StringVec {
    fn from(arr: [&str; N]) -> Self {
        Self(arr.iter().map(|s| s.to_string()).collect())
    }
}

impl From<Vec<&str>> for StringVec {
    fn from(v: Vec<&str>) -> Self {
        Self(v.into_iter().map(|s| s.to_string()).collect())
    }
}
