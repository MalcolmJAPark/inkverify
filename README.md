# InkVerify: Cellular Automaton Proof-of-Work

**InkVerify** is an experimental **Memory-Hard Proof-of-Work (PoW) Algorithm** designed for secure user authentication and bot prevention.

By leveraging **Chaos Theory** and **Cellular Automata (CA)**, InkVerify transforms credentials into a complex, high-entropy grid state. Unlike traditional cryptographic hashes (like SHA-256) which rely purely on CPU arithmetic, InkVerify enforces a **memory-intensive process**, making it highly resistant to GPU and ASIC-based brute-force attacks.

---

## 🚀 The Core Concept

The algorithm models verification as a simulation of a "chaotic ocean" in a 2D grid:

1.  **Input Mixing:** A user's `Username` and `Password` are hashed to generate a massive binary string.
2.  **The Seed:** This string populates the *entire* 2D grid with initial static noise (0s and 1s).
3.  **The Expansion:** The grid evolves for a fixed time $t$ using a custom, non-monotonic CA rule (similar to Conway's Game of Life or Rule 30).
4.  **Verification:** The final state of the grid is compared against the stored "lock" image.

---

## 🛡️ Security Evolution & Auditing

The architecture of InkVerify was arrived at after identifying and patching critical flaws in standard expansion models.

### 🔴 v1: The "Expanding Blob" (Discarded)
* **Concept:** `Username` = Start Coordinate. `Password` = Duration. Ink expands from the center.
* **The Flaw (Erosion Attack):** Since the ink grew monotonically (never receding), an attacker could geometrically "peel" the outer layers of the final image to reveal the center seed coordinate instantly.

### 🟠 v2: The "Fossilized Wavefront" (Discarded)
* **Concept:** `Password` = Duration. Cells "freeze" after a short lifespan to prevent loops.
* **The Flaw (Time Leakage):** The "frozen" cells created tree-ring-like patterns. An attacker could count the rings to deduce the password duration without running the code.

### 🟢 v3: The "Chaotic Ocean" (Current Standard)
* **Concept:** `Username + Password` = Full Grid State. Fixed Duration for all users.
* **The Fix:** By seeding the whole grid and allowing cells to toggle alive/dead indefinitely, we destroy the geometric history. The final image is a result of pure chaos entropy, making it mathematically irreversible and immune to erosion attacks.

---

## 📊 Industry Comparison & Viability

### How does this compare to existing methods?
InkVerify occupies a similar niche to **Argon2** and **Scrypt** (Memory-Hard Functions), but with distinct architectural differences:

| Feature | Standard Hash (SHA-256) | Standard PoW (Scrypt/Argon2) | InkVerify (CA Approach) |
| :--- | :--- | :--- | :--- |
| **Primary Resource** | CPU Cycles | RAM + CPU | RAM Bandwidth |
| **ASIC Resistance** | Low (Vulnerable) | High | **Very High** |
| **Output Type** | Hex String | Hex String | **Visual Image (2D Array)** |
| **Tunability** | Hard (Fixed Math) | Configurable | **Granular** (Grid Size + Time) |

### Key Benefits
1.  **ASIC Resistance:** Because the algorithm requires reading and writing to a large 2D grid ($N \times N$) at every step, it creates a memory bottleneck. This prevents attackers from building cheap, specialized chips (ASICs) to crack passwords, as memory is expensive to scale on hardware.
2.  **Visual Debugging:** Unlike mathematical hashes, InkVerify produces an image. This allows engineers to visually audit the "quality" of the randomness. If the final grid looks like a repeating pattern, the rule is weak. If it looks like TV static, the entropy is high.
3.  **Future-Proofing:** As computers get faster, the difficulty can be scaled linearly by increasing the Grid Size or the Time $t$.

---

## 🌍 Real-World Application: The "Anti-Bot" Layer

While InkVerify can be used for password hashing, its most viable industrial application is **Client-Side Proof of Work (Anti-Bot).**

* **The Problem:** Standard CAPTCHAs annoy humans. Simple mathematical puzzles (Hashcash) are easily solved by bot farms using specialized hardware (ASICs).
* **The InkVerify Solution:**
    1.  When a user clicks "Login," the browser quietly runs an InkVerify simulation in the background.
    2.  It takes a legitimate user **1 second** of compute time.
    3.  A bot farm trying to spam 1,000,000 requests would require massive amounts of RAM to run 1,000,000 simultaneous grid simulations.
    4.  **Result:** Spamming becomes economically impossible for the attacker, without annoying the human user.

---

## ⚙️ Implementation Details

### Requirements
* **Language:** Python 3.8+ (Prototype), Rust/C++ (Production).
* **Libraries:** `numpy` (for grid management), `scipy` (for convolution/neighbor detection).

### Usage Pattern

```python
import numpy as np
import hashlib
from engine import CellularAutomataEngine

# 1. Generate the Challenge
# Combine credentials to form the high-entropy seed
raw_string = username + password
seed_hash = hashlib.sha256(raw_string.encode()).digest()

# 2. Seed the Grid
# Convert hash to a 1024x1024 boolean array (The Memory Hardness)
initial_grid = generate_grid_from_seed(seed_hash, size=(1024, 1024))

# 3. Run the Work (The "Ink" Expansion)
# Uses memory-hard cellular automaton rules for t=1000 steps
final_grid = CellularAutomataEngine.run(initial_grid, steps=1000)

# 4. Verify
# Check against the stored "Lock" image
if np.array_equal(final_grid, stored_lock_image):
    grant_access()
else:
    deny_access()
```

## 🔮 Future Improvements
* **3D Expansion:** Moving the simulation to a 3D voxel grid to exponentially increase memory requirements (hardening against GPU attacks).
* **Dynamic Rules:** Incorporating user-specific "Security Answers" to procedurally generate the expansion rules themselves, adding another layer of obscurity.
