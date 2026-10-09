//! 确定性动画原语：缓动、补间、序列、错峰与可注入时钟。

use std::time::Duration;

/// 动画缓动函数。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Easing {
    /// 匀速变化。
    Linear,
    /// 先快后慢。
    EaseOut,
    /// 先慢后快再慢。
    EaseInOut,
    /// 带轻微回弹的结束效果。
    EaseOutBack,
}

impl Easing {
    fn apply(self, progress: f32) -> f32 {
        match self {
            Self::Linear => progress,
            Self::EaseOut => 1.0 - (1.0 - progress).powi(2),
            Self::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - (-2.0 * progress + 2.0).powi(2) / 2.0
                }
            }
            Self::EaseOutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (progress - 1.0).powi(3) + c1 * (progress - 1.0).powi(2)
            }
        }
    }
}

/// 一段从起点到终点的标量补间动画。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tween {
    from: f32,
    to: f32,
    duration: Duration,
    easing: Easing,
}

impl Tween {
    /// 创建一段标量补间动画。
    pub fn new(from: f32, to: f32, duration: Duration, easing: Easing) -> Self {
        Self {
            from,
            to,
            duration,
            easing,
        }
    }

    /// 按经过时间采样动画值；超出时长时固定返回终点。
    pub fn sample(&self, elapsed: Duration) -> f32 {
        if self.duration.is_zero() {
            return self.to;
        }
        let progress = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0);
        self.from + (self.to - self.from) * self.easing.apply(progress)
    }

    /// 判断动画是否已经达到结束时刻。
    pub fn is_finished(&self, elapsed: Duration) -> bool {
        elapsed >= self.duration
    }

    /// 返回该补间的持续时间。
    pub fn duration(&self) -> Duration {
        self.duration
    }
}

/// 可注入的动画时钟。
pub trait Clock {
    /// 返回从时钟起点开始经过的时间。
    fn now(&self) -> Duration;
}

/// 用于单元测试和确定性预览的手动时钟。
#[derive(Debug, Default, Clone, Copy)]
pub struct ManualClock {
    elapsed: Duration,
}

impl ManualClock {
    /// 推进时钟。
    pub fn advance(&mut self, duration: Duration) {
        self.elapsed += duration;
    }
}

impl Clock for ManualClock {
    fn now(&self) -> Duration {
        self.elapsed
    }
}

/// 按顺序播放多段标量补间动画。
#[derive(Debug, Clone)]
pub struct Sequence {
    tweens: Vec<Tween>,
}

impl Sequence {
    /// 创建一段动画序列。
    pub fn new(tweens: Vec<Tween>) -> Self {
        Self { tweens }
    }

    /// 按总经过时间采样当前动画值。
    pub fn sample(&self, elapsed: Duration) -> Option<f32> {
        let mut offset = Duration::ZERO;
        for tween in &self.tweens {
            let end = offset + tween.duration();
            if elapsed <= end {
                return Some(tween.sample(elapsed.saturating_sub(offset)));
            }
            offset = end;
        }
        None
    }
}

/// 为一组项目计算错峰启动延迟。
#[derive(Debug, Clone, Copy)]
pub struct Stagger {
    count: usize,
    interval: Duration,
    max_items: usize,
}

impl Stagger {
    /// 创建错峰参数；超过 `max_items` 的项目共享最后一个延迟档位。
    pub fn new(count: usize, interval: Duration, max_items: usize) -> Self {
        Self {
            count,
            interval,
            max_items,
        }
    }

    /// 返回指定项目的启动延迟。
    pub fn delay(&self, index: usize) -> Duration {
        let _ = self.count;
        let slot = index.min(self.max_items);
        self.interval.saturating_mul(slot as u32)
    }
}
