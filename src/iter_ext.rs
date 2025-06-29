pub enum ZeroOneOrMany<T> {
    Zero,
    One(T),
    Many(Vec<T>),
}

pub trait IteratorExt<T> {
    fn zero_one_or_many(self) -> ZeroOneOrMany<T>;
}

impl<T, I> IteratorExt<T> for I
where
    I: Iterator<Item = T>,
{
    fn zero_one_or_many(mut self) -> ZeroOneOrMany<T> {
        let Some(first) = self.next() else {
            return ZeroOneOrMany::Zero;
        };
        let Some(second) = self.next() else {
            return ZeroOneOrMany::One(first);
        };

        let mut vec = vec![first, second];
        vec.extend(self);
        return ZeroOneOrMany::Many(vec);
    }
}
