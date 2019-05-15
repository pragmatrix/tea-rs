/// A view can be attached to any component. A component can
/// support multiple views as long they are rendering a different
/// type of data.
pub trait View<R> {
    fn render(&self) -> R;
}
