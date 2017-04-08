
pub trait IteratorExtensions<T: Iterator> {
    fn memoize(self) -> MemoizedSequence<T>;
}

impl<T> IteratorExtensions<T> for T
    where T::Item: Clone,
          T: Iterator
{
    fn memoize(self) -> MemoizedSequence<T> {
        MemoizedSequence {
            cache: Vec::<T::Item>::new(),
            source: self,
        }
    }
}

pub struct MemoizedSequence<T: Iterator> {
    cache: Vec<T::Item>,
    source: T,
}

impl<T> MemoizedSequence<T>
    where T: Iterator
{
    pub fn into_iter(&mut self) -> MemoizedIteraror<T> {
        MemoizedIteraror {
            index: 0usize,
            source: self,
        }
    }
}


pub struct MemoizedIteraror<'a, T>
    where T: Iterator + 'a
{
    index: usize,
    source: &'a mut MemoizedSequence<T>,
}


impl<'a, T> Iterator for MemoizedIteraror<'a, T>
    where T: Iterator + 'a,
          T::Item: Clone
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while self.source.cache.len() <= self.index {
            let c = self.source.source.next();
            match c {
                Some(v) => self.source.cache.push(v),
                None => return c,
            }
        }
        let r = self.source.cache[self.index].clone();
        self.index += 1usize;
        Some(r)
    }
}
