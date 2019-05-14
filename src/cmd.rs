/// A command to run asynchronously that feeds back an event to the component from where it was invoked.
/// TODO: can we make the internal cases private so that we can unpack it when necessary?
pub enum Cmd<E> {
    None,
    Fn(Box<dyn Fn() -> E + Send>),
    Batch(Vec<Cmd<E>>),
}

impl<F, E> From<F> for Cmd<E>
where
    F: Fn() -> E + 'static + Send,
    E: 'static,
{
    fn from(f: F) -> Self {
        Cmd::Fn(Box::new(f))
    }
}

impl<E> Cmd<E>
where
    E: 'static,
{
    // TODO: can we make f / F non-Send?
    pub fn map<'a, F, E2>(self, f: F) -> Cmd<E2>
    where
        F: Fn(E) -> E2 + 'static + Send + Clone,
        E2: 'static,
    {
        match self {
            Cmd::None => Cmd::None,
            Cmd::Fn(fe) => Cmd::Fn(Box::new(move || f(fe()))),
            Cmd::Batch(v) => Cmd::Batch(v.into_iter().map(|c| c.map(f.clone())).collect()),
        }
    }

    pub(crate) fn unpack(self) -> Vec<Box<dyn Fn() -> E + Send>> {
        fn unpack<E>(cmd: Cmd<E>, v: &mut Vec<Box<dyn Fn() -> E + Send>>) {
            match cmd {
                Cmd::None => {}
                Cmd::Fn(fe) => v.push(fe),
                Cmd::Batch(batch) => batch.into_iter().for_each(|cmd| unpack(cmd, v)),
            }
        }

        let mut v = Vec::new();
        unpack(self, &mut v);
        v
    }
}

#[test]
fn test_simple_cmd_mapping_syntax() {
    let cmd1: Cmd<i32> = Cmd::from(|| 10);
    let _cmd2: Cmd<f32> = cmd1.map(|e| e as f32);
}
