
use std::fmt;
use core::Matcher;

pub struct BeSome<E> {
    expected: Option<E>,
}

pub fn be_some<E>() -> BeSome<E> {
    BeSome {
        expected: None,
    }
}

impl<E> BeSome<E> {
    pub fn value(mut self, v: E) -> BeSome<E> {
        self.expected = Some(v);
        self
    }
}

impl<A, E> Matcher<Option<A>, Option<E>> for BeSome<E>
    where
        A: PartialEq<E> + fmt::Debug,
        E: fmt::Debug {

    fn failure_message(&self, join: &'static str, actual: &Option<A>) -> String {
        if self.expected.is_none() {
            format!("expected {} be Some, got <{:?}>", join, actual)
        } else {
            format!("expected {} be equal to <{:?}>, got <{:?}>",
                join, self.expected, actual)
        }
    }

    fn matches(&self, actual: &Option<A>) -> bool {
        if let Some(ref expected) = self.expected {
            if let Some(ref a) = *actual {
                a == expected
            } else {
                false
            }
        } else {
            actual.is_some()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::Matcher;

    #[test]
    fn some_value_some_value() {
        assert!(be_some().value(5).matches(&Some(5)));
    }

    #[test]
    fn some_some_value() {
        assert!(be_some().matches(&Some(5)));
    }

    #[test]
    fn be_some_failure_message() {
        let message = be_some().failure_message("to", &None::<u8>);
        assert!(message == "expected to be Some, got <None>");
    }

    #[test]
    fn be_some_value_failure_message() {
        let message = be_some().value(1).failure_message("to", &None::<u8>);
        assert!(message == "expected to be equal to <Some(1)>, got <None>");
    }

    #[test]
    #[should_panic]
    fn some_value_none_should_panic() {
        assert!(be_some().value(5).matches(&None::<u8>));
    }

    #[test]
    #[should_panic]
    fn some_none_should_panic() {
        assert!(be_some().matches(&None::<u8>));
    }
}
