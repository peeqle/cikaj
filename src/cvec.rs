use std::fmt;
use std::ops::Deref;

pub(crate) struct CVec<T>(pub(crate) Vec<T>);
impl<T> Deref for CVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: fmt::Display> fmt::Display for CVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for (i, item) in self.0.iter().enumerate() {
            if i != 0 {
                result.push_str(", ");
            }
            result.push_str(&format!("{}", item));
        }

        write!(f, "[{}]", result)
    }
}