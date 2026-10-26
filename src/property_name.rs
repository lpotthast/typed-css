//! The name of a standard or custom property, which is static or shared.

use std::{borrow::Cow, fmt, hash, sync::Arc};

/// The name of a standard or custom property, such as `padding` or `--accent`.
///
/// A static name costs nothing to copy. A name that was created at runtime is shared, so that
/// every `var()` and every declaration of a custom property only counts a reference to the name
/// instead of copying its text.
#[derive(Clone)]
pub(crate) enum PropertyName {
    Static(&'static str),
    Shared(Arc<str>),
}

impl PropertyName {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Static(name) => name,
            Self::Shared(name) => name,
        }
    }
}

impl From<Cow<'static, str>> for PropertyName {
    fn from(name: Cow<'static, str>) -> Self {
        match name {
            Cow::Borrowed(name) => Self::Static(name),
            Cow::Owned(name) => Self::Shared(name.into()),
        }
    }
}

impl fmt::Debug for PropertyName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl PartialEq for PropertyName {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for PropertyName {}

impl hash::Hash for PropertyName {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}
