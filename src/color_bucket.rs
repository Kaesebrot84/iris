use crate::utils::mean;
use crate::Color;
use crate::ColorChannel;

/// Struct holding an `Vec<Color>`.
/// Implements helpful functions for the median cut algorithm.
///
#[derive(Debug, PartialEq)]
pub struct ColorBucket {
    colors: Vec<Color>,
}

impl ColorBucket {
    /// Creates a ColorBucket based on the colors passed. Returns `None` if passed an empty vector.
    ///
    /// # Arguments
    ///
    /// * `colors` - Color vector from which the mean color is created.
    ///
    /// # Examples
    ///
    /// ```
    /// let data = vec![Color { r: 15, g: 131, b: 0, a: 255 }, Color { r: 221, g: 11, b: 22, a: 130 }, Color { r: 81, g: 11, b: 16, a: 0 }];
    /// let result = ColorBucket::from_pixels(data.clone()).expect("Passed empty color vector to test.");
    /// assert_eq!(result.colors, data);
    /// ```
    ///
    pub fn from_pixels(pixels: Vec<Color>) -> Option<Self> {
        if pixels.is_empty() {
            None
        } else {
            Some(Self { colors: pixels })
        }
    }

    /// Recursivly performs the median cut algorithm on self if iteration has not reached 0 yet.
    /// Creates two new buckets based on own colors. One bucket with values above and one bucket with value below the median, then performs the algorithm on them again.
    ///
    /// If iteration has reached 0 the color mean for self is pushed to the result vector.
    ///
    /// # Arguments
    ///
    /// * `iter_count` - Iteration index is used as termination criteria. Recursion stop when 0 is reached.
    /// * `result` - Vector holding color means for each bucket in the iteration.
    ///
    fn recurse(&mut self, iter_count: u8, result: &mut Vec<Color>) {
        if iter_count == 0 {
            result.push(self.color_mean())
        } else {
            let new_buckets = self.median_cut();
            if let Some(mut bucket) = new_buckets.0 {
                bucket.recurse(iter_count - 1, result);
            }
            if let Some(mut bucket) = new_buckets.1 {
                bucket.recurse(iter_count - 1, result)
            }
        }
    }

    /// Creates a color palette from own pixels.
    ///
    /// # Arguments
    ///
    /// * `iter_count` - number of iterations to be performed on the bucket.
    ///
    /// # Example
    ///
    /// ```
    /// let data = vec![Color { r: 15, g: 131, b: 0, a: 255 }, Color { r: 221, g: 11, b: 22, a: 130 }, Color { r: 81, g: 11, b: 16, a: 0 }];
    /// let bucket = ColorBucket::from_pixels(data.clone()).expect("Passed empty color vector to test.");
    /// let result = bucket.make_palette();
    /// ```
    ///
    pub fn make_palette(&mut self, iter_count: u8) -> Vec<Color> {
        let mut result = vec![];
        self.recurse(iter_count, &mut result);
        result
    }

    /// Performs the median cut on a own vector (bucket) of `Color`.
    /// Returns two `Color` vectors representing the colors above and colors below median value.
    ///
    fn median_cut(&mut self) -> (Option<ColorBucket>, Option<ColorBucket>) {
        let highest_range_channel = self.highest_range_channel();
        let median = self.color_median(highest_range_channel);
        let mut above_median = vec![];
        let mut below_median = vec![];
        for color in &self.colors {
            if color[highest_range_channel] > median {
                above_median.push(*color);
            } else {
                below_median.push(*color)
            }
        }

        (ColorBucket::from_pixels(above_median), ColorBucket::from_pixels(below_median))
    }

    /// Returns the color channel with the highest range.
    /// IMPORTANT: Ignores alpha channel!
    ///
    fn highest_range_channel(&self) -> ColorChannel {
        let ranges = self.color_ranges();
        let mut highest_range_channel = ColorChannel::R;
        let mut highest_value = ranges.r;

        if ranges.g > highest_value {
            highest_range_channel = ColorChannel::G;
            highest_value = ranges.g;
        }

        if ranges.b > highest_value {
            highest_range_channel = ColorChannel::B;
        }

        highest_range_channel
    }

