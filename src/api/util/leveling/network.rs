//! This module provides utility fuctions to work with Hypixel's network level and network experience.
//!
//!
//! This module follows the java implementation of the Hypixel API.\
//! See [here](https://github.com/HypixelDev/PublicAPI/blob/master/hypixel-api-core/src/main/java/net/hypixel/api/util/ILeveling.java).
//!
//! From the formulas written in this implementation, it can be seen
//! that the relation between the network level and the network experience can be expressed as:
//! ```math
//! \frac{dy}{dx} = gx + b \;\;\;\;\; \text{and} \;\;\;\;\; y(1) = 0
//! ```
//! where:
//! - y = network xp
//! - x = network lvl
//! - g = [`GROWTH`]
//! - b = [`BASE`]
//!
//! More specific: the integer part of the network level corresponds to an amount of experience
//! that follows these equations. The fractional part of a network level corresponds to an amount of
//! experience that is is linearly interpolated between two experience amounts corresponding to nearest surrounding
//! integer network levels.

pub const BASE: f64 = 10000.0;
pub const GROWTH: f64 = 2500.0;

const HALF_GROWTH: f64 = GROWTH * 0.5;

const REVERSE_PQ_PREFIX: f64 = -(BASE - 0.5 * GROWTH) / GROWTH;
const REVERSE_CONST: f64 = REVERSE_PQ_PREFIX * REVERSE_PQ_PREFIX;
const GROWTH_DIVIDES_2: f64 = 2.0 / GROWTH;

/// This function returns the level of a player calculated by the
/// current experience gathered. Unlike [`exact_level`], this function returns
/// the largest integer smaller than the exact level (= floored).
///
/// The result cannot be smaller than `1.0` and negative experience results in `1.0`.
///
/// # Examples
/// ```ignore
///            0 XP -> 1.0
///         5000 XP -> 1.0
///        10000 XP -> 2.0
///        50000 XP -> 4.0
///     79342431 XP -> 249.0
/// ```
pub fn calculate_level(exp: f64) -> f64 {
    if exp < 0.0 {
        1.0
    } else {
        (1.0 + REVERSE_PQ_PREFIX + (REVERSE_CONST + GROWTH_DIVIDES_2 * exp).sqrt()).floor()
    }
}

/// This function returns the exact level of a player calculated by the
/// current experience gathered. Unlike [`calculate_level`], this function does
/// not floor its result and will return an accurate level.
///
/// The result cannot be smaller than `1.0` and negative experience results in `1.0`.
///
/// # Examples
/// ```ignore
///            0 XP -> 1.0
///         5000 XP -> 1.5
///        10000 XP -> 2.0
///        50000 XP -> 4.71...
///     79342431 XP -> 249.46...
/// ```
pub fn exact_level(exp: f64) -> f64 {
    calculate_level(exp) + percentage_to_next_level(exp)
}

/// This function returns the amount of experience that is needed to progress from `level` to `level + 1`. (e.g. 5 to 6)
/// The levels passed *must* be absolute levels with the smallest level being 1.
/// Smaller values always return the `BASE` constant. The calculation is precise and
/// if a decimal is passed, it returns the XP from the progress of this level to the next
/// level with the same progress. (e.g. 5.5 to 6.5)
///
/// # Examples
/// ```ignore
///       1 (to 2)   =  10000.0 XP
///       2 (to 3)   =  12500.0 XP
///       3 (to 4)   =  15000.0 XP
///       5 (to 6)   =  20000.0 XP
///     5.5 (to 6.5) =  21250.0 XP
///     130 (to 131) = 332500.0 XP
///     250 (to 251) = 632500.0 XP
/// ```
pub fn xp_to_next_level(level: f64) -> f64 {
    if level < 1.0 {
        BASE
    } else {
        GROWTH * (level - 1.0) + BASE
    }
}

/// This method returns the experience required to reach that level. This method is precise, that means
/// you can pass any progress of a level to receive the experience to reach that progress. (e.g. 5.764 returns
/// the experience required to reach level 5 and 76.4% of the experience required to go from level 5 to level 6).
///
/// # Examples
/// ```ignore
///        1.0 =        0.0 XP
///        2.0 =    10000.0 XP
///        3.0 =    22500.0 XP
///        5.0 =    55000.0 XP
///      5.764 =    70280.0 XP
///      130.0 = 21930000.0 XP
///     250.43 = 79951975.0 XP
/// ```
pub fn total_xp_to_level(level: f64) -> f64 {
    let lvl = level.floor();
    let x0 = total_xp_to_full_level(lvl);
    (total_xp_to_full_level(lvl + 1.0) - x0) * (level % 1.0) + x0
}

/// Helper method that may only be called by full levels (meaning no fractional part)
/// and has the same functionality as [`total_xp_to_level`] but doesn't support progress
/// and will return wrong values due to following a parabola instead of a line in between integer levels.
pub fn total_xp_to_full_level(level: f64) -> f64 {
    (HALF_GROWTH * (level - 2.0) + BASE) * (level - 1.0)
}

/// This method returns the current progress of this level to reach the next level. This method is as
/// precise as possible due to rounding errors on the mantissa. The first 10 decimals are totally
/// accurate.
///
/// # Examples
/// ```ìgnore
///         5000.0 XP   (Lv. 1) = 0.5                               (50 %)
///        22499.0 XP   (Lv. 2) = 0.99992                       (99.992 %)
///      5324224.0 XP  (Lv. 62) = 0.856763076923077   (85.6763076923077 %)
///     23422443.0 XP (Lv. 134) = 0.4304905109489051 (43.04905109489051 %)
/// ```
pub fn percentage_to_next_level(exp: f64) -> f64 {
    let lvl = calculate_level(exp);
    let x0 = total_xp_to_level(lvl);
    (exp - x0) / (total_xp_to_level(lvl + 1.0) - x0)
}