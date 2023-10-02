use std::{cmp::Reverse, collections::BinaryHeap, time::Duration, time::Instant};

#[derive(Debug, Clone)]
struct DelayItem<T> {
    data: T,
    timeout: Instant,
}

impl<T> DelayItem<T> {
    fn is_before(&self, instant: Instant) -> bool {
        self.timeout <= instant
    }
}

impl<T> PartialEq for DelayItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.timeout == other.timeout
    }
}

impl<T> Eq for DelayItem<T> {}

impl<T> PartialOrd for DelayItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.timeout.partial_cmp(&other.timeout)
    }
}

impl<T> Ord for DelayItem<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timeout.cmp(&other.timeout)
    }
}

pub struct DrainExpired<'a, T> {
    q: &'a mut DelayQueue<T>,
    t: Instant,
}

impl<'a, T> Iterator for DrainExpired<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.q.try_pop(self.t)
    }
}

#[derive(Debug)]
pub struct DelayQueue<T>(BinaryHeap<Reverse<DelayItem<T>>>);

impl<T> Default for DelayQueue<T> {
    fn default() -> Self {
        Self(BinaryHeap::new())
    }
}

impl<T> DelayQueue<T> {
    pub fn new() -> Self {
        Self::default()
    }

    fn try_pop(&mut self, t: Instant) -> Option<T> {
        if self.0.peek().map(|x| x.0.is_before(t)).unwrap_or(false) {
            self.0.pop().map(|x| x.0.data)
        } else {
            None
        }
    }

    pub fn push(&mut self, data: T, timeout: Instant) {
        self.0.push(Reverse(DelayItem { data, timeout }));
    }

    pub fn push_after(&mut self, data: T, dur: Duration) {
        self.push(data, Instant::now() + dur);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.try_pop(Instant::now())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn drain_expired(&mut self) -> DrainExpired<'_, T> {
        DrainExpired {
            q: self,
            t: Instant::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delay_queue_test() {
        let mut q = DelayQueue::new();
        q.push_after(1i32, Duration::from_millis(10));
        q.push_after(2, Duration::from_millis(20));

        assert_eq!(q.pop(), None);
        std::thread::sleep(Duration::from_millis(10));
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.pop(), None);
        std::thread::sleep(Duration::from_millis(10));
        assert_eq!(q.pop(), Some(2));

        q.push_after(4, Duration::from_millis(10));
        q.push_after(5, Duration::from_millis(10));

        std::thread::sleep(Duration::from_millis(10));
        assert_eq!(vec![4, 5], q.drain_expired().collect::<Vec<_>>());
    }
}
