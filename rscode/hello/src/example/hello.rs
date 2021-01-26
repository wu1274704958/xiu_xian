use gdnative::prelude::*;
use gdnative::api::Sprite;
#[derive(NativeClass)]
#[inherit(Sprite)]
pub struct HelloWorld{
    rad:f64
}

#[methods]
impl HelloWorld {
    fn new(_owner: &Sprite) -> Self {
        HelloWorld{
            rad : 0.0
        }
    }

    #[export]
    fn _ready(&self, _owner: &Sprite) {
        godot_print!("hello, i am rust!!!");
    }
    #[export]
    fn _process(&mut self,_owner:&Sprite,delta:f64)
    {
        self.rad += delta;
        _owner.set_rotation(self.rad);
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<HelloWorld>();
}

godot_init!(init);