    /// Returns the ranges for each color channel.
    ///
    /// # Examples
    ///
    fn color_ranges(&self) -> Color {
        // Unwrap is ok here, because `max_by_key` only returns `None` for empty vectors
        Color {
            r: self.colors.iter().max_by_key(|c| c.r).unwrap().r - self.colors.iter().min_by_key(|c| c.r).unwrap().r,
            g: self.colors.iter().max_by_key(|c| c.g).unwrap().g - self.colors.iter().min_by_key(|c| c.g).unwrap().g,
            b: self.colors.iter().max_by_key(|c| c.b).unwrap().b - self.colors.iter().min_by_key(|c| c.b).unwrap().b,
            a: self.colors.iter().max_by_key(|c| c.a).unwrap().a - self.colors.iter().min_by_key(|c| c.a).unwrap().a,
        }
    }

    /// Sort a colors for a specific channel.
    ///
    /// # Arguments
    ///
    /// * `channel` - Target channel. The sorting is performed based on this value.
    ///
    /// # Examples
    ///
    fn sort_colors(&mut self, channel: ColorChannel) {
        self.colors.sort_by_key(|x| x[channel])
    }

    /// Returns median value for a specific `ColorChannel`.
    ///
    /// # Arguments
    ///
    /// * `channel` - Target channel for which the median is calculated.
    ///
    fn color_median(&mut self, channel: ColorChannel) -> u8 {
        self.sort_colors(channel);

        let mid = self.colors.len() / 2;
        if self.colors.len() % 2 == 0 {
            let bucket = ColorBucket::from_pixels(vec![self.colors[mid - 1], self.colors[mid]]).unwrap();
            bucket.channel_mean(channel)
        } else {
            self.channel_value_by_index(mid, channel)
        }
    }

    /// Returns a color value based on the provided channel and index parameters.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the target color in the vector.
    /// * `channel` - Color channel of the searched value.
    ///
    fn channel_value_by_index(&self, index: usize, channel: ColorChannel) -> u8 {
        self.colors[index][channel]
    }

    /// Calculate the mean value for a specific color channel on own vector of `Color`.
    ///
    /// # Arguments
    ///
    /// * `channel` - Target channel for which the mean is calculated.
    ///
    /// # Examples
    ///
    fn channel_mean(&self, channel: ColorChannel) -> u8 {
        mean(self.colors.iter().map(|x| x[channel]))
    }

