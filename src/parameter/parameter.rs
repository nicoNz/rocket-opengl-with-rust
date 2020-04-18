use std::collections::hash_map::HashMap;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::ops::{Deref, DerefMut};

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

pub struct ParameterVec3 {
    value: glm::Vec3,
    min: glm::Vec3,
    max: glm::Vec3,
    subscriptions: Subscriptions<glm::Vec3>
}

impl ParameterVec3 {
    pub fn new() -> Self {
        Self {
            value : glm::vec3(0., 0., 0.),
            min : glm::vec3(0., 0., 0.),
            max : glm::vec3(1., 1., 1.),
            subscriptions : Subscriptions::new()
        }
    }
}

pub enum ParameterContent {
    F32(ParameterF32),
    Vec3(ParameterVec3)
}

pub enum CallbackArgument {
    F32(f32),
    Vec3(glm::Vec3)
}

pub enum CallbackSignature {
    F32(Box<dyn Fn(f32)>),
    Vec3(Box<dyn Fn(glm::Vec3)>)
}

impl ParameterContent {
    pub fn set_value(&mut self, value: CallbackArgument) {
        println!("set v 2");
        match self {
            Self::F32(param) => {
                println!("dispatching float");
                match value {
                    
                    CallbackArgument::F32(value)=>{       
                        param.value = value;                 
                        //param.value = value;
                        for (key, cb) in param.subscriptions.subscriptions.iter() {
                            println!("tak");
                            cb(value);
                        }
                    },
                    _ => {
                        println!("badType not a float");
                    }
                }
            }
            Self::Vec3(param) => {
                println!("dispatching vec");
                match value {
                    CallbackArgument::Vec3(value)=>{                        
                        param.value = value;    
                        for (key, cb) in param.subscriptions.subscriptions.iter() {
                            cb(value);
                            println!("tok");
                        }
                    },
                    _ => {
                        println!("badType not a vec");
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
                    },
                    _ => {-1}
                }
            },
            Self::Vec3 (p) => {
                match f {
                    CallbackSignature::Vec3(f) => {
                        p.subscriptions.add_subscription(f)
                    },
                    _ => {-1}
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
        println!("set value");
        self.content.set_value(v);
    }

    pub fn register_callback(&mut self, f: CallbackSignature) -> i16 {
        self.content.subscribe(f)
    }
}


type Parameters = HashMap<i16, Rc<RefCell<Parameter>>>;


struct ParametersPool {
    count: i16,
    pub parameters: RefCell<Parameters>
}

impl ParametersPool {
    pub fn new () -> Self {
        Self {
            count : 0,
            parameters :  RefCell::new (HashMap::new())
        }
    }
    pub fn push (&mut self, mut p: Parameter) -> i16 {
        self.count += 1;
        let count = self.count;
        p.set_id(Some(count));
        self.parameters.get_mut().insert(count, Rc::new(RefCell::new(p)));
        self.count = count;
        count
    }

    pub fn get_mut (&mut self, k: i16) -> Option<Weak<RefCell<Parameter>>> {
        let v = self.parameters.get_mut().get(&k);
        match v {
            Some(v) => {
                Some(Rc::downgrade(v))
            },
            None => None
        }
    }
}


pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


pub trait InnerListenner {
    fn set_value(&mut self, v: CallbackArgument);
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
        println!("creating a function");

        match &p.content {
            ParameterContent::F32(v) => {
                let id = p.register_callback( CallbackSignature::F32 (Box::new( move |v| {
                    println!("cb is being called");
                    match delegate.upgrade() {
                        Some (listenner) => {
                            match listenner.try_borrow_mut() {
                                Ok(mut listenner) => {
                                    listenner.set_value(CallbackArgument::F32(v));
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
            ParameterContent::Vec3(v) => {
                let id = p.register_callback( CallbackSignature::Vec3 (Box::new( move |v| {
                    println!("cb is being called");
                    match delegate.upgrade() {
                        Some (listenner) => {
                            match listenner.try_borrow_mut() {
                                Ok(mut listenner) => {
                                    listenner.set_value(CallbackArgument::Vec3(v));
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

    }
}

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



struct FloatValue {
    value: f32
}

impl InnerListenner for FloatValue {
    fn set_value(&mut self, v: CallbackArgument) {
        match v {
            CallbackArgument::F32(v) => {
                self.value = v;
            },
            _ => {}
        }
    }
}


struct Vec3Value {
    value: glm::Vec3
}

impl InnerListenner for Vec3Value {
    fn set_value(&mut self, v: CallbackArgument) {
        match v {
            CallbackArgument::Vec3(v) => {
                println!("{:?}", v);
                self.value = v;
                
            },
            _ => {}
        }
    }
}

fn get_borrow(parameters: RefCell<ParametersPool>, i: i16) -> std::rc::Weak<RefCell<Parameter>> {
    match parameters.borrow_mut().get_mut(i) {
        Some(v) => v,
        None => panic!("bad index")
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn self_struct_test() {
        
        // 1 - create parameters pool
        let parameters: Rc<RefCell<ParametersPool>> = Rc::new( RefCell::new(ParametersPool::new())) ;

        // 2 - create some params ton listen to
        parameters.borrow_mut().push(
            Parameter::new(
                String::from("my P1"), 
                ParameterContent::F32(ParameterF32::new())
            )
        );

        parameters.borrow_mut().push(
            Parameter::new(
                String::from("my P2"), 
                ParameterContent::Vec3(ParameterVec3::new())
            )
        );
        
        // 3 - get some references to the created paramters
        let subr1 = match parameters.borrow_mut().get_mut(1) {
            Some(v) => v,
            None => panic!("bad index")
        };

        let subr2 = match parameters.borrow_mut().get_mut(2) {
            Some(v) => v,
            None => panic!("bad index")
        };
        
        // 4 - create some structure that can receive updates from the parameters
        let mut listenner1: Listenner<FloatValue> = Listenner::new(FloatValue{value : 0.1});
        let mut listenner2: Listenner<Vec3Value> = Listenner::new(Vec3Value{value : glm::vec3(0.2, 0.1, 0.5)});
        

        // 5 - bind the receiver to the parameter
        match Weak::upgrade(&subr1) {
            Some(subr)=> {
                listenner1.listen(&mut subr.borrow_mut());
            },
            None => {
                println!("fail to register");
            }
        };
        match Weak::upgrade(&subr2) {
            Some(subr)=> {
                listenner2.listen(&mut subr.borrow_mut());
            },
            None => {
                println!("fail to register");
            }
        };


        // 6 - send events by stimulating our parameters
        match Weak::upgrade(&subr1) {
            Some(subr)=> {
                subr.borrow_mut().set_value(CallbackArgument::F32(0.4));
            },
            None => {}
        };
        match Weak::upgrade(&subr2) {
            Some(subr)=> {
                subr.borrow_mut().set_value(CallbackArgument::Vec3(glm::vec3(0.3, 0.3, 0.3)));
            },
            None => {}
        };

        assert_eq!(listenner1.borrow().value, 0.4);
        assert_eq!(listenner2.borrow().value, glm::vec3(0.3, 0.3, 0.3));
    }
}