pub trait ParamaterFromData<T> {
    fn make_collision_data(data: &T) -> Self;
}

impl<T> ParamaterFromData<T> for () {
    fn make_collision_data(_: &T) -> () {
        ()
    }
}