    /// Returns the mean color value based on own colors.
    ///
    /// # Examples
    ///
    /// ```
    /// let colors = Vec::<Color>::new();
    /// let result = color_mean(&colors);
    /// ```
    ///
    fn color_mean(&self) -> Color {
        let r = mean(self.colors.iter().map(|c| c.r));
        let g = mean(self.colors.iter().map(|c| c.g));
        let b = mean(self.colors.iter().map(|c| c.b));
        let a = mean(self.colors.iter().map(|c| c.a));

        Color { r, g, b, a }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_pixels_ut() {
        let bucket = ColorBucket::from_pixels(vec![]);
        assert_eq!(bucket, None);

        let data = vec![Color { r: 15, g: 131, b: 0, a: 255 }, Color { r: 221, g: 11, b: 22, a: 130 }, Color { r: 81, g: 11, b: 16, a: 0 }];
        let bucket = ColorBucket::from_pixels(data.clone()).expect("Passed empty color vector to test.");
        assert_eq!(bucket.colors, data);
    }

    #[test]
    fn recurse_ut() {
        let pixels = vec![Color { r: 255, g: 0, b: 0, a: 255 }, Color { r: 0, g: 255, b: 0, a: 255 }];
        let mut bucket = ColorBucket::from_pixels(pixels.clone()).expect("Passed empty color vector to test.");
        let mut result = vec![];
        bucket.recurse(1, &mut result);
        assert_eq!(result, pixels);
    }

    #[test]
    fn make_palette_ut() {
        let pixels = vec![Color { r: 100, g: 120, b: 120, a: 0 }, Color { r: 150, g: 150, b: 150, a: 0 }, Color { r: 255, g: 255, b: 255, a: 0 }];
        let mut bucket = ColorBucket::from_pixels(pixels.clone()).expect("Passed empty color vector to test.");

        let colors = bucket.make_palette(3);
        let expected = vec![Color { r: 255, g: 255, b: 255, a: 0 }, Color { r: 150, g: 150, b: 150, a: 0 }, Color { r: 100, g: 120, b: 120, a: 0 }];
        assert_eq!(colors, expected);
    }

    #[test]
    pub fn sort_colors_ut() {
        let colors = generate_unsorted_colors();
        let mut bucket = ColorBucket::from_pixels(colors.clone()).expect("Passed empty color vector to test");
        bucket.sort_colors(ColorChannel::R);

        assert_eq!(bucket.colors[0], Color { r: 0, g: 2, b: 1, a: 20 });
        assert_eq!(bucket.colors[1], Color { r: 1, g: 23, b: 16, a: 20 });
        assert_eq!(bucket.colors[2], Color { r: 3, g: 4, b: 15, a: 2 });
        assert_eq!(bucket.colors[3], Color { r: 55, g: 17, b: 0, a: 118 });
    }

    #[test]
    pub fn color_median_ut() {
        let colors = generate_unsorted_colors();
        let mut bucket = ColorBucket::from_pixels(colors.clone()).expect("Passed empty color vector to test");
        let result = bucket.color_median(ColorChannel::R);
        assert_eq!(result, 2);
    }

    #[test]
    fn channel_value_by_index_ut() {
        let colors = vec![
            Color { r: 100, g: 22, b: 12, a: 0 },
            Color { r: 126, g: 175, b: 137, a: 1 },
            Color { r: 221, g: 225, b: 0, a: 113 },
            Color { r: 13, g: 226, b: 0, a: 17 },
        ];

        let bucket = ColorBucket::from_pixels(colors).expect("Passing empty color vector to test");

        assert_eq!(100, bucket.channel_value_by_index(0, ColorChannel::R));
        assert_eq!(22, bucket.channel_value_by_index(0, ColorChannel::G));
        assert_eq!(12, bucket.channel_value_by_index(0, ColorChannel::B));
        assert_eq!(0, bucket.channel_value_by_index(0, ColorChannel::A));

        assert_eq!(126, bucket.channel_value_by_index(1, ColorChannel::R));
        assert_eq!(175, bucket.channel_value_by_index(1, ColorChannel::G));
        assert_eq!(137, bucket.channel_value_by_index(1, ColorChannel::B));
        assert_eq!(1, bucket.channel_value_by_index(1, ColorChannel::A));

        assert_eq!(221, bucket.channel_value_by_index(2, ColorChannel::R));
        assert_eq!(225, bucket.channel_value_by_index(2, ColorChannel::G));
        assert_eq!(0, bucket.channel_value_by_index(2, ColorChannel::B));
        assert_eq!(113, bucket.channel_value_by_index(2, ColorChannel::A));

        assert_eq!(13, bucket.channel_value_by_index(3, ColorChannel::R));
        assert_eq!(226, bucket.channel_value_by_index(3, ColorChannel::G));
        assert_eq!(0, bucket.channel_value_by_index(3, ColorChannel::B));
        assert_eq!(17, bucket.channel_value_by_index(3, ColorChannel::A));
    }

    #[test]
    fn channel_mean_ut() {
        let colors = vec![
            Color { r: 100, g: 50, b: 12, a: 255 },
            Color { r: 100, g: 50, b: 12, a: 255 },
            Color { r: 100, g: 50, b: 12, a: 255 },
            Color { r: 100, g: 50, b: 12, a: 255 },
        ];

        let bucket = ColorBucket::from_pixels(colors).expect("Passed empty color vector to test.");
        let mut result = bucket.channel_mean(ColorChannel::R);
        assert_eq!(100, result);
        result = bucket.channel_mean(ColorChannel::G);
        assert_eq!(50, result);
        result = bucket.channel_mean(ColorChannel::B);
        assert_eq!(12, result);
        result = bucket.channel_mean(ColorChannel::A);
        assert_eq!(255, result);

        // More precise check
        let colors = vec![
            Color { r: 100, g: 22, b: 12, a: 0 },
            Color { r: 126, g: 175, b: 137, a: 1 },
            Color { r: 221, g: 225, b: 0, a: 113 },
            Color { r: 13, g: 226, b: 0, a: 17 },
        ];

        let bucket = ColorBucket::from_pixels(colors).expect("Passed empty color vector to test.");

        result = bucket.channel_mean(ColorChannel::R);
        assert_eq!(115, result);
        result = bucket.channel_mean(ColorChannel::G);
        assert_eq!(162, result);
        result = bucket.channel_mean(ColorChannel::B);
        assert_eq!(37, result);
        result = bucket.channel_mean(ColorChannel::A);
        assert_eq!(32, result);
    }

    #[test]
    pub fn ut_color_mean() {
        let colors = generate_unsorted_colors();
        let bucket = ColorBucket::from_pixels(colors).expect("Passed empty color vector to test.");

        let result = bucket.color_mean();
        let expected = Color { r: 14, g: 11, b: 8, a: 40 };
        assert_eq!(expected, result);
    }

    #[test]
    fn median_cut_ut() {
        let mut bucket = ColorBucket::from_pixels(generate_unsorted_colors()).expect("Passed empty color vector to test.");
        let result = bucket.median_cut();
        assert_eq!(
            result.0,
            Some(ColorBucket::from_pixels(vec![Color { r: 3, g: 4, b: 15, a: 2 }, Color { r: 55, g: 17, b: 0, a: 118 }]).unwrap())
        );
        assert_eq!(
            result.1,
            Some(ColorBucket::from_pixels(vec![Color { r: 0, g: 2, b: 1, a: 20 }, Color { r: 1, g: 23, b: 16, a: 20 }]).unwrap())
        );

        let mut bucket = ColorBucket::from_pixels(vec![Color { r: 0, g: 0, b: 0, a: 0 }]).expect("Passed empty color vector to test.");
        let result = bucket.median_cut();
        assert_eq!(result.0, None);
        assert_eq!(result.1, Some(ColorBucket::from_pixels(vec![Color { r: 0, g: 0, b: 0, a: 0 }])).unwrap());
    }

    #[test]
    fn highest_range_channel_ut() {
        let bucket = ColorBucket::from_pixels(generate_unsorted_colors()).expect("Passed empty color vector to test");
        assert_eq!(ColorChannel::R, bucket.highest_range_channel());
        assert_ne!(ColorChannel::G, bucket.highest_range_channel());
        assert_ne!(ColorChannel::B, bucket.highest_range_channel());
        assert_ne!(ColorChannel::A, bucket.highest_range_channel());
    }

    #[test]
    fn color_ranges_ut() {
        let bucket = ColorBucket::from_pixels(generate_unsorted_colors()).expect("Passed empty color vector to test");
        let expected = Color { r: 55, g: 21, b: 16, a: 116 };
        assert_eq!(expected, bucket.color_ranges());
    }

    fn generate_unsorted_colors() -> Vec<Color> {
        vec![
            Color { r: 55, g: 17, b: 0, a: 118 },
            Color { r: 0, g: 2, b: 1, a: 20 },
            Color { r: 3, g: 4, b: 15, a: 2 },
            Color { r: 1, g: 23, b: 16, a: 20 },
        ]
    }
}
