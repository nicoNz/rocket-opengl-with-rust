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

impl ParameterContent {
    pub fn set_value(&mut self, value: f32) {
        match self {
            Self::F32(param) => {
                param.value = value;
                for (key, cb) in param.subscriptions.subscriptions.iter() {
                    cb(value);
                }
            }
        }
    }
    pub fn subscribe(&mut self, f: Box<dyn Fn(f32)>) -> i16 {
        match self {
            Self::F32 (p) => {
                p.subscriptions.add_subscription(f)
                
            }
        }
    }
}


pub struct Parameter {
    label: String,
    content: ParameterContent,
    id: i16,
}

impl Parameter {

    pub fn new(label: String, id: i16) -> Self {
        Self {
            label,
            id,
            content : ParameterContent::F32(Box::new(ParameterF32::new()))
        }
    }

    pub fn set_value(&mut self, v: f32) {
        self.content.set_value(v);
    }

    pub fn register_callback(&mut self, f: Box<dyn Fn(f32)>) -> i16 {
        self.content.subscribe(f)
    }
}


type Parameters = HashMap<i16, Parameter>;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
    #[test]
    fn parameter_test() {
        let mut p = Parameters::new();

        //TODO => wrap in order to throw an error if exist, and send back the key if exist
        p.insert(1, Parameter::new(String::from("my P1"), 2));
        let subr =  match p.get_mut(&1) {
            Some(v) => v,
            None => panic!("bad index")
        };
        let _addr = subr.register_callback( Box::new(move |v: f32|{
            assert_eq!(v, 0.7);
        }));
        
        subr.set_value(0.7);
    }


}