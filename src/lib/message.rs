pub trait Task {
    fn name(&self)->&str;
    fn invoke(&mut self);
}

pub enum Message<Obj:Task+Send>{
    Quit,
    Exited(String),
    Invoke(Obj),
    Done(String, Obj),
    Resend(Obj),
    Nothing
}
