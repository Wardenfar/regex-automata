pub enum ZeroOneOrMany<T> {
    Zero,
    One(T),
    Many(Vec<T>),
}

pub trait IteratorExt<T> {
    fn zero_one_or_many_unique(self) -> ZeroOneOrMany<T>
    where
        T: Eq;

    fn collect_unique_vec(self) -> Vec<T>
    where
        T: Eq;
}

impl<T, I> IteratorExt<T> for I
where
    I: Iterator<Item = T>,
{
    fn zero_one_or_many_unique(mut self) -> ZeroOneOrMany<T>
    where
        T: Eq,
    {
        let Some(first) = self.next() else {
            return ZeroOneOrMany::Zero;
        };

        loop {
            let Some(second) = self.next() else {
                return ZeroOneOrMany::One(first);
            };

            if first == second {
                continue;
            }

            let mut vec = vec![first, second];

            for x in self {
                if !vec.contains(&x) {
                    vec.push(x);
                }
            }

            break ZeroOneOrMany::Many(vec);
        }
    }

    fn collect_unique_vec(self) -> Vec<T>
    where
        T: Eq,
    {
        let mut vec = Vec::new();

        for x in self {
            if !vec.contains(&x) {
                vec.push(x);
            }
        }

        return vec;
    }
}
