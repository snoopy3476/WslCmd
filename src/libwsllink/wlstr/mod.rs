/// Manages WslLink path
mod wlpath;
pub use wlpath::WLPath;

/// WslLink str
///
/// Includes submodule trait [`WLPath`], which means all [`WLPath`] can utilize WLStr functions
pub trait WLStr: WLPath + Clone {
    /// Get reference of [`WLStr`]
    fn wlstr_as_ref(&self) -> Option<&str>;

    /// Get ownership of self [`WLStr`] or create new one if ref type,
    /// then return it in the form of [`Option`]<[`String`]>
    fn wlstr_to_string(self) -> Option<String>;

    /// Do replace_all on [`WLStr`]
    fn wlstr_replace_all_regex(&self, match_pattern: &str, replace_with: &str) -> Option<String> {
        self.wlstr_as_ref().and_then(|s| {
            regex::Regex::new(match_pattern)
                .map(|re| re.replace_all(&s, replace_with).into_owned())
                .ok()
        })
    }

    /// Invoke function on [`WLStr`], for call chaining
    fn wlstr_invoke<T: WLStr, F: FnOnce(&Self) -> T>(&self, f: F) -> T {
        f(self)
    }
}

/// [`String`] implementations for [`WLStr`]
impl WLStr for String {
    fn wlstr_as_ref(&self) -> Option<&str> {
        Some(self)
    }
    fn wlstr_to_string(self) -> Option<String> {
        Some(self)
    }
}
/// [`Option`]<[`String`]> implementations for [`WLStr`]
impl WLStr for Option<String> {
    fn wlstr_as_ref(&self) -> Option<&str> {
        self.as_deref()
    }
    fn wlstr_to_string(self) -> Option<String> {
        self // return self (do nothing)
    }
}

/// &[`String`] implementations for [`WLStr`]
impl WLStr for &String {
    fn wlstr_as_ref(&self) -> Option<&str> {
        Some(self)
    }
    fn wlstr_to_string(self) -> Option<String> {
        Some(String::from(self))
    }
}
/// [`Option`]<&[`String`]> implementations for [`WLStr`]
impl WLStr for Option<&String> {
    fn wlstr_as_ref(&self) -> Option<&str> {
        self.map(String::as_str)
    }
    fn wlstr_to_string(self) -> Option<String> {
        self.map(String::from)
    }
}

/// &[`str`] implementations for [`WLStr`]
impl WLStr for &str {
    fn wlstr_as_ref(&self) -> Option<&str> {
        Some(self)
    }
    fn wlstr_to_string(self) -> Option<String> {
        Some((*self).to_string())
    }
}
/// [`Option`]<&[`str`]> implementations for [`WLStr`]
impl WLStr for Option<&str> {
    fn wlstr_as_ref(&self) -> Option<&str> {
        *self // return self (do nothing)
    }
    fn wlstr_to_string(self) -> Option<String> {
        self.map(String::from)
    }
}

/*

/// [`Option`]<&[`str`]> implementations for [`WLStr`]
impl Eq for WLStr {
}

*/
