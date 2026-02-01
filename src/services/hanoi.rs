use num::{BigUint, pow::pow};
use tokio::task;
pub async fn calc_hanoi_num(num_cell: usize) -> Result<BigUint, task::JoinError> {
    if num_cell < 1_000_000 {
        Ok(pow(BigUint::from(2usize), num_cell) - BigUint::from(1usize))
    } else {
        task::spawn_blocking(move || {
            Ok(pow(BigUint::from(2usize), num_cell) - BigUint::from(1usize))
        })
        .await?
    }
}
pub async fn calc_hanoi_rec(num_cell: usize) -> Result<Vec<(u8, u8)>, task::JoinError> {
    task::spawn_blocking(move || {
        let mut orders: Vec<(u8, u8)> = Vec::new();
        calc_hanoi_inner_(num_cell, 1, 3, 2, &mut orders);
        Ok(orders)
    })
    .await?
}
fn calc_hanoi_inner_(num_cell: usize, from: u8, to: u8, via: u8, res_vec: &mut Vec<(u8, u8)>) {
    if num_cell == 1 {
        res_vec.push((from, to));
    } else {
        calc_hanoi_inner_(num_cell - 1, from, via, to, res_vec);
        res_vec.push((from, to));
        calc_hanoi_inner_(num_cell - 1, via, to, from, res_vec);
    }
}
