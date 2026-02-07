use lazy_static::lazy_static;
use num::BigUint;
use std::collections::HashMap;
use std::sync::RwLock;

lazy_static! {
    static ref FIBO_CACHE: RwLock<HashMap<usize, BigUint>> = RwLock::new(HashMap::new());
}
pub fn calc_fibo_rec(n: usize) -> BigUint {
    {
        if let Some(res) = FIBO_CACHE
            .read()
            .ok()
            .and_then(|cache| cache.get(&n).cloned())
        {
            return res;
        }
    }
    if n == 0 || n == 1 {
        BigUint::from(n)
    } else {
        let res = calc_fibo_rec(n - 1) + calc_fibo_rec(n - 2);
        {
            if let Ok(mut cache) = FIBO_CACHE.write() {
                cache.insert(n, res.clone());
            }
        }
        res
    }
}
