pub trait ParamaterFromData<T>: Sized {
    fn make_collision_data(data: Option<&T>) -> Option<Self>;
}

impl<T> ParamaterFromData<T> for () {
    fn make_collision_data(_: Option<&T>) -> Option<()> {
        Some(())
    }
}
