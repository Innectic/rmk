
#[macro_export]

macro_rules! board {
    ($name:expr, $maker:expr, $ver:expr, $desc:expr, $keymap:expr) => {{
    	println!("{:?} {:?} {:?} {:?} {:?}", $name, $maker, $ver, $desc, $keymap);
    }}
}
