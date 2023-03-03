use rand::Rng;

pub fn rand_range<T>(min: T, max: T) -> T
where
    T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform + Default,
{
    if min >= max {
        T::default()
    } else {
        rand::thread_rng().gen_range(min..max)
    }
}
