/// A view describes data that can be built from the model's state without modifying it.
///
/// A model can support multiple views as long they are rendering different
/// types of data.
pub trait View<R> {
    fn render(&self) -> R;
}
