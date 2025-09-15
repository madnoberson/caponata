use std::{
    fmt,
    hash::{
        Hash,
        Hasher,
    },
    marker::Tuple,
    sync::Arc,
};

use uuid::Uuid;

pub struct Callable<Args: Tuple, R> {
    id: Uuid,
    function: Arc<dyn Fn(Args) -> R>,
}

impl<Args: Tuple, R> fmt::Debug for Callable<Args, R> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Callable")
    }
}

impl<Args: Tuple, R> PartialEq for Callable<Args, R> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<Args: Tuple, R> Eq for Callable<Args, R> {}

impl<Args: Tuple, R> Clone for Callable<Args, R> {
    fn clone(&self) -> Self {
        Callable {
            id: self.id,
            function: self.function.clone(),
        }
    }
}

impl<Args: Tuple, R> Hash for Callable<Args, R> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<Args: Tuple, R> Callable<Args, R> {
    pub fn new(function: Arc<dyn Fn(Args) -> R>) -> Self {
        Self {
            id: Uuid::new_v4(),
            function: function,
        }
    }

    pub fn call(&self, args: Args) -> R {
        self.function.call((args,))
    }
}
