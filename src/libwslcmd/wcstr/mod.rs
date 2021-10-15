/// Manages WslCmd path
mod wcpath;
pub use wcpath::WCPath;

/// WslCmd str
pub trait WCStr: WCPath + Clone {
    /// Get reference of [`WCStr`]
    fn wcstr_as_ref(&self) -> Option<&str>;

    /// Get ownership of self [`WCStr`] or create new one if ref type,
    /// then return it in the form of [`Option`]<[`String`]>
    fn wcstr_to_string(self) -> Option<String>;

    /// Clone new [`WCStr`] from self ref,
    /// then return it in form of [`String`]
    fn wcstr_clone_to_string(&self) -> Option<String> {
        self.clone().wcstr_to_string()
    }

    /// Do replace_all on [`WCStr`]
    fn wcstr_replace_all_regex(&self, match_pattern: &str, replace_with: &str) -> Option<String> {
        self.wcstr_as_ref().and_then(|s| {
            regex::Regex::new(match_pattern)
                .map(|re| re.replace_all(&s, replace_with).into_owned())
                .ok()
        })
    }

    /// Invoke function on [`WCStr`], for call chaining
    fn wcstr_invoke<T: WCStr, F: FnOnce(&Self) -> T>(&self, f: F) -> T {
        f(self)
    }
}

/// [`String`] implementations for [`WCStr`]
impl WCStr for String {
    fn wcstr_as_ref(&self) -> Option<&str> {
        Some(self)
    }
    fn wcstr_to_string(self) -> Option<String> {
        Some(self)
    }
}
/// [`Option`]<[`String`]> implementations for [`WCStr`]
impl WCStr for Option<String> {
    fn wcstr_as_ref(&self) -> Option<&str> {
        self.as_deref()
    }
    fn wcstr_to_string(self) -> Option<String> {
        self // return self (do nothing)
    }
}

/// &[`String`] implementations for [`WCStr`]
impl WCStr for &String {
    fn wcstr_as_ref(&self) -> Option<&str> {
        Some(self)
    }
    fn wcstr_to_string(self) -> Option<String> {
        Some(String::from(self))
    }
}
/// [`Option`]<&[`String`]> implementations for [`WCStr`]
impl WCStr for Option<&String> {
    fn wcstr_as_ref(&self) -> Option<&str> {
        self.map(String::as_str)
    }
    fn wcstr_to_string(self) -> Option<String> {
        self.map(String::from)
    }
}

/// &[`str`] implementations for [`WCStr`]
impl WCStr for &str {
    fn wcstr_as_ref(&self) -> Option<&str> {
        Some(self)
    }
    fn wcstr_to_string(self) -> Option<String> {
        Some((*self).to_string())
    }
}
/// [`Option`]<&[`str`]> implementations for [`WCStr`]
impl WCStr for Option<&str> {
    fn wcstr_as_ref(&self) -> Option<&str> {
        *self // return self (do nothing)
    }
    fn wcstr_to_string(self) -> Option<String> {
        self.map(String::from)
    }
}
