#[test]
fn test_game_end() {
    use crate::field::BlockKind::*;
    use crate::field::Field;
    let field = [[Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Black, Empty, Empty, Empty, Empty, Empty, Empty],
        [White, Black, Empty, Empty, Empty, Empty, Empty],
        [White, White, Black, Empty, Empty, Empty, Empty],
        [White, White, White, Black, Empty, Empty, Empty]
    ];
    let mut field = Field::from_array(field);
    let actual = field.game_end_process();
    let expect = true;
    println!("{}", field);
    println!("{}", actual);
    assert_eq!(actual, expect);
}
#[test]
fn test_draw() {
    use crate::field::BlockKind::*;
    use crate::field::Field;
    let field = [[Black, Black, White, Black, White, Black, Black],
        [Black, White, White, Black, White, White, Black],
        [White, Black, Black, White, Black, Black, White],
        [Black, White, White, Black, White, White, Black],
        [Black, Black, White, Black, White, Black, Black],
        [Black, White, White, Black, White, White, Black]
    ];
    let mut field = Field::from_array(field);
    let actual = field.check_draw();
    let expect = true;
    println!("{}", field);
    println!("{}", actual);
    assert_eq!(actual, expect);
}

