use crate::Objective;
use rand::seq::SliceRandom;
use rand::Rng;
use log::{trace, debug};

pub trait Solver {
    fn solve<'a, O: Objective>(&mut self, objective: &O, initial: &'a [String], swappable: &[usize]) -> Vec<&'a str>;
}

pub struct SawtoothAnnealingSchedule {
    max_temp: f32,
    count: usize,
    temp: f32,
}

impl SawtoothAnnealingSchedule {
    pub fn new(max_temp: f32) -> Self {
        Self {
            max_temp,
            count: 0,
            temp: max_temp,
        }
    }
}

impl Iterator for SawtoothAnnealingSchedule {
    type Item = f32;

    // TODO parameterize cycle length and number
    // Derive decay rates
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count % 1000 == 0 {
            self.temp = self.max_temp;
            self.max_temp /= 2.0;
        }
        else {
            self.temp *= 0.99;
        }

        if self.count >= 20_000 {
            None
        }
        else {
            Some(self.temp)
        }
    }
}

pub struct AnnealingSolver<S: Iterator<Item = f32>, F: Fn() -> S> {
    scheduler: F,
}

impl <S: Iterator<Item = f32>, F: Fn() -> S> AnnealingSolver<S, F> {
    pub fn new(scheduler: F) -> Self {
        Self {
            scheduler,
        }
    }
}

impl <S: Iterator<Item = f32>, F: Fn() -> S> Solver for AnnealingSolver<S, F> {
    fn solve<'a, O: Objective>(&mut self, objective: &O, initial: &'a [String], swappable: &[usize]) -> Vec<&'a str> {
        let mut solution: Vec<&str> = initial.iter()
            .map(|x| x.as_str())
            .collect();

        let mut rng = rand::thread_rng();

        let mut buf = [0_usize; 2];

        let mut last_loss: Option<f32> = None;

        let schedule = (self.scheduler)();

        for temp in schedule {
            select_swap(&mut rng, swappable, &mut buf);
            solution.swap(buf[0], buf[1]);

            let loss: f32 = objective.loss(&solution);

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
        }

        solution
    }
}

fn select_swap<R: rand::Rng>(rng: &mut R, swappable: &[usize], buf: &mut [usize]) {
    for (b, slot) in swappable.choose_multiple(rng, buf.len()).zip(buf.iter_mut()) {
        *slot = *b;
    }
}

