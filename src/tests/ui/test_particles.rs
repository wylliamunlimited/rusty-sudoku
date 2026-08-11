use crate::ui::Cloud;

#[cfg(test)]
mod tests {
    use super::*;

    const W: usize = 37;
    const H: usize = 8;

    fn frame(n: u64) -> String {
        Cloud::cube(12).render(n, W, H)
    }

    fn visible(s: &str) -> String {
        s.replace("\x1B[2m", "").replace("\x1B[0m", "")
    }

    #[test]
    fn test_cube_has_a_point_for_every_edge_sample() {
        assert_eq!(Cloud::cube(1).len(), 12);
        assert_eq!(Cloud::cube(12).len(), 12 * 12);
        assert!(!Cloud::cube(1).is_empty());
    }

    #[test]
    fn test_render_has_the_requested_shape() {
        let lines: Vec<String> = visible(&frame(0)).lines().map(String::from).collect();

        assert_eq!(lines.len(), H);
        for line in lines {
            assert_eq!(line.chars().count(), W, "line was {line:?}");
        }
    }

    #[test]
    fn test_render_only_emits_braille_or_blanks() {
        for ch in visible(&frame(7)).chars() {
            assert!(
                ch == ' ' || ch == '\n' || ('\u{2800}'..='\u{28FF}').contains(&ch),
                "unexpected glyph {ch:?}"
            );
        }
    }

    #[test]
    fn test_the_cloud_actually_turns() {
        assert_ne!(frame(0), frame(4));
        assert_ne!(frame(4), frame(9));
    }

    #[test]
    fn test_rotation_is_a_pure_function_of_the_frame() {
        assert_eq!(frame(3), frame(3));
    }

    #[test]
    fn test_something_is_actually_drawn() {
        let drawn = visible(&frame(0))
            .chars()
            .filter(|&c| c != ' ' && c != '\n')
            .count();
        assert!(drawn > 20, "only {drawn} glyphs lit");
    }

    #[test]
    fn test_far_side_is_dimmed() {
        assert!(frame(0).contains("\x1B[2m"));
    }

    #[test]
    fn test_nothing_escapes_the_canvas() {
        for n in 0..60 {
            assert_eq!(visible(&frame(n)).lines().count(), H, "frame {n}");
        }
    }
}
