use std::collections::hash_map::HashMap;

// pub trait EventListennerF32 {
//     fn handleEvent(&mut self, value: f32);
// }

struct Subscriptions<T> {
    count: i16,
    pub subscriptions: HashMap<i16, Box<dyn Fn(T)>>
}

impl<T> Subscriptions<T> {
    pub fn new () -> Self {
        Self {
            count : 0,
            subscriptions : HashMap::new()
        }
    }
    pub fn add_subscription (&mut self, f: Box<dyn Fn(T)>) -> i16 {
        self.count += 1;
        let count = self.count + 1;
        self.subscriptions.insert(count, f);
        self.count = count;
        count
    }
}



pub struct ParameterF32 {
    value: f32,
    min: f32,
    max: f32,
    subscriptions: Subscriptions<f32>
}

impl ParameterF32 {
    pub fn new() -> Self {
        Self {
            value : 0.0,
            min : 0.0,
            max : 1.0,
            subscriptions : Subscriptions::new()
        }

    }
}

pub enum ParameterContent {
    F32(Box<ParameterF32>)
}

pub enum CallbackArgument {
    F32(f32)
}

pub enum CallbackSignature {
    F32(Box<dyn Fn(f32)>)
}

impl ParameterContent {
    pub fn set_value(&mut self, value: CallbackArgument) {
        match self {
            Self::F32(param) => {
                match value {
                    CallbackArgument::F32(value)=>{                        
                        param.value = value;
                        for (key, cb) in param.subscriptions.subscriptions.iter() {
                            cb(value);
                        }
                    }
                }
            }
        }
    }
    pub fn subscribe(&mut self, f: CallbackSignature) -> i16 {
        match self {
            Self::F32 (p) => {
                match f {
                    CallbackSignature::F32(f) => {
                        p.subscriptions.add_subscription(f)
                    }
                }
                
            }
        }
    }
}


pub struct Parameter {
    label: String,
    content: ParameterContent,
    id: Option<i16>,
}

impl Parameter {

    pub fn new(label: String, content: ParameterContent) -> Self {
        Self {
            label,
            id: None,
            content
        }
    }

    pub fn set_id(&mut self, id: Option<i16>) {
        self.id = id;
    }

    pub fn set_value(&mut self, v: CallbackArgument) {
        self.content.set_value(v);
    }

    pub fn register_callback(&mut self, f: CallbackSignature) -> i16 {
        self.content.subscribe(f)
    }
}

use std::cell::RefCell;
use std::rc::Rc;
type Parameters = HashMap<i16, Parameter>;


struct ParametersPool {
    count: i16,
    pub parameters: Parameters
}

impl ParametersPool {
    pub fn new () -> Self {
        Self {
            count : 0,
            parameters : HashMap::new()
        }
    }
    pub fn push (&mut self, mut p: Parameter) -> i16 {
        self.count += 1;
        let count = self.count;
        p.set_id(Some(count));
        self.parameters.insert(count, p);
        self.count = count;
        count
    }

    pub fn get_mut (&mut self, k: i16) -> Option<&mut Parameter>{
        self.parameters.get_mut(&k)
    }
}


pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


trait InnerListenner {
    fn set_value(&mut self, v: f32) {
        println!("{}", v)
    }
}

#[derive(Clone)]
struct Listenner<T: InnerListenner> {
    inner: Rc<RefCell<T>>
}


impl<T: 'static +  InnerListenner> Listenner<T> {
    pub fn new(t: T) -> Self {
        Self {
            inner : Rc::new(RefCell::new(t))
        }
    }

//Parameter::new(String::from("my P1"), 2)
    pub fn listen(&mut self, p: &mut Parameter) {
        let delegate = Rc::downgrade(self);

        let id = p.register_callback( CallbackSignature::F32 (Box::new( move |v| {
          
            match delegate.upgrade() {
                Some (listenner) => {
                    match listenner.try_borrow_mut() {
                        Ok(mut r) => {
                            r.set_value(v);
                        },
                        Err(e)=>{
                            panic!("fail to borrow rc for listenner")
                        }
                    }
                },
                None => {
                    panic!("fail to upgrade, Weak pointer of listenner, consider removing the store lambda function")
                }
            }
            
            
        })));
    }
}
use std::ops::{Deref, DerefMut};

impl<T: InnerListenner> Deref for Listenner<T> {
    type Target = Rc<RefCell<T>>;

  fn deref(&self) -> &Rc<RefCell<T>> {
      &self.inner
  }

}

impl<T: InnerListenner> DerefMut for Listenner<T> {


    fn deref_mut(&mut self) -> &mut Rc<RefCell<T>> {
        &mut self.inner
    }

}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn parameter_test() {
        let mut p = Parameters::new();

        //TODO => wrap in order to throw an error if exist, and send back the key if exist
        p.insert(
            1, 
            Parameter::new(
                String::from("my P1"), 
                ParameterContent::F32(Box::new(ParameterF32::new()))
            )
        );
        let subr =  match p.get_mut(&1) {
            Some(v) => v,
            None => panic!("bad index")
        };
        let _addr = subr.register_callback( CallbackSignature::F32(Box::new(move |v: f32|{
            assert_eq!(v, 0.7);
        })));
        
        subr.set_value( CallbackArgument::F32(0.7));
    }


    struct My {
        value: f32
    }

    impl InnerListenner for My {
        fn set_value(&mut self, v: f32) {
            self.value = v;
            assert_eq!(v, 0.7);
        }
    }

    #[test]
    fn self_struct_test() {

        let mut parameters = ParametersPool::new();
        parameters.push(
            Parameter::new(
                String::from("my P1"), 
                ParameterContent::F32(Box::new(ParameterF32::new()))
            )
        );

        let mut listenner: Listenner<My> = Listenner::new(My{value : 0.1});

        let subr =  match parameters.get_mut(1) {
            Some(v) => v,
            None => panic!("bad index")
        };

        listenner.listen(subr);
        subr.set_value(CallbackArgument::F32(0.7));

    }


}