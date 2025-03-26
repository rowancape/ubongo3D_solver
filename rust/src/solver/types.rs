pub type Grid3D = Vec<Vec<Vec<u8>>>;
pub type BlockVec = Vec<Grid3D>;
pub type BlockCombination = Vec<Grid3D>;
pub type BlockCombinations = Vec<Vec<Grid3D>>;
pub type Coordinate = [usize; 3];
pub type Coordinates = Vec<[usize; 3]>;
pub type CoordinateCombination = Vec<[usize; 3]>;
pub type CoordinateCombinations = Vec<Vec<[usize; 3]>>;


pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
}