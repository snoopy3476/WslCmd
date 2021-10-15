// crate
use super::WCPathBase;

// traits

/// Trait for extending WCStr
pub trait WCStr: WCStrBase + Clone {}
// types for the trait (excluding from ancestors)
impl WCStr for String {}
impl WCStr for Option<String> {}
impl WCStr for &String {}
impl WCStr for Option<&String> {}
impl WCStr for &str {}
impl WCStr for Option<&str> {}

/// WslCmd str
pub trait WCStrBase: Clone {
    /// Get reference of [`WCStrBase`]
    fn wcstr_as_str(&self) -> Option<&str>;

    /// Get ownership of self [`WCStrBase`] or create new one if ref type,
    /// then return it in the form of [`Option`]<[`String`]>
    fn wcstr_to_string(self) -> Option<String>;

    /// Clone new [`WCStrBase`] from self ref,
    /// then return it in form of [`String`]
    fn wcstr_clone_to_string(&self) -> Option<String> {
        self.clone().wcstr_to_string()
    }

    /// Do replace_all on [`WCStrBase`]
    fn wcstr_replace_all_regex(&self, match_pattern: &str, replace_with: &str) -> Option<String> {
        self.wcstr_as_str().and_then(|s| {
            regex::Regex::new(match_pattern)
                .map(|re| re.replace_all(&s, replace_with).into_owned())
                .ok()
        })
    }

    /// Invoke function on [`WCStrBase`], for call chaining
    fn wcstr_invoke<T: WCStrBase, F: FnOnce(&Self) -> T>(&self, f: F) -> T {
        f(self)
    }
}

/// [`String`] implementations for [`WCStrBase`]
impl WCStrBase for String {
    fn wcstr_as_str(&self) -> Option<&str> {
        Some(self)
    }
    fn wcstr_to_string(self) -> Option<String> {
        Some(self)
    }
}
/// [`Option`]<[`String`]> implementations for [`WCStrBase`]
impl WCStrBase for Option<String> {
    fn wcstr_as_str(&self) -> Option<&str> {
        self.as_deref()
    }
    fn wcstr_to_string(self) -> Option<String> {
        self // return self (do nothing)
    }
}

/// &[`String`] implementations for [`WCStrBase`]
impl WCStrBase for &String {
    fn wcstr_as_str(&self) -> Option<&str> {
        Some(self)
    }
    fn wcstr_to_string(self) -> Option<String> {
        Some(String::from(self))
    }
}
/// [`Option`]<&[`String`]> implementations for [`WCStrBase`]
impl WCStrBase for Option<&String> {
    fn wcstr_as_str(&self) -> Option<&str> {
        self.map(String::as_str)
    }
    fn wcstr_to_string(self) -> Option<String> {
        self.map(String::from)
    }
}

/// &[`str`] implementations for [`WCStrBase`]
impl WCStrBase for &str {
    fn wcstr_as_str(&self) -> Option<&str> {
        Some(self)
    }
    fn wcstr_to_string(self) -> Option<String> {
        Some((*self).to_string())
    }
}
/// [`Option`]<&[`str`]> implementations for [`WCStrBase`]
impl WCStrBase for Option<&str> {
    fn wcstr_as_str(&self) -> Option<&str> {
        *self // return self (do nothing)
    }
    fn wcstr_to_string(self) -> Option<String> {
        self.map(String::from)
    }
}
