    use super::*;
    use iced::widget::text;

    #[test]
    fn pager_builder_and_defaults() {
        let p_default: Pager<'_, (), iced::Theme> =
            pager(vec![text("Page 1").into(), text("Page 2").into()]);
        assert!(!p_default.animated);
        assert!(!p_default.interactive);
        assert_eq!(p_default.anim_duration, Duration::ZERO);

        let pages: Vec<Element<'_, (), iced::Theme>> =
            vec![text("Page 1").into(), text("Page 2").into()];
        let p = pager(pages)
            .current_page(1)
            .swipe_commit(64.0)
            .touch_slop(12.0)
            .interactive(true)
            .animated(true)
            .anim_duration(Duration::from_millis(300))
            .curve(Curve::EaseOutQuad);

        assert_eq!(p.pages.len(), 2);
        assert_eq!(p.current_page, 1);
        assert_eq!(p.swipe_commit, 64.0);
        assert_eq!(p.touch_slop, 12.0);
        assert!(p.interactive);
        assert!(p.animated);
        assert_eq!(p.anim_duration, Duration::from_millis(300));
        assert_eq!(p.curve, Curve::EaseOutQuad);
    }

    #[test]
    fn pager_default_unanimated_snaps_instantly() {
        let pages: Vec<Element<'_, usize, iced::Theme>> =
            vec![text("Page 1").into(), text("Page 2").into()];
        let p = pager(pages)
            .current_page(0)
            .on_change(|page| page);

        assert!(!p.animated);

        let mut state = State::default();
        let mut messages = Vec::new();
        let mut shell = Shell::new(&mut messages);

        // Release at offset -200px on a 400px wide screen (50% commit threshold)
        p.start_settle(&mut state, 400.0, 0, 2, -200.0, &mut shell);

        // Must NOT start any animation
        assert!(!state.animator.is_animating());
        assert_eq!(state.settle_target_page, Some(1));
        assert_eq!(state.drag_offset, -400.0);
        assert_eq!(messages, vec![1]);
    }

    #[test]
    fn pager_settle_animation_progression() {
        let pages: Vec<Element<'_, usize, iced::Theme>> =
            vec![text("Page 1").into(), text("Page 2").into()];
        let p = pager(pages)
            .current_page(0)
            .animated(true)
            .on_change(|page| page)
            .anim_duration(Duration::from_millis(200))
            .curve(Curve::Linear);

        let mut state = State::default();
        let mut messages = Vec::new();
        let mut shell = Shell::new(&mut messages);
        let t0 = Instant::now();

        // Release at offset -200px on a 400px wide screen (50% commit threshold)
        p.start_settle(&mut state, 400.0, 0, 2, -200.0, &mut shell);

        assert!(state.animator.is_animating());
        assert_eq!(state.settle_target_page, Some(1));
        assert_eq!(state.animator.value(), -200.0);

        // Advance halfway through duration
        let t_half = t0 + Duration::from_millis(100);
        let running = state.animator.update(t_half);
        assert!(running);
        assert!(state.animator.value() < -200.0);

        // Advance past duration
        let t_end = t0 + Duration::from_millis(250);
        let running = state.animator.update(t_end);
        assert!(!running);
        assert_eq!(state.animator.value(), -400.0);
    }

    #[test]
    fn pager_flick_velocity_commits_early() {
        let pages: Vec<Element<'_, usize, iced::Theme>> =
            vec![text("Page 1").into(), text("Page 2").into()];
        let p = pager(pages)
            .current_page(0)
            .animated(true)
            .swipe_commit(200.0)
            .on_change(|page| page);

        let mut state = State {
            velocity_x: -600.0, // Fast flick left
            ..Default::default()
        };
        let mut messages = Vec::new();
        let mut shell = Shell::new(&mut messages);

        // Offset is only -20px (well below 200px threshold), but flick velocity is high
        p.start_settle(&mut state, 400.0, 0, 2, -20.0, &mut shell);

        assert_eq!(state.settle_target_page, Some(1));
        assert!(state.animator.is_animating());
    }

    #[test]
    fn pager_slow_drag_under_threshold_snaps_back() {
        let pages: Vec<Element<'_, usize, iced::Theme>> =
            vec![text("Page 1").into(), text("Page 2").into()];
        let p = pager(pages)
            .current_page(0)
            .animated(true)
            .swipe_commit(200.0)
            .on_change(|page| page);

        let mut state = State {
            velocity_x: -50.0, // Slow drag
            ..Default::default()
        };
        let mut messages = Vec::new();
        let mut shell = Shell::new(&mut messages);

        // Offset is -80px on 400px screen (20% < 45% threshold), slow velocity
        p.start_settle(&mut state, 400.0, 0, 2, -80.0, &mut shell);

        assert_eq!(state.settle_target_page, None);
        assert!(state.animator.is_animating());
        assert_eq!(state.animator.value(), -80.0);
    }

    #[test]
    fn pager_seamless_page_handoff_no_flicker() {
        let pages: Vec<Element<'_, usize, iced::Theme>> =
            vec![text("Page 1").into(), text("Page 2").into()];
        let p = pager(pages)
            .current_page(0)
            .on_change(|page| page)
            .anim_duration(Duration::from_millis(100))
            .curve(Curve::Linear);

        let mut state = State::default();
        let mut messages = Vec::new();
        let mut shell = Shell::new(&mut messages);
        let t0 = Instant::now();

        // Release dragging to next page
        p.start_settle(&mut state, 400.0, 0, 2, -200.0, &mut shell);
        assert_eq!(state.settle_target_page, Some(1));

        // Advance to animation end
        let t_end = t0 + Duration::from_millis(150);
        let _ = state.animator.update(t_end);

        // Simulate RedrawRequested handling when animation completes
        let is_running = state.animator.is_animating();
        assert!(!is_running);

        state.drag_offset = state.animator.value();
        assert_eq!(state.drag_offset, -400.0);
        // Notice: drag_offset is STILL -400.0 (holding target page in view!)
        // It must NOT be reset to 0.0 while current_page is still 0!
        assert_eq!(state.settle_target_page, Some(1));

        // Now parent app receives on_change(1) and rebuilds with current_page = 1:
        if state.settle_target_page == Some(1) {
            // Rebuilt with current_page == target_page:
            let p_next: Pager<'_, usize, iced::Theme> = pager(vec![text("Page 1").into(), text("Page 2").into()])
                .current_page(1);
            if p_next.current_page == state.settle_target_page.unwrap() {
                state.settle_target_page = None;
                state.drag_offset = 0.0;
            }
        }

        assert_eq!(state.settle_target_page, None);
        assert_eq!(state.drag_offset, 0.0);
    }

    #[test]
    fn pager_drag_reversal_is_continuous_and_does_not_pop() {
        use iced::advanced::Widget;

        let mut p = pager(vec![text("Page 1").into(), text("Page 2").into()])
            .current_page(0)
            .touch_slop(18.0)
            .swipe_commit(200.0);

        let mut tree = Tree::new(&p as &dyn Widget<(), iced::Theme, ()>);
        let mut messages = Vec::new();
        let mut shell = Shell::new(&mut messages);
        let bounds = Rectangle::new(Point::ORIGIN, Size::new(480.0, 480.0));
        let layout_node = layout::Node::new(Size::new(480.0, 480.0));
        let layout = Layout::new(&layout_node);
        let mut clipboard = iced::advanced::clipboard::Null;

        // 1. Touch down at x = 100.0
        p.update(
            &mut tree,
            &Event::Touch(touch::Event::FingerPressed {
                id: touch::Finger(0),
                position: Point::new(100.0, 240.0),
            }),
            layout,
            mouse::Cursor::Unavailable,
            &(),
            &mut clipboard,
            &mut shell,
            &bounds,
        );

        let state = tree.state.downcast_ref::<State>();
        assert!(!state.is_dragging);
        assert_eq!(state.drag_offset, 0.0);

        // 2. Drag right past slop (delta_x = +25.0) -> dragged_x should be +7.0, damped to +1.75 on page 0
        p.update(
            &mut tree,
            &Event::Touch(touch::Event::FingerMoved {
                id: touch::Finger(0),
                position: Point::new(125.0, 240.0),
            }),
            layout,
            mouse::Cursor::Unavailable,
            &(),
            &mut clipboard,
            &mut shell,
            &bounds,
        );

        let state = tree.state.downcast_ref::<State>();
        assert!(state.is_dragging);
        assert!((state.drag_offset - 1.75).abs() < 1e-4);

        // 3. Move finger back towards origin to x = 110.0 (delta = +10.0 from down)
        // With previous copysign bug, dragged_x inverted sign to -8.0!
        // Now it must stay on the right (+110 - 118 = -8 in raw, but continuous!)
        // Specifically, as finger moves from 125 -> 120 -> 118 -> 115 -> 100:
        // raw dragged_x moves smoothly from +7 -> +2 -> 0 -> -3 -> -18 without ANY 36px jump!
        let mut prev_offset = state.drag_offset;
        for x in [120.0, 118.0, 115.0, 105.0, 100.0, 95.0, 80.0] {
            p.update(
                &mut tree,
                &Event::Touch(touch::Event::FingerMoved {
                    id: touch::Finger(0),
                    position: Point::new(x, 240.0),
                }),
                layout,
                mouse::Cursor::Unavailable,
                &(),
                &mut clipboard,
                &mut shell,
                &bounds,
            );
            let cur_offset = tree.state.downcast_ref::<State>().drag_offset;
            // Displacement must decrease monotonically as x decreases
            assert!(
                cur_offset <= prev_offset + 1e-5,
                "Offset must decrease continuously when moving left: prev={}, cur={}",
                prev_offset,
                cur_offset
            );
            // Must have NO large discontinuities (jump <= 15px for small steps)
            assert!(
                (cur_offset - prev_offset).abs() < 20.0,
                "Detected jump discontinuity: prev={}, cur={}",
                prev_offset,
                cur_offset
            );
            prev_offset = cur_offset;
        }
    }
