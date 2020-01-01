use std::collections::VecDeque;
use std::iter::Fuse;

/// Create a lookahead iterator from `iterable`.
pub fn lookahead<I>(iterable: I) -> Lookahead<I::IntoIter>
    where I: IntoIterator {
    Lookahead {
        iter: iterable.into_iter().fuse(),
        queue: VecDeque::new(),
    }
}

pub struct Lookahead<I: Iterator> {
	iter: Fuse<I>,
	queue: VecDeque<I::Item>,
}

impl<I: Iterator> Lookahead<I> {
    /// Return a reference to the item `n` iterations ahead without advancing the iterator.
    /// 
    /// When `n` is `0`, it is equivalent to [`Peekable::peek`].
    ///
    /// [`Peekable::peek`]: https://doc.rust-lang.org/std/iter/struct.Peekable.html#method.peek
	pub fn lookahead(&mut self, n: usize) -> Option<&I::Item> {
        let enqueued = self.queue.len();
		if n >= enqueued {
            let iter = &mut self.iter;
			let items = iter.take(n - enqueued + 1);
			self.queue.extend(items);
		}
		self.queue.get(n)
	}
}

impl<I> Iterator for Lookahead<I>
	where I: Iterator {
	type Item = I::Item;

	fn next(&mut self) -> Option<Self::Item> {
		self.queue.pop_front()
			.or_else(|| self.iter.next())
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let queued = self.queue.len();
		let (lower, upper) = self.iter.size_hint();
		(lower + queued, upper.and_then(|n| upper.map(|m| n + m)))
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let inner = [1, 2].into_iter();
        let mut iter = lookahead(inner);
        assert_eq!(iter.lookahead(0), Some(&&1));
    }

    #[test]
    fn one() {
        let inner = [1, 2].into_iter();
        let mut iter = lookahead(inner);
        assert_eq!(iter.lookahead(1), Some(&&2));
    }

    #[test]
    fn two() {
        let inner = [1, 2].into_iter();
        let mut iter = lookahead(inner);
        assert_eq!(iter.lookahead(2), None);
    }

    #[test]
    fn next() {
        let inner = [1, 2].into_iter();
        let mut iter = lookahead(inner);
        let _ = iter.next();
        assert_eq!(iter.lookahead(0), Some(&&2));
    }
}
