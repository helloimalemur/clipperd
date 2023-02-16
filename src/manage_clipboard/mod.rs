struct Clipboard {
    selection: i32,
    content: String,
    sekret: String,
    base_dir: String,
}

trait Clipping {
    fn set_clip_board_one() {}
    fn set_clip_board_two() {}
    fn set_clip_board_three() {}
    fn set_clip_board_four() {}
    fn set_clip_board_five() {}
}

impl Clipping for Clipboard {

}
