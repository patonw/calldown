use crate::Objective;
use rand::seq::SliceRandom;
use rand::Rng;
use log::{trace, debug};

pub trait Solver {
    fn solve<'a>(&self, initial: &'a [String], swappable: &[usize]) -> Vec<&'a str>;
}

pub struct AnnealingSolver<T: Objective> {
    max_temp: f32,
    objective: T,
}

impl <T: Objective> AnnealingSolver<T> {
    pub fn new(max_temp: f32, objective: T) -> Self {
        Self {
            max_temp,
            objective,
        }
    }
}

impl <T: Objective> Solver for AnnealingSolver<T> {
    fn solve<'a>(&self, initial: &'a [String], swappable: &[usize]) -> Vec<&'a str> {
        let mut max_temp = self.max_temp;
        let mut solution: Vec<&str> = initial.iter()
            .map(|x| x.as_str())
            .collect();

        let mut rng = rand::thread_rng();

        let mut buf = [0_usize; 2];

        let mut last_loss: Option<f32> = None;

        for _ in 1..=20 {
            let mut temp = max_temp;

            for _ in 1..=1000 {
                select_swap(&mut rng, swappable, &mut buf);
                solution.swap(buf[0], buf[1]);

                let loss: f32 = self.objective.loss(&solution);

                debug!("Solution is {:?} with loss {}", solution, loss);

                if let Some(ll) = last_loss {
                    let scale = ll.abs().max(1.0);
                    let diff = (loss - ll) / scale;

                    if diff > 0.0 {
                        let metro = (-diff / temp).exp();
                        let dice = rng.gen_range(0.0..1.0);
                        trace!("temp {} diff {} dice {} vs metro {}", temp, diff, dice, metro);

                        // Reject new solution
                        if dice > metro {
                            trace!("REJECTED");
                            solution.swap(buf[0], buf[1]);
                            continue;
                        }
                    }
                }

                last_loss = Some(loss);
                temp *= 0.99;
            }

            max_temp /= 2.0;
        }
        solution
    }
}

fn select_swap<R: rand::Rng>(rng: &mut R, swappable: &[usize], buf: &mut [usize]) {
    for (b, slot) in swappable.choose_multiple(rng, buf.len()).zip(buf.iter_mut()) {
        *slot = *b;
    }
}

