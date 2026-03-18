
pub fn give_simulated_sample(time: f64, freq: f64) -> f64 {
   // Standard sine wave: sin(2 * PI * freq * time)
   (std::f64::consts::TAU * freq * time).sin() 
}

#[cfg(test)]
mod tests {

    #[allow(unused)]
    use super::*;

    #[test]
    fn smoke_test() {
        assert!(true);
    }

} 