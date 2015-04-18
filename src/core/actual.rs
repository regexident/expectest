
use core::{ SourceLocation, Matcher };

pub fn expect<T>(value: T) -> ActualValue<T> {
    ActualValue::new(value)
}

pub struct ActualValue<T> {
    value: T,
    location: Option<SourceLocation>,
}

impl<T> ActualValue<T> {
    fn new(value: T) -> ActualValue<T> {
        ActualValue {
            value: value,
            location: None,
        }
    }

    pub fn location(mut self, l: SourceLocation) -> Self {
        self.location = Some(l);
        self
    }

    pub fn to<M>(self, matcher: M) where M: Matcher<T> {
        let location = self.location;
        if let Err(mismatch) = matcher.matches(self.value) {
            failure(format!("\nexpected <{}>, got <{}>", matcher, mismatch), location);
        }
    }
}

pub fn failure(message: String, location: Option<SourceLocation>) {
    println!("{}", message);
    panic!("test failure");
}
