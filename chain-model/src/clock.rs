use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub enum Clock {
    Real,
    Fake(Rc<RefCell<Instant>>),
}

impl Clock {
    /// New Fake.
    pub fn fake_new() -> Self {
        Clock::Fake(Rc::new(RefCell::new(Instant::now())))
    }
    /// Returns now.
    pub fn now(&self) -> Instant {
        match self {
            Clock::Real => Instant::now(),
            Clock::Fake(current_time) => *current_time.borrow(),
        }
    }

    /// Advances the time for the Fake clock; no-op for the Real clock
    pub fn advance(&mut self, duration: Duration) {
        if let Clock::Fake(ref current_time) = self {
            let mut time = current_time.borrow_mut();
            *time += duration;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manual_time_control() {
        let mut clock = Clock::fake_new();
        let start_point = clock.now();

        // Advance time by 399ms.
        clock.advance(Duration::from_millis(399));

        assert_eq!(
            clock.now().duration_since(start_point),
            Duration::from_millis(399)
        );

        // Advance time for 1 minute more.
        clock.advance(Duration::from_millis(1));

        assert_eq!(
            clock.now().duration_since(start_point),
            Duration::from_millis(400)
        );
    }
}
