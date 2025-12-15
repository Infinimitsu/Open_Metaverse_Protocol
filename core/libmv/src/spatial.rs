pub struct Geography;

impl Geography {
    // 100m City Blocks
    pub const BLOCK_SIZE: f32 = 100.0;
    // 20m Road Width (10m from center line)
    pub const ROAD_HALF_WIDTH: f32 = 10.0; 

    pub fn is_public_street(x: f32, z: f32) -> bool {
        // Use Euclidean modulo to handle negative coordinates correctly.
        // Maps any coordinate to range [0.0, 100.0)
        let mod_x = x.rem_euclid(Self::BLOCK_SIZE);
        let mod_z = z.rem_euclid(Self::BLOCK_SIZE);

        // Check X Axis Street
        // A road exists if we are close to 0 (start of block) or 100 (end of block).
        // e.g. [0..10] OR [90..100]
        let on_road_x = mod_x < Self::ROAD_HALF_WIDTH || mod_x > (Self::BLOCK_SIZE - Self::ROAD_HALF_WIDTH);

        // Check Z Axis Street
        let on_road_z = mod_z < Self::ROAD_HALF_WIDTH || mod_z > (Self::BLOCK_SIZE - Self::ROAD_HALF_WIDTH);

        // If we are on either road, it's public territory (The Street)
        on_road_x || on_road_z
    }
}