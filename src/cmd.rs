/// A command that runs asynchronously and sends back an event to the application.
/// TODO: can we make the internal cases private so that we can unpack it when necessary?
pub enum Cmd<M> {
    None,
    Fn(Box<dyn FnOnce() -> M + Send>),
    Batch(Vec<Cmd<M>>),
}

impl<F, M> From<F> for Cmd<M>
where
    F: FnOnce() -> M + 'static + Send,
    M: 'static,
{
    fn from(f: F) -> Self {
        Cmd::Fn(Box::new(f))
    }
}

impl<M> Cmd<M>
where
    M: 'static,
{
    // TODO: can we make f / F non-Send?
    pub fn map<M2>(self, f: impl FnOnce(M) -> M2 + 'static + Send + Clone) -> Cmd<M2>
    where
        M2: 'static,
    {
        match self {
            Cmd::None => Cmd::None,
            Cmd::Fn(fe) => Cmd::Fn(Box::new(move || f(fe()))),
            Cmd::Batch(v) => Cmd::Batch(v.into_iter().map(|c| c.map(f.clone())).collect()),
        }
    }

    pub(crate) fn unpack(self) -> Vec<Box<dyn FnOnce() -> M + Send>> {
        fn unpack<M>(cmd: Cmd<M>, v: &mut Vec<Box<dyn FnOnce() -> M + Send>>) {
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
