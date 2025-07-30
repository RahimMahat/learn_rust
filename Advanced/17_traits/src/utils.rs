use crate::lodging::{Accommodation, Description, Hotel};

pub fn book_for_one_night<T: Accommodation>(entity: &mut T, guest: &str) {
    // entity can be any type as long as it implements Accommodation trait. trait for function parameter syntax eg. fn name(var: impl Accommodation)
    // trait bound syntax is something that requires a generic type to implement a specific trait. eg. fn name<T: Accommodation>(var: T)
    entity.book(guest, 1);
}

pub fn mix_and_match<T, U>(first: &mut T, second: &mut U, guest: &str)
where
    T: Accommodation + Description,
    U: Accommodation,
{
    // Multiple trait bounds is the example of first parameter which is requires it to make sure
    // that the type that is passes in place of first implements Accommodation & Description both traits.
    first.book(guest, 1);
    first.get_description();

    second.book(guest, 2);
}

pub fn choose_best_place_to_stay() -> impl Accommodation + Description {
    Hotel::new("The Luxe")
}
