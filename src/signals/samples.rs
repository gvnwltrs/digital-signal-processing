
// Time: our step through a sine at t0..
// Freq: derives our phase or position in that wave at a time t0.. gives its frequency
pub fn give_simulated_sample(time: f64, freq: f64) -> f64 {
   // Standard sine wave: sin(2 * PI * freq * time)
   (std::f64::consts::TAU * freq * time).sin() 
}

/* Test Pairs (configure for sine sample generation 16-bit)
Time (sec)      Freq (Hz)       Sine Result ()	Quantized Result ()	    Meaning
0.0	            1.0	            0.0	            32768	            Midpoint (Start of wave)
0.25	        1.0	            1.0	            65535	            Peak (Top of wave)
0.5	            1.0	            0.0	            32768	            Midpoint (Crossing zero)
0.75	        1.0	            -1.0	        0	                Trough (Bottom of wave)
1.0	            1.0	            0.0	            32767	            Midpoint (Full cycle)
*/
#[cfg(test)]
mod tests {

    #[allow(unused)]
    use super::*;

    #[test]
    fn smoke_test() {
        assert!(true);
    }

    #[test]
    fn test_zero_sine() {
        let sample = give_simulated_sample(0.0, 1.0);
        assert!(sample == 0.0);
    }

} 

