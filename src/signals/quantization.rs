
pub fn quantize_sample(sample: f64, bits: u64) -> i64 {
    let levels = (1u64 << bits) as f64;

    // Normalize from [-1, 1] → [0, 1]
    let normalized = ((sample + 1.0) / 2.0).clamp(0.0, 1.0);

    // Quantize
    (normalized * (levels - 1.0)).round() as i64
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
    use crate::signals::samples::give_simulated_sample;

    #[test]
    fn smoke_test() {
        assert!(true);
    }

    // Rising from zero
    #[test]
    fn test_start_of_wave() {
        // 1. Get the raw sine (-1.0 to 1.0). At (0,0) this is 0.0
        let sample = give_simulated_sample(0.0, 1.0);

        // 2. Quantize
        let quantized_sample = quantize_sample(sample, 16);

        assert!(quantized_sample == 32768);
    }

    // Top
    #[test]
    fn test_top_of_wave() {
        // 1. Get the raw sine (-1.0 to 1.0). At (0,0) this is 0.0
        let sample = give_simulated_sample(0.25, 1.0);

        // 2. Quantize
        let quantized_sample = quantize_sample(sample, 16);

        assert!(quantized_sample == 65535);
    }

    // Back down through zero
    #[test]
    fn test_crossing_zero() {
        // 1. Get the raw sine (-1.0 to 1.0). At (0,0) this is 0.0
        let sample = give_simulated_sample(0.5, 1.0);

        // 2. Quantize
        let quantized_sample = quantize_sample(sample, 16);

        assert!(quantized_sample == 32768);
    }

    // Bottom of wave
    #[test]
    fn test_bottom_of_wave() {
        // 1. Get the raw sine (-1.0 to 1.0). At (0,0) this is 0.0
        let sample = give_simulated_sample(0.75, 1.0);

        // 2. Quantize
        let quantized_sample = quantize_sample(sample, 16);

        assert!(quantized_sample == 0);
    }

    // Back to the start
    #[test]
    fn test_full_cycle() {
        // 1. Get the raw sine (-1.0 to 1.0). At (0,0) this is 0.0
        let sample = give_simulated_sample(1.0, 1.0);

        // 2. Quantize
        let quantized_sample = quantize_sample(sample, 16);

        assert!(quantized_sample == 32767);
    }

} 