//! orb-core：基础设施（玻璃面板原语、渐变/阴影辅助、像素对齐、动画原语 Tween/Sequence/Stagger）。

pub mod animation;
pub mod overlay;
pub mod pixel;

pub use animation::{Clock, Easing, ManualClock, Sequence, Stagger, Tween};
pub use overlay::{InteractionEvent, InteractionState, StartupPhase, StartupSequence, ToastState};
pub use pixel::{pixel_scale, snap, snap_rect};

#[cfg(test)]
mod tests {
    use super::{
        pixel_scale, snap, snap_rect, Clock, Easing, ManualClock, Sequence, Stagger, Tween,
    };
    use iced::Rectangle;
    use std::time::Duration;

    #[test]
    fn tween_reaches_endpoints_and_midpoint() {
        let tween = Tween::new(10.0, 30.0, Duration::from_millis(100), Easing::Linear);

        assert_eq!(tween.sample(Duration::ZERO), 10.0);
        assert_eq!(tween.sample(Duration::from_millis(50)), 20.0);
        assert_eq!(tween.sample(Duration::from_millis(100)), 30.0);
        assert!(tween.is_finished(Duration::from_millis(100)));
    }

    #[test]
    fn sequence_plays_tweens_in_order() {
        let sequence = Sequence::new(vec![
            Tween::new(0.0, 1.0, Duration::from_millis(100), Easing::Linear),
            Tween::new(1.0, 3.0, Duration::from_millis(200), Easing::Linear),
        ]);

        assert_eq!(sequence.sample(Duration::from_millis(50)), Some(0.5));
        assert_eq!(sequence.sample(Duration::from_millis(100)), Some(1.0));
        assert_eq!(sequence.sample(Duration::from_millis(200)), Some(2.0));
        assert_eq!(sequence.sample(Duration::from_millis(300)), Some(3.0));
        assert_eq!(sequence.sample(Duration::from_millis(301)), None);
    }

    #[test]
    fn stagger_delays_items_and_caps_count() {
        let stagger = Stagger::new(5, Duration::from_millis(40), 3);

        assert_eq!(stagger.delay(0), Duration::ZERO);
        assert_eq!(stagger.delay(2), Duration::from_millis(80));
        assert_eq!(stagger.delay(3), Duration::from_millis(120));
        assert_eq!(stagger.delay(4), Duration::from_millis(120));
    }

    #[test]
    fn pixel_scaling_and_snapping_are_deterministic() {
        assert_eq!(pixel_scale(100.0, 1.25), 125.0);
        assert_eq!(snap(10.24, 1.25), 10.4);
        assert_eq!(snap(10.24, 1.5), 10.0);
        assert_eq!(
            snap_rect(
                Rectangle {
                    x: 1.12,
                    y: 2.24,
                    width: 10.0,
                    height: 4.0,
                },
                1.25
            ),
            Rectangle {
                x: 0.8,
                y: 2.4,
                width: 10.4,
                height: 4.0,
            }
        );
    }

    #[test]
    fn manual_clock_is_deterministic() {
        let mut clock = ManualClock::default();
        assert_eq!(clock.now(), Duration::ZERO);
        clock.advance(Duration::from_millis(40));
        assert_eq!(clock.now(), Duration::from_millis(40));
    }
}
