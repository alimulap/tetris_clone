pub fn arr_to_vec2d<T: Copy, const N: usize>(arr: [[T; N]; N]) -> Vec<Vec<T>> {
    arr.iter().map(|x| x.to_vec()).collect()
}

pub trait LayoutParse {
    fn parse(&self) -> Vec<Vec<u8>>;
}

impl LayoutParse for [[u8; 4]; 4] {
    fn parse(&self) -> Vec<Vec<u8>> {
        arr_to_vec2d(*self)
    }
}

impl LayoutParse for [[u8; 3]; 3] {
    fn parse(&self) -> Vec<Vec<u8>> {
        arr_to_vec2d(*self)
    }
}

impl LayoutParse for [[u8; 2]; 2] {
    fn parse(&self) -> Vec<Vec<u8>> {
        arr_to_vec2d(*self)
    }
}
