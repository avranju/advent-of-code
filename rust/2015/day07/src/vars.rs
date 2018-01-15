use std::cell::RefCell;
use std::collections::HashMap;

pub trait Subscriber {
    fn value_changed(&self, var: &str, value: u16);
}

impl<F> Subscriber for F
where
    F: Fn(&str, u16),
{
    fn value_changed(&self, var: &str, value: u16) {
        (*self)(var, value)
    }
}

pub struct Entry {
    pub value: Option<u16>,
    subscribers: Vec<(Box<dyn Subscriber>, Option<String>)>,
}

impl Entry {
    fn new(value: Option<u16>) -> Self {
        Entry {
            value,
            subscribers: vec![],
        }
    }

    fn notify(&self, name: &str) {
        if let Some(val) = self.value {
            for s in self.subscribers.iter() {
                s.0.value_changed(name, val);
            }
        }
    }

    fn add_subscriber(&mut self, subscriber: Box<dyn Subscriber>, diag: Option<String>) {
        self.subscribers.push((subscriber, diag));
    }
}

pub struct VarTable {
    table: RefCell<HashMap<String, RefCell<Entry>>>,
}

impl VarTable {
    pub fn new() -> Self {
        VarTable {
            table: RefCell::new(HashMap::new()),
        }
    }

    pub fn add(&mut self, name: &str) {
        self.table
            .borrow_mut()
            .insert(name.to_owned(), RefCell::new(Entry::new(None)));
    }

    pub fn set(&self, name: &str, val: Option<u16>) {
        if let Some(e) = self.table.borrow().get(name) {
            e.borrow_mut().value = val;
        }

        self.table
            .borrow()
            .get(name)
            .map(|e| e.borrow().notify(name));
    }

    pub fn print_entries(&self) {
        for (name, entry) in self.table.borrow().iter() {
            println!("{} = {}", name, entry.borrow().value.unwrap());
        }
    }

    pub fn get_var(&self, name: &str) -> Option<u16> {
        self.table.borrow().get(name).and_then(|e| e.borrow().value)
    }

    pub fn subscribe<T>(&self, name: &str, subscriber: T, diag: Option<String>)
    where
        T: Subscriber + 'static,
    {
        // in case the entry already had a value, then we want
        // this subscriber to know about it
        self.table
            .borrow()
            .get(name)
            .and_then(|e| e.borrow().value)
            .map(|val| subscriber.value_changed(name, val));

        self.table
            .borrow()
            .get(name)
            .map(|e| e.borrow_mut().add_subscriber(Box::new(subscriber), diag));
    }
}
