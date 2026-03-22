import numpy as np
import matplotlib.pyplot as plt
from scipy.signal import butter, filtfilt

# Parameters
fs = 1000
t = np.arange(0, 1.0, 1/fs)

f_signal = 55
f_lo = 50

# Signal and LO
signal = np.cos(2 * np.pi * f_signal * t)
lo = np.cos(2 * np.pi * f_lo * t)

# Mix
mixed = signal * lo

# Low-pass filter
b, a = butter(4, 10/(fs/2))   # 10 Hz cutoff
filtered = filtfilt(b, a, mixed)

# Plot
plt.figure(figsize=(10, 8))

plt.subplot(4, 1, 1)
plt.plot(t, signal)
plt.title(f"Original Signal ({f_signal} Hz)")

plt.subplot(4, 1, 2)
plt.plot(t, lo)
plt.title(f"Local Oscillator ({f_lo} Hz)")

plt.subplot(4, 1, 3)
plt.plot(t, mixed)
plt.title("After Mixing (Difference + Sum)")

plt.subplot(4, 1, 4)
plt.plot(t, filtered)
plt.title("After Low-Pass Filter (Baseband)")

plt.tight_layout()
plt.show